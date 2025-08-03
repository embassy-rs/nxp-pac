use std::{collections::BTreeMap, fmt::Write, fs, path::Path};

use anyhow::Context;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::{iomuxc::IomuxcRegisters, rustfmt};

#[derive(Debug)]
pub struct Peripheral {
    /// Peripheral name.
    pub name: String,

    /// Pin options for this peripheral.
    pub pins: Vec<PeripheralPin>,
}

#[derive(Debug)]
pub struct PeripheralPin {
    /// The pin selection.
    pub pin: String,

    /// The signal.
    pub signal: String,

    /// The pin function to select this pin.
    pub function: u8,

    /// The daisy register to select the signal.
    ///
    /// This is optional (but will be required depending on the peripheral) for IMXRT1xxx parts.
    /// This must always be [`None`] if this is not an RT1xxx part.
    pub iomuxc_daisy: Option<Daisy>,
}

#[derive(Debug)]
/// The daisy chain setting for a peripheral and pin.
pub struct Daisy {
    /// The daisy register address.
    pub register: u32,

    /// The value for the daisy register.
    pub value: u8,
}

pub fn generate_core(
    metadata_dir: &Path,
    core: &str,
    peripherals: &BTreeMap<String, Peripheral>,
    iomuxc: &BTreeMap<String, IomuxcRegisters>,
) -> anyhow::Result<()> {
    // IMXRT10xx and 11xx require extra metadata for IOMUXC
    let has_iomuxc = core.starts_with("MIMXRT1");

    let mut metadata = String::new();

    writeln!(
        metadata,
        "#![doc = \"This file is auto generated. Do not modify directly.\"]"
    )?;
    writeln!(metadata, "use crate::metadata::*;")?;

    if has_iomuxc {
        writeln!(metadata, "{}", generate_iomuxc(&iomuxc)?)?;
    }

    writeln!(
        metadata,
        "{}",
        generate_peripherals(has_iomuxc, &peripherals)?
    )?;

    fs::create_dir_all(metadata_dir)?;

    let metadata_rs = metadata_dir.join(format!("{}.rs", core.to_lowercase()));
    fs::write(&metadata_rs, metadata)?;
    rustfmt(&metadata_rs).context(format!("Format metadata for {}", core))?;

    Ok(())
}

fn generate_peripherals(
    has_iomuxc: bool,
    peripherals: &BTreeMap<String, Peripheral>,
) -> anyhow::Result<TokenStream> {
    let peripherals = peripherals.values().map(|peripheral| {
        let name = &peripheral.name;
        let pins = peripheral.pins.iter().map(|pin| {
            let function = Literal::u8_unsuffixed(pin.function);
            let signal = &pin.signal;

            let daisy = pin
                .iomuxc_daisy
                .as_ref()
                .map(|daisy| {
                    let register = Literal::u32_unsuffixed(daisy.register);
                    let value = Literal::u8_unsuffixed(daisy.value);

                    quote! {
                        Some(Daisy {
                            register: #register,
                            value: #value,
                        })
                    }
                })
                .take_if(|_| has_iomuxc)
                .or_else(|| Some(quote! { None }));

            let pin = &pin.pin;

            if let Some(daisy) = daisy {
                quote! {
                    PeripheralPin {
                        pin: #pin,
                        signal: #signal,
                        function: #function,
                        iomuxc_daisy: #daisy,
                    }
                }
            } else {
                quote! {
                    PeripheralPin {
                        pin: #pin,
                        signal: #signal,
                        function: #function,
                    }
                }
            }
        });

        quote! {
            Peripheral {
                name: #name,
                pins: &[#(#pins),*],
            }
        }
    });

    Ok(quote! {
        pub static PERIPHERALS: &[Peripheral] = &[
            #(#peripherals),*
        ];
    })
}

fn generate_iomuxc(registers: &BTreeMap<String, IomuxcRegisters>) -> anyhow::Result<TokenStream> {
    let registers = registers.values().map(|registers| {
        let name = &registers.name;
        let mux_ctl = Literal::u32_unsuffixed(registers.mux_address);
        let pad_ctl = Literal::u32_unsuffixed(registers.pad_address);

        quote! {
            IomuxcRegisters { name: #name, mux_ctl: #mux_ctl, pad_ctl: #pad_ctl }
        }
    });

    Ok(quote! {
        pub const IOMUXC_REGISTERS: &[IomuxcRegisters] = &[
            #(
                #registers
            ),*
        ];
    })
}
