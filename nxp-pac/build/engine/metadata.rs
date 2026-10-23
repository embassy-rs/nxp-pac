use std::{fs, path::Path};

use anyhow::{Context, ensure};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;

use super::super::chips::Chip;

#[derive(Clone, Deserialize)]
pub struct Metadata {
    pub chips: Vec<String>,
    pub pins: Vec<Pin>,
    pub nvic_prio_bits: u32,
    pub interrupts: IndexMap<String, u32>,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Clone, Deserialize)]
pub struct Pin {
    pub name: String,
    pub iomuxc: Option<PinIomuxc>,
    pub feature: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct PinIomuxc {
    pub mux: Option<String>,
    pub pad: String,
}

#[derive(Clone, Deserialize)]
pub struct Peripheral {
    pub name: String,
    #[serde(rename = "block")]
    pub peripheral_block: Option<String>,
    #[serde(rename = "module")]
    pub rust_module_name: Option<String>,
    #[serde(rename = "address")]
    pub peripheral_address: Option<String>,
    pub signals: Vec<Signal>,
    pub flexcomm: Option<String>,
    #[serde(default)]
    pub dma_muxing: Vec<DmaMux>,
    pub only_in: Option<String>,
    #[serde(default)]
    pub pac_only: bool,
    pub gate: Option<Gate>,
}

pub struct BlockPath {
    pub path: String,
    pub rust_mod_name: String,
    pub rust_type_name: String,
}

impl Peripheral {
    pub fn parse_block_path(&self) -> anyhow::Result<Option<BlockPath>> {
        let Some(block) = self.peripheral_block.as_deref() else {
            return Ok(None);
        };
        let (path, ty) = block
            .split_once("::")
            .map_or((block, None), |(p, t)| (p, Some(t)));
        super::asset_path(path)?;
        let original = path.rsplit('/').next().context("empty block path")?;
        Ok(Some(BlockPath {
            path: path.to_owned(),
            rust_mod_name: self
                .rust_module_name
                .as_deref()
                .unwrap_or(original)
                .to_lowercase(),
            rust_type_name: inflections::Inflect::to_pascal_case(ty.unwrap_or(original)),
        }))
    }
}

#[derive(Clone, Deserialize)]
pub struct Signal {
    pub name: String,
    pub pins: Vec<SignalPin>,
    pub iomuxc_daisy: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct SignalPin {
    pub pin: String,
    pub alt: u8,
    pub iomuxc_daisy: Option<u8>,
}

#[derive(Clone, Deserialize)]
pub struct DmaMux {
    pub signal: String,
    pub mux: String,
    pub request: u8,
}

#[derive(Clone, Deserialize)]
pub struct Gate {
    pub enable: String,
    pub reset: Option<String>,
    pub config: Option<String>,
    pub bit: Option<String>,
}

pub fn load(inputs: &Path, chip: &Chip) -> anyhow::Result<(Metadata, Vec<std::path::PathBuf>)> {
    let path = inputs
        .join("metadata")
        .join(chip.metadata)
        .with_extension("json");
    let mut files = vec![path.clone()];
    let mut source: serde_json::Value = serde_json::from_slice(
        &fs::read(&path).with_context(|| format!("reading {}", path.display()))?,
    )?;
    let overlay = source
        .as_object_mut()
        .context("metadata must be an object")?;
    let included = if let Some(include) = overlay.remove("include") {
        let include = include
            .as_str()
            .context("metadata include must be a string")?;
        ensure!(
            !include.is_empty()
                && include
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-'),
            "invalid metadata include: {include}"
        );
        let path = inputs.join("metadata").join(include).with_extension("json");
        let mut base: serde_json::Value = serde_json::from_slice(
            &fs::read(&path).with_context(|| format!("reading {}", path.display()))?,
        )?;
        files.push(path);
        let base = base
            .as_object_mut()
            .context("included metadata must be an object")?;
        ensure!(
            !base.contains_key("include"),
            "nested metadata includes are not supported"
        );
        base.extend(std::mem::take(overlay));
        *overlay = std::mem::take(base);
        true
    } else {
        false
    };
    let mut metadata: Metadata = serde_json::from_value(source)?;
    if included {
        ensure!(
            metadata
                .chips
                .iter()
                .any(|name| name.eq_ignore_ascii_case(chip.feature)),
            "metadata does not describe {}",
            chip.feature
        );
    }
    let mut selected = Vec::new();
    for peripheral in metadata.peripherals {
        let keep = match &peripheral.only_in {
            Some(pattern) => regex::Regex::new(pattern)
                .with_context(|| format!("invalid only_in for {}", peripheral.name))?
                .is_match(chip.metadata),
            None => true,
        };
        if keep {
            selected.push(peripheral);
        }
    }
    metadata.peripherals = selected;
    Ok((metadata, files))
}

fn some<T: quote::ToTokens>(value: Option<T>) -> TokenStream {
    match value {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}

fn address(value: &str) -> anyhow::Result<u32> {
    u32::from_str_radix(
        value
            .strip_prefix("0x")
            .context("address requires 0x prefix")?
            .replace('_', "")
            .as_str(),
        16,
    )
    .with_context(|| format!("invalid address: {value}"))
}

pub fn render(chip: &str, metadata: &Metadata) -> anyhow::Result<TokenStream> {
    let pins = metadata
        .pins
        .iter()
        .map(|pin| {
            let name = &pin.name;
            let feature = some(pin.feature.as_deref());
            let iomuxc = match &pin.iomuxc {
                Some(iomuxc) => {
                    let pad = address(&iomuxc.pad)?;
                    let mux = some(iomuxc.mux.as_deref().map(address).transpose()?);
                    quote!(Some(PinIomuxc { mux: #mux, pad: #pad }))
                }
                None => quote!(None),
            };
            Ok(quote!(Pin { name: #name, iomuxc: #iomuxc, feature: #feature }))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let peripherals = metadata.peripherals.iter().map(|peripheral| {
        let name = &peripheral.name;
        let addr = peripheral.peripheral_address.as_deref().map(address).transpose()?.unwrap_or(0);
        let driver = if peripheral.pac_only { "" } else { peripheral.peripheral_block.as_deref().unwrap_or_default() };
        let flexcomm = some(peripheral.flexcomm.as_deref());
        let signals = peripheral.signals.iter().map(|signal| {
            let name = &signal.name;
            let daisy = some(signal.iomuxc_daisy.as_deref().map(address).transpose()?);
            let pins = signal.pins.iter().map(|pin| {
                let name = &pin.pin;
                let alt = pin.alt;
                let daisy = some(pin.iomuxc_daisy);
                quote!(SignalPin { pin: #name, alt: #alt, iomuxc_daisy: #daisy })
            });
            Ok(quote!(Signal { name: #name, pins: &[#(#pins),*], iomuxc_daisy: #daisy }))
        }).collect::<anyhow::Result<Vec<_>>>()?;
        let dma = peripheral.dma_muxing.iter().map(|dma| {
            let signal = &dma.signal;
            let mux = &dma.mux;
            let request = dma.request;
            quote!(DmaMux { signal: #signal, mux: #mux, request: #request })
        });
        let gate = match &peripheral.gate {
            Some(gate) => {
                let enable = &gate.enable;
                let reset = some(gate.reset.as_deref());
                let config = some(gate.config.as_deref());
                let bit = gate.bit.clone().unwrap_or_else(|| name.to_lowercase());
                quote!(Some(Gate { enable: #enable, reset: #reset, config: #config, bit: #bit }))
            }
            None => quote!(None),
        };
        Ok(quote!(Peripheral { name: #name, address: #addr, driver_name: #driver, signals: &[#(#signals),*], flexcomm: #flexcomm, dma_muxing: &[#(#dma),*], gate: #gate }))
    }).collect::<anyhow::Result<Vec<_>>>()?;
    let interrupts = metadata
        .interrupts
        .iter()
        .map(|(name, val)| quote!((#name, #val)));
    Ok(quote! {
        use crate::metadata::*;
        pub const METADATA: Metadata = Metadata { name: #chip, pins: PINS, peripherals: PERIPHERALS, interrupts: INTERRUPTS };
        pub const PINS: &[Pin] = &[#(#pins),*];
        pub const PERIPHERALS: &[Peripheral] = &[#(#peripherals),*];
        pub const INTERRUPTS: &[(&str, u32)] = &[#(#interrupts),*];
    })
}
