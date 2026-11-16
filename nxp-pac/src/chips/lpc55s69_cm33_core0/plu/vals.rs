#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterClksel {
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    Fro1mhz = 0x0,
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    Fro12mhz = 0x01,
    #[doc = "Selects a third filter clock source, if provided."]
    OtherClock = 0x02,
    _RESERVED_3 = 0x03,
}
impl FilterClksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterClksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterClksel {
    #[inline(always)]
    fn from(val: u8) -> FilterClksel {
        FilterClksel::from_bits(val)
    }
}
impl From<FilterClksel> for u8 {
    #[inline(always)]
    fn from(val: FilterClksel) -> u8 {
        FilterClksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterMode {
    #[doc = "Bypass mode."]
    Bypass = 0x0,
    #[doc = "Filter 1 clock period."]
    Filter1clk = 0x01,
    #[doc = "Filter 2 clock period."]
    Filter2clk = 0x02,
    #[doc = "Filter 3 clock period."]
    Filter3clk = 0x03,
}
impl FilterMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterMode {
    #[inline(always)]
    fn from(val: u8) -> FilterMode {
        FilterMode::from_bits(val)
    }
}
impl From<FilterMode> for u8 {
    #[inline(always)]
    fn from(val: FilterMode) -> u8 {
        FilterMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LuTnInPx {
    #[doc = "The PLU primary inputs 0."]
    PluInputs0 = 0x0,
    #[doc = "The PLU primary inputs 1."]
    PluInputs1 = 0x01,
    #[doc = "The PLU primary inputs 2."]
    PluInputs2 = 0x02,
    #[doc = "The PLU primary inputs 3."]
    PluInputs3 = 0x03,
    #[doc = "The PLU primary inputs 4."]
    PluInputs4 = 0x04,
    #[doc = "The PLU primary inputs 5."]
    PluInputs5 = 0x05,
    #[doc = "The output of LUT0."]
    LutOutputs0 = 0x06,
    #[doc = "The output of LUT1."]
    LutOutputs1 = 0x07,
    #[doc = "The output of LUT2."]
    LutOutputs2 = 0x08,
    #[doc = "The output of LUT3."]
    LutOutputs3 = 0x09,
    #[doc = "The output of LUT4."]
    LutOutputs4 = 0x0a,
    #[doc = "The output of LUT5."]
    LutOutputs5 = 0x0b,
    #[doc = "The output of LUT6."]
    LutOutputs6 = 0x0c,
    #[doc = "The output of LUT7."]
    LutOutputs7 = 0x0d,
    #[doc = "The output of LUT8."]
    LutOutputs8 = 0x0e,
    #[doc = "The output of LUT9."]
    LutOutputs9 = 0x0f,
    #[doc = "The output of LUT10."]
    LutOutputs10 = 0x10,
    #[doc = "The output of LUT11."]
    LutOutputs11 = 0x11,
    #[doc = "The output of LUT12."]
    LutOutputs12 = 0x12,
    #[doc = "The output of LUT13."]
    LutOutputs13 = 0x13,
    #[doc = "The output of LUT14."]
    LutOutputs14 = 0x14,
    #[doc = "The output of LUT15."]
    LutOutputs15 = 0x15,
    #[doc = "The output of LUT16."]
    LutOutputs16 = 0x16,
    #[doc = "The output of LUT17."]
    LutOutputs17 = 0x17,
    #[doc = "The output of LUT18."]
    LutOutputs18 = 0x18,
    #[doc = "The output of LUT19."]
    LutOutputs19 = 0x19,
    #[doc = "The output of LUT20."]
    LutOutputs20 = 0x1a,
    #[doc = "The output of LUT21."]
    LutOutputs21 = 0x1b,
    #[doc = "The output of LUT22."]
    LutOutputs22 = 0x1c,
    #[doc = "The output of LUT23."]
    LutOutputs23 = 0x1d,
    #[doc = "The output of LUT24."]
    LutOutputs24 = 0x1e,
    #[doc = "The output of LUT25."]
    LutOutputs25 = 0x1f,
    #[doc = "state(0)."]
    State0 = 0x20,
    #[doc = "state(1)."]
    State1 = 0x21,
    #[doc = "state(2)."]
    State2 = 0x22,
    #[doc = "state(3)."]
    State3 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl LuTnInPx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LuTnInPx {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LuTnInPx {
    #[inline(always)]
    fn from(val: u8) -> LuTnInPx {
        LuTnInPx::from_bits(val)
    }
}
impl From<LuTnInPx> for u8 {
    #[inline(always)]
    fn from(val: LuTnInPx) -> u8 {
        LuTnInPx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutpuTn {
    #[doc = "The PLU output 0."]
    PluOutput0 = 0x0,
    #[doc = "The PLU output 1."]
    PluOutput1 = 0x01,
    #[doc = "The PLU output 2."]
    PluOutput2 = 0x02,
    #[doc = "The PLU output 3."]
    PluOutput3 = 0x03,
    #[doc = "The PLU output 4."]
    PluOutput4 = 0x04,
    #[doc = "The PLU output 5."]
    PluOutput5 = 0x05,
    #[doc = "The PLU output 6."]
    PluOutput6 = 0x06,
    #[doc = "The PLU output 7."]
    PluOutput7 = 0x07,
    #[doc = "The PLU output 8."]
    PluOutput8 = 0x08,
    #[doc = "The PLU output 9."]
    PluOutput9 = 0x09,
    #[doc = "The PLU output 10."]
    PluOutput10 = 0x0a,
    #[doc = "The PLU output 11."]
    PluOutput11 = 0x0b,
    #[doc = "The PLU output 12."]
    PluOutput12 = 0x0c,
    #[doc = "The PLU output 13."]
    PluOutput13 = 0x0d,
    #[doc = "The PLU output 14."]
    PluOutput14 = 0x0e,
    #[doc = "The PLU output 15."]
    PluOutput15 = 0x0f,
    #[doc = "The PLU output 16."]
    PluOutput16 = 0x10,
    #[doc = "The PLU output 17."]
    PluOutput17 = 0x11,
    #[doc = "The PLU output 18."]
    PluOutput18 = 0x12,
    #[doc = "The PLU output 19."]
    PluOutput19 = 0x13,
    #[doc = "The PLU output 20."]
    PluOutput20 = 0x14,
    #[doc = "The PLU output 21."]
    PluOutput21 = 0x15,
    #[doc = "The PLU output 22."]
    PluOutput22 = 0x16,
    #[doc = "The PLU output 23."]
    PluOutput23 = 0x17,
    #[doc = "The PLU output 24."]
    PluOutput24 = 0x18,
    #[doc = "The PLU output 25."]
    PluOutput25 = 0x19,
    #[doc = "state(0)."]
    State0 = 0x1a,
    #[doc = "state(1)."]
    State1 = 0x1b,
    #[doc = "state(2)."]
    State2 = 0x1c,
    #[doc = "state(3)."]
    State3 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OutpuTn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutpuTn {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutpuTn {
    #[inline(always)]
    fn from(val: u8) -> OutpuTn {
        OutpuTn::from_bits(val)
    }
}
impl From<OutpuTn> for u8 {
    #[inline(always)]
    fn from(val: OutpuTn) -> u8 {
        OutpuTn::to_bits(val)
    }
}
