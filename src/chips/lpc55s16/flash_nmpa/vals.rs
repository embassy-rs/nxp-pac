#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smb {
    #[doc = "Low leakage."]
    LOW = 0x0,
    #[doc = "Medium leakage."]
    MEDIUM = 0x01,
    #[doc = "Highest leakage."]
    HIGHEST = 0x02,
    #[doc = "Disable."]
    DISABLE = 0x03,
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
