#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Basic implementation"]
    pub const FEATURE0: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FEATURE0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe {
    #[inline(always)]
    fn from(val: u8) -> Gpwe {
        Gpwe::from_bits(val)
    }
}
impl From<Gpwe> for u8 {
    #[inline(always)]
    fn from(val: Gpwe) -> u8 {
        Gpwe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrDse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl PcrDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrDse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrDse {
    #[inline(always)]
    fn from(val: u8) -> PcrDse {
        PcrDse::from_bits(val)
    }
}
impl From<PcrDse> for u8 {
    #[inline(always)]
    fn from(val: PcrDse) -> u8 {
        PcrDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrDse1 {
    #[doc = "Normal"]
    DSE10 = 0x0,
    #[doc = "Double"]
    DSE11 = 0x01,
}
impl PcrDse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrDse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrDse1 {
    #[inline(always)]
    fn from(val: u8) -> PcrDse1 {
        PcrDse1::from_bits(val)
    }
}
impl From<PcrDse1> for u8 {
    #[inline(always)]
    fn from(val: PcrDse1) -> u8 {
        PcrDse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrIbe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl PcrIbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrIbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrIbe {
    #[inline(always)]
    fn from(val: u8) -> PcrIbe {
        PcrIbe::from_bits(val)
    }
}
impl From<PcrIbe> for u8 {
    #[inline(always)]
    fn from(val: PcrIbe) -> u8 {
        PcrIbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrInv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl PcrInv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrInv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrInv {
    #[inline(always)]
    fn from(val: u8) -> PcrInv {
        PcrInv::from_bits(val)
    }
}
impl From<PcrInv> for u8 {
    #[inline(always)]
    fn from(val: PcrInv) -> u8 {
        PcrInv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrLk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl PcrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrLk {
    #[inline(always)]
    fn from(val: u8) -> PcrLk {
        PcrLk::from_bits(val)
    }
}
impl From<PcrLk> for u8 {
    #[inline(always)]
    fn from(val: PcrLk) -> u8 {
        PcrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrMux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PcrMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrMux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrMux {
    #[inline(always)]
    fn from(val: u8) -> PcrMux {
        PcrMux::from_bits(val)
    }
}
impl From<PcrMux> for u8 {
    #[inline(always)]
    fn from(val: PcrMux) -> u8 {
        PcrMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrOde {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl PcrOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrOde {
    #[inline(always)]
    fn from(val: u8) -> PcrOde {
        PcrOde::from_bits(val)
    }
}
impl From<PcrOde> for u8 {
    #[inline(always)]
    fn from(val: PcrOde) -> u8 {
        PcrOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrPe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl PcrPe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrPe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrPe {
    #[inline(always)]
    fn from(val: u8) -> PcrPe {
        PcrPe::from_bits(val)
    }
}
impl From<PcrPe> for u8 {
    #[inline(always)]
    fn from(val: PcrPe) -> u8 {
        PcrPe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrPfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl PcrPfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrPfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrPfe {
    #[inline(always)]
    fn from(val: u8) -> PcrPfe {
        PcrPfe::from_bits(val)
    }
}
impl From<PcrPfe> for u8 {
    #[inline(always)]
    fn from(val: PcrPfe) -> u8 {
        PcrPfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrPs {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl PcrPs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrPs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrPs {
    #[inline(always)]
    fn from(val: u8) -> PcrPs {
        PcrPs::from_bits(val)
    }
}
impl From<PcrPs> for u8 {
    #[inline(always)]
    fn from(val: PcrPs) -> u8 {
        PcrPs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrShortMux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
}
impl PcrShortMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrShortMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrShortMux {
    #[inline(always)]
    fn from(val: u8) -> PcrShortMux {
        PcrShortMux::from_bits(val)
    }
}
impl From<PcrShortMux> for u8 {
    #[inline(always)]
    fn from(val: PcrShortMux) -> u8 {
        PcrShortMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcrSre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl PcrSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcrSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcrSre {
    #[inline(always)]
    fn from(val: u8) -> PcrSre {
        PcrSre::from_bits(val)
    }
}
impl From<PcrSre> for u8 {
    #[inline(always)]
    fn from(val: PcrSre) -> u8 {
        PcrSre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pv {
    #[inline(always)]
    fn from(val: u8) -> Pv {
        Pv::from_bits(val)
    }
}
impl From<Pv> for u8 {
    #[inline(always)]
    fn from(val: Pv) -> u8 {
        Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "1.71 V-3.6 V"]
    RANGE0 = 0x0,
    #[doc = "2.70 V-3.6 V"]
    RANGE1 = 0x01,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Range {
    #[inline(always)]
    fn from(val: u8) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(val: Range) -> u8 {
        Range::to_bits(val)
    }
}
