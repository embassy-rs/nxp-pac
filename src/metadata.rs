#[derive(Debug)]
pub struct Peripheral {
    /// Name of this peripheral.
    pub name: &'static str,

    /// Pins that can provide signals from this peripheral.
    pub pins: &'static [PeripheralPin],
}

/// A peripheral pin.
#[derive(Debug)]
pub struct PeripheralPin {
    /// The name of the physical pin this peripheral is available at.
    pub pin: &'static str,

    /// The signal provided by this pin.
    pub signal: &'static str,

    /// The number to select this function on the physical pin.
    pub function: u8,

    /// For i.MXRT parts with IOMUXC, the additional daisy register used to select this signal.
    ///
    /// If this is [`None`], then no daisy register needs to be set.
    pub iomuxc_daisy: Option<Daisy>,
}

/// IOMUXC daisy register for a signal.
#[derive(Debug)]
pub struct Daisy {
    /// The daisy register address.
    pub register: u32,

    /// The value to write to select the signal.
    pub value: u8,
}

/// IOMUXC registers for a pin.
pub struct IomuxcRegisters {
    /// The physical pin.
    pub name: &'static str,

    /// Address of mux control register for this pin.
    pub mux_ctl: u32,

    /// Address of pad control register for this pin.
    pub pad_ctl: u32,
}

#[cfg_attr(feature = "mimxrt1011", path = "./metadata/mimxrt1011.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./metadata/mimxrt1062.rs")]
#[cfg_attr(
    feature = "mcxn947_cm33_core0",
    path = "./metadata/mcxn947_cm33_core0.rs"
)]
#[cfg_attr(
    feature = "mcxn947_cm33_core1",
    path = "./metadata/mcxn947_cm33_core1.rs"
)]
mod _generated;

pub use _generated::*;
