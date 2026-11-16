#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinDbgen {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinFaMeCmdEn {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinIspCmdEn {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinNiden {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinSpiden {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinSpniden {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcfgCcSocuPinTapen {
    #[doc = "Use DAP to enable."]
    Enable = 0x0,
    #[doc = "Fixed state."]
    Disable = 0x01,
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
