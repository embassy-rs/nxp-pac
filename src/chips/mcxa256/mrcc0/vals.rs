#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdcClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccAdcClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdcClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdcClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccAdcClkdivHalt {
        MrccAdcClkdivHalt::from_bits(val)
    }
}
impl From<MrccAdcClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccAdcClkdivHalt) -> u8 {
        MrccAdcClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdcClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccAdcClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdcClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdcClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccAdcClkdivReset {
        MrccAdcClkdivReset::from_bits(val)
    }
}
impl From<MrccAdcClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccAdcClkdivReset) -> u8 {
        MrccAdcClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdcClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccAdcClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdcClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdcClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccAdcClkdivUnstab {
        MrccAdcClkdivUnstab::from_bits(val)
    }
}
impl From<MrccAdcClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccAdcClkdivUnstab) -> u8 {
        MrccAdcClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdcClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccAdcClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdcClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdcClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccAdcClkselMux {
        MrccAdcClkselMux::from_bits(val)
    }
}
impl From<MrccAdcClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccAdcClkselMux) -> u8 {
        MrccAdcClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccClkoutClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkdivHalt {
        MrccClkoutClkdivHalt::from_bits(val)
    }
}
impl From<MrccClkoutClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkdivHalt) -> u8 {
        MrccClkoutClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccClkoutClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkdivReset {
        MrccClkoutClkdivReset::from_bits(val)
    }
}
impl From<MrccClkoutClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkdivReset) -> u8 {
        MrccClkoutClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccClkoutClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkdivUnstab {
        MrccClkoutClkdivUnstab::from_bits(val)
    }
}
impl From<MrccClkoutClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkdivUnstab) -> u8 {
        MrccClkoutClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkselMux {
    #[doc = "FRO_12M"]
    CLKROOT_12M = 0x0,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FIRC_DIV = 0x01,
    #[doc = "CLK_IN"]
    CLKROOT_SOSC = 0x02,
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "PLL1_CLK"]
    CLKROOT_SPLL = 0x05,
    #[doc = "SLOW_CLK"]
    CLKROOT_SLOW = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccClkoutClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkselMux {
        MrccClkoutClkselMux::from_bits(val)
    }
}
impl From<MrccClkoutClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkselMux) -> u8 {
        MrccClkoutClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpFuncClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmpFuncClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpFuncClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpFuncClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpFuncClkdivHalt {
        MrccCmpFuncClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmpFuncClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpFuncClkdivHalt) -> u8 {
        MrccCmpFuncClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpFuncClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmpFuncClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpFuncClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpFuncClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpFuncClkdivReset {
        MrccCmpFuncClkdivReset::from_bits(val)
    }
}
impl From<MrccCmpFuncClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpFuncClkdivReset) -> u8 {
        MrccCmpFuncClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpFuncClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmpFuncClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpFuncClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpFuncClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpFuncClkdivUnstab {
        MrccCmpFuncClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmpFuncClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpFuncClkdivUnstab) -> u8 {
        MrccCmpFuncClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpRrClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmpRrClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpRrClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpRrClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpRrClkdivHalt {
        MrccCmpRrClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmpRrClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpRrClkdivHalt) -> u8 {
        MrccCmpRrClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpRrClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmpRrClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpRrClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpRrClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpRrClkdivReset {
        MrccCmpRrClkdivReset::from_bits(val)
    }
}
impl From<MrccCmpRrClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpRrClkdivReset) -> u8 {
        MrccCmpRrClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpRrClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmpRrClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpRrClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpRrClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpRrClkdivUnstab {
        MrccCmpRrClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmpRrClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpRrClkdivUnstab) -> u8 {
        MrccCmpRrClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmpRrClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCmpRrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmpRrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmpRrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCmpRrClkselMux {
        MrccCmpRrClkselMux::from_bits(val)
    }
}
impl From<MrccCmpRrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCmpRrClkselMux) -> u8 {
        MrccCmpRrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimerClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCtimerClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimerClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimerClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimerClkdivHalt {
        MrccCtimerClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimerClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimerClkdivHalt) -> u8 {
        MrccCtimerClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimerClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCtimerClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimerClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimerClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimerClkdivReset {
        MrccCtimerClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimerClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimerClkdivReset) -> u8 {
        MrccCtimerClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimerClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCtimerClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimerClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimerClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimerClkdivUnstab {
        MrccCtimerClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimerClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimerClkdivUnstab) -> u8 {
        MrccCtimerClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimerClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    #[doc = "CLK_16K"]
    CLKROOT_FUNC_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCtimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimerClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimerClkselMux {
        MrccCtimerClkselMux::from_bits(val)
    }
}
impl From<MrccCtimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimerClkselMux) -> u8 {
        MrccCtimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccDac0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkdivHalt {
        MrccDac0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccDac0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkdivHalt) -> u8 {
        MrccDac0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccDac0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkdivReset {
        MrccDac0ClkdivReset::from_bits(val)
    }
}
impl From<MrccDac0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkdivReset) -> u8 {
        MrccDac0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccDac0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkdivUnstab {
        MrccDac0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccDac0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkdivUnstab) -> u8 {
        MrccDac0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccDac0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkselMux {
        MrccDac0ClkselMux::from_bits(val)
    }
}
impl From<MrccDac0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkselMux) -> u8 {
        MrccDac0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccDbgTraceClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkdivHalt {
        MrccDbgTraceClkdivHalt::from_bits(val)
    }
}
impl From<MrccDbgTraceClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkdivHalt) -> u8 {
        MrccDbgTraceClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccDbgTraceClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkdivReset {
        MrccDbgTraceClkdivReset::from_bits(val)
    }
}
impl From<MrccDbgTraceClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkdivReset) -> u8 {
        MrccDbgTraceClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccDbgTraceClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkdivUnstab {
        MrccDbgTraceClkdivUnstab::from_bits(val)
    }
}
impl From<MrccDbgTraceClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkdivUnstab) -> u8 {
        MrccDbgTraceClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkselMux {
    #[doc = "CPU_CLK"]
    CLKROOT_CPU = 0x0,
    #[doc = "CLK_1M"]
    CLKROOT_1M = 0x01,
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccDbgTraceClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkselMux {
        MrccDbgTraceClkselMux::from_bits(val)
    }
}
impl From<MrccDbgTraceClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkselMux) -> u8 {
        MrccDbgTraceClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcanClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccFlexcanClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcanClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcanClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcanClkdivHalt {
        MrccFlexcanClkdivHalt::from_bits(val)
    }
}
impl From<MrccFlexcanClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcanClkdivHalt) -> u8 {
        MrccFlexcanClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcanClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccFlexcanClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcanClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcanClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcanClkdivReset {
        MrccFlexcanClkdivReset::from_bits(val)
    }
}
impl From<MrccFlexcanClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcanClkdivReset) -> u8 {
        MrccFlexcanClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcanClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccFlexcanClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcanClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcanClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcanClkdivUnstab {
        MrccFlexcanClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFlexcanClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcanClkdivUnstab) -> u8 {
        MrccFlexcanClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcanClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FIRC_GATED = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FIRC_DIV = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_SOSC = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccFlexcanClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcanClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcanClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcanClkselMux {
        MrccFlexcanClkselMux::from_bits(val)
    }
}
impl From<MrccFlexcanClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcanClkselMux) -> u8 {
        MrccFlexcanClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccFlexio0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkdivHalt {
        MrccFlexio0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccFlexio0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkdivHalt) -> u8 {
        MrccFlexio0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccFlexio0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkdivReset {
        MrccFlexio0ClkdivReset::from_bits(val)
    }
}
impl From<MrccFlexio0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkdivReset) -> u8 {
        MrccFlexio0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccFlexio0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkdivUnstab {
        MrccFlexio0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFlexio0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkdivUnstab) -> u8 {
        MrccFlexio0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccFlexio0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkselMux {
        MrccFlexio0ClkselMux::from_bits(val)
    }
}
impl From<MrccFlexio0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkselMux) -> u8 {
        MrccFlexio0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccI3c0FclkClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkdivHalt {
        MrccI3c0FclkClkdivHalt::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkdivHalt) -> u8 {
        MrccI3c0FclkClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccI3c0FclkClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkdivReset {
        MrccI3c0FclkClkdivReset::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkdivReset) -> u8 {
        MrccI3c0FclkClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccI3c0FclkClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkdivUnstab {
        MrccI3c0FclkClkdivUnstab::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkdivUnstab) -> u8 {
        MrccI3c0FclkClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccI3c0FclkClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkselMux {
        MrccI3c0FclkClkselMux::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkselMux) -> u8 {
        MrccI3c0FclkClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2cClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpi2cClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2cClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2cClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2cClkdivHalt {
        MrccLpi2cClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2cClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2cClkdivHalt) -> u8 {
        MrccLpi2cClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2cClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpi2cClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2cClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2cClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2cClkdivReset {
        MrccLpi2cClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2cClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2cClkdivReset) -> u8 {
        MrccLpi2cClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2cClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpi2cClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2cClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2cClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2cClkdivUnstab {
        MrccLpi2cClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2cClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2cClkdivUnstab) -> u8 {
        MrccLpi2cClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2cClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpi2cClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2cClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2cClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2cClkselMux {
        MrccLpi2cClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2cClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2cClkselMux) -> u8 {
        MrccLpi2cClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspiClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpspiClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspiClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspiClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspiClkdivHalt {
        MrccLpspiClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpspiClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspiClkdivHalt) -> u8 {
        MrccLpspiClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspiClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpspiClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspiClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspiClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspiClkdivReset {
        MrccLpspiClkdivReset::from_bits(val)
    }
}
impl From<MrccLpspiClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspiClkdivReset) -> u8 {
        MrccLpspiClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspiClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpspiClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspiClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspiClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspiClkdivUnstab {
        MrccLpspiClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpspiClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspiClkdivUnstab) -> u8 {
        MrccLpspiClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspiClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspiClkselMux {
        MrccLpspiClkselMux::from_bits(val)
    }
}
impl From<MrccLpspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspiClkselMux) -> u8 {
        MrccLpspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLptmr0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkdivHalt {
        MrccLptmr0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLptmr0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkdivHalt) -> u8 {
        MrccLptmr0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLptmr0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkdivReset {
        MrccLptmr0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLptmr0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkdivReset) -> u8 {
        MrccLptmr0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLptmr0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkdivUnstab {
        MrccLptmr0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLptmr0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkdivUnstab) -> u8 {
        MrccLptmr0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLptmr0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkselMux {
        MrccLptmr0ClkselMux::from_bits(val)
    }
}
impl From<MrccLptmr0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkselMux) -> u8 {
        MrccLptmr0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuartClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuartClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuartClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuartClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuartClkdivHalt {
        MrccLpuartClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuartClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuartClkdivHalt) -> u8 {
        MrccLpuartClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuartClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuartClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuartClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuartClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuartClkdivReset {
        MrccLpuartClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuartClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuartClkdivReset) -> u8 {
        MrccLpuartClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuartClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuartClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuartClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuartClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuartClkdivUnstab {
        MrccLpuartClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuartClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuartClkdivUnstab) -> u8 {
        MrccLpuartClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuartClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    #[doc = "CLK_16K"]
    CLKROOT_FUNC_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpuartClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuartClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuartClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuartClkselMux {
        MrccLpuartClkselMux::from_bits(val)
    }
}
impl From<MrccLpuartClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuartClkselMux) -> u8 {
        MrccLpuartClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccOstimer0ClkselMux {
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "CLK_1M"]
    CLKROOT_1M = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccOstimer0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccOstimer0ClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccOstimer0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccOstimer0ClkselMux {
        MrccOstimer0ClkselMux::from_bits(val)
    }
}
impl From<MrccOstimer0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccOstimer0ClkselMux) -> u8 {
        MrccOstimer0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccSystickClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkdivHalt {
        MrccSystickClkdivHalt::from_bits(val)
    }
}
impl From<MrccSystickClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkdivHalt) -> u8 {
        MrccSystickClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccSystickClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkdivReset {
        MrccSystickClkdivReset::from_bits(val)
    }
}
impl From<MrccSystickClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkdivReset) -> u8 {
        MrccSystickClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccSystickClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkdivUnstab {
        MrccSystickClkdivUnstab::from_bits(val)
    }
}
impl From<MrccSystickClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkdivUnstab) -> u8 {
        MrccSystickClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkselMux {
    #[doc = "CPU_CLK"]
    CLKROOT_CPU = 0x0,
    #[doc = "CLK_1M"]
    CLKROOT_1M = 0x01,
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccSystickClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkselMux {
        MrccSystickClkselMux::from_bits(val)
    }
}
impl From<MrccSystickClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkselMux) -> u8 {
        MrccSystickClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccUsb0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccUsb0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccUsb0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccUsb0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccUsb0ClkdivHalt {
        MrccUsb0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccUsb0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccUsb0ClkdivHalt) -> u8 {
        MrccUsb0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccUsb0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccUsb0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccUsb0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccUsb0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccUsb0ClkdivReset {
        MrccUsb0ClkdivReset::from_bits(val)
    }
}
impl From<MrccUsb0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccUsb0ClkdivReset) -> u8 {
        MrccUsb0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccUsb0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccUsb0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccUsb0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccUsb0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccUsb0ClkdivUnstab {
        MrccUsb0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccUsb0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccUsb0ClkdivUnstab) -> u8 {
        MrccUsb0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccUsb0ClkselMux {
    #[doc = "PLL1_CLK"]
    CLKROOT_SPLL = 0x0,
    #[doc = "CLK_48M"]
    SCG_SCG_FIRC_48MHZ_CLK = 0x01,
    #[doc = "CLK_IN"]
    CLKROOT_SOSC = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccUsb0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccUsb0ClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccUsb0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccUsb0ClkselMux {
        MrccUsb0ClkselMux::from_bits(val)
    }
}
impl From<MrccUsb0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccUsb0ClkselMux) -> u8 {
        MrccUsb0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccWwdt0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccWwdt0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccWwdt0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccWwdt0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccWwdt0ClkdivHalt {
        MrccWwdt0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccWwdt0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccWwdt0ClkdivHalt) -> u8 {
        MrccWwdt0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccWwdt0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccWwdt0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccWwdt0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccWwdt0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccWwdt0ClkdivReset {
        MrccWwdt0ClkdivReset::from_bits(val)
    }
}
impl From<MrccWwdt0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccWwdt0ClkdivReset) -> u8 {
        MrccWwdt0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccWwdt0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccWwdt0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccWwdt0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccWwdt0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccWwdt0ClkdivUnstab {
        MrccWwdt0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccWwdt0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccWwdt0ClkdivUnstab) -> u8 {
        MrccWwdt0ClkdivUnstab::to_bits(val)
    }
}
