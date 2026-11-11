#[cfg(nxp_metapac)]
mod _generated {
    include!(concat!(env!("OUT_DIR"), "/metadata.rs"));
}

#[cfg(nxp_metapac)]
mod _extensions {
    include!(concat!(env!("OUT_DIR"), "/metadata_ext.rs"));
}

#[cfg(not(nxp_metapac))]
mod _extensions {
    use super::{DirectSignal, NamedGate, ResetGate};

    pub const DIRECT_SIGNALS: &[DirectSignal] = &[];
    pub const RESET_GATES: &[ResetGate] = &[];
    pub const NAMED_GATES: &[NamedGate] = &[];
}

#[cfg(not(nxp_metapac))]
#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/metadata.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/metadata.rs")]
#[cfg_attr(feature = "mimxrt1064", path = "./chips/mimxrt1064/metadata.rs")]
#[cfg_attr(
    feature = "mimxrt685s_cm33",
    path = "./chips/mimxrt685s_cm33/metadata.rs"
)]
#[cfg_attr(feature = "lpc55s16", path = "./chips/lpc55s16/metadata.rs")]
#[cfg_attr(
    feature = "lpc55s69_cm33_core0",
    path = "./chips/lpc55s69_cm33_core0/metadata.rs"
)]
#[cfg_attr(
    feature = "lpc55s69_cm33_core1",
    path = "./chips/lpc55s69_cm33_core1/metadata.rs"
)]
mod _generated;

mod meta_peripherals {
    include!(concat!(env!("OUT_DIR"), "/meta_peripherals.rs"));
}

#[derive(Debug)]
pub struct Metadata {
    pub name: &'static str,
    pub pins: &'static [Pin],
    pub peripherals: &'static [Peripheral],
    pub interrupts: &'static [(&'static str, u32)],
}

#[derive(Debug)]
pub struct Pin {
    pub name: &'static str,
    pub iomuxc: Option<PinIomuxc>,
    pub feature: Option<&'static str>,
}

#[derive(Debug)]
pub struct PinIomuxc {
    pub mux: Option<u32>,
    pub pad: u32,
}

#[derive(Debug)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u32,
    pub driver_name: &'static str,
    pub signals: &'static [Signal],
    pub flexcomm: Option<&'static str>,
    pub dma_muxing: &'static [DmaMux],
    pub gate: Option<Gate>,
}

impl Peripheral {
    /// Non-PORT-mux connectivity for this peripheral on the selected chip.
    pub fn direct_signals(&self) -> impl Iterator<Item = &'static DirectSignal> + '_ {
        DIRECT_SIGNALS
            .iter()
            .filter(move |signal| signal.peripheral == self.name)
    }

    /// Named controls for this peripheral, without an implied initialization sequence.
    pub fn named_gates(&self) -> impl Iterator<Item = &'static NamedGate> + '_ {
        NAMED_GATES
            .iter()
            .filter(move |gate| gate.peripheral == self.name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectSignalKind {
    Analog,
    DirectInput,
}

#[derive(Debug)]
pub struct DirectSignalPin {
    pub pin: &'static str,
    pub kind: DirectSignalKind,
}

/// A peripheral connection, not a pin configuration recipe or digital ALT route.
#[derive(Debug)]
pub struct DirectSignal {
    pub peripheral: &'static str,
    pub name: &'static str,
    pub pins: &'static [DirectSignalPin],
}

#[derive(Debug)]
pub struct Signal {
    pub name: &'static str,
    pub pins: &'static [SignalPin],
    pub iomuxc_daisy: Option<u32>,
}

#[derive(Debug)]
pub struct SignalPin {
    pub pin: &'static str,
    pub alt: u8,
    pub iomuxc_daisy: Option<u8>,
}

#[derive(Debug)]
pub struct DmaMux {
    pub signal: &'static str,
    pub mux: &'static str,
    pub request: u8,
}

#[derive(Debug)]
pub struct Gate {
    pub enable: &'static str,
    pub reset: Option<&'static str>,
    pub config: Option<&'static str>,
    pub bit: &'static str,
}

impl Gate {
    /// Reset field for `self.reset`, falling back to the clock field name.
    /// This lookup does not access hardware and is only meaningful with a reset register.
    pub fn reset_bit(&self) -> &'static str {
        RESET_GATES
            .iter()
            .find(|gate| {
                gate.enable == self.enable && gate.bit == self.bit && Some(gate.reset) == self.reset
            })
            .map_or(self.bit, |gate| gate.reset_bit)
    }
}

#[derive(Debug)]
pub struct ResetGate {
    pub enable: &'static str,
    pub bit: &'static str,
    pub reset: &'static str,
    pub reset_bit: &'static str,
}

#[derive(Debug)]
pub struct NamedGate {
    pub peripheral: &'static str,
    pub name: &'static str,
    pub gate: Gate,
}

pub use _extensions::{DIRECT_SIGNALS, NAMED_GATES, RESET_GATES};
pub use _generated::*;
pub use meta_peripherals::META_PERIPHERALS;
