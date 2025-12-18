#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Accel {
    #[doc = "Flash acceleration is disabled."]
    DISABLE = 0x0,
    #[doc = "Flash acceleration is enabled."]
    ENABLE = 0x01,
}
impl Accel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Accel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Accel {
    #[inline(always)]
    fn from(val: u8) -> Accel {
        Accel::from_bits(val)
    }
}
impl From<Accel> for u8 {
    #[inline(always)]
    fn from(val: Accel) -> u8 {
        Accel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Adc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc {
    #[inline(always)]
    fn from(val: u8) -> Adc {
        Adc::from_bits(val)
    }
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(val: Adc) -> u8 {
        Adc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl AdcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcRst {
    #[inline(always)]
    fn from(val: u8) -> AdcRst {
        AdcRst::from_bits(val)
    }
}
impl From<AdcRst> for u8 {
    #[inline(always)]
    fn from(val: AdcRst) -> u8 {
        AdcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl AdcclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> AdcclkdivHalt {
        AdcclkdivHalt::from_bits(val)
    }
}
impl From<AdcclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: AdcclkdivHalt) -> u8 {
        AdcclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl AdcclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> AdcclkdivReqflag {
        AdcclkdivReqflag::from_bits(val)
    }
}
impl From<AdcclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: AdcclkdivReqflag) -> u8 {
        AdcclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl AdcclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> AdcclkdivReset {
        AdcclkdivReset::from_bits(val)
    }
}
impl From<AdcclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: AdcclkdivReset) -> u8 {
        AdcclkdivReset::to_bits(val)
    }
}
#[doc = "ADC clock source select"]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkselSel {
    #[doc = "Main clock."]
    mainclk = 0x0,
    #[doc = "PLL0 clock."]
    pll0 = 0x01,
    #[doc = "FRO 96 MHZ clock."]
    fro96 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Xtal clock coming directly."]
    xtal = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "No clock."]
    none = 0x07,
}
impl AdcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> AdcclkselSel {
        AdcclkselSel::from_bits(val)
    }
}
impl From<AdcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: AdcclkselSel) -> u8 {
        AdcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0Crcgen {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Ahbclkctrl0Crcgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0Crcgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0Crcgen {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0Crcgen {
        Ahbclkctrl0Crcgen::from_bits(val)
    }
}
impl From<Ahbclkctrl0Crcgen> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0Crcgen) -> u8 {
        Ahbclkctrl0Crcgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbclkctrl0Rom {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Ahbclkctrl0Rom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbclkctrl0Rom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbclkctrl0Rom {
    #[inline(always)]
    fn from(val: u8) -> Ahbclkctrl0Rom {
        Ahbclkctrl0Rom::from_bits(val)
    }
}
impl From<Ahbclkctrl0Rom> for u8 {
    #[inline(always)]
    fn from(val: Ahbclkctrl0Rom) -> u8 {
        Ahbclkctrl0Rom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl AhbclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivHalt {
        AhbclkdivHalt::from_bits(val)
    }
}
impl From<AhbclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivHalt) -> u8 {
        AhbclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl AhbclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivReqflag {
        AhbclkdivReqflag::from_bits(val)
    }
}
impl From<AhbclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivReqflag) -> u8 {
        AhbclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl AhbclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivReset {
        AhbclkdivReset::from_bits(val)
    }
}
impl From<AhbclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivReset) -> u8 {
        AhbclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnaFro12mClkEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl AnaFro12mClkEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnaFro12mClkEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnaFro12mClkEna {
    #[inline(always)]
    fn from(val: u8) -> AnaFro12mClkEna {
        AnaFro12mClkEna::from_bits(val)
    }
}
impl From<AnaFro12mClkEna> for u8 {
    #[inline(always)]
    fn from(val: AnaFro12mClkEna) -> u8 {
        AnaFro12mClkEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnalogCtrl {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl AnalogCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnalogCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnalogCtrl {
    #[inline(always)]
    fn from(val: u8) -> AnalogCtrl {
        AnalogCtrl::from_bits(val)
    }
}
impl From<AnalogCtrl> for u8 {
    #[inline(always)]
    fn from(val: AnalogCtrl) -> u8 {
        AnalogCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnalogCtrlRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl AnalogCtrlRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnalogCtrlRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnalogCtrlRst {
    #[inline(always)]
    fn from(val: u8) -> AnalogCtrlRst {
        AnalogCtrlRst::from_bits(val)
    }
}
impl From<AnalogCtrlRst> for u8 {
    #[inline(always)]
    fn from(val: AnalogCtrlRst) -> u8 {
        AnalogCtrlRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApFsDevNeedclk {
    #[doc = "Under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED = 0x01,
}
impl ApFsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApFsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApFsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApFsDevNeedclk {
        ApFsDevNeedclk::from_bits(val)
    }
}
impl From<ApFsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApFsDevNeedclk) -> u8 {
        ApFsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApFsHostNeedclk {
    #[doc = "Under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED = 0x01,
}
impl ApFsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApFsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApFsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApFsHostNeedclk {
        ApFsHostNeedclk::from_bits(val)
    }
}
impl From<ApFsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApFsHostNeedclk) -> u8 {
        ApFsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApHsDevNeedclk {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED = 0x01,
}
impl ApHsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApHsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApHsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApHsDevNeedclk {
        ApHsDevNeedclk::from_bits(val)
    }
}
impl From<ApHsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApHsDevNeedclk) -> u8 {
        ApHsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApHsHostNeedclk {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED = 0x01,
}
impl ApHsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApHsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApHsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ApHsHostNeedclk {
        ApHsHostNeedclk::from_bits(val)
    }
}
impl From<ApHsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ApHsHostNeedclk) -> u8 {
        ApHsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideCrcgen {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideCrcgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideCrcgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideCrcgen {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideCrcgen {
        AutoclkgateoverrideCrcgen::from_bits(val)
    }
}
impl From<AutoclkgateoverrideCrcgen> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideCrcgen) -> u8 {
        AutoclkgateoverrideCrcgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoclkgateoverrideRom {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl AutoclkgateoverrideRom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoclkgateoverrideRom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoclkgateoverrideRom {
    #[inline(always)]
    fn from(val: u8) -> AutoclkgateoverrideRom {
        AutoclkgateoverrideRom::from_bits(val)
    }
}
impl From<AutoclkgateoverrideRom> for u8 {
    #[inline(always)]
    fn from(val: AutoclkgateoverrideRom) -> u8 {
        AutoclkgateoverrideRom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Can {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can {
    #[inline(always)]
    fn from(val: u8) -> Can {
        Can::from_bits(val)
    }
}
impl From<Can> for u8 {
    #[inline(always)]
    fn from(val: Can) -> u8 {
        Can::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CanRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanRst {
    #[inline(always)]
    fn from(val: u8) -> CanRst {
        CanRst::from_bits(val)
    }
}
impl From<CanRst> for u8 {
    #[inline(always)]
    fn from(val: CanRst) -> u8 {
        CanRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl CanclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> CanclkdivHalt {
        CanclkdivHalt::from_bits(val)
    }
}
impl From<CanclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: CanclkdivHalt) -> u8 {
        CanclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl CanclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> CanclkdivReqflag {
        CanclkdivReqflag::from_bits(val)
    }
}
impl From<CanclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: CanclkdivReqflag) -> u8 {
        CanclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl CanclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> CanclkdivReset {
        CanclkdivReset::from_bits(val)
    }
}
impl From<CanclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: CanclkdivReset) -> u8 {
        CanclkdivReset::to_bits(val)
    }
}
#[doc = "CAN clock source select"]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanclkselSel {
    #[doc = "CAN divided clock."]
    canclk = 0x0,
    #[doc = "FRO 1MHz clock."]
    fro1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    osc32 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "No clock."]
    none = 0x07,
}
impl CanclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanclkselSel {
    #[inline(always)]
    fn from(val: u8) -> CanclkselSel {
        CanclkselSel::from_bits(val)
    }
}
impl From<CanclkselSel> for u8 {
    #[inline(always)]
    fn from(val: CanclkselSel) -> u8 {
        CanclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Casper {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Casper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Casper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Casper {
    #[inline(always)]
    fn from(val: u8) -> Casper {
        Casper::from_bits(val)
    }
}
impl From<Casper> for u8 {
    #[inline(always)]
    fn from(val: Casper) -> u8 {
        Casper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CasperRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CasperRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CasperRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CasperRst {
    #[inline(always)]
    fn from(val: u8) -> CasperRst {
        CasperRst::from_bits(val)
    }
}
impl From<CasperRst> for u8 {
    #[inline(always)]
    fn from(val: CasperRst) -> u8 {
        CasperRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Cdog {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog {
    #[inline(always)]
    fn from(val: u8) -> Cdog {
        Cdog::from_bits(val)
    }
}
impl From<Cdog> for u8 {
    #[inline(always)]
    fn from(val: Cdog) -> u8 {
        Cdog::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CdogRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CdogRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CdogRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CdogRst {
    #[inline(always)]
    fn from(val: u8) -> CdogRst {
        CdogRst::from_bits(val)
    }
}
impl From<CdogRst> for u8 {
    #[inline(always)]
    fn from(val: CdogRst) -> u8 {
        CdogRst::to_bits(val)
    }
}
#[doc = "Clock 32k clock select"]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clk32kclkselSel {
    #[doc = "Oscillator 32 kHz clock."]
    osc32 = 0x0,
    #[doc = "FRO 1MHz clock."]
    fro1 = 0x01,
}
impl Clk32kclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clk32kclkselSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clk32kclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Clk32kclkselSel {
        Clk32kclkselSel::from_bits(val)
    }
}
impl From<Clk32kclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Clk32kclkselSel) -> u8 {
        Clk32kclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkinEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl ClkinEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkinEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkinEna {
    #[inline(always)]
    fn from(val: u8) -> ClkinEna {
        ClkinEna::from_bits(val)
    }
}
impl From<ClkinEna> for u8 {
    #[inline(always)]
    fn from(val: ClkinEna) -> u8 {
        ClkinEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl ClkoutdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivHalt {
        ClkoutdivHalt::from_bits(val)
    }
}
impl From<ClkoutdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivHalt) -> u8 {
        ClkoutdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl ClkoutdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReqflag {
        ClkoutdivReqflag::from_bits(val)
    }
}
impl From<ClkoutdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReqflag) -> u8 {
        ClkoutdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl ClkoutdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReset {
        ClkoutdivReset::from_bits(val)
    }
}
impl From<ClkoutdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReset) -> u8 {
        ClkoutdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "No clock."]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock."]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock."]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock."]
    ENUM_0X_F = 0x0f,
}
impl ClkoutselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutselSel {
    #[inline(always)]
    fn from(val: u8) -> ClkoutselSel {
        ClkoutselSel::from_bits(val)
    }
}
impl From<ClkoutselSel> for u8 {
    #[inline(always)]
    fn from(val: ClkoutselSel) -> u8 {
        ClkoutselSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clockgenupdatelockout(u32);
impl Clockgenupdatelockout {
    #[doc = "all hardware clock configruration are freeze."]
    pub const FREEZE: Self = Self(0x0);
    #[doc = "update all clock configuration."]
    pub const ENABLE: Self = Self(0x01);
}
impl Clockgenupdatelockout {
    pub const fn from_bits(val: u32) -> Clockgenupdatelockout {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Clockgenupdatelockout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FREEZE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clockgenupdatelockout {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FREEZE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Clockgenupdatelockout {
    #[inline(always)]
    fn from(val: u32) -> Clockgenupdatelockout {
        Clockgenupdatelockout::from_bits(val)
    }
}
impl From<Clockgenupdatelockout> for u32 {
    #[inline(always)]
    fn from(val: Clockgenupdatelockout) -> u32 {
        Clockgenupdatelockout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Comp {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Comp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Comp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Comp {
    #[inline(always)]
    fn from(val: u8) -> Comp {
        Comp::from_bits(val)
    }
}
impl From<Comp> for u8 {
    #[inline(always)]
    fn from(val: Comp) -> u8 {
        Comp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CompRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CompRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CompRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CompRst {
    #[inline(always)]
    fn from(val: u8) -> CompRst {
        CompRst::from_bits(val)
    }
}
impl From<CompRst> for u8 {
    #[inline(always)]
    fn from(val: CompRst) -> u8 {
        CompRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "the CPU is not in lockup."]
    AWAKE = 0x0,
    #[doc = "the CPU is in lockup."]
    SLEEPING = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "the CPU is not sleeping."]
    AWAKE = 0x0,
    #[doc = "the CPU is sleeping."]
    SLEEPING = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcgenRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CrcgenRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcgenRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcgenRst {
    #[inline(always)]
    fn from(val: u8) -> CrcgenRst {
        CrcgenRst::from_bits(val)
    }
}
impl From<CrcgenRst> for u8 {
    #[inline(always)]
    fn from(val: CrcgenRst) -> u8 {
        CrcgenRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel0Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel0Sel {
        Ctimerclksel0Sel::from_bits(val)
    }
}
impl From<Ctimerclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel0Sel) -> u8 {
        Ctimerclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel1Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel1Sel {
        Ctimerclksel1Sel::from_bits(val)
    }
}
impl From<Ctimerclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel1Sel) -> u8 {
        Ctimerclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel2Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel2Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel2Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel2Sel {
        Ctimerclksel2Sel::from_bits(val)
    }
}
impl From<Ctimerclksel2Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel2Sel) -> u8 {
        Ctimerclksel2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel3Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel3Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel3Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel3Sel {
        Ctimerclksel3Sel::from_bits(val)
    }
}
impl From<Ctimerclksel3Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel3Sel) -> u8 {
        Ctimerclksel3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimerclksel4Sel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Ctimerclksel4Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimerclksel4Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimerclksel4Sel {
    #[inline(always)]
    fn from(val: u8) -> Ctimerclksel4Sel {
        Ctimerclksel4Sel::from_bits(val)
    }
}
impl From<Ctimerclksel4Sel> for u8 {
    #[inline(always)]
    fn from(val: Ctimerclksel4Sel) -> u8 {
        Ctimerclksel4Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datacfg {
    #[doc = "Data accesses from flash are not buffered."]
    NOBUF = 0x0,
    #[doc = "One buffer is used for all data accesses."]
    ONEBUF = 0x01,
    #[doc = "All buffers can be used for data accesses."]
    ALLBUF = 0x02,
    _RESERVED_3 = 0x03,
}
impl Datacfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datacfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datacfg {
    #[inline(always)]
    fn from(val: u8) -> Datacfg {
        Datacfg::from_bits(val)
    }
}
impl From<Datacfg> for u8 {
    #[inline(always)]
    fn from(val: Datacfg) -> u8 {
        Datacfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spiden {
        DebugFeaturesCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spiden) -> u8 {
        DebugFeaturesCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spniden {
        DebugFeaturesCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spniden) -> u8 {
        DebugFeaturesCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spiden {
        DebugFeaturesDpCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spiden) -> u8 {
        DebugFeaturesDpCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spniden {
        DebugFeaturesDpCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spniden) -> u8 {
        DebugFeaturesDpCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Dma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0 {
    #[inline(always)]
    fn from(val: u8) -> Dma0 {
        Dma0::from_bits(val)
    }
}
impl From<Dma0> for u8 {
    #[inline(always)]
    fn from(val: Dma0) -> u8 {
        Dma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Dma0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma0Rst {
        Dma0Rst::from_bits(val)
    }
}
impl From<Dma0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma0Rst) -> u8 {
        Dma0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Dma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1 {
    #[inline(always)]
    fn from(val: u8) -> Dma1 {
        Dma1::from_bits(val)
    }
}
impl From<Dma1> for u8 {
    #[inline(always)]
    fn from(val: Dma1) -> u8 {
        Dma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Dma1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma1Rst {
        Dma1Rst::from_bits(val)
    }
}
impl From<Dma1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma1Rst) -> u8 {
        Dma1Rst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enableupdate(u16);
impl Enableupdate {
    #[doc = "Bit Fields 0 - 15 of this register are not updated"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Bit Fields 0 - 15 of this register are updated"]
    pub const ENABLE: Self = Self(0xc0de);
}
impl Enableupdate {
    pub const fn from_bits(val: u16) -> Enableupdate {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Enableupdate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0xc0de => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enableupdate {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0xc0de => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Enableupdate {
    #[inline(always)]
    fn from(val: u16) -> Enableupdate {
        Enableupdate::from_bits(val)
    }
}
impl From<Enableupdate> for u16 {
    #[inline(always)]
    fn from(val: Enableupdate) -> u16 {
        Enableupdate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0 {
    #[inline(always)]
    fn from(val: u8) -> Fc0 {
        Fc0::from_bits(val)
    }
}
impl From<Fc0> for u8 {
    #[inline(always)]
    fn from(val: Fc0) -> u8 {
        Fc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1 {
    #[inline(always)]
    fn from(val: u8) -> Fc1 {
        Fc1::from_bits(val)
    }
}
impl From<Fc1> for u8 {
    #[inline(always)]
    fn from(val: Fc1) -> u8 {
        Fc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2 {
    #[inline(always)]
    fn from(val: u8) -> Fc2 {
        Fc2::from_bits(val)
    }
}
impl From<Fc2> for u8 {
    #[inline(always)]
    fn from(val: Fc2) -> u8 {
        Fc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3 {
    #[inline(always)]
    fn from(val: u8) -> Fc3 {
        Fc3::from_bits(val)
    }
}
impl From<Fc3> for u8 {
    #[inline(always)]
    fn from(val: Fc3) -> u8 {
        Fc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4 {
    #[inline(always)]
    fn from(val: u8) -> Fc4 {
        Fc4::from_bits(val)
    }
}
impl From<Fc4> for u8 {
    #[inline(always)]
    fn from(val: Fc4) -> u8 {
        Fc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5 {
    #[inline(always)]
    fn from(val: u8) -> Fc5 {
        Fc5::from_bits(val)
    }
}
impl From<Fc5> for u8 {
    #[inline(always)]
    fn from(val: Fc5) -> u8 {
        Fc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6 {
    #[inline(always)]
    fn from(val: u8) -> Fc6 {
        Fc6::from_bits(val)
    }
}
impl From<Fc6> for u8 {
    #[inline(always)]
    fn from(val: Fc6) -> u8 {
        Fc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7 {
    #[inline(always)]
    fn from(val: u8) -> Fc7 {
        Fc7::from_bits(val)
    }
}
impl From<Fc7> for u8 {
    #[inline(always)]
    fn from(val: Fc7) -> u8 {
        Fc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcRst {
    #[inline(always)]
    fn from(val: u8) -> FcRst {
        FcRst::from_bits(val)
    }
}
impl From<FcRst> for u8 {
    #[inline(always)]
    fn from(val: FcRst) -> u8 {
        FcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl FcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcclkselSel {
        FcclkselSel::from_bits(val)
    }
}
impl From<FcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcclkselSel) -> u8 {
        FcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fetchcfg {
    #[doc = "Instruction fetches from flash are not buffered."]
    NOBUF = 0x0,
    #[doc = "One buffer is used for all instruction fetches."]
    ONEBUF = 0x01,
    #[doc = "All buffers may be used for instruction fetches."]
    ALLBUF = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fetchcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fetchcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fetchcfg {
    #[inline(always)]
    fn from(val: u8) -> Fetchcfg {
        Fetchcfg::from_bits(val)
    }
}
impl From<Fetchcfg> for u8 {
    #[inline(always)]
    fn from(val: Fetchcfg) -> u8 {
        Fetchcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Flash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash {
    #[inline(always)]
    fn from(val: u8) -> Flash {
        Flash::from_bits(val)
    }
}
impl From<Flash> for u8 {
    #[inline(always)]
    fn from(val: Flash) -> u8 {
        Flash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FlashRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashRst {
    #[inline(always)]
    fn from(val: u8) -> FlashRst {
        FlashRst::from_bits(val)
    }
}
impl From<FlashRst> for u8 {
    #[inline(always)]
    fn from(val: FlashRst) -> u8 {
        FlashRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashtim {
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    FLASHTIM0 = 0x0,
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    FLASHTIM1 = 0x01,
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    FLASHTIM2 = 0x02,
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    FLASHTIM3 = 0x03,
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    FLASHTIM4 = 0x04,
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    FLASHTIM5 = 0x05,
    #[doc = "7 system clocks flash access time (for system clock rates up to 84 MHz)."]
    FLASHTIM6 = 0x06,
    #[doc = "8 system clocks flash access time (for system clock rates up to 104 MHz)."]
    FLASHTIM7 = 0x07,
    #[doc = "9 system clocks flash access time (for system clock rates up to 119 MHz)."]
    FLASHTIM8 = 0x08,
    #[doc = "10 system clocks flash access time (for system clock rates up to 129 MHz)."]
    FLASHTIM9 = 0x09,
    #[doc = "11 system clocks flash access time (for system clock rates up to 144 MHz)."]
    FLASHTIM10 = 0x0a,
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    FLASHTIM11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Flashtim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashtim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashtim {
    #[inline(always)]
    fn from(val: u8) -> Flashtim {
        Flashtim::from_bits(val)
    }
}
impl From<Flashtim> for u8 {
    #[inline(always)]
    fn from(val: Flashtim) -> u8 {
        Flashtim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flush {
    #[doc = "No action is performed."]
    NO_FLUSH = 0x0,
    #[doc = "Flush the FMC buffer contents."]
    FLUSH = 0x01,
}
impl Flush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flush {
    #[inline(always)]
    fn from(val: u8) -> Flush {
        Flush::from_bits(val)
    }
}
impl From<Flush> for u8 {
    #[inline(always)]
    fn from(val: Flush) -> u8 {
        Flush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmc {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Fmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmc {
    #[inline(always)]
    fn from(val: u8) -> Fmc {
        Fmc::from_bits(val)
    }
}
impl From<Fmc> for u8 {
    #[inline(always)]
    fn from(val: Fmc) -> u8 {
        Fmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FmcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmcRst {
    #[inline(always)]
    fn from(val: u8) -> FmcRst {
        FmcRst::from_bits(val)
    }
}
impl From<FmcRst> for u8 {
    #[inline(always)]
    fn from(val: FmcRst) -> u8 {
        FmcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freqme {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Freqme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freqme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freqme {
    #[inline(always)]
    fn from(val: u8) -> Freqme {
        Freqme::from_bits(val)
    }
}
impl From<Freqme> for u8 {
    #[inline(always)]
    fn from(val: Freqme) -> u8 {
        Freqme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FreqmeRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRst {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRst {
        FreqmeRst::from_bits(val)
    }
}
impl From<FreqmeRst> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRst) -> u8 {
        FreqmeRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frmen {
    #[doc = "free running mode is disable."]
    DISABLE = 0x0,
    #[doc = "free running mode is enable."]
    ENABLE = 0x01,
}
impl Frmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frmen {
    #[inline(always)]
    fn from(val: u8) -> Frmen {
        Frmen::from_bits(val)
    }
}
impl From<Frmen> for u8 {
    #[inline(always)]
    fn from(val: Frmen) -> u8 {
        Frmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro12mhzFreqmEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl Fro12mhzFreqmEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro12mhzFreqmEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro12mhzFreqmEna {
    #[inline(always)]
    fn from(val: u8) -> Fro12mhzFreqmEna {
        Fro12mhzFreqmEna::from_bits(val)
    }
}
impl From<Fro12mhzFreqmEna> for u8 {
    #[inline(always)]
    fn from(val: Fro12mhzFreqmEna) -> u8 {
        Fro12mhzFreqmEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Fro1mclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Fro1mclkdivHalt {
        Fro1mclkdivHalt::from_bits(val)
    }
}
impl From<Fro1mclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Fro1mclkdivHalt) -> u8 {
        Fro1mclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Fro1mclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> Fro1mclkdivReqflag {
        Fro1mclkdivReqflag::from_bits(val)
    }
}
impl From<Fro1mclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: Fro1mclkdivReqflag) -> u8 {
        Fro1mclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Fro1mclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Fro1mclkdivReset {
        Fro1mclkdivReset::from_bits(val)
    }
}
impl From<Fro1mclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Fro1mclkdivReset) -> u8 {
        Fro1mclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mhzClkEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl Fro1mhzClkEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mhzClkEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mhzClkEna {
    #[inline(always)]
    fn from(val: u8) -> Fro1mhzClkEna {
        Fro1mhzClkEna::from_bits(val)
    }
}
impl From<Fro1mhzClkEna> for u8 {
    #[inline(always)]
    fn from(val: Fro1mhzClkEna) -> u8 {
        Fro1mhzClkEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro1mhzUtickEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl Fro1mhzUtickEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro1mhzUtickEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro1mhzUtickEna {
    #[inline(always)]
    fn from(val: u8) -> Fro1mhzUtickEna {
        Fro1mhzUtickEna::from_bits(val)
    }
}
impl From<Fro1mhzUtickEna> for u8 {
    #[inline(always)]
    fn from(val: Fro1mhzUtickEna) -> u8 {
        Fro1mhzUtickEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FroHfFreqmEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl FroHfFreqmEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FroHfFreqmEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FroHfFreqmEna {
    #[inline(always)]
    fn from(val: u8) -> FroHfFreqmEna {
        FroHfFreqmEna::from_bits(val)
    }
}
impl From<FroHfFreqmEna> for u8 {
    #[inline(always)]
    fn from(val: FroHfFreqmEna) -> u8 {
        FroHfFreqmEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl FrohfdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReqflag {
        FrohfdivReqflag::from_bits(val)
    }
}
impl From<FrohfdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReqflag) -> u8 {
        FrohfdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl FrohfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReset {
        FrohfdivReset::from_bits(val)
    }
}
impl From<FrohfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReset) -> u8 {
        FrohfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funcretena {
    #[doc = "disable functional retention."]
    DISABLE = 0x0,
    #[doc = "enable functional retention."]
    ENABLE = 0x01,
}
impl Funcretena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funcretena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funcretena {
    #[inline(always)]
    fn from(val: u8) -> Funcretena {
        Funcretena::from_bits(val)
    }
}
impl From<Funcretena> for u8 {
    #[inline(always)]
    fn from(val: Funcretena) -> u8 {
        Funcretena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gint {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Gint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gint {
    #[inline(always)]
    fn from(val: u8) -> Gint {
        Gint::from_bits(val)
    }
}
impl From<Gint> for u8 {
    #[inline(always)]
    fn from(val: Gint) -> u8 {
        Gint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GintRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GintRst {
    #[inline(always)]
    fn from(val: u8) -> GintRst {
        GintRst::from_bits(val)
    }
}
impl From<GintRst> for u8 {
    #[inline(always)]
    fn from(val: GintRst) -> u8 {
        GintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Gpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0 {
        Gpio0::from_bits(val)
    }
}
impl From<Gpio0> for u8 {
    #[inline(always)]
    fn from(val: Gpio0) -> u8 {
        Gpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Gpio0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Rst {
        Gpio0Rst::from_bits(val)
    }
}
impl From<Gpio0Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Rst) -> u8 {
        Gpio0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Gpio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1 {
        Gpio1::from_bits(val)
    }
}
impl From<Gpio1> for u8 {
    #[inline(always)]
    fn from(val: Gpio1) -> u8 {
        Gpio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Gpio1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Rst {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Rst {
        Gpio1Rst::from_bits(val)
    }
}
impl From<Gpio1Rst> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Rst) -> u8 {
        Gpio1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSec {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl GpioSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSec {
    #[inline(always)]
    fn from(val: u8) -> GpioSec {
        GpioSec::from_bits(val)
    }
}
impl From<GpioSec> for u8 {
    #[inline(always)]
    fn from(val: GpioSec) -> u8 {
        GpioSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSecInt {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl GpioSecInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSecInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSecInt {
    #[inline(always)]
    fn from(val: u8) -> GpioSecInt {
        GpioSecInt::from_bits(val)
    }
}
impl From<GpioSecInt> for u8 {
    #[inline(always)]
    fn from(val: GpioSecInt) -> u8 {
        GpioSecInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSecIntRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GpioSecIntRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSecIntRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSecIntRst {
    #[inline(always)]
    fn from(val: u8) -> GpioSecIntRst {
        GpioSecIntRst::from_bits(val)
    }
}
impl From<GpioSecIntRst> for u8 {
    #[inline(always)]
    fn from(val: GpioSecIntRst) -> u8 {
        GpioSecIntRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioSecRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GpioSecRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioSecRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioSecRst {
    #[inline(always)]
    fn from(val: u8) -> GpioSecRst {
        GpioSecRst::from_bits(val)
    }
}
impl From<GpioSecRst> for u8 {
    #[inline(always)]
    fn from(val: GpioSecRst) -> u8 {
        GpioSecRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HashAes {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl HashAes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashAes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashAes {
    #[inline(always)]
    fn from(val: u8) -> HashAes {
        HashAes::from_bits(val)
    }
}
impl From<HashAes> for u8 {
    #[inline(always)]
    fn from(val: HashAes) -> u8 {
        HashAes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HashAesRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl HashAesRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HashAesRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HashAesRst {
    #[inline(always)]
    fn from(val: u8) -> HashAesRst {
        HashAesRst::from_bits(val)
    }
}
impl From<HashAesRst> for u8 {
    #[inline(always)]
    fn from(val: HashAesRst) -> u8 {
        HashAesRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsDevWakeupN {
    #[doc = "Forces USB1_PHY to wake-up."]
    FORCE_WUP = 0x0,
    #[doc = "Normal USB1_PHY behavior."]
    NORMAL_WUP = 0x01,
}
impl HsDevWakeupN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsDevWakeupN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsDevWakeupN {
    #[inline(always)]
    fn from(val: u8) -> HsDevWakeupN {
        HsDevWakeupN::from_bits(val)
    }
}
impl From<HsDevWakeupN> for u8 {
    #[inline(always)]
    fn from(val: HsDevWakeupN) -> u8 {
        HsDevWakeupN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsLspi {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl HsLspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsLspi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsLspi {
    #[inline(always)]
    fn from(val: u8) -> HsLspi {
        HsLspi::from_bits(val)
    }
}
impl From<HsLspi> for u8 {
    #[inline(always)]
    fn from(val: HsLspi) -> u8 {
        HsLspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsLspiRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl HsLspiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsLspiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsLspiRst {
    #[inline(always)]
    fn from(val: u8) -> HsLspiRst {
        HsLspiRst::from_bits(val)
    }
}
impl From<HsLspiRst> for u8 {
    #[inline(always)]
    fn from(val: HsLspiRst) -> u8 {
        HsLspiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HslspiclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl HslspiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HslspiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HslspiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> HslspiclkselSel {
        HslspiclkselSel::from_bits(val)
    }
}
impl From<HslspiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: HslspiclkselSel) -> u8 {
        HslspiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntClear {
    #[doc = "No effect."]
    NONE = 0x0,
    #[doc = "Clear the interrupt. Self-cleared bit."]
    CLEAR = 0x01,
}
impl IntClear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntClear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntClear {
    #[inline(always)]
    fn from(val: u8) -> IntClear {
        IntClear::from_bits(val)
    }
}
impl From<IntClear> for u8 {
    #[inline(always)]
    fn from(val: IntClear) -> u8 {
        IntClear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrl {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE = 0x0,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE = 0x01,
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING = 0x02,
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH = 0x03,
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING = 0x04,
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    LVL_LOW = 0x05,
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH = 0x06,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2 = 0x07,
}
impl IntCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrl {
    #[inline(always)]
    fn from(val: u8) -> IntCtrl {
        IntCtrl::from_bits(val)
    }
}
impl From<IntCtrl> for u8 {
    #[inline(always)]
    fn from(val: IntCtrl) -> u8 {
        IntCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntEnable {
    #[doc = "interrupt disable."]
    INT_DISABLE = 0x0,
    #[doc = "interrupt enable."]
    INT_ENABLE = 0x01,
}
impl IntEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntEnable {
    #[inline(always)]
    fn from(val: u8) -> IntEnable {
        IntEnable::from_bits(val)
    }
}
impl From<IntEnable> for u8 {
    #[inline(always)]
    fn from(val: IntEnable) -> u8 {
        IntEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntSource {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT = 0x0,
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT = 0x01,
}
impl IntSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntSource {
    #[inline(always)]
    fn from(val: u8) -> IntSource {
        IntSource::from_bits(val)
    }
}
impl From<IntSource> for u8 {
    #[inline(always)]
    fn from(val: IntSource) -> u8 {
        IntSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatus {
    #[doc = "no interrupt pending."]
    NO_INT = 0x0,
    #[doc = "interrupt pending."]
    PENDING = 0x01,
}
impl IntStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatus {
    #[inline(always)]
    fn from(val: u8) -> IntStatus {
        IntStatus::from_bits(val)
    }
}
impl From<IntStatus> for u8 {
    #[inline(always)]
    fn from(val: IntStatus) -> u8 {
        IntStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access to RAMX0 and RAMX1 is consecutive."]
    NORMAL = 0x0,
    #[doc = "RAM access to RAMX0 and RAMX1 is interleaved."]
    INTERLEAVE = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iocon {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Iocon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iocon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iocon {
    #[inline(always)]
    fn from(val: u8) -> Iocon {
        Iocon::from_bits(val)
    }
}
impl From<Iocon> for u8 {
    #[inline(always)]
    fn from(val: Iocon) -> u8 {
        Iocon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IoconRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl IoconRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IoconRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IoconRst {
    #[inline(always)]
    fn from(val: u8) -> IoconRst {
        IoconRst::from_bits(val)
    }
}
impl From<IoconRst> for u8 {
    #[inline(always)]
    fn from(val: IoconRst) -> u8 {
        IoconRst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lock(u32);
impl Lock {
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    pub const UNLOCK: Self = Self(0x3cc3_5aa5);
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    pub const LOCK: Self = Self(0xc33c_a55a);
}
impl Lock {
    pub const fn from_bits(val: u32) -> Lock {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x3cc3_5aa5 => f.write_str("UNLOCK"),
            0xc33c_a55a => f.write_str("LOCK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x3cc3_5aa5 => defmt::write!(f, "UNLOCK"),
            0xc33c_a55a => defmt::write!(f, "LOCK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Lock {
    #[inline(always)]
    fn from(val: u32) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u32 {
    #[inline(always)]
    fn from(val: Lock) -> u32 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disable write access to all registers."]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "1010: Enable write access to all registers."]
    ENABLE = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockBootSeed {
    _RESERVED_0 = 0x0,
    #[doc = "write access to all 8 registers BOOT_SEED_REG is locked. This register is write once."]
    LOCK = 0x01,
}
impl LockBootSeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockBootSeed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockBootSeed {
    #[inline(always)]
    fn from(val: u8) -> LockBootSeed {
        LockBootSeed::from_bits(val)
    }
}
impl From<LockBootSeed> for u8 {
    #[inline(always)]
    fn from(val: LockBootSeed) -> u8 {
        LockBootSeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockHmac {
    _RESERVED_0 = 0x0,
    #[doc = "write access to all 8 registers HMAC_REG is locked. This register is write once."]
    LOCK = 0x01,
}
impl LockHmac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockHmac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockHmac {
    #[inline(always)]
    fn from(val: u8) -> LockHmac {
        LockHmac::from_bits(val)
    }
}
impl From<LockHmac> for u8 {
    #[inline(always)]
    fn from(val: LockHmac) -> u8 {
        LockHmac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mailbox {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Mailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mailbox {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mailbox {
    #[inline(always)]
    fn from(val: u8) -> Mailbox {
        Mailbox::from_bits(val)
    }
}
impl From<Mailbox> for u8 {
    #[inline(always)]
    fn from(val: Mailbox) -> u8 {
        Mailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MailboxRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MailboxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MailboxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MailboxRst {
    #[inline(always)]
    fn from(val: u8) -> MailboxRst {
        MailboxRst::from_bits(val)
    }
}
impl From<MailboxRst> for u8 {
    #[inline(always)]
    fn from(val: MailboxRst) -> u8 {
        MailboxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkselaSel {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MainclkselaSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselaSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselaSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselaSel {
        MainclkselaSel::from_bits(val)
    }
}
impl From<MainclkselaSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselaSel) -> u8 {
        MainclkselaSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkselbSel {
    #[doc = "Main Clock A."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "PLL1 clock."]
    ENUM_0X2 = 0x02,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MainclkselbSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselbSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselbSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselbSel {
        MainclkselbSel::from_bits(val)
    }
}
impl From<MainclkselbSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselbSel) -> u8 {
        MainclkselbSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Map {
    #[doc = "Vector Table in ROM."]
    ROM0 = 0x0,
    #[doc = "Vector Table in RAM."]
    RAM1 = 0x01,
    #[doc = "Vector Table in Flash."]
    FLASH0 = 0x02,
    #[doc = "Vector Table in Flash."]
    FLASH1 = 0x03,
}
impl Map {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Map {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Map {
    #[inline(always)]
    fn from(val: u8) -> Map {
        Map::from_bits(val)
    }
}
impl From<Map> for u8 {
    #[inline(always)]
    fn from(val: Map) -> u8 {
        Map::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkclkselSel {
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl MclkclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkclkselSel {
    #[inline(always)]
    fn from(val: u8) -> MclkclkselSel {
        MclkclkselSel::from_bits(val)
    }
}
impl From<MclkclkselSel> for u8 {
    #[inline(always)]
    fn from(val: MclkclkselSel) -> u8 {
        MclkclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl MclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MclkdivHalt {
        MclkdivHalt::from_bits(val)
    }
}
impl From<MclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MclkdivHalt) -> u8 {
        MclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl MclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> MclkdivReqflag {
        MclkdivReqflag::from_bits(val)
    }
}
impl From<MclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: MclkdivReqflag) -> u8 {
        MclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl MclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MclkdivReset {
        MclkdivReset::from_bits(val)
    }
}
impl From<MclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MclkdivReset) -> u8 {
        MclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkio {
    #[doc = "input mode."]
    INPUT = 0x0,
    #[doc = "output mode."]
    OUTPUT = 0x01,
}
impl Mclkio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkio {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkio {
    #[inline(always)]
    fn from(val: u8) -> Mclkio {
        Mclkio::from_bits(val)
    }
}
impl From<Mclkio> for u8 {
    #[inline(always)]
    fn from(val: Mclkio) -> u8 {
        Mclkio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Mrt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt {
    #[inline(always)]
    fn from(val: u8) -> Mrt {
        Mrt::from_bits(val)
    }
}
impl From<Mrt> for u8 {
    #[inline(always)]
    fn from(val: Mrt) -> u8 {
        Mrt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrtRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MrtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrtRst {
    #[inline(always)]
    fn from(val: u8) -> MrtRst {
        MrtRst::from_bits(val)
    }
}
impl From<MrtRst> for u8 {
    #[inline(always)]
    fn from(val: MrtRst) -> u8 {
        MrtRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mux {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mux {
    #[inline(always)]
    fn from(val: u8) -> Mux {
        Mux::from_bits(val)
    }
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(val: Mux) -> u8 {
        Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MuxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRst {
    #[inline(always)]
    fn from(val: u8) -> MuxRst {
        MuxRst::from_bits(val)
    }
}
impl From<MuxRst> for u8 {
    #[inline(always)]
    fn from(val: MuxRst) -> u8 {
        MuxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostimer {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Ostimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimer {
    #[inline(always)]
    fn from(val: u8) -> Ostimer {
        Ostimer::from_bits(val)
    }
}
impl From<Ostimer> for u8 {
    #[inline(always)]
    fn from(val: Ostimer) -> u8 {
        Ostimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl OstimerRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerRst {
    #[inline(always)]
    fn from(val: u8) -> OstimerRst {
        OstimerRst::from_bits(val)
    }
}
impl From<OstimerRst> for u8 {
    #[inline(always)]
    fn from(val: OstimerRst) -> u8 {
        OstimerRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pint {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Pint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pint {
    #[inline(always)]
    fn from(val: u8) -> Pint {
        Pint::from_bits(val)
    }
}
impl From<Pint> for u8 {
    #[inline(always)]
    fn from(val: Pint) -> u8 {
        Pint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintRst {
    #[inline(always)]
    fn from(val: u8) -> PintRst {
        PintRst::from_bits(val)
    }
}
impl From<PintRst> for u8 {
    #[inline(always)]
    fn from(val: PintRst) -> u8 {
        PintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Pll0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkdivHalt {
        Pll0clkdivHalt::from_bits(val)
    }
}
impl From<Pll0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkdivHalt) -> u8 {
        Pll0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Pll0clkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkdivReqflag {
        Pll0clkdivReqflag::from_bits(val)
    }
}
impl From<Pll0clkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkdivReqflag) -> u8 {
        Pll0clkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Pll0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkdivReset {
        Pll0clkdivReset::from_bits(val)
    }
}
impl From<Pll0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkdivReset) -> u8 {
        Pll0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0clkselSel {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Pll0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Pll0clkselSel {
        Pll0clkselSel::from_bits(val)
    }
}
impl From<Pll0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Pll0clkselSel) -> u8 {
        Pll0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBwdirect {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0x0,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT = 0x01,
}
impl Pll0ctrlBwdirect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBwdirect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBwdirect {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBwdirect {
        Pll0ctrlBwdirect::from_bits(val)
    }
}
impl From<Pll0ctrlBwdirect> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBwdirect) -> u8 {
        Pll0ctrlBwdirect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypasspll {
    #[doc = "use PLL."]
    USED = 0x0,
    #[doc = "Bypass PLL input clock is sent directly to the PLL output."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypasspll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypasspll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypasspll {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypasspll {
        Pll0ctrlBypasspll::from_bits(val)
    }
}
impl From<Pll0ctrlBypasspll> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypasspll) -> u8 {
        Pll0ctrlBypasspll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypasspostdiv {
    #[doc = "use the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the post-divider."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypasspostdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypasspostdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypasspostdiv {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypasspostdiv {
        Pll0ctrlBypasspostdiv::from_bits(val)
    }
}
impl From<Pll0ctrlBypasspostdiv> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypasspostdiv) -> u8 {
        Pll0ctrlBypasspostdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypasspostdiv2 {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypasspostdiv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypasspostdiv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypasspostdiv2 {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypasspostdiv2 {
        Pll0ctrlBypasspostdiv2::from_bits(val)
    }
}
impl From<Pll0ctrlBypasspostdiv2> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypasspostdiv2) -> u8 {
        Pll0ctrlBypasspostdiv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlBypassprediv {
    #[doc = "use the pre-divider."]
    USED = 0x0,
    #[doc = "bypass of the pre-divider."]
    BYPASSED = 0x01,
}
impl Pll0ctrlBypassprediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlBypassprediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlBypassprediv {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlBypassprediv {
        Pll0ctrlBypassprediv::from_bits(val)
    }
}
impl From<Pll0ctrlBypassprediv> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlBypassprediv) -> u8 {
        Pll0ctrlBypassprediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlClken {
    #[doc = "disable the output clock."]
    DISABLE = 0x0,
    #[doc = "enable the output clock."]
    ENABLE = 0x01,
}
impl Pll0ctrlClken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlClken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlClken {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlClken {
        Pll0ctrlClken::from_bits(val)
    }
}
impl From<Pll0ctrlClken> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlClken) -> u8 {
        Pll0ctrlClken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll0ctrlSkewen {
    #[doc = "skew mode is disable."]
    DISABLE = 0x0,
    #[doc = "skew mode is enable."]
    ENABLE = 0x01,
}
impl Pll0ctrlSkewen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll0ctrlSkewen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll0ctrlSkewen {
    #[inline(always)]
    fn from(val: u8) -> Pll0ctrlSkewen {
        Pll0ctrlSkewen::from_bits(val)
    }
}
impl From<Pll0ctrlSkewen> for u8 {
    #[inline(always)]
    fn from(val: Pll0ctrlSkewen) -> u8 {
        Pll0ctrlSkewen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkselSel {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Pll1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkselSel {
        Pll1clkselSel::from_bits(val)
    }
}
impl From<Pll1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkselSel) -> u8 {
        Pll1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBwdirect {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0x0,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT = 0x01,
}
impl Pll1ctrlBwdirect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBwdirect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBwdirect {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBwdirect {
        Pll1ctrlBwdirect::from_bits(val)
    }
}
impl From<Pll1ctrlBwdirect> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBwdirect) -> u8 {
        Pll1ctrlBwdirect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypasspll {
    #[doc = "use PLL."]
    USED = 0x0,
    #[doc = "PLL input clock is sent directly to the PLL output."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypasspll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypasspll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypasspll {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypasspll {
        Pll1ctrlBypasspll::from_bits(val)
    }
}
impl From<Pll1ctrlBypasspll> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypasspll) -> u8 {
        Pll1ctrlBypasspll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypasspostdiv {
    #[doc = "use the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the post-divider."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypasspostdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypasspostdiv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypasspostdiv {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypasspostdiv {
        Pll1ctrlBypasspostdiv::from_bits(val)
    }
}
impl From<Pll1ctrlBypasspostdiv> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypasspostdiv) -> u8 {
        Pll1ctrlBypasspostdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypasspostdiv2 {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypasspostdiv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypasspostdiv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypasspostdiv2 {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypasspostdiv2 {
        Pll1ctrlBypasspostdiv2::from_bits(val)
    }
}
impl From<Pll1ctrlBypasspostdiv2> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypasspostdiv2) -> u8 {
        Pll1ctrlBypasspostdiv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlBypassprediv {
    #[doc = "use the pre-divider."]
    USED = 0x0,
    #[doc = "bypass of the pre-divider."]
    BYPASSED = 0x01,
}
impl Pll1ctrlBypassprediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlBypassprediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlBypassprediv {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlBypassprediv {
        Pll1ctrlBypassprediv::from_bits(val)
    }
}
impl From<Pll1ctrlBypassprediv> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlBypassprediv) -> u8 {
        Pll1ctrlBypassprediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlClken {
    #[doc = "Disable the output clock."]
    DISABLE = 0x0,
    #[doc = "Enable the output clock."]
    ENABLE = 0x01,
}
impl Pll1ctrlClken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlClken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlClken {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlClken {
        Pll1ctrlClken::from_bits(val)
    }
}
impl From<Pll1ctrlClken> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlClken) -> u8 {
        Pll1ctrlClken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1ctrlSkewen {
    #[doc = "skewmode is disable."]
    DISABLE = 0x0,
    #[doc = "skewmode is enable."]
    ENABLE = 0x01,
}
impl Pll1ctrlSkewen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1ctrlSkewen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1ctrlSkewen {
    #[inline(always)]
    fn from(val: u8) -> Pll1ctrlSkewen {
        Pll1ctrlSkewen::from_bits(val)
    }
}
impl From<Pll1ctrlSkewen> for u8 {
    #[inline(always)]
    fn from(val: Pll1ctrlSkewen) -> u8 {
        Pll1ctrlSkewen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PluDeglitchClkEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl PluDeglitchClkEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PluDeglitchClkEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PluDeglitchClkEna {
    #[inline(always)]
    fn from(val: u8) -> PluDeglitchClkEna {
        PluDeglitchClkEna::from_bits(val)
    }
}
impl From<PluDeglitchClkEna> for u8 {
    #[inline(always)]
    fn from(val: PluDeglitchClkEna) -> u8 {
        PluDeglitchClkEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plulut {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Plulut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plulut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plulut {
    #[inline(always)]
    fn from(val: u8) -> Plulut {
        Plulut::from_bits(val)
    }
}
impl From<Plulut> for u8 {
    #[inline(always)]
    fn from(val: Plulut) -> u8 {
        Plulut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlulutRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PlulutRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlulutRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlulutRst {
    #[inline(always)]
    fn from(val: u8) -> PlulutRst {
        PlulutRst::from_bits(val)
    }
}
impl From<PlulutRst> for u8 {
    #[inline(always)]
    fn from(val: PlulutRst) -> u8 {
        PlulutRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolFsDevNeedclk {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolFsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolFsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolFsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolFsDevNeedclk {
        PolFsDevNeedclk::from_bits(val)
    }
}
impl From<PolFsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolFsDevNeedclk) -> u8 {
        PolFsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolFsHostNeedclk {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolFsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolFsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolFsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolFsHostNeedclk {
        PolFsHostNeedclk::from_bits(val)
    }
}
impl From<PolFsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolFsHostNeedclk) -> u8 {
        PolFsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolHsDevNeedclk {
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolHsDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolHsDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolHsDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolHsDevNeedclk {
        PolHsDevNeedclk::from_bits(val)
    }
}
impl From<PolHsDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolHsDevNeedclk) -> u8 {
        PolHsDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolHsHostNeedclk {
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl PolHsHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolHsHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolHsHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> PolHsHostNeedclk {
        PolHsHostNeedclk::from_bits(val)
    }
}
impl From<PolHsHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: PolHsHostNeedclk) -> u8 {
        PolHsHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prefen {
    #[doc = "No instruction prefetch is performed."]
    DISABLE = 0x0,
    #[doc = "Instruction prefetch is enabled."]
    ENABLE = 0x01,
}
impl Prefen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prefen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prefen {
    #[inline(always)]
    fn from(val: u8) -> Prefen {
        Prefen::from_bits(val)
    }
}
impl From<Prefen> for u8 {
    #[inline(always)]
    fn from(val: Prefen) -> u8 {
        Prefen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prefovr {
    #[doc = "Any previously initiated prefetch will be completed."]
    NORMAL = 0x0,
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    OVERRIDE = 0x01,
}
impl Prefovr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prefovr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prefovr {
    #[inline(always)]
    fn from(val: u8) -> Prefovr {
        Prefovr::from_bits(val)
    }
}
impl From<Prefovr> for u8 {
    #[inline(always)]
    fn from(val: Prefovr) -> u8 {
        Prefovr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psync {
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    USED = 0x0,
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    BYPASS = 0x01,
}
impl Psync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psync {
    #[inline(always)]
    fn from(val: u8) -> Psync {
        Psync::from_bits(val)
    }
}
impl From<Psync> for u8 {
    #[inline(always)]
    fn from(val: Psync) -> u8 {
        Psync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Puf {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Puf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Puf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Puf {
    #[inline(always)]
    fn from(val: u8) -> Puf {
        Puf::from_bits(val)
    }
}
impl From<Puf> for u8 {
    #[inline(always)]
    fn from(val: Puf) -> u8 {
        Puf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PufRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufRst {
    #[inline(always)]
    fn from(val: u8) -> PufRst {
        PufRst::from_bits(val)
    }
}
impl From<PufRst> for u8 {
    #[inline(always)]
    fn from(val: PufRst) -> u8 {
        PufRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram0Ctrl {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Ram0Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram0Ctrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram0Ctrl {
    #[inline(always)]
    fn from(val: u8) -> Ram0Ctrl {
        Ram0Ctrl::from_bits(val)
    }
}
impl From<Ram0Ctrl> for u8 {
    #[inline(always)]
    fn from(val: Ram0Ctrl) -> u8 {
        Ram0Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram1Ctrl {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Ram1Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram1Ctrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram1Ctrl {
    #[inline(always)]
    fn from(val: u8) -> Ram1Ctrl {
        Ram1Ctrl::from_bits(val)
    }
}
impl From<Ram1Ctrl> for u8 {
    #[inline(always)]
    fn from(val: Ram1Ctrl) -> u8 {
        Ram1Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ram2Ctrl {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Ram2Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ram2Ctrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ram2Ctrl {
    #[inline(always)]
    fn from(val: u8) -> Ram2Ctrl {
        Ram2Ctrl::from_bits(val)
    }
}
impl From<Ram2Ctrl> for u8 {
    #[inline(always)]
    fn from(val: Ram2Ctrl) -> u8 {
        Ram2Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxCtrl {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl RamxCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxCtrl {
    #[inline(always)]
    fn from(val: u8) -> RamxCtrl {
        RamxCtrl::from_bits(val)
    }
}
impl From<RamxCtrl> for u8 {
    #[inline(always)]
    fn from(val: RamxCtrl) -> u8 {
        RamxCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rng {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Rng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rng {
    #[inline(always)]
    fn from(val: u8) -> Rng {
        Rng::from_bits(val)
    }
}
impl From<Rng> for u8 {
    #[inline(always)]
    fn from(val: Rng) -> u8 {
        Rng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RngRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RngRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RngRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RngRst {
    #[inline(always)]
    fn from(val: u8) -> RngRst {
        RngRst::from_bits(val)
    }
}
impl From<RngRst> for u8 {
    #[inline(always)]
    fn from(val: RngRst) -> u8 {
        RngRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RomRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomRst {
    #[inline(always)]
    fn from(val: u8) -> RomRst {
        RomRst::from_bits(val)
    }
}
impl From<RomRst> for u8 {
    #[inline(always)]
    fn from(val: RomRst) -> u8 {
        RomRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Rtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc {
    #[inline(always)]
    fn from(val: u8) -> Rtc {
        Rtc::from_bits(val)
    }
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(val: Rtc) -> u8 {
        Rtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RtcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRst {
    #[inline(always)]
    fn from(val: u8) -> RtcRst {
        RtcRst::from_bits(val)
    }
}
impl From<RtcRst> for u8 {
    #[inline(always)]
    fn from(val: RtcRst) -> u8 {
        RtcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sct {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Sct {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sct {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sct {
    #[inline(always)]
    fn from(val: u8) -> Sct {
        Sct::from_bits(val)
    }
}
impl From<Sct> for u8 {
    #[inline(always)]
    fn from(val: Sct) -> u8 {
        Sct::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SctRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctRst {
    #[inline(always)]
    fn from(val: u8) -> SctRst {
        SctRst::from_bits(val)
    }
}
impl From<SctRst> for u8 {
    #[inline(always)]
    fn from(val: SctRst) -> u8 {
        SctRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl SctclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivHalt {
        SctclkdivHalt::from_bits(val)
    }
}
impl From<SctclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivHalt) -> u8 {
        SctclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl SctclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivReqflag {
        SctclkdivReqflag::from_bits(val)
    }
}
impl From<SctclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivReqflag) -> u8 {
        SctclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl SctclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivReset {
        SctclkdivReset::from_bits(val)
    }
}
impl From<SctclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivReset) -> u8 {
        SctclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl SctclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctclkselSel {
        SctclkselSel::from_bits(val)
    }
}
impl From<SctclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctclkselSel) -> u8 {
        SctclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdma0 {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Sdma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdma0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdma0 {
    #[inline(always)]
    fn from(val: u8) -> Sdma0 {
        Sdma0::from_bits(val)
    }
}
impl From<Sdma0> for u8 {
    #[inline(always)]
    fn from(val: Sdma0) -> u8 {
        Sdma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdma1 {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Sdma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdma1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdma1 {
    #[inline(always)]
    fn from(val: u8) -> Sdma1 {
        Sdma1::from_bits(val)
    }
}
impl From<Sdma1> for u8 {
    #[inline(always)]
    fn from(val: Sdma1) -> u8 {
        Sdma1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SecCode(u32);
impl SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register will be read as 0x5."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register will be read as 0xA."]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SecCode {
    pub const fn from_bits(val: u32) -> SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SecCode {
    #[inline(always)]
    fn from(val: u32) -> SecCode {
        SecCode::from_bits(val)
    }
}
impl From<SecCode> for u32 {
    #[inline(always)]
    fn from(val: SecCode) -> u32 {
        SecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl1 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl SramCtrl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl1 {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl1 {
        SramCtrl1::from_bits(val)
    }
}
impl From<SramCtrl1> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl1) -> u8 {
        SramCtrl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SramCtrl1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl1Rst {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl1Rst {
        SramCtrl1Rst::from_bits(val)
    }
}
impl From<SramCtrl1Rst> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl1Rst) -> u8 {
        SramCtrl1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl2 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl SramCtrl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl2 {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl2 {
        SramCtrl2::from_bits(val)
    }
}
impl From<SramCtrl2> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl2) -> u8 {
        SramCtrl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramCtrl2Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SramCtrl2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramCtrl2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramCtrl2Rst {
    #[inline(always)]
    fn from(val: u8) -> SramCtrl2Rst {
        SramCtrl2Rst::from_bits(val)
    }
}
impl From<SramCtrl2Rst> for u8 {
    #[inline(always)]
    fn from(val: SramCtrl2Rst) -> u8 {
        SramCtrl2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status {
    #[doc = "no interrupt pending."]
    NO_INT = 0x0,
    #[doc = "interrupt pending."]
    PENDING = 0x01,
}
impl Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status {
    #[inline(always)]
    fn from(val: u8) -> Status {
        Status::from_bits(val)
    }
}
impl From<Status> for u8 {
    #[inline(always)]
    fn from(val: Status) -> u8 {
        Status::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwrReset(u32);
impl SwrReset {
    #[doc = "Bloc is not reset."]
    pub const RELEASED: Self = Self(0x0);
    #[doc = "Generate a software reset."]
    pub const ASSERTED: Self = Self(0x5a00_0001);
}
impl SwrReset {
    pub const fn from_bits(val: u32) -> SwrReset {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwrReset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RELEASED"),
            0x5a00_0001 => f.write_str("ASSERTED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwrReset {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RELEASED"),
            0x5a00_0001 => defmt::write!(f, "ASSERTED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwrReset {
    #[inline(always)]
    fn from(val: u32) -> SwrReset {
        SwrReset::from_bits(val)
    }
}
impl From<SwrReset> for u32 {
    #[inline(always)]
    fn from(val: SwrReset) -> u32 {
        SwrReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sync0Apb {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Sync0Apb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sync0Apb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sync0Apb {
    #[inline(always)]
    fn from(val: u8) -> Sync0Apb {
        Sync0Apb::from_bits(val)
    }
}
impl From<Sync0Apb> for u8 {
    #[inline(always)]
    fn from(val: Sync0Apb) -> u8 {
        Sync0Apb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sync1Apb {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Sync1Apb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sync1Apb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sync1Apb {
    #[inline(always)]
    fn from(val: u8) -> Sync1Apb {
        Sync1Apb::from_bits(val)
    }
}
impl From<Sync1Apb> for u8 {
    #[inline(always)]
    fn from(val: Sync1Apb) -> u8 {
        Sync1Apb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syscon {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Syscon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscon {
    #[inline(always)]
    fn from(val: u8) -> Syscon {
        Syscon::from_bits(val)
    }
}
impl From<Syscon> for u8 {
    #[inline(always)]
    fn from(val: Syscon) -> u8 {
        Syscon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysctl {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Sysctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysctl {
    #[inline(always)]
    fn from(val: u8) -> Sysctl {
        Sysctl::from_bits(val)
    }
}
impl From<Sysctl> for u8 {
    #[inline(always)]
    fn from(val: Sysctl) -> u8 {
        Sysctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysctlRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SysctlRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysctlRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysctlRst {
    #[inline(always)]
    fn from(val: u8) -> SysctlRst {
        SysctlRst::from_bits(val)
    }
}
impl From<SysctlRst> for u8 {
    #[inline(always)]
    fn from(val: SysctlRst) -> u8 {
        SysctlRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Halt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Systickclkdiv0Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Halt {
        Systickclkdiv0Halt::from_bits(val)
    }
}
impl From<Systickclkdiv0Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Halt) -> u8 {
        Systickclkdiv0Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Systickclkdiv0Reqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reqflag {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reqflag {
        Systickclkdiv0Reqflag::from_bits(val)
    }
}
impl From<Systickclkdiv0Reqflag> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reqflag) -> u8 {
        Systickclkdiv0Reqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Systickclkdiv0Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reset {
        Systickclkdiv0Reset::from_bits(val)
    }
}
impl From<Systickclkdiv0Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reset) -> u8 {
        Systickclkdiv0Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel0Sel {
    #[doc = "System Tick 0 divided clock."]
    ENUM_0X0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "No clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Systickclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel0Sel {
        Systickclksel0Sel::from_bits(val)
    }
}
impl From<Systickclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel0Sel) -> u8 {
        Systickclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Timer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0 {
    #[inline(always)]
    fn from(val: u8) -> Timer0 {
        Timer0::from_bits(val)
    }
}
impl From<Timer0> for u8 {
    #[inline(always)]
    fn from(val: Timer0) -> u8 {
        Timer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer0Rst {
        Timer0Rst::from_bits(val)
    }
}
impl From<Timer0Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer0Rst) -> u8 {
        Timer0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Timer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1 {
    #[inline(always)]
    fn from(val: u8) -> Timer1 {
        Timer1::from_bits(val)
    }
}
impl From<Timer1> for u8 {
    #[inline(always)]
    fn from(val: Timer1) -> u8 {
        Timer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer1Rst {
        Timer1Rst::from_bits(val)
    }
}
impl From<Timer1Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer1Rst) -> u8 {
        Timer1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Timer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2 {
    #[inline(always)]
    fn from(val: u8) -> Timer2 {
        Timer2::from_bits(val)
    }
}
impl From<Timer2> for u8 {
    #[inline(always)]
    fn from(val: Timer2) -> u8 {
        Timer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer2Rst {
        Timer2Rst::from_bits(val)
    }
}
impl From<Timer2Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer2Rst) -> u8 {
        Timer2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Timer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3 {
    #[inline(always)]
    fn from(val: u8) -> Timer3 {
        Timer3::from_bits(val)
    }
}
impl From<Timer3> for u8 {
    #[inline(always)]
    fn from(val: Timer3) -> u8 {
        Timer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer3Rst {
        Timer3Rst::from_bits(val)
    }
}
impl From<Timer3Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer3Rst) -> u8 {
        Timer3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4 {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Timer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4 {
    #[inline(always)]
    fn from(val: u8) -> Timer4 {
        Timer4::from_bits(val)
    }
}
impl From<Timer4> for u8 {
    #[inline(always)]
    fn from(val: Timer4) -> u8 {
        Timer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4Rst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Timer4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer4Rst {
        Timer4Rst::from_bits(val)
    }
}
impl From<Timer4Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer4Rst) -> u8 {
        Timer4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl TraceclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivHalt {
        TraceclkdivHalt::from_bits(val)
    }
}
impl From<TraceclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivHalt) -> u8 {
        TraceclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl TraceclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReqflag {
        TraceclkdivReqflag::from_bits(val)
    }
}
impl From<TraceclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReqflag) -> u8 {
        TraceclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl TraceclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReset {
        TraceclkdivReset::from_bits(val)
    }
}
impl From<TraceclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReset) -> u8 {
        TraceclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkselSel {
    #[doc = "Trace divided clock."]
    ENUM_0X0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X2 = 0x02,
    #[doc = "No clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "No clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl TraceclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TraceclkselSel {
        TraceclkselSel::from_bits(val)
    }
}
impl From<TraceclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TraceclkselSel) -> u8 {
        TraceclkselSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Unlockcode(u32);
impl Unlockcode {
    #[doc = "HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    pub const UNLOCK: Self = Self(0xc33c_a55a);
}
impl Unlockcode {
    pub const fn from_bits(val: u32) -> Unlockcode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Unlockcode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xc33c_a55a => f.write_str("UNLOCK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unlockcode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xc33c_a55a => defmt::write!(f, "UNLOCK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Unlockcode {
    #[inline(always)]
    fn from(val: u32) -> Unlockcode {
        Unlockcode::from_bits(val)
    }
}
impl From<Unlockcode> for u32 {
    #[inline(always)]
    fn from(val: Unlockcode) -> u32 {
        Unlockcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0 {
    #[doc = "Automatic clock gating is not overridden."]
    DISABLE = 0x0,
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 0x01,
}
impl Usb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0 {
    #[inline(always)]
    fn from(val: u8) -> Usb0 {
        Usb0::from_bits(val)
    }
}
impl From<Usb0> for u8 {
    #[inline(always)]
    fn from(val: Usb0) -> u8 {
        Usb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0Dev {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb0Dev {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0Dev {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0Dev {
    #[inline(always)]
    fn from(val: u8) -> Usb0Dev {
        Usb0Dev::from_bits(val)
    }
}
impl From<Usb0Dev> for u8 {
    #[inline(always)]
    fn from(val: Usb0Dev) -> u8 {
        Usb0Dev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0DevRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb0DevRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0DevRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0DevRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0DevRst {
        Usb0DevRst::from_bits(val)
    }
}
impl From<Usb0DevRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0DevRst) -> u8 {
        Usb0DevRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0Hostm {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb0Hostm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0Hostm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0Hostm {
    #[inline(always)]
    fn from(val: u8) -> Usb0Hostm {
        Usb0Hostm::from_bits(val)
    }
}
impl From<Usb0Hostm> for u8 {
    #[inline(always)]
    fn from(val: Usb0Hostm) -> u8 {
        Usb0Hostm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0HostmRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb0HostmRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0HostmRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0HostmRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0HostmRst {
        Usb0HostmRst::from_bits(val)
    }
}
impl From<Usb0HostmRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0HostmRst) -> u8 {
        Usb0HostmRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0Hosts {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb0Hosts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0Hosts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0Hosts {
    #[inline(always)]
    fn from(val: u8) -> Usb0Hosts {
        Usb0Hosts::from_bits(val)
    }
}
impl From<Usb0Hosts> for u8 {
    #[inline(always)]
    fn from(val: Usb0Hosts) -> u8 {
        Usb0Hosts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0HostsRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb0HostsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0HostsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0HostsRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0HostsRst {
        Usb0HostsRst::from_bits(val)
    }
}
impl From<Usb0HostsRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0HostsRst) -> u8 {
        Usb0HostsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl Usb0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivHalt {
        Usb0clkdivHalt::from_bits(val)
    }
}
impl From<Usb0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivHalt) -> u8 {
        Usb0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Usb0clkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivReqflag {
        Usb0clkdivReqflag::from_bits(val)
    }
}
impl From<Usb0clkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivReqflag) -> u8 {
        Usb0clkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Usb0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivReset {
        Usb0clkdivReset::from_bits(val)
    }
}
impl From<Usb0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivReset) -> u8 {
        Usb0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkselSel {
    #[doc = "Main clock."]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0X1 = 0x01,
    #[doc = "No clock."]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3 = 0x03,
    #[doc = "No clock."]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0X5 = 0x05,
    #[doc = "No clock."]
    ENUM_0X6 = 0x06,
    #[doc = "No clock."]
    ENUM_0X7 = 0x07,
}
impl Usb0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkselSel {
        Usb0clkselSel::from_bits(val)
    }
}
impl From<Usb0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkselSel) -> u8 {
        Usb0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0needclkstatDevNeedclk {
    #[doc = "USB0-FS Device clock is low."]
    LOW = 0x0,
    #[doc = "USB0-FS Device clock is high."]
    HIGH = 0x01,
}
impl Usb0needclkstatDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0needclkstatDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0needclkstatDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb0needclkstatDevNeedclk {
        Usb0needclkstatDevNeedclk::from_bits(val)
    }
}
impl From<Usb0needclkstatDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb0needclkstatDevNeedclk) -> u8 {
        Usb0needclkstatDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0needclkstatHostNeedclk {
    #[doc = "USB0-FS Host clock is low."]
    LOW = 0x0,
    #[doc = "USB0-FS Host clock is high."]
    HIGH = 0x01,
}
impl Usb0needclkstatHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0needclkstatHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0needclkstatHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb0needclkstatHostNeedclk {
        Usb0needclkstatHostNeedclk::from_bits(val)
    }
}
impl From<Usb0needclkstatHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb0needclkstatHostNeedclk) -> u8 {
        Usb0needclkstatHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1Dev {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb1Dev {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1Dev {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1Dev {
    #[inline(always)]
    fn from(val: u8) -> Usb1Dev {
        Usb1Dev::from_bits(val)
    }
}
impl From<Usb1Dev> for u8 {
    #[inline(always)]
    fn from(val: Usb1Dev) -> u8 {
        Usb1Dev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1DevRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1DevRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1DevRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1DevRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1DevRst {
        Usb1DevRst::from_bits(val)
    }
}
impl From<Usb1DevRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1DevRst) -> u8 {
        Usb1DevRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1Host {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb1Host {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1Host {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1Host {
    #[inline(always)]
    fn from(val: u8) -> Usb1Host {
        Usb1Host::from_bits(val)
    }
}
impl From<Usb1Host> for u8 {
    #[inline(always)]
    fn from(val: Usb1Host) -> u8 {
        Usb1Host::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1HostRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1HostRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1HostRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1HostRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1HostRst {
        Usb1HostRst::from_bits(val)
    }
}
impl From<Usb1HostRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1HostRst) -> u8 {
        Usb1HostRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1Phy {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb1Phy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1Phy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1Phy {
    #[inline(always)]
    fn from(val: u8) -> Usb1Phy {
        Usb1Phy::from_bits(val)
    }
}
impl From<Usb1Phy> for u8 {
    #[inline(always)]
    fn from(val: Usb1Phy) -> u8 {
        Usb1Phy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1PhyRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1PhyRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1PhyRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1PhyRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1PhyRst {
        Usb1PhyRst::from_bits(val)
    }
}
impl From<Usb1PhyRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1PhyRst) -> u8 {
        Usb1PhyRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1Ram {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Usb1Ram {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1Ram {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1Ram {
    #[inline(always)]
    fn from(val: u8) -> Usb1Ram {
        Usb1Ram::from_bits(val)
    }
}
impl From<Usb1Ram> for u8 {
    #[inline(always)]
    fn from(val: Usb1Ram) -> u8 {
        Usb1Ram::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1RamRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl Usb1RamRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1RamRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1RamRst {
    #[inline(always)]
    fn from(val: u8) -> Usb1RamRst {
        Usb1RamRst::from_bits(val)
    }
}
impl From<Usb1RamRst> for u8 {
    #[inline(always)]
    fn from(val: Usb1RamRst) -> u8 {
        Usb1RamRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1needclkstatDevNeedclk {
    #[doc = "DEV_NEEDCLK is low."]
    LOW = 0x0,
    #[doc = "DEV_NEEDCLK is high."]
    HIGH = 0x01,
}
impl Usb1needclkstatDevNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1needclkstatDevNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1needclkstatDevNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb1needclkstatDevNeedclk {
        Usb1needclkstatDevNeedclk::from_bits(val)
    }
}
impl From<Usb1needclkstatDevNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb1needclkstatDevNeedclk) -> u8 {
        Usb1needclkstatDevNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1needclkstatHostNeedclk {
    #[doc = "HOST_NEEDCLK is low."]
    LOW = 0x0,
    #[doc = "HOST_NEEDCLK is high."]
    HIGH = 0x01,
}
impl Usb1needclkstatHostNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1needclkstatHostNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1needclkstatHostNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Usb1needclkstatHostNeedclk {
        Usb1needclkstatHostNeedclk::from_bits(val)
    }
}
impl From<Usb1needclkstatHostNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Usb1needclkstatHostNeedclk) -> u8 {
        Usb1needclkstatHostNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Utick {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Utick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Utick {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Utick {
    #[inline(always)]
    fn from(val: u8) -> Utick {
        Utick::from_bits(val)
    }
}
impl From<Utick> for u8 {
    #[inline(always)]
    fn from(val: Utick) -> u8 {
        Utick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl UtickRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickRst {
    #[inline(always)]
    fn from(val: u8) -> UtickRst {
        UtickRst::from_bits(val)
    }
}
impl From<UtickRst> for u8 {
    #[inline(always)]
    fn from(val: UtickRst) -> u8 {
        UtickRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val {
    #[doc = "P+ is smaller than P-."]
    SMALLER = 0x0,
    #[doc = "P+ is greater than P-."]
    GREATER = 0x01,
}
impl Val {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val {
    #[inline(always)]
    fn from(val: u8) -> Val {
        Val::from_bits(val)
    }
}
impl From<Val> for u8 {
    #[inline(always)]
    fn from(val: Val) -> u8 {
        Val::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl WdtclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> WdtclkdivHalt {
        WdtclkdivHalt::from_bits(val)
    }
}
impl From<WdtclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: WdtclkdivHalt) -> u8 {
        WdtclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtclkdivReqflag {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl WdtclkdivReqflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtclkdivReqflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtclkdivReqflag {
    #[inline(always)]
    fn from(val: u8) -> WdtclkdivReqflag {
        WdtclkdivReqflag::from_bits(val)
    }
}
impl From<WdtclkdivReqflag> for u8 {
    #[inline(always)]
    fn from(val: WdtclkdivReqflag) -> u8 {
        WdtclkdivReqflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdtclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl WdtclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> WdtclkdivReset {
        WdtclkdivReset::from_bits(val)
    }
}
impl From<WdtclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: WdtclkdivReset) -> u8 {
        WdtclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt {
    #[doc = "Disable Clock."]
    DISABLE = 0x0,
    #[doc = "Enable Clock."]
    ENABLE = 0x01,
}
impl Wwdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt {
    #[inline(always)]
    fn from(val: u8) -> Wwdt {
        Wwdt::from_bits(val)
    }
}
impl From<Wwdt> for u8 {
    #[inline(always)]
    fn from(val: Wwdt) -> u8 {
        Wwdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WwdtRst {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl WwdtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WwdtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WwdtRst {
    #[inline(always)]
    fn from(val: u8) -> WwdtRst {
        WwdtRst::from_bits(val)
    }
}
impl From<WwdtRst> for u8 {
    #[inline(always)]
    fn from(val: WwdtRst) -> u8 {
        WwdtRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XoCalClkEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl XoCalClkEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XoCalClkEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XoCalClkEna {
    #[inline(always)]
    fn from(val: u8) -> XoCalClkEna {
        XoCalClkEna::from_bits(val)
    }
}
impl From<XoCalClkEna> for u8 {
    #[inline(always)]
    fn from(val: XoCalClkEna) -> u8 {
        XoCalClkEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtal32mhzFreqmEna {
    #[doc = "The clock is not enabled."]
    DISABLE = 0x0,
    #[doc = "The clock is enabled."]
    ENABLE = 0x01,
}
impl Xtal32mhzFreqmEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtal32mhzFreqmEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtal32mhzFreqmEna {
    #[inline(always)]
    fn from(val: u8) -> Xtal32mhzFreqmEna {
        Xtal32mhzFreqmEna::from_bits(val)
    }
}
impl From<Xtal32mhzFreqmEna> for u8 {
    #[inline(always)]
    fn from(val: Xtal32mhzFreqmEna) -> u8 {
        Xtal32mhzFreqmEna::to_bits(val)
    }
}
