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
pub enum MrccCmp0FuncClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmp0FuncClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0FuncClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0FuncClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0FuncClkdivHalt {
        MrccCmp0FuncClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp0FuncClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0FuncClkdivHalt) -> u8 {
        MrccCmp0FuncClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0FuncClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmp0FuncClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0FuncClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0FuncClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0FuncClkdivReset {
        MrccCmp0FuncClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp0FuncClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0FuncClkdivReset) -> u8 {
        MrccCmp0FuncClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0FuncClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmp0FuncClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0FuncClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0FuncClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0FuncClkdivUnstab {
        MrccCmp0FuncClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp0FuncClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0FuncClkdivUnstab) -> u8 {
        MrccCmp0FuncClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmp0RrClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkdivHalt {
        MrccCmp0RrClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp0RrClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkdivHalt) -> u8 {
        MrccCmp0RrClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmp0RrClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkdivReset {
        MrccCmp0RrClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp0RrClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkdivReset) -> u8 {
        MrccCmp0RrClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmp0RrClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkdivUnstab {
        MrccCmp0RrClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp0RrClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkdivUnstab) -> u8 {
        MrccCmp0RrClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkselMux {
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
impl MrccCmp0RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkselMux {
        MrccCmp0RrClkselMux::from_bits(val)
    }
}
impl From<MrccCmp0RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkselMux) -> u8 {
        MrccCmp0RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1FuncClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmp1FuncClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1FuncClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1FuncClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1FuncClkdivHalt {
        MrccCmp1FuncClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp1FuncClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1FuncClkdivHalt) -> u8 {
        MrccCmp1FuncClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1FuncClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmp1FuncClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1FuncClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1FuncClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1FuncClkdivReset {
        MrccCmp1FuncClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp1FuncClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1FuncClkdivReset) -> u8 {
        MrccCmp1FuncClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1FuncClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmp1FuncClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1FuncClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1FuncClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1FuncClkdivUnstab {
        MrccCmp1FuncClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp1FuncClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1FuncClkdivUnstab) -> u8 {
        MrccCmp1FuncClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmp1RrClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkdivHalt {
        MrccCmp1RrClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp1RrClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkdivHalt) -> u8 {
        MrccCmp1RrClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmp1RrClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkdivReset {
        MrccCmp1RrClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp1RrClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkdivReset) -> u8 {
        MrccCmp1RrClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmp1RrClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkdivUnstab {
        MrccCmp1RrClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp1RrClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkdivUnstab) -> u8 {
        MrccCmp1RrClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkselMux {
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
impl MrccCmp1RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkselMux {
        MrccCmp1RrClkselMux::from_bits(val)
    }
}
impl From<MrccCmp1RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkselMux) -> u8 {
        MrccCmp1RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2FuncClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmp2FuncClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2FuncClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2FuncClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2FuncClkdivHalt {
        MrccCmp2FuncClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp2FuncClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2FuncClkdivHalt) -> u8 {
        MrccCmp2FuncClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2FuncClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmp2FuncClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2FuncClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2FuncClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2FuncClkdivReset {
        MrccCmp2FuncClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp2FuncClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2FuncClkdivReset) -> u8 {
        MrccCmp2FuncClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2FuncClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmp2FuncClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2FuncClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2FuncClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2FuncClkdivUnstab {
        MrccCmp2FuncClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp2FuncClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2FuncClkdivUnstab) -> u8 {
        MrccCmp2FuncClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2RrClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCmp2RrClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2RrClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2RrClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2RrClkdivHalt {
        MrccCmp2RrClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp2RrClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2RrClkdivHalt) -> u8 {
        MrccCmp2RrClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2RrClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCmp2RrClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2RrClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2RrClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2RrClkdivReset {
        MrccCmp2RrClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp2RrClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2RrClkdivReset) -> u8 {
        MrccCmp2RrClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2RrClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCmp2RrClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2RrClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2RrClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2RrClkdivUnstab {
        MrccCmp2RrClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp2RrClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2RrClkdivUnstab) -> u8 {
        MrccCmp2RrClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp2RrClkselMux {
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
impl MrccCmp2RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp2RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp2RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp2RrClkselMux {
        MrccCmp2RrClkselMux::from_bits(val)
    }
}
impl From<MrccCmp2RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp2RrClkselMux) -> u8 {
        MrccCmp2RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCtimer0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkdivHalt {
        MrccCtimer0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkdivHalt) -> u8 {
        MrccCtimer0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCtimer0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkdivReset {
        MrccCtimer0ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkdivReset) -> u8 {
        MrccCtimer0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCtimer0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkdivUnstab {
        MrccCtimer0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkdivUnstab) -> u8 {
        MrccCtimer0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkselMux {
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
impl MrccCtimer0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkselMux {
        MrccCtimer0ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkselMux) -> u8 {
        MrccCtimer0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCtimer1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkdivHalt {
        MrccCtimer1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkdivHalt) -> u8 {
        MrccCtimer1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCtimer1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkdivReset {
        MrccCtimer1ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkdivReset) -> u8 {
        MrccCtimer1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCtimer1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkdivUnstab {
        MrccCtimer1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkdivUnstab) -> u8 {
        MrccCtimer1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkselMux {
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
impl MrccCtimer1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkselMux {
        MrccCtimer1ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkselMux) -> u8 {
        MrccCtimer1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCtimer2ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkdivHalt {
        MrccCtimer2ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer2ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkdivHalt) -> u8 {
        MrccCtimer2ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCtimer2ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkdivReset {
        MrccCtimer2ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer2ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkdivReset) -> u8 {
        MrccCtimer2ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCtimer2ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkdivUnstab {
        MrccCtimer2ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer2ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkdivUnstab) -> u8 {
        MrccCtimer2ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkselMux {
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
impl MrccCtimer2ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkselMux {
        MrccCtimer2ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer2ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkselMux) -> u8 {
        MrccCtimer2ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCtimer3ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkdivHalt {
        MrccCtimer3ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer3ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkdivHalt) -> u8 {
        MrccCtimer3ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCtimer3ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkdivReset {
        MrccCtimer3ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer3ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkdivReset) -> u8 {
        MrccCtimer3ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCtimer3ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkdivUnstab {
        MrccCtimer3ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer3ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkdivUnstab) -> u8 {
        MrccCtimer3ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkselMux {
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
impl MrccCtimer3ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkselMux {
        MrccCtimer3ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer3ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkselMux) -> u8 {
        MrccCtimer3ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccCtimer4ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkdivHalt {
        MrccCtimer4ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer4ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkdivHalt) -> u8 {
        MrccCtimer4ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccCtimer4ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkdivReset {
        MrccCtimer4ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer4ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkdivReset) -> u8 {
        MrccCtimer4ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccCtimer4ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkdivUnstab {
        MrccCtimer4ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer4ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkdivUnstab) -> u8 {
        MrccCtimer4ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkselMux {
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
impl MrccCtimer4ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkselMux {
        MrccCtimer4ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer4ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkselMux) -> u8 {
        MrccCtimer4ClkselMux::to_bits(val)
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
pub enum MrccFlexcan0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccFlexcan0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkdivHalt {
        MrccFlexcan0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkdivHalt) -> u8 {
        MrccFlexcan0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccFlexcan0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkdivReset {
        MrccFlexcan0ClkdivReset::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkdivReset) -> u8 {
        MrccFlexcan0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccFlexcan0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkdivUnstab {
        MrccFlexcan0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkdivUnstab) -> u8 {
        MrccFlexcan0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkselMux {
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
impl MrccFlexcan0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkselMux {
        MrccFlexcan0ClkselMux::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkselMux) -> u8 {
        MrccFlexcan0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan1ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccFlexcan1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan1ClkdivHalt {
        MrccFlexcan1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccFlexcan1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan1ClkdivHalt) -> u8 {
        MrccFlexcan1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan1ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccFlexcan1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan1ClkdivReset {
        MrccFlexcan1ClkdivReset::from_bits(val)
    }
}
impl From<MrccFlexcan1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan1ClkdivReset) -> u8 {
        MrccFlexcan1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan1ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccFlexcan1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan1ClkdivUnstab {
        MrccFlexcan1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFlexcan1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan1ClkdivUnstab) -> u8 {
        MrccFlexcan1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan1ClkselMux {
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
impl MrccFlexcan1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan1ClkselMux {
        MrccFlexcan1ClkselMux::from_bits(val)
    }
}
impl From<MrccFlexcan1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan1ClkselMux) -> u8 {
        MrccFlexcan1ClkselMux::to_bits(val)
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
pub enum MrccLpi2c0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpi2c0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkdivHalt {
        MrccLpi2c0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkdivHalt) -> u8 {
        MrccLpi2c0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpi2c0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkdivReset {
        MrccLpi2c0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkdivReset) -> u8 {
        MrccLpi2c0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpi2c0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkdivUnstab {
        MrccLpi2c0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkdivUnstab) -> u8 {
        MrccLpi2c0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkselMux {
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
impl MrccLpi2c0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkselMux {
        MrccLpi2c0ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkselMux) -> u8 {
        MrccLpi2c0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpi2c1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkdivHalt {
        MrccLpi2c1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkdivHalt) -> u8 {
        MrccLpi2c1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpi2c1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkdivReset {
        MrccLpi2c1ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkdivReset) -> u8 {
        MrccLpi2c1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpi2c1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkdivUnstab {
        MrccLpi2c1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkdivUnstab) -> u8 {
        MrccLpi2c1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkselMux {
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
impl MrccLpi2c1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkselMux {
        MrccLpi2c1ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkselMux) -> u8 {
        MrccLpi2c1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpi2c2ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkdivHalt {
        MrccLpi2c2ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkdivHalt) -> u8 {
        MrccLpi2c2ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpi2c2ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkdivReset {
        MrccLpi2c2ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkdivReset) -> u8 {
        MrccLpi2c2ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpi2c2ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkdivUnstab {
        MrccLpi2c2ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkdivUnstab) -> u8 {
        MrccLpi2c2ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkselMux {
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
impl MrccLpi2c2ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkselMux {
        MrccLpi2c2ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkselMux) -> u8 {
        MrccLpi2c2ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpi2c3ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkdivHalt {
        MrccLpi2c3ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkdivHalt) -> u8 {
        MrccLpi2c3ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpi2c3ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkdivReset {
        MrccLpi2c3ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkdivReset) -> u8 {
        MrccLpi2c3ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpi2c3ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkdivUnstab {
        MrccLpi2c3ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkdivUnstab) -> u8 {
        MrccLpi2c3ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkselMux {
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
impl MrccLpi2c3ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkselMux {
        MrccLpi2c3ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkselMux) -> u8 {
        MrccLpi2c3ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpspi0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkdivHalt {
        MrccLpspi0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpspi0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkdivHalt) -> u8 {
        MrccLpspi0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpspi0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkdivReset {
        MrccLpspi0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpspi0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkdivReset) -> u8 {
        MrccLpspi0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpspi0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkdivUnstab {
        MrccLpspi0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpspi0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkdivUnstab) -> u8 {
        MrccLpspi0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkselMux {
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
impl MrccLpspi0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkselMux {
        MrccLpspi0ClkselMux::from_bits(val)
    }
}
impl From<MrccLpspi0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkselMux) -> u8 {
        MrccLpspi0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpspi1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkdivHalt {
        MrccLpspi1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpspi1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkdivHalt) -> u8 {
        MrccLpspi1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpspi1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkdivReset {
        MrccLpspi1ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpspi1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkdivReset) -> u8 {
        MrccLpspi1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpspi1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkdivUnstab {
        MrccLpspi1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpspi1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkdivUnstab) -> u8 {
        MrccLpspi1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkselMux {
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
impl MrccLpspi1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkselMux {
        MrccLpspi1ClkselMux::from_bits(val)
    }
}
impl From<MrccLpspi1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkselMux) -> u8 {
        MrccLpspi1ClkselMux::to_bits(val)
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
pub enum MrccLpuart0ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuart0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkdivHalt {
        MrccLpuart0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkdivHalt) -> u8 {
        MrccLpuart0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuart0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkdivReset {
        MrccLpuart0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkdivReset) -> u8 {
        MrccLpuart0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuart0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkdivUnstab {
        MrccLpuart0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkdivUnstab) -> u8 {
        MrccLpuart0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkselMux {
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
impl MrccLpuart0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkselMux {
        MrccLpuart0ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkselMux) -> u8 {
        MrccLpuart0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuart1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkdivHalt {
        MrccLpuart1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkdivHalt) -> u8 {
        MrccLpuart1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuart1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkdivReset {
        MrccLpuart1ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkdivReset) -> u8 {
        MrccLpuart1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuart1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkdivUnstab {
        MrccLpuart1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkdivUnstab) -> u8 {
        MrccLpuart1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkselMux {
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
impl MrccLpuart1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkselMux {
        MrccLpuart1ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkselMux) -> u8 {
        MrccLpuart1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuart2ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkdivHalt {
        MrccLpuart2ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart2ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkdivHalt) -> u8 {
        MrccLpuart2ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuart2ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkdivReset {
        MrccLpuart2ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart2ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkdivReset) -> u8 {
        MrccLpuart2ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuart2ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkdivUnstab {
        MrccLpuart2ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart2ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkdivUnstab) -> u8 {
        MrccLpuart2ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkselMux {
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
impl MrccLpuart2ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkselMux {
        MrccLpuart2ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart2ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkselMux) -> u8 {
        MrccLpuart2ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuart3ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkdivHalt {
        MrccLpuart3ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart3ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkdivHalt) -> u8 {
        MrccLpuart3ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuart3ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkdivReset {
        MrccLpuart3ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart3ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkdivReset) -> u8 {
        MrccLpuart3ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuart3ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkdivUnstab {
        MrccLpuart3ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart3ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkdivUnstab) -> u8 {
        MrccLpuart3ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkselMux {
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
impl MrccLpuart3ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkselMux {
        MrccLpuart3ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart3ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkselMux) -> u8 {
        MrccLpuart3ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuart4ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkdivHalt {
        MrccLpuart4ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart4ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkdivHalt) -> u8 {
        MrccLpuart4ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuart4ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkdivReset {
        MrccLpuart4ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart4ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkdivReset) -> u8 {
        MrccLpuart4ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuart4ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkdivUnstab {
        MrccLpuart4ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart4ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkdivUnstab) -> u8 {
        MrccLpuart4ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkselMux {
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
impl MrccLpuart4ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkselMux {
        MrccLpuart4ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart4ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkselMux) -> u8 {
        MrccLpuart4ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart5ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl MrccLpuart5ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart5ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart5ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart5ClkdivHalt {
        MrccLpuart5ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart5ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart5ClkdivHalt) -> u8 {
        MrccLpuart5ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart5ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl MrccLpuart5ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart5ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart5ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart5ClkdivReset {
        MrccLpuart5ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart5ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart5ClkdivReset) -> u8 {
        MrccLpuart5ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart5ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl MrccLpuart5ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart5ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart5ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart5ClkdivUnstab {
        MrccLpuart5ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart5ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart5ClkdivUnstab) -> u8 {
        MrccLpuart5ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart5ClkselMux {
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
impl MrccLpuart5ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart5ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart5ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart5ClkselMux {
        MrccLpuart5ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart5ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart5ClkselMux) -> u8 {
        MrccLpuart5ClkselMux::to_bits(val)
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
