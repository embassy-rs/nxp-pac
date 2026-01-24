#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltDbgen {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltDbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltDbgen {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltDbgen {
        DcfgCcSocuDfltDbgen::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltDbgen> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltDbgen) -> u8 {
        DcfgCcSocuDfltDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltFaMeCmdEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltFaMeCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltFaMeCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltFaMeCmdEn {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltFaMeCmdEn {
        DcfgCcSocuDfltFaMeCmdEn::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltFaMeCmdEn> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltFaMeCmdEn) -> u8 {
        DcfgCcSocuDfltFaMeCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltIspCmdEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltIspCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltIspCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltIspCmdEn {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltIspCmdEn {
        DcfgCcSocuDfltIspCmdEn::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltIspCmdEn> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltIspCmdEn) -> u8 {
        DcfgCcSocuDfltIspCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltNiden {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltNiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltNiden {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltNiden {
        DcfgCcSocuDfltNiden::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltNiden> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltNiden) -> u8 {
        DcfgCcSocuDfltNiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltSpiden {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltSpiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltSpiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltSpiden {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltSpiden {
        DcfgCcSocuDfltSpiden::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltSpiden> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltSpiden) -> u8 {
        DcfgCcSocuDfltSpiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltSpniden {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltSpniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltSpniden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltSpniden {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltSpniden {
        DcfgCcSocuDfltSpniden::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltSpniden> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltSpniden) -> u8 {
        DcfgCcSocuDfltSpniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuDfltTapen {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcfgCcSocuDfltTapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuDfltTapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuDfltTapen {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuDfltTapen {
        DcfgCcSocuDfltTapen::from_bits(val)
    }
}
impl From<DcfgCcSocuDfltTapen> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuDfltTapen) -> u8 {
        DcfgCcSocuDfltTapen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinDbgen {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinDbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinDbgen {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinDbgen {
        DcfgCcSocuPinDbgen::from_bits(val)
    }
}
impl From<DcfgCcSocuPinDbgen> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinDbgen) -> u8 {
        DcfgCcSocuPinDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinFaMeCmdEn {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinFaMeCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinFaMeCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinFaMeCmdEn {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinFaMeCmdEn {
        DcfgCcSocuPinFaMeCmdEn::from_bits(val)
    }
}
impl From<DcfgCcSocuPinFaMeCmdEn> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinFaMeCmdEn) -> u8 {
        DcfgCcSocuPinFaMeCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinIspCmdEn {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinIspCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinIspCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinIspCmdEn {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinIspCmdEn {
        DcfgCcSocuPinIspCmdEn::from_bits(val)
    }
}
impl From<DcfgCcSocuPinIspCmdEn> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinIspCmdEn) -> u8 {
        DcfgCcSocuPinIspCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinNiden {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinNiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinNiden {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinNiden {
        DcfgCcSocuPinNiden::from_bits(val)
    }
}
impl From<DcfgCcSocuPinNiden> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinNiden) -> u8 {
        DcfgCcSocuPinNiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinSpiden {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinSpiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinSpiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinSpiden {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinSpiden {
        DcfgCcSocuPinSpiden::from_bits(val)
    }
}
impl From<DcfgCcSocuPinSpiden> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinSpiden) -> u8 {
        DcfgCcSocuPinSpiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinSpniden {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinSpniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinSpniden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinSpniden {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinSpniden {
        DcfgCcSocuPinSpniden::from_bits(val)
    }
}
impl From<DcfgCcSocuPinSpniden> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinSpniden) -> u8 {
        DcfgCcSocuPinSpniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinTapen {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl DcfgCcSocuPinTapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcfgCcSocuPinTapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcfgCcSocuPinTapen {
    #[inline(always)]
    fn from(val: u8) -> DcfgCcSocuPinTapen {
        DcfgCcSocuPinTapen::from_bits(val)
    }
}
impl From<DcfgCcSocuPinTapen> for u8 {
    #[inline(always)]
    fn from(val: DcfgCcSocuPinTapen) -> u8 {
        DcfgCcSocuPinTapen::to_bits(val)
    }
}
