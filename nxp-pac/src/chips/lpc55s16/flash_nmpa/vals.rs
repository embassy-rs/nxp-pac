#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smb {
    #[doc = "Low leakage."]
    Low = 0x0,
    #[doc = "Medium leakage."]
    Medium = 0x01,
    #[doc = "Highest leakage."]
    Highest = 0x02,
    #[doc = "Disable."]
    Disable = 0x03,
}
impl Smb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smb {
    #[inline(always)]
    fn from(val: u8) -> Smb {
        Smb::from_bits(val)
    }
}
impl From<Smb> for u8 {
    #[inline(always)]
    fn from(val: Smb) -> u8 {
        Smb::to_bits(val)
    }
}
