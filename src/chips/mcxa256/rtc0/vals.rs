#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Control Register is locked and writes are ignored."]
    CRL_0 = 0x0,
    #[doc = "Control Register is not locked and writes complete as normal."]
    CRL_1 = 0x01,
}
impl Crl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crl {
    #[inline(always)]
    fn from(val: u8) -> Crl {
        Crl::from_bits(val)
    }
}
impl From<Crl> for u8 {
    #[inline(always)]
    fn from(val: Crl) -> u8 {
        Crl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpos {
    #[doc = "RTC prescaler increments using 32.768 kHz clock."]
    LPOS_0 = 0x0,
    #[doc = "RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\] of the prescaler are ignored."]
    LPOS_1 = 0x01,
}
impl Lpos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpos {
    #[inline(always)]
    fn from(val: u8) -> Lpos {
        Lpos::from_bits(val)
    }
}
impl From<Lpos> for u8 {
    #[inline(always)]
    fn from(val: Lpos) -> u8 {
        Lpos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Lock Register is locked and writes are ignored."]
    LRL_0 = 0x0,
    #[doc = "Lock Register is not locked and writes complete as normal."]
    LRL_1 = 0x01,
}
impl Lrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrl {
    #[inline(always)]
    fn from(val: u8) -> Lrl {
        Lrl::from_bits(val)
    }
}
impl From<Lrl> for u8 {
    #[inline(always)]
    fn from(val: Lrl) -> u8 {
        Lrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srl {
    #[doc = "Status Register is locked and writes are ignored."]
    SRL_0 = 0x0,
    #[doc = "Status Register is not locked and writes complete as normal."]
    SRL_1 = 0x01,
}
impl Srl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srl {
    #[inline(always)]
    fn from(val: u8) -> Srl {
        Srl::from_bits(val)
    }
}
impl From<Srl> for u8 {
    #[inline(always)]
    fn from(val: Srl) -> u8 {
        Srl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "No effect."]
    SWR_0 = 0x0,
    #[doc = "Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    SWR_1 = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taf {
    #[doc = "Time alarm has not occurred."]
    TAF_0 = 0x0,
    #[doc = "Time alarm has occurred."]
    TAF_1 = 0x01,
}
impl Taf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taf {
    #[inline(always)]
    fn from(val: u8) -> Taf {
        Taf::from_bits(val)
    }
}
impl From<Taf> for u8 {
    #[inline(always)]
    fn from(val: Taf) -> u8 {
        Taf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taie {
    #[doc = "No interrupt is generated."]
    TAIE_0 = 0x0,
    #[doc = "An interrupt is generated."]
    TAIE_1 = 0x01,
}
impl Taie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taie {
    #[inline(always)]
    fn from(val: u8) -> Taie {
        Taie::from_bits(val)
    }
}
impl From<Taie> for u8 {
    #[inline(always)]
    fn from(val: Taie) -> u8 {
        Taie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tce {
    #[doc = "Disables."]
    TCE_0 = 0x0,
    #[doc = "Enables."]
    TCE_1 = 0x01,
}
impl Tce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tce {
    #[inline(always)]
    fn from(val: u8) -> Tce {
        Tce::from_bits(val)
    }
}
impl From<Tce> for u8 {
    #[inline(always)]
    fn from(val: Tce) -> u8 {
        Tce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcl {
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    TCL_0 = 0x0,
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    TCL_1 = 0x01,
}
impl Tcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcl {
    #[inline(always)]
    fn from(val: u8) -> Tcl {
        Tcl::from_bits(val)
    }
}
impl From<Tcl> for u8 {
    #[inline(always)]
    fn from(val: Tcl) -> u8 {
        Tcl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tcr(u8);
impl Tcr {
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    pub const TCR_0: Self = Self(0x0);
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    pub const TCR_1: Self = Self(0x01);
    #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
    pub const TCR_126: Self = Self(0x7e);
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    pub const TCR_127: Self = Self(0x7f);
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    pub const TCR_128: Self = Self(0x80);
    #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
    pub const TCR_129: Self = Self(0x81);
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    pub const TCR_255: Self = Self(0xff);
}
impl Tcr {
    pub const fn from_bits(val: u8) -> Tcr {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TCR_0"),
            0x01 => f.write_str("TCR_1"),
            0x7e => f.write_str("TCR_126"),
            0x7f => f.write_str("TCR_127"),
            0x80 => f.write_str("TCR_128"),
            0x81 => f.write_str("TCR_129"),
            0xff => f.write_str("TCR_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TCR_0"),
            0x01 => defmt::write!(f, "TCR_1"),
            0x7e => defmt::write!(f, "TCR_126"),
            0x7f => defmt::write!(f, "TCR_127"),
            0x80 => defmt::write!(f, "TCR_128"),
            0x81 => defmt::write!(f, "TCR_129"),
            0xff => defmt::write!(f, "TCR_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Tcr {
    #[inline(always)]
    fn from(val: u8) -> Tcr {
        Tcr::from_bits(val)
    }
}
impl From<Tcr> for u8 {
    #[inline(always)]
    fn from(val: Tcr) -> u8 {
        Tcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif {
    #[doc = "Time is valid."]
    TIF_0 = 0x0,
    #[doc = "Time is invalid and time counter is read as zero."]
    TIF_1 = 0x01,
}
impl Tif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif {
    #[inline(always)]
    fn from(val: u8) -> Tif {
        Tif::from_bits(val)
    }
}
impl From<Tif> for u8 {
    #[inline(always)]
    fn from(val: Tif) -> u8 {
        Tif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie {
    #[doc = "No interrupt is generated."]
    TIIE_0 = 0x0,
    #[doc = "An interrupt is generated."]
    TIIE_1 = 0x01,
}
impl Tiie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie {
    #[inline(always)]
    fn from(val: u8) -> Tiie {
        Tiie::from_bits(val)
    }
}
impl From<Tiie> for u8 {
    #[inline(always)]
    fn from(val: Tiie) -> u8 {
        Tiie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tof {
    #[doc = "Time overflow has not occurred."]
    TOF_0 = 0x0,
    #[doc = "Time overflow has occurred and time counter reads as zero."]
    TOF_1 = 0x01,
}
impl Tof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tof {
    #[inline(always)]
    fn from(val: u8) -> Tof {
        Tof::from_bits(val)
    }
}
impl From<Tof> for u8 {
    #[inline(always)]
    fn from(val: Tof) -> u8 {
        Tof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toie {
    #[doc = "No interrupt is generated."]
    TOIE_0 = 0x0,
    #[doc = "An interrupt is generated."]
    TOIE_1 = 0x01,
}
impl Toie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toie {
    #[inline(always)]
    fn from(val: u8) -> Toie {
        Toie::from_bits(val)
    }
}
impl From<Toie> for u8 {
    #[inline(always)]
    fn from(val: Toie) -> u8 {
        Toie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsic {
    #[doc = "1 Hz."]
    TSIC_0 = 0x0,
    #[doc = "2 Hz."]
    TSIC_1 = 0x01,
    #[doc = "4 Hz."]
    TSIC_2 = 0x02,
    #[doc = "8 Hz."]
    TSIC_3 = 0x03,
    #[doc = "16 Hz."]
    TSIC_4 = 0x04,
    #[doc = "32 Hz."]
    TSIC_5 = 0x05,
    #[doc = "64 Hz."]
    TSIC_6 = 0x06,
    #[doc = "128 Hz."]
    TSIC_7 = 0x07,
}
impl Tsic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsic {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsic {
    #[inline(always)]
    fn from(val: u8) -> Tsic {
        Tsic::from_bits(val)
    }
}
impl From<Tsic> for u8 {
    #[inline(always)]
    fn from(val: Tsic) -> u8 {
        Tsic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsie {
    #[doc = "Disables"]
    TSIE_0 = 0x0,
    #[doc = "Enables"]
    TSIE_1 = 0x01,
}
impl Tsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsie {
    #[inline(always)]
    fn from(val: u8) -> Tsie {
        Tsie::from_bits(val)
    }
}
impl From<Tsie> for u8 {
    #[inline(always)]
    fn from(val: Tsie) -> u8 {
        Tsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "Registers cannot be written when locked."]
    UM_0 = 0x0,
    #[doc = "Registers can be written when locked under limited conditions."]
    UM_1 = 0x01,
}
impl Um {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Um {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Um {
    #[inline(always)]
    fn from(val: u8) -> Um {
        Um::from_bits(val)
    }
}
impl From<Um> for u8 {
    #[inline(always)]
    fn from(val: Um) -> u8 {
        Um::to_bits(val)
    }
}
