#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AcbufPassEnable {
    #[doc = "XO AC buffer bypass is disabled."]
    DISABLE = 0x0,
    #[doc = "XO AC buffer bypass is enabled."]
    ENABLE = 0x01,
}
impl AcbufPassEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AcbufPassEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AcbufPassEnable {
    #[inline(always)]
    fn from(val: u8) -> AcbufPassEnable {
        AcbufPassEnable::from_bits(val)
    }
}
impl From<AcbufPassEnable> for u8 {
    #[inline(always)]
    fn from(val: AcbufPassEnable) -> u8 {
        AcbufPassEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodcoreIntEnable {
    #[doc = "BOD CORE interrupt is disabled."]
    DISABLE = 0x0,
    #[doc = "BOD CORE interrupt is enabled."]
    ENABLE = 0x01,
}
impl BodcoreIntEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodcoreIntEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodcoreIntEnable {
    #[inline(always)]
    fn from(val: u8) -> BodcoreIntEnable {
        BodcoreIntEnable::from_bits(val)
    }
}
impl From<BodcoreIntEnable> for u8 {
    #[inline(always)]
    fn from(val: BodcoreIntEnable) -> u8 {
        BodcoreIntEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodcoreIntStatus {
    #[doc = "No interrupt pending.."]
    NOT_PENDING = 0x0,
    #[doc = "Interrupt pending.."]
    PENDING = 0x01,
}
impl BodcoreIntStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodcoreIntStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodcoreIntStatus {
    #[inline(always)]
    fn from(val: u8) -> BodcoreIntStatus {
        BodcoreIntStatus::from_bits(val)
    }
}
impl From<BodcoreIntStatus> for u8 {
    #[inline(always)]
    fn from(val: BodcoreIntStatus) -> u8 {
        BodcoreIntStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodcoreStatus {
    #[doc = "No interrupt pending.."]
    NOT_PENDING = 0x0,
    #[doc = "Interrupt pending.."]
    PENDING = 0x01,
}
impl BodcoreStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodcoreStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodcoreStatus {
    #[inline(always)]
    fn from(val: u8) -> BodcoreStatus {
        BodcoreStatus::from_bits(val)
    }
}
impl From<BodcoreStatus> for u8 {
    #[inline(always)]
    fn from(val: BodcoreStatus) -> u8 {
        BodcoreStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodcoreVal {
    #[doc = "CORE voltage level is below the threshold."]
    NOT_OK = 0x0,
    #[doc = "CORE voltage level is above the threshold."]
    OK = 0x01,
}
impl BodcoreVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodcoreVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodcoreVal {
    #[inline(always)]
    fn from(val: u8) -> BodcoreVal {
        BodcoreVal::from_bits(val)
    }
}
impl From<BodcoreVal> for u8 {
    #[inline(always)]
    fn from(val: BodcoreVal) -> u8 {
        BodcoreVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatIntEnable {
    #[doc = "BOD VBAT interrupt is disabled."]
    DISABLE = 0x0,
    #[doc = "BOD VBAT interrupt is enabled."]
    ENABLE = 0x01,
}
impl BodvbatIntEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatIntEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatIntEnable {
    #[inline(always)]
    fn from(val: u8) -> BodvbatIntEnable {
        BodvbatIntEnable::from_bits(val)
    }
}
impl From<BodvbatIntEnable> for u8 {
    #[inline(always)]
    fn from(val: BodvbatIntEnable) -> u8 {
        BodvbatIntEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatIntStatus {
    #[doc = "No interrupt pending.."]
    NOT_PENDING = 0x0,
    #[doc = "Interrupt pending.."]
    PENDING = 0x01,
}
impl BodvbatIntStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatIntStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatIntStatus {
    #[inline(always)]
    fn from(val: u8) -> BodvbatIntStatus {
        BodvbatIntStatus::from_bits(val)
    }
}
impl From<BodvbatIntStatus> for u8 {
    #[inline(always)]
    fn from(val: BodvbatIntStatus) -> u8 {
        BodvbatIntStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatStatus {
    #[doc = "No interrupt pending.."]
    NOT_PENDING = 0x0,
    #[doc = "Interrupt pending.."]
    PENDING = 0x01,
}
impl BodvbatStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatStatus {
    #[inline(always)]
    fn from(val: u8) -> BodvbatStatus {
        BodvbatStatus::from_bits(val)
    }
}
impl From<BodvbatStatus> for u8 {
    #[inline(always)]
    fn from(val: BodvbatStatus) -> u8 {
        BodvbatStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatVal {
    #[doc = "VBAT voltage level is below the threshold."]
    NOT_OK = 0x0,
    #[doc = "VBAT voltage level is above the threshold."]
    OK = 0x01,
}
impl BodvbatVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatVal {
    #[inline(always)]
    fn from(val: u8) -> BodvbatVal {
        BodvbatVal::from_bits(val)
    }
}
impl From<BodvbatVal> for u8 {
    #[inline(always)]
    fn from(val: BodvbatVal) -> u8 {
        BodvbatVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bypass {
    #[doc = "Disable bypass mode (for normal operations)."]
    DISABLE = 0x0,
    #[doc = "Activate LDO bypass."]
    ENABLE = 0x01,
}
impl Bypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypass {
    #[inline(always)]
    fn from(val: u8) -> Bypass {
        Bypass::from_bits(val)
    }
}
impl From<Bypass> for u8 {
    #[inline(always)]
    fn from(val: Bypass) -> u8 {
        Bypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkValid {
    #[doc = "No output clock present (None of 12 MHz, 48 MHz or 96 MHz clock is available)."]
    NOCLKOUT = 0x0,
    #[doc = "Clock is present (12 MHz, 48 MHz or 96 MHz can be output if they are enable respectively by FRO192M_CTRL.ENA_12MHZCLK/ENA_48MHZCLK/ENA_96MHZCLK)."]
    CLKOUT = 0x01,
}
impl ClkValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkValid {
    #[inline(always)]
    fn from(val: u8) -> ClkValid {
        ClkValid::from_bits(val)
    }
}
impl From<ClkValid> for u8 {
    #[inline(always)]
    fn from(val: ClkValid) -> u8 {
        ClkValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcIntEnable {
    #[doc = "DCDC interrupt is disabled."]
    DISABLE = 0x0,
    #[doc = "DCDC interrupt is enabled."]
    ENABLE = 0x01,
}
impl DcdcIntEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcIntEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcIntEnable {
    #[inline(always)]
    fn from(val: u8) -> DcdcIntEnable {
        DcdcIntEnable::from_bits(val)
    }
}
impl From<DcdcIntEnable> for u8 {
    #[inline(always)]
    fn from(val: DcdcIntEnable) -> u8 {
        DcdcIntEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcIntStatus {
    #[doc = "No interrupt pending.."]
    NOT_PENDING = 0x0,
    #[doc = "Interrupt pending.."]
    PENDING = 0x01,
}
impl DcdcIntStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcIntStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcIntStatus {
    #[inline(always)]
    fn from(val: u8) -> DcdcIntStatus {
        DcdcIntStatus::from_bits(val)
    }
}
impl From<DcdcIntStatus> for u8 {
    #[inline(always)]
    fn from(val: DcdcIntStatus) -> u8 {
        DcdcIntStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcStatus {
    #[doc = "No interrupt pending.."]
    NOT_PENDING = 0x0,
    #[doc = "Interrupt pending.."]
    PENDING = 0x01,
}
impl DcdcStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcStatus {
    #[inline(always)]
    fn from(val: u8) -> DcdcStatus {
        DcdcStatus::from_bits(val)
    }
}
impl From<DcdcStatus> for u8 {
    #[inline(always)]
    fn from(val: DcdcStatus) -> u8 {
        DcdcStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcVal {
    #[doc = "DCDC output Voltage is below the targeted regulation level."]
    NOT_OK = 0x0,
    #[doc = "DCDC output Voltage is above the targeted regulation level."]
    OK = 0x01,
}
impl DcdcVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcVal {
    #[inline(always)]
    fn from(val: u8) -> DcdcVal {
        DcdcVal::from_bits(val)
    }
}
impl From<DcdcVal> for u8 {
    #[inline(always)]
    fn from(val: DcdcVal) -> u8 {
        DcdcVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EIv0 {
    #[doc = "First INV-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "First INV-based ringo is enabled."]
    ENABLE = 0x01,
}
impl EIv0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EIv0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EIv0 {
    #[inline(always)]
    fn from(val: u8) -> EIv0 {
        EIv0::from_bits(val)
    }
}
impl From<EIv0> for u8 {
    #[inline(always)]
    fn from(val: EIv0) -> u8 {
        EIv0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EIv1 {
    #[doc = "Second INV-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Second INV-based ringo is enabled."]
    ENABLE = 0x01,
}
impl EIv1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EIv1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EIv1 {
    #[inline(always)]
    fn from(val: u8) -> EIv1 {
        EIv1::from_bits(val)
    }
}
impl From<EIv1> for u8 {
    #[inline(always)]
    fn from(val: EIv1) -> u8 {
        EIv1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENd0 {
    #[doc = "First NAND2-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "First NAND2-based ringo is enabled."]
    ENABLE = 0x01,
}
impl ENd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENd0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENd0 {
    #[inline(always)]
    fn from(val: u8) -> ENd0 {
        ENd0::from_bits(val)
    }
}
impl From<ENd0> for u8 {
    #[inline(always)]
    fn from(val: ENd0) -> u8 {
        ENd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENd1 {
    #[doc = "Second NAND2-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Second NAND2-based ringo is enabled."]
    ENABLE = 0x01,
}
impl ENd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENd1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENd1 {
    #[inline(always)]
    fn from(val: u8) -> ENd1 {
        ENd1::from_bits(val)
    }
}
impl From<ENd1> for u8 {
    #[inline(always)]
    fn from(val: ENd1) -> u8 {
        ENd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENr0 {
    #[doc = "First NOR2-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "First NOR2-based ringo is enabled."]
    ENABLE = 0x01,
}
impl ENr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENr0 {
    #[inline(always)]
    fn from(val: u8) -> ENr0 {
        ENr0::from_bits(val)
    }
}
impl From<ENr0> for u8 {
    #[inline(always)]
    fn from(val: ENr0) -> u8 {
        ENr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENr1 {
    #[doc = "Second NORD2-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Second NORD2-based ringo is enabled."]
    ENABLE = 0x01,
}
impl ENr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENr1 {
    #[inline(always)]
    fn from(val: u8) -> ENr1 {
        ENr1::from_bits(val)
    }
}
impl From<ENr1> for u8 {
    #[inline(always)]
    fn from(val: ENr1) -> u8 {
        ENr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EPn0 {
    #[doc = "First PN-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "First PN-based ringo is enabled."]
    ENABLE = 0x01,
}
impl EPn0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EPn0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EPn0 {
    #[inline(always)]
    fn from(val: u8) -> EPn0 {
        EPn0::from_bits(val)
    }
}
impl From<EPn0> for u8 {
    #[inline(always)]
    fn from(val: EPn0) -> u8 {
        EPn0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EPn1 {
    #[doc = "Second PN-based ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Second PN-based ringo is enabled."]
    ENABLE = 0x01,
}
impl EPn1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EPn1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EPn1 {
    #[inline(always)]
    fn from(val: u8) -> EPn1 {
        EPn1::from_bits(val)
    }
}
impl From<EPn1> for u8 {
    #[inline(always)]
    fn from(val: EPn1) -> u8 {
        EPn1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena12mhzclk {
    #[doc = "12 MHz clock is disabled."]
    DISABLE = 0x0,
    #[doc = "12 MHz clock is enabled."]
    ENABLE = 0x01,
}
impl Ena12mhzclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena12mhzclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena12mhzclk {
    #[inline(always)]
    fn from(val: u8) -> Ena12mhzclk {
        Ena12mhzclk::from_bits(val)
    }
}
impl From<Ena12mhzclk> for u8 {
    #[inline(always)]
    fn from(val: Ena12mhzclk) -> u8 {
        Ena12mhzclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ena96mhzclk {
    #[doc = "96 MHz clock is disabled."]
    DISABLE = 0x0,
    #[doc = "96 MHz clock is enabled."]
    ENABLE = 0x01,
}
impl Ena96mhzclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena96mhzclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena96mhzclk {
    #[inline(always)]
    fn from(val: u8) -> Ena96mhzclk {
        Ena96mhzclk::from_bits(val)
    }
}
impl From<Ena96mhzclk> for u8 {
    #[inline(always)]
    fn from(val: Ena96mhzclk) -> u8 {
        Ena96mhzclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnablePllUsbOut {
    #[doc = "High speed Crystal oscillator output to USB HS PLL is disabled."]
    DISABLE = 0x0,
    #[doc = "High speed Crystal oscillator output to USB HS PLL is enabled."]
    ENABLE = 0x01,
}
impl EnablePllUsbOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnablePllUsbOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnablePllUsbOut {
    #[inline(always)]
    fn from(val: u8) -> EnablePllUsbOut {
        EnablePllUsbOut::from_bits(val)
    }
}
impl From<EnablePllUsbOut> for u8 {
    #[inline(always)]
    fn from(val: EnablePllUsbOut) -> u8 {
        EnablePllUsbOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnableSystemClkOut {
    #[doc = "High speed Crystal oscillator output to CPU system is disabled."]
    DISABLE = 0x0,
    #[doc = "High speed Crystal oscillator output to CPU system is enabled."]
    ENABLE = 0x01,
}
impl EnableSystemClkOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnableSystemClkOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnableSystemClkOut {
    #[inline(always)]
    fn from(val: u8) -> EnableSystemClkOut {
        EnableSystemClkOut::from_bits(val)
    }
}
impl From<EnableSystemClkOut> for u8 {
    #[inline(always)]
    fn from(val: EnableSystemClkOut) -> u8 {
        EnableSystemClkOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashInitError {
    #[doc = "No error."]
    NOERROR = 0x0,
    #[doc = "At least one error occured during flash initialization.."]
    ERROR = 0x01,
}
impl FlashInitError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashInitError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashInitError {
    #[inline(always)]
    fn from(val: u8) -> FlashInitError {
        FlashInitError::from_bits(val)
    }
}
impl From<FlashInitError> for u8 {
    #[inline(always)]
    fn from(val: FlashInitError) -> u8 {
        FlashInitError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashPwrdwn {
    #[doc = "Flash is not in power down mode."]
    PWRUP = 0x0,
    #[doc = "Flash is in power down mode."]
    PWRDWN = 0x01,
}
impl FlashPwrdwn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashPwrdwn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashPwrdwn {
    #[inline(always)]
    fn from(val: u8) -> FlashPwrdwn {
        FlashPwrdwn::from_bits(val)
    }
}
impl From<FlashPwrdwn> for u8 {
    #[inline(always)]
    fn from(val: FlashPwrdwn) -> u8 {
        FlashPwrdwn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fro192mTrimSrc {
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    EFUSE = 0x0,
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    FRO192MCTRL = 0x01,
}
impl Fro192mTrimSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fro192mTrimSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fro192mTrimSrc {
    #[inline(always)]
    fn from(val: u8) -> Fro192mTrimSrc {
        Fro192mTrimSrc::from_bits(val)
    }
}
impl From<Fro192mTrimSrc> for u8 {
    #[inline(always)]
    fn from(val: Fro192mTrimSrc) -> u8 {
        Fro192mTrimSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Highz {
    #[doc = "Output in High normal state."]
    NORMALMPEDANCE = 0x0,
    #[doc = "Output in High Impedance state."]
    HIGHIMPEDANCE = 0x01,
}
impl Highz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Highz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Highz {
    #[inline(always)]
    fn from(val: u8) -> Highz {
        Highz::from_bits(val)
    }
}
impl From<Highz> for u8 {
    #[inline(always)]
    fn from(val: Highz) -> u8 {
        Highz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo0CtrlFs {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl Ringo0CtrlFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo0CtrlFs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo0CtrlFs {
    #[inline(always)]
    fn from(val: u8) -> Ringo0CtrlFs {
        Ringo0CtrlFs::from_bits(val)
    }
}
impl From<Ringo0CtrlFs> for u8 {
    #[inline(always)]
    fn from(val: Ringo0CtrlFs) -> u8 {
        Ringo0CtrlFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo0CtrlPd {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl Ringo0CtrlPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo0CtrlPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo0CtrlPd {
    #[inline(always)]
    fn from(val: u8) -> Ringo0CtrlPd {
        Ringo0CtrlPd::from_bits(val)
    }
}
impl From<Ringo0CtrlPd> for u8 {
    #[inline(always)]
    fn from(val: Ringo0CtrlPd) -> u8 {
        Ringo0CtrlPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlEM2 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo1CtrlEM2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlEM2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlEM2 {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlEM2 {
        Ringo1CtrlEM2::from_bits(val)
    }
}
impl From<Ringo1CtrlEM2> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlEM2) -> u8 {
        Ringo1CtrlEM2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlEM3 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo1CtrlEM3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlEM3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlEM3 {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlEM3 {
        Ringo1CtrlEM3::from_bits(val)
    }
}
impl From<Ringo1CtrlEM3> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlEM3) -> u8 {
        Ringo1CtrlEM3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlEM4 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo1CtrlEM4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlEM4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlEM4 {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlEM4 {
        Ringo1CtrlEM4::from_bits(val)
    }
}
impl From<Ringo1CtrlEM4> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlEM4) -> u8 {
        Ringo1CtrlEM4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlEM5 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo1CtrlEM5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlEM5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlEM5 {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlEM5 {
        Ringo1CtrlEM5::from_bits(val)
    }
}
impl From<Ringo1CtrlEM5> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlEM5) -> u8 {
        Ringo1CtrlEM5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlER24 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo1CtrlER24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlER24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlER24 {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlER24 {
        Ringo1CtrlER24::from_bits(val)
    }
}
impl From<Ringo1CtrlER24> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlER24) -> u8 {
        Ringo1CtrlER24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlER35 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo1CtrlER35 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlER35 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlER35 {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlER35 {
        Ringo1CtrlER35::from_bits(val)
    }
}
impl From<Ringo1CtrlER35> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlER35) -> u8 {
        Ringo1CtrlER35::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlFs {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl Ringo1CtrlFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlFs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlFs {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlFs {
        Ringo1CtrlFs::from_bits(val)
    }
}
impl From<Ringo1CtrlFs> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlFs) -> u8 {
        Ringo1CtrlFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlPd {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl Ringo1CtrlPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlPd {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlPd {
        Ringo1CtrlPd::from_bits(val)
    }
}
impl From<Ringo1CtrlPd> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlPd) -> u8 {
        Ringo1CtrlPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo1CtrlS {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl Ringo1CtrlS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo1CtrlS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo1CtrlS {
    #[inline(always)]
    fn from(val: u8) -> Ringo1CtrlS {
        Ringo1CtrlS::from_bits(val)
    }
}
impl From<Ringo1CtrlS> for u8 {
    #[inline(always)]
    fn from(val: Ringo1CtrlS) -> u8 {
        Ringo1CtrlS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlEM2 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo2CtrlEM2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlEM2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlEM2 {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlEM2 {
        Ringo2CtrlEM2::from_bits(val)
    }
}
impl From<Ringo2CtrlEM2> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlEM2) -> u8 {
        Ringo2CtrlEM2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlEM3 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo2CtrlEM3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlEM3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlEM3 {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlEM3 {
        Ringo2CtrlEM3::from_bits(val)
    }
}
impl From<Ringo2CtrlEM3> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlEM3) -> u8 {
        Ringo2CtrlEM3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlEM4 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo2CtrlEM4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlEM4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlEM4 {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlEM4 {
        Ringo2CtrlEM4::from_bits(val)
    }
}
impl From<Ringo2CtrlEM4> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlEM4) -> u8 {
        Ringo2CtrlEM4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlEM5 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo2CtrlEM5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlEM5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlEM5 {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlEM5 {
        Ringo2CtrlEM5::from_bits(val)
    }
}
impl From<Ringo2CtrlEM5> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlEM5) -> u8 {
        Ringo2CtrlEM5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlER24 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo2CtrlER24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlER24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlER24 {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlER24 {
        Ringo2CtrlER24::from_bits(val)
    }
}
impl From<Ringo2CtrlER24> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlER24) -> u8 {
        Ringo2CtrlER24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlER35 {
    #[doc = "Ringo is disabled."]
    DISABLE = 0x0,
    #[doc = "Ringo is enabled."]
    ENABLE = 0x01,
}
impl Ringo2CtrlER35 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlER35 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlER35 {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlER35 {
        Ringo2CtrlER35::from_bits(val)
    }
}
impl From<Ringo2CtrlER35> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlER35) -> u8 {
        Ringo2CtrlER35::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlFs {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl Ringo2CtrlFs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlFs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlFs {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlFs {
        Ringo2CtrlFs::from_bits(val)
    }
}
impl From<Ringo2CtrlFs> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlFs) -> u8 {
        Ringo2CtrlFs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlPd {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl Ringo2CtrlPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlPd {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlPd {
        Ringo2CtrlPd::from_bits(val)
    }
}
impl From<Ringo2CtrlPd> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlPd) -> u8 {
        Ringo2CtrlPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ringo2CtrlS {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl Ringo2CtrlS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ringo2CtrlS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ringo2CtrlS {
    #[inline(always)]
    fn from(val: u8) -> Ringo2CtrlS {
        Ringo2CtrlS::from_bits(val)
    }
}
impl From<Ringo2CtrlS> for u8 {
    #[inline(always)]
    fn from(val: Ringo2CtrlS) -> u8 {
        Ringo2CtrlS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sl {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl Sl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sl {
    #[inline(always)]
    fn from(val: u8) -> Sl {
        Sl::from_bits(val)
    }
}
impl From<Sl> for u8 {
    #[inline(always)]
    fn from(val: Sl) -> u8 {
        Sl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwnSwp {
    #[doc = "Normal mode."]
    NORMAL = 0x0,
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR = 0x01,
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR = 0x02,
    #[doc = "Don't use."]
    FORBIDDEN = 0x03,
}
impl SwnSwp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwnSwp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwnSwp {
    #[inline(always)]
    fn from(val: u8) -> SwnSwp {
        SwnSwp::from_bits(val)
    }
}
impl From<SwnSwp> for u8 {
    #[inline(always)]
    fn from(val: SwnSwp) -> u8 {
        SwnSwp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbatdivenable {
    #[doc = "VBAT divider branch is disabled."]
    DISABLE = 0x0,
    #[doc = "VBAT divider branch is enabled."]
    ENABLE = 0x01,
}
impl Vbatdivenable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbatdivenable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbatdivenable {
    #[inline(always)]
    fn from(val: u8) -> Vbatdivenable {
        Vbatdivenable::from_bits(val)
    }
}
impl From<Vbatdivenable> for u8 {
    #[inline(always)]
    fn from(val: Vbatdivenable) -> u8 {
        Vbatdivenable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vout {
    #[doc = "0.750 V."]
    V_0P750 = 0x0,
    #[doc = "0.775 V."]
    V_0P775 = 0x01,
    #[doc = "0.800 V."]
    V_0P800 = 0x02,
    #[doc = "0.825 V."]
    V_0P825 = 0x03,
    #[doc = "0.850 V."]
    V_0P850 = 0x04,
    #[doc = "0.875 V."]
    V_0P875 = 0x05,
    #[doc = "0.900 V."]
    V_0P900 = 0x06,
    #[doc = "0.925 V."]
    V_0P925 = 0x07,
}
impl Vout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout {
    #[inline(always)]
    fn from(val: u8) -> Vout {
        Vout::from_bits(val)
    }
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(val: Vout) -> u8 {
        Vout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref1venable {
    #[doc = "Output of 1V reference voltage buffer is bypassed."]
    DISABLE = 0x0,
    #[doc = "Output of 1V reference voltage is enabled."]
    ENABLE = 0x01,
}
impl Vref1venable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref1venable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref1venable {
    #[inline(always)]
    fn from(val: u8) -> Vref1venable {
        Vref1venable::from_bits(val)
    }
}
impl From<Vref1venable> for u8 {
    #[inline(always)]
    fn from(val: Vref1venable) -> u8 {
        Vref1venable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xo32mAdcClkMode {
    #[doc = "High speed Crystal oscillator output to ADC is disabled."]
    DISABLE = 0x0,
    #[doc = "High speed Crystal oscillator output to ADC is enable."]
    XO_ADC_ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Xo32mAdcClkMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xo32mAdcClkMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xo32mAdcClkMode {
    #[inline(always)]
    fn from(val: u8) -> Xo32mAdcClkMode {
        Xo32mAdcClkMode::from_bits(val)
    }
}
impl From<Xo32mAdcClkMode> for u8 {
    #[inline(always)]
    fn from(val: Xo32mAdcClkMode) -> u8 {
        Xo32mAdcClkMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XoReady {
    #[doc = "XO output frequency is not yet stable."]
    NOT_STABLE = 0x0,
    #[doc = "XO output frequency is stable."]
    STABLE = 0x01,
}
impl XoReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XoReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XoReady {
    #[inline(always)]
    fn from(val: u8) -> XoReady {
        XoReady::from_bits(val)
    }
}
impl From<XoReady> for u8 {
    #[inline(always)]
    fn from(val: XoReady) -> u8 {
        XoReady::to_bits(val)
    }
}
