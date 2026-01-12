#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse0 {
        Mbc0Dom0Mem0BlkCfgW0Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse1 {
        Mbc0Dom0Mem0BlkCfgW0Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse2 {
        Mbc0Dom0Mem0BlkCfgW0Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse3 {
        Mbc0Dom0Mem0BlkCfgW0Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse4 {
        Mbc0Dom0Mem0BlkCfgW0Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse5 {
        Mbc0Dom0Mem0BlkCfgW0Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse6 {
        Mbc0Dom0Mem0BlkCfgW0Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse7 {
        Mbc0Dom0Mem0BlkCfgW0Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW10Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse0 {
        Mbc0Dom0Mem0BlkCfgW10Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse1 {
        Mbc0Dom0Mem0BlkCfgW10Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse2 {
        Mbc0Dom0Mem0BlkCfgW10Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse3 {
        Mbc0Dom0Mem0BlkCfgW10Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse4 {
        Mbc0Dom0Mem0BlkCfgW10Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse5 {
        Mbc0Dom0Mem0BlkCfgW10Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse6 {
        Mbc0Dom0Mem0BlkCfgW10Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW10Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW10Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW10Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW10Nse7 {
        Mbc0Dom0Mem0BlkCfgW10Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW10Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW10Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW10Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW11Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse0 {
        Mbc0Dom0Mem0BlkCfgW11Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse1 {
        Mbc0Dom0Mem0BlkCfgW11Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse2 {
        Mbc0Dom0Mem0BlkCfgW11Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse3 {
        Mbc0Dom0Mem0BlkCfgW11Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse4 {
        Mbc0Dom0Mem0BlkCfgW11Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse5 {
        Mbc0Dom0Mem0BlkCfgW11Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse6 {
        Mbc0Dom0Mem0BlkCfgW11Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW11Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW11Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW11Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW11Nse7 {
        Mbc0Dom0Mem0BlkCfgW11Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW11Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW11Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW11Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW12Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse0 {
        Mbc0Dom0Mem0BlkCfgW12Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse1 {
        Mbc0Dom0Mem0BlkCfgW12Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse2 {
        Mbc0Dom0Mem0BlkCfgW12Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse3 {
        Mbc0Dom0Mem0BlkCfgW12Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse4 {
        Mbc0Dom0Mem0BlkCfgW12Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse5 {
        Mbc0Dom0Mem0BlkCfgW12Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse6 {
        Mbc0Dom0Mem0BlkCfgW12Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW12Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW12Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW12Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW12Nse7 {
        Mbc0Dom0Mem0BlkCfgW12Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW12Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW12Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW12Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW13Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse0 {
        Mbc0Dom0Mem0BlkCfgW13Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse1 {
        Mbc0Dom0Mem0BlkCfgW13Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse2 {
        Mbc0Dom0Mem0BlkCfgW13Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse3 {
        Mbc0Dom0Mem0BlkCfgW13Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse4 {
        Mbc0Dom0Mem0BlkCfgW13Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse5 {
        Mbc0Dom0Mem0BlkCfgW13Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse6 {
        Mbc0Dom0Mem0BlkCfgW13Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW13Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW13Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW13Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW13Nse7 {
        Mbc0Dom0Mem0BlkCfgW13Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW13Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW13Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW13Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW14Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse0 {
        Mbc0Dom0Mem0BlkCfgW14Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse1 {
        Mbc0Dom0Mem0BlkCfgW14Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse2 {
        Mbc0Dom0Mem0BlkCfgW14Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse3 {
        Mbc0Dom0Mem0BlkCfgW14Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse4 {
        Mbc0Dom0Mem0BlkCfgW14Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse5 {
        Mbc0Dom0Mem0BlkCfgW14Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse6 {
        Mbc0Dom0Mem0BlkCfgW14Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW14Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW14Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW14Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW14Nse7 {
        Mbc0Dom0Mem0BlkCfgW14Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW14Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW14Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW14Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW15Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse0 {
        Mbc0Dom0Mem0BlkCfgW15Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse1 {
        Mbc0Dom0Mem0BlkCfgW15Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse2 {
        Mbc0Dom0Mem0BlkCfgW15Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse3 {
        Mbc0Dom0Mem0BlkCfgW15Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse4 {
        Mbc0Dom0Mem0BlkCfgW15Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse5 {
        Mbc0Dom0Mem0BlkCfgW15Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse6 {
        Mbc0Dom0Mem0BlkCfgW15Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW15Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW15Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW15Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW15Nse7 {
        Mbc0Dom0Mem0BlkCfgW15Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW15Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW15Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW15Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse0 {
        Mbc0Dom0Mem0BlkCfgW1Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse1 {
        Mbc0Dom0Mem0BlkCfgW1Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse2 {
        Mbc0Dom0Mem0BlkCfgW1Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse3 {
        Mbc0Dom0Mem0BlkCfgW1Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse4 {
        Mbc0Dom0Mem0BlkCfgW1Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse5 {
        Mbc0Dom0Mem0BlkCfgW1Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse6 {
        Mbc0Dom0Mem0BlkCfgW1Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse7 {
        Mbc0Dom0Mem0BlkCfgW1Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse0 {
        Mbc0Dom0Mem0BlkCfgW2Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse1 {
        Mbc0Dom0Mem0BlkCfgW2Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse2 {
        Mbc0Dom0Mem0BlkCfgW2Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse3 {
        Mbc0Dom0Mem0BlkCfgW2Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse4 {
        Mbc0Dom0Mem0BlkCfgW2Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse5 {
        Mbc0Dom0Mem0BlkCfgW2Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse6 {
        Mbc0Dom0Mem0BlkCfgW2Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse7 {
        Mbc0Dom0Mem0BlkCfgW2Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse0 {
        Mbc0Dom0Mem0BlkCfgW3Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse1 {
        Mbc0Dom0Mem0BlkCfgW3Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse2 {
        Mbc0Dom0Mem0BlkCfgW3Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse3 {
        Mbc0Dom0Mem0BlkCfgW3Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse4 {
        Mbc0Dom0Mem0BlkCfgW3Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse5 {
        Mbc0Dom0Mem0BlkCfgW3Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse6 {
        Mbc0Dom0Mem0BlkCfgW3Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse7 {
        Mbc0Dom0Mem0BlkCfgW3Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse0 {
        Mbc0Dom0Mem0BlkCfgW4Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse1 {
        Mbc0Dom0Mem0BlkCfgW4Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse2 {
        Mbc0Dom0Mem0BlkCfgW4Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse3 {
        Mbc0Dom0Mem0BlkCfgW4Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse4 {
        Mbc0Dom0Mem0BlkCfgW4Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse5 {
        Mbc0Dom0Mem0BlkCfgW4Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse6 {
        Mbc0Dom0Mem0BlkCfgW4Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse7 {
        Mbc0Dom0Mem0BlkCfgW4Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse0 {
        Mbc0Dom0Mem0BlkCfgW5Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse1 {
        Mbc0Dom0Mem0BlkCfgW5Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse2 {
        Mbc0Dom0Mem0BlkCfgW5Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse3 {
        Mbc0Dom0Mem0BlkCfgW5Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse4 {
        Mbc0Dom0Mem0BlkCfgW5Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse5 {
        Mbc0Dom0Mem0BlkCfgW5Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse6 {
        Mbc0Dom0Mem0BlkCfgW5Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse7 {
        Mbc0Dom0Mem0BlkCfgW5Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse0 {
        Mbc0Dom0Mem0BlkCfgW6Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse1 {
        Mbc0Dom0Mem0BlkCfgW6Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse2 {
        Mbc0Dom0Mem0BlkCfgW6Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse3 {
        Mbc0Dom0Mem0BlkCfgW6Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse4 {
        Mbc0Dom0Mem0BlkCfgW6Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse5 {
        Mbc0Dom0Mem0BlkCfgW6Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse6 {
        Mbc0Dom0Mem0BlkCfgW6Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse7 {
        Mbc0Dom0Mem0BlkCfgW6Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse0 {
        Mbc0Dom0Mem0BlkCfgW7Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse1 {
        Mbc0Dom0Mem0BlkCfgW7Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse2 {
        Mbc0Dom0Mem0BlkCfgW7Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse3 {
        Mbc0Dom0Mem0BlkCfgW7Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse4 {
        Mbc0Dom0Mem0BlkCfgW7Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse5 {
        Mbc0Dom0Mem0BlkCfgW7Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse6 {
        Mbc0Dom0Mem0BlkCfgW7Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse7 {
        Mbc0Dom0Mem0BlkCfgW7Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW8Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse0 {
        Mbc0Dom0Mem0BlkCfgW8Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse1 {
        Mbc0Dom0Mem0BlkCfgW8Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse2 {
        Mbc0Dom0Mem0BlkCfgW8Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse3 {
        Mbc0Dom0Mem0BlkCfgW8Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse4 {
        Mbc0Dom0Mem0BlkCfgW8Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse5 {
        Mbc0Dom0Mem0BlkCfgW8Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse6 {
        Mbc0Dom0Mem0BlkCfgW8Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW8Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW8Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW8Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW8Nse7 {
        Mbc0Dom0Mem0BlkCfgW8Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW8Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW8Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW8Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW9Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse0 {
        Mbc0Dom0Mem0BlkCfgW9Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse1 {
        Mbc0Dom0Mem0BlkCfgW9Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse2 {
        Mbc0Dom0Mem0BlkCfgW9Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse3 {
        Mbc0Dom0Mem0BlkCfgW9Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse4 {
        Mbc0Dom0Mem0BlkCfgW9Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse5 {
        Mbc0Dom0Mem0BlkCfgW9Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse6 {
        Mbc0Dom0Mem0BlkCfgW9Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW9Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW9Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW9Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW9Nse7 {
        Mbc0Dom0Mem0BlkCfgW9Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW9Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW9Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW9Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel0) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel1) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel2) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel3) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel4) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel5) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel6) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel7) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse0 {
        Mbc0Dom0Mem1BlkCfgW0Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse0) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse1 {
        Mbc0Dom0Mem1BlkCfgW0Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse1) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse2 {
        Mbc0Dom0Mem1BlkCfgW0Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse2) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse3 {
        Mbc0Dom0Mem1BlkCfgW0Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse3) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse4 {
        Mbc0Dom0Mem1BlkCfgW0Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse4) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse5 {
        Mbc0Dom0Mem1BlkCfgW0Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse5) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse6 {
        Mbc0Dom0Mem1BlkCfgW0Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse6) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse7 {
        Mbc0Dom0Mem1BlkCfgW0Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse7) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel0 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel0) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel1 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel1) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel2 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel2) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel3 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel3) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel4 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel4) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel5 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel5) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel6 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel6) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW1Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Mbacsel7 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Mbacsel7) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse0 {
        Mbc0Dom0Mem1BlkCfgW1Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse0) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse1 {
        Mbc0Dom0Mem1BlkCfgW1Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse1) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse2 {
        Mbc0Dom0Mem1BlkCfgW1Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse2) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse3 {
        Mbc0Dom0Mem1BlkCfgW1Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse3) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse4 {
        Mbc0Dom0Mem1BlkCfgW1Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse4) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse5 {
        Mbc0Dom0Mem1BlkCfgW1Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse5) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse6 {
        Mbc0Dom0Mem1BlkCfgW1Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse6) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW1Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW1Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW1Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW1Nse7 {
        Mbc0Dom0Mem1BlkCfgW1Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW1Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW1Nse7) -> u8 {
        Mbc0Dom0Mem1BlkCfgW1Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel0) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel1) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel2) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel3) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel4) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel5) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel6) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel7) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse0 {
        Mbc0Dom0Mem2BlkCfgW0Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse0) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse1 {
        Mbc0Dom0Mem2BlkCfgW0Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse1) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse2 {
        Mbc0Dom0Mem2BlkCfgW0Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse2) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse3 {
        Mbc0Dom0Mem2BlkCfgW0Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse3) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse4 {
        Mbc0Dom0Mem2BlkCfgW0Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse4) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse5 {
        Mbc0Dom0Mem2BlkCfgW0Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse5) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse6 {
        Mbc0Dom0Mem2BlkCfgW0Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse6) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse7 {
        Mbc0Dom0Mem2BlkCfgW0Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse7) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse7::to_bits(val)
    }
}
