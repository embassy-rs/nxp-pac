use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    fs,
    path::Path,
};

use anyhow::{Context, bail};
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use serde::Deserialize;

use crate::rustfmt;

#[derive(Debug, Clone, Deserialize)]
pub struct PinMetadata {
    pub chips: Vec<String>,
    pub pins: Vec<Pin>,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pin {
    pub name: String,

    /// Supply for this pin.
    ///
    /// An example of when this is [`None`] is supply for a VREF pin (the pin is itself a supply).
    pub supply: Option<String>,

    /// IOMUXC information for this pin. Only applicable on RT1xxx chips.
    pub iomuxc: Option<PinIomuxc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinIomuxc {
    /// Some pins only have a mux, thereby not being usable as GPIO.
    pub mux: Option<String>,

    /// Pins that are usable by IOMUXC require a pad register.
    pub pad: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Peripheral {
    pub name: String,
    pub signals: Vec<Signal>,
    pub flexcomm: Option<String>,
    #[serde(default)]
    pub dma_muxing: Vec<DmaMux>,
    pub only_in: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Signal {
    pub name: String,
    pub pins: Vec<SignalPin>,

    /// IOMUXC daisy register used for this signal.
    ///
    /// Depending on the peripheral type and instance, this may some be [`None`] even for a
    /// peripheral which usually has a daisy register.
    ///
    /// If this is [`Some`], each pin's [`Signal::iomuxc_daisy`] value must be [`Some`].
    pub iomuxc_daisy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignalPin {
    pub pin: String,
    pub alt: u8,

    /// IOMUXC daisy value to write into the daisy register of the parent [`Signal`].
    ///
    /// This is required if [`Signal::iomuxc_daisy`] is [`Some`]
    pub iomuxc_daisy: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DmaMux {
    pub signal: String,
    pub mux: String,
    pub request: u8,
}

/// Validate that common errors, such as conflicting alt modes and daisy registers are not duplicated.
fn validate(metadata: &PinMetadata) -> anyhow::Result<()> {
    if metadata.chips.is_empty() {
        bail!("No chips");
    }

    let mut pins = HashSet::<&str>::with_capacity(metadata.pins.len());
    let mut daisy_registers = HashSet::<&str>::new();

    for pin in metadata.pins.iter() {
        if !pins.insert(&pin.name) {
            bail!("Duplicate pin: {}", pin.name);
        }
    }

    let mut peripherals = HashSet::<&str>::with_capacity(metadata.peripherals.len());
    let mut pin_alts = HashMap::<&str, HashSet<u8>>::new();

    for peripheral in metadata.peripherals.iter() {
        if !peripherals.insert(&peripheral.name) {
            bail!("Duplicate peripheral: {}", peripheral.name);
        }

        let mut signals = HashSet::<&str>::with_capacity(peripheral.signals.len());

        for signal in peripheral.signals.iter() {
            if !signals.insert(&signal.name) {
                bail!(
                    "Duplicate signal for peripheral {}: {}",
                    peripheral.name,
                    signal.name
                );
            }

            if let Some(ref daisy) = signal.iomuxc_daisy {
                if !daisy_registers.insert(daisy) {
                    bail!(
                        "Multiple signals have same daisy value: {} (peripheral is {})",
                        daisy,
                        peripheral.name,
                    );
                }
            }

            let mut used_daisy_values = HashSet::<u8>::with_capacity(signal.pins.len());
            let mut used_pins = HashSet::<&str>::with_capacity(signal.pins.len());

            for pin in signal.pins.iter() {
                if !pins.contains(pin.pin.as_str()) {
                    bail!(
                        "For peripheral {}, signal {}, pin {} is not defined in pins",
                        peripheral.name,
                        signal.name,
                        pin.pin
                    );
                }

                if !used_pins.insert(&pin.pin) {
                    bail!(
                        "For peripheral {}, signal {}, pin {} is defined more than once",
                        peripheral.name,
                        signal.name,
                        pin.pin
                    );
                }

                if !pin_alts.entry(&pin.pin).or_default().insert(pin.alt) {
                    bail!(
                        "pin {} has more than one signal defined for alt mode {}",
                        pin.pin,
                        pin.alt
                    );
                }

                let pin_iomuxc = pin.iomuxc_daisy.is_some();
                let signal_iomuxc = signal.iomuxc_daisy.is_some();
                // None or both must be defined.
                let incomplete = (pin_iomuxc && !signal_iomuxc) || (!pin_iomuxc && signal_iomuxc);

                if incomplete {
                    bail!(
                        "For peripheral {}, signal {}, an IOMUXC daisy register is defined, but pin {} has no daisy value",
                        peripheral.name,
                        signal.name,
                        pin.pin
                    );
                }

                if let Some(daisy) = pin.iomuxc_daisy {
                    if !used_daisy_values.insert(daisy) {
                        bail!(
                            "For peripheral {}, signal {}, a duplicate IOMUXC daisy value of {} is used.",
                            peripheral.name,
                            signal.name,
                            daisy
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn generate_metadata(name: &str, interrupts: &[String], metadata: &PinMetadata) -> TokenStream {
    let pins = metadata.pins.iter().map(|pin| {
        let name = &pin.name;
        let iomuxc = pin
            .iomuxc
            .as_ref()
            .map(|iomuxc| {
                let pad = u32::from_str_radix(&iomuxc.pad[2..], 16).unwrap();

                let mux = iomuxc
                    .mux
                    .as_ref()
                    .map(|mux| {
                        let mux = u32::from_str_radix(&mux[2..], 16).unwrap();
                        quote! { Some(#mux) }
                    })
                    .unwrap_or_else(|| quote! { None });

                quote! {
                    Some(PinIomuxc {
                        mux: #mux,
                        pad: #pad,
                    })
                }
            })
            .unwrap_or_else(|| quote! { None });

        quote! {
            Pin {
                name: #name,
                iomuxc: #iomuxc,
            }
        }
    });

    let peripherals = metadata.peripherals.iter().map(|peripheral| {
        let name = &peripheral.name;
        let flexcomm = peripheral
            .flexcomm
            .as_ref()
            .map(|ref fc| quote! { Some(#fc) })
            .unwrap_or_else(|| quote! { None });

        let signals = peripheral.signals.iter().map(|signal| {
            let name = &signal.name;

            let iomuxc_daisy = signal
                .iomuxc_daisy
                .as_ref()
                .map(|iomuxc| {
                    let daisy = u32::from_str_radix(&iomuxc[2..], 16).unwrap();

                    quote! {
                        Some(#daisy)
                    }
                })
                .unwrap_or_else(|| quote! { None });

            let pins = signal.pins.iter().map(|signal| {
                let pin = &signal.pin;
                let alt = signal.alt;
                let iomuxc_daisy = signal
                    .iomuxc_daisy
                    .as_ref()
                    .map(|daisy| quote! { Some(#daisy) })
                    .unwrap_or_else(|| quote! { None });

                quote! {
                    SignalPin {
                        pin: #pin,
                        alt: #alt,
                        iomuxc_daisy: #iomuxc_daisy,
                    }
                }
            });

            quote! {
                Signal {
                    name: #name,
                    pins: &[#(#pins),*],
                    iomuxc_daisy: #iomuxc_daisy,
                }
            }
        });

        let dma_muxing = peripheral.dma_muxing.iter().map(|dma_mux| {
            let signal = &dma_mux.signal;
            let mux = &dma_mux.mux;
            let request = Literal::u8_unsuffixed(dma_mux.request);

            quote! {
                DmaMux {
                    signal: #signal,
                    mux: #mux,
                    request: #request,
                }
            }
        });

        quote! {
            Peripheral {
                name: #name,
                signals: &[#(#signals),*],
                flexcomm: #flexcomm,
                dma_muxing: &[#(#dma_muxing),*],
            }
        }
    });

    quote! {
        use crate::metadata::*;

        pub const METADATA: Metadata = Metadata {
            name: #name,
            pins: PINS,
            peripherals: PERIPHERALS,
            interrupts: INTERRUPTS,
        };

        pub const PINS: &[Pin] = &[#(#pins),*];
        pub const PERIPHERALS: &[Peripheral] = &[#(#peripherals),*];
        pub const INTERRUPTS: &[&str] = &[#(#interrupts),*];
    }
}

pub fn generate_core(
    chips_dir: &Path,
    svd: &Path,
    metadata: &Path,
    core: &str,
) -> anyhow::Result<()> {
    let metadata = fs::read_to_string(metadata).context("Read metadata")?;
    let metadata =
        serde_json::from_str::<PinMetadata>(&metadata).context("Deserialize metadata")?;
    validate(&metadata)?;

    let svd_contents = fs::read_to_string(svd).context("Read SVD")?;
    let svd = svd_parser::parse(&svd_contents).context("Parse SVD")?;

    let mut interrupts = Vec::new();

    for peripheral in svd.peripherals.iter() {
        for interrupt in peripheral.interrupt.iter() {
            // Rust uses fully capitalized interrupt names for singletons.
            interrupts.push(interrupt.name.clone().to_uppercase());
        }
    }

    // LPC55S6x has duplicate FLEXCOMM entries. dedup requires sorting to work.
    interrupts.sort();
    interrupts.dedup();

    let mut metadata_out = String::new();
    write!(
        metadata_out,
        "{}",
        generate_metadata(core, &interrupts, &metadata)
    )?;

    let metadata_rs = chips_dir.join(core.to_lowercase()).join("metadata.rs");
    fs::write(&metadata_rs, metadata_out)?;
    rustfmt(&metadata_rs)?;

    Ok(())
}
