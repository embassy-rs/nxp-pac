#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BlockEnroll {
    #[doc = "Allow PUF enroll operation"]
    ALLOW = 0x0,
    #[doc = "Disable PUF enroll operation"]
    DISABLE = 0x01,
    #[doc = "Disable PUF enroll operation"]
    VALUE_2 = 0x02,
    #[doc = "Disable PUF enroll operation"]
    VALUE_3 = 0x03,
}
impl BlockEnroll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BlockEnroll {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BlockEnroll {
    #[inline(always)]
    fn from(val: u8) -> BlockEnroll {
        BlockEnroll::from_bits(val)
    }
}
impl From<BlockEnroll> for u8 {
    #[inline(always)]
    fn from(val: BlockEnroll) -> u8 {
        BlockEnroll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BlockSetKey {
    #[doc = "Allow PUF Key Code generation"]
    ALLOW = 0x0,
    #[doc = "Disable PUF Key Code generation"]
    DISABLE = 0x01,
    #[doc = "Disable PUF Key Code generation"]
    VALUE_2 = 0x02,
    #[doc = "Disable PUF Key Code generation"]
    VALUE_3 = 0x03,
}
impl BlockSetKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BlockSetKey {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BlockSetKey {
    #[inline(always)]
    fn from(val: u8) -> BlockSetKey {
        BlockSetKey::from_bits(val)
    }
}
impl From<BlockSetKey> for u8 {
    #[inline(always)]
    fn from(val: BlockSetKey) -> u8 {
        BlockSetKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootSeedCustCfg {
    #[doc = "not included"]
    NOT_INCLUD = 0x0,
    #[doc = "included"]
    INCLUD = 0x01,
    #[doc = "included"]
    VALUE_2 = 0x02,
    #[doc = "included"]
    VALUE_3 = 0x03,
}
impl BootSeedCustCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootSeedCustCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootSeedCustCfg {
    #[inline(always)]
    fn from(val: u8) -> BootSeedCustCfg {
        BootSeedCustCfg::from_bits(val)
    }
}
impl From<BootSeedCustCfg> for u8 {
    #[inline(always)]
    fn from(val: BootSeedCustCfg) -> u8 {
        BootSeedCustCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootSeedIncEpoch {
    #[doc = "not included"]
    NOT_INCLUD = 0x0,
    #[doc = "included"]
    INCLUD = 0x01,
    #[doc = "included"]
    VALUE_2 = 0x02,
    #[doc = "included"]
    VALUE_3 = 0x03,
}
impl BootSeedIncEpoch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootSeedIncEpoch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootSeedIncEpoch {
    #[inline(always)]
    fn from(val: u8) -> BootSeedIncEpoch {
        BootSeedIncEpoch::from_bits(val)
    }
}
impl From<BootSeedIncEpoch> for u8 {
    #[inline(always)]
    fn from(val: BootSeedIncEpoch) -> u8 {
        BootSeedIncEpoch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootSeedIncNxpCfg {
    #[doc = "not included"]
    NOT_INCLUD = 0x0,
    #[doc = "included"]
    INCLUD = 0x01,
    #[doc = "included"]
    VALUE_2 = 0x02,
    #[doc = "included"]
    VALUE_3 = 0x03,
}
impl BootSeedIncNxpCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootSeedIncNxpCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootSeedIncNxpCfg {
    #[inline(always)]
    fn from(val: u8) -> BootSeedIncNxpCfg {
        BootSeedIncNxpCfg::from_bits(val)
    }
}
impl From<BootSeedIncNxpCfg> for u8 {
    #[inline(always)]
    fn from(val: BootSeedIncNxpCfg) -> u8 {
        BootSeedIncNxpCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootSpeed {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    VALUE_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "48MHz FRO"]
    VALUE_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl BootSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootSpeed {
    #[inline(always)]
    fn from(val: u8) -> BootSpeed {
        BootSpeed::from_bits(val)
    }
}
impl From<BootSpeed> for u8 {
    #[inline(always)]
    fn from(val: BootSpeed) -> u8 {
        BootSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltDbgen {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltDbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltDbgen {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltDbgen {
        CcSocuDfltDbgen::from_bits(val)
    }
}
impl From<CcSocuDfltDbgen> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltDbgen) -> u8 {
        CcSocuDfltDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltFaMeCmdEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltFaMeCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltFaMeCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltFaMeCmdEn {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltFaMeCmdEn {
        CcSocuDfltFaMeCmdEn::from_bits(val)
    }
}
impl From<CcSocuDfltFaMeCmdEn> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltFaMeCmdEn) -> u8 {
        CcSocuDfltFaMeCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltIspCmdEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltIspCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltIspCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltIspCmdEn {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltIspCmdEn {
        CcSocuDfltIspCmdEn::from_bits(val)
    }
}
impl From<CcSocuDfltIspCmdEn> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltIspCmdEn) -> u8 {
        CcSocuDfltIspCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltNiden {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltNiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltNiden {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltNiden {
        CcSocuDfltNiden::from_bits(val)
    }
}
impl From<CcSocuDfltNiden> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltNiden) -> u8 {
        CcSocuDfltNiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltSpiden {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltSpiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltSpiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltSpiden {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltSpiden {
        CcSocuDfltSpiden::from_bits(val)
    }
}
impl From<CcSocuDfltSpiden> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltSpiden) -> u8 {
        CcSocuDfltSpiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltSpniden {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltSpniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltSpniden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltSpniden {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltSpniden {
        CcSocuDfltSpniden::from_bits(val)
    }
}
impl From<CcSocuDfltSpniden> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltSpniden) -> u8 {
        CcSocuDfltSpniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuDfltTapen {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CcSocuDfltTapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuDfltTapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuDfltTapen {
    #[inline(always)]
    fn from(val: u8) -> CcSocuDfltTapen {
        CcSocuDfltTapen::from_bits(val)
    }
}
impl From<CcSocuDfltTapen> for u8 {
    #[inline(always)]
    fn from(val: CcSocuDfltTapen) -> u8 {
        CcSocuDfltTapen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinDbgen {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinDbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinDbgen {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinDbgen {
        CcSocuPinDbgen::from_bits(val)
    }
}
impl From<CcSocuPinDbgen> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinDbgen) -> u8 {
        CcSocuPinDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinFaMeCmdEn {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinFaMeCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinFaMeCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinFaMeCmdEn {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinFaMeCmdEn {
        CcSocuPinFaMeCmdEn::from_bits(val)
    }
}
impl From<CcSocuPinFaMeCmdEn> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinFaMeCmdEn) -> u8 {
        CcSocuPinFaMeCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinIspCmdEn {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinIspCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinIspCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinIspCmdEn {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinIspCmdEn {
        CcSocuPinIspCmdEn::from_bits(val)
    }
}
impl From<CcSocuPinIspCmdEn> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinIspCmdEn) -> u8 {
        CcSocuPinIspCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinNiden {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinNiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinNiden {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinNiden {
        CcSocuPinNiden::from_bits(val)
    }
}
impl From<CcSocuPinNiden> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinNiden) -> u8 {
        CcSocuPinNiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinSpiden {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinSpiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinSpiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinSpiden {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinSpiden {
        CcSocuPinSpiden::from_bits(val)
    }
}
impl From<CcSocuPinSpiden> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinSpiden) -> u8 {
        CcSocuPinSpiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinSpniden {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinSpniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinSpniden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinSpniden {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinSpniden {
        CcSocuPinSpniden::from_bits(val)
    }
}
impl From<CcSocuPinSpniden> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinSpniden) -> u8 {
        CcSocuPinSpniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcSocuPinTapen {
    #[doc = "Use DAP to enable"]
    ENABLE = 0x0,
    #[doc = "Fixed state"]
    DISABLE = 0x01,
}
impl CcSocuPinTapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcSocuPinTapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcSocuPinTapen {
    #[inline(always)]
    fn from(val: u8) -> CcSocuPinTapen {
        CcSocuPinTapen::from_bits(val)
    }
}
impl From<CcSocuPinTapen> for u8 {
    #[inline(always)]
    fn from(val: CcSocuPinTapen) -> u8 {
        CcSocuPinTapen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DefaultIspMode {
    #[doc = "Auto ISP"]
    AUTO_ISP = 0x0,
    #[doc = "USB_HID_ISP"]
    USB_HID_ISP = 0x01,
    #[doc = "UART ISP"]
    UART_ISP = 0x02,
    #[doc = "SPI Slave ISP"]
    SPI_ISP = 0x03,
    #[doc = "I2C Slave ISP"]
    I2C_ISP = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Disable ISP fall through"]
    DISABLE = 0x07,
}
impl DefaultIspMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DefaultIspMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DefaultIspMode {
    #[inline(always)]
    fn from(val: u8) -> DefaultIspMode {
        DefaultIspMode::from_bits(val)
    }
}
impl From<DefaultIspMode> for u8 {
    #[inline(always)]
    fn from(val: DefaultIspMode) -> u8 {
        DefaultIspMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DiceCustCfg {
    #[doc = "not included"]
    NOT_INCLUD = 0x0,
    #[doc = "included"]
    UNCLUD = 0x01,
    #[doc = "included"]
    VALUE_2 = 0x02,
    #[doc = "included"]
    VALUE_3 = 0x03,
}
impl DiceCustCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DiceCustCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DiceCustCfg {
    #[inline(always)]
    fn from(val: u8) -> DiceCustCfg {
        DiceCustCfg::from_bits(val)
    }
}
impl From<DiceCustCfg> for u8 {
    #[inline(always)]
    fn from(val: DiceCustCfg) -> u8 {
        DiceCustCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DiceIncNxpCfg {
    #[doc = "not included"]
    NOT_INCLUD = 0x0,
    #[doc = "included"]
    INCLUD = 0x01,
    #[doc = "included"]
    VALUE_2 = 0x02,
    #[doc = "included"]
    VALUE_3 = 0x03,
}
impl DiceIncNxpCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DiceIncNxpCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DiceIncNxpCfg {
    #[inline(always)]
    fn from(val: u8) -> DiceIncNxpCfg {
        DiceIncNxpCfg::from_bits(val)
    }
}
impl From<DiceIncNxpCfg> for u8 {
    #[inline(always)]
    fn from(val: DiceIncNxpCfg) -> u8 {
        DiceIncNxpCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DiceIncSecEpoch {
    #[doc = "not included"]
    NOT_INCLUD = 0x0,
    #[doc = "included"]
    INCLUD = 0x01,
    #[doc = "included"]
    VALUE_2 = 0x02,
    #[doc = "included"]
    VALUE_3 = 0x03,
}
impl DiceIncSecEpoch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DiceIncSecEpoch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DiceIncSecEpoch {
    #[inline(always)]
    fn from(val: u8) -> DiceIncSecEpoch {
        DiceIncSecEpoch::from_bits(val)
    }
}
impl From<DiceIncSecEpoch> for u8 {
    #[inline(always)]
    fn from(val: DiceIncSecEpoch) -> u8 {
        DiceIncSecEpoch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockReg0 {
    #[doc = "Region is not locked"]
    UNLOCK = 0x0,
    #[doc = "Region is locked"]
    LOCK = 0x01,
    #[doc = "Region is locked"]
    VALUE_2 = 0x02,
    #[doc = "Region is locked"]
    VALUE_3 = 0x03,
}
impl LockReg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockReg0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockReg0 {
    #[inline(always)]
    fn from(val: u8) -> LockReg0 {
        LockReg0::from_bits(val)
    }
}
impl From<LockReg0> for u8 {
    #[inline(always)]
    fn from(val: LockReg0) -> u8 {
        LockReg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockReg1 {
    #[doc = "Region is not locked"]
    UNLOCK = 0x0,
    #[doc = "Region is locked"]
    LOCK = 0x01,
    #[doc = "Region is locked"]
    VALUE_2 = 0x02,
    #[doc = "Region is locked"]
    VALUE_3 = 0x03,
}
impl LockReg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockReg1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockReg1 {
    #[inline(always)]
    fn from(val: u8) -> LockReg1 {
        LockReg1::from_bits(val)
    }
}
impl From<LockReg1> for u8 {
    #[inline(always)]
    fn from(val: LockReg1) -> u8 {
        LockReg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg0EraseCheckEn {
    #[doc = "Region is disabled"]
    DISABLE = 0x0,
    #[doc = "Region is enabled"]
    ENABLE = 0x01,
    #[doc = "Region is enabled"]
    VALUE_2 = 0x02,
    #[doc = "Region is enabled"]
    VALUE_3 = 0x03,
}
impl Reg0EraseCheckEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg0EraseCheckEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg0EraseCheckEn {
    #[inline(always)]
    fn from(val: u8) -> Reg0EraseCheckEn {
        Reg0EraseCheckEn::from_bits(val)
    }
}
impl From<Reg0EraseCheckEn> for u8 {
    #[inline(always)]
    fn from(val: Reg0EraseCheckEn) -> u8 {
        Reg0EraseCheckEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1EraseCheckEn {
    #[doc = "Region is disabled"]
    DISABLE = 0x0,
    #[doc = "Region is enabled"]
    ENABLE = 0x01,
    #[doc = "Region is enabled"]
    VALUE_2 = 0x02,
    #[doc = "Region is enabled"]
    VALUE_3 = 0x03,
}
impl Reg1EraseCheckEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1EraseCheckEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1EraseCheckEn {
    #[inline(always)]
    fn from(val: u8) -> Reg1EraseCheckEn {
        Reg1EraseCheckEn::from_bits(val)
    }
}
impl From<Reg1EraseCheckEn> for u8 {
    #[inline(always)]
    fn from(val: Reg1EraseCheckEn) -> u8 {
        Reg1EraseCheckEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg2EraseCheckEn {
    #[doc = "Region is disabled"]
    DISABLE = 0x0,
    #[doc = "Region is enabled"]
    ENABLE = 0x01,
    #[doc = "Region is enabled"]
    VALUE_2 = 0x02,
    #[doc = "Region is enabled"]
    VALUE_3 = 0x03,
}
impl Reg2EraseCheckEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg2EraseCheckEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg2EraseCheckEn {
    #[inline(always)]
    fn from(val: u8) -> Reg2EraseCheckEn {
        Reg2EraseCheckEn::from_bits(val)
    }
}
impl From<Reg2EraseCheckEn> for u8 {
    #[inline(always)]
    fn from(val: Reg2EraseCheckEn) -> u8 {
        Reg2EraseCheckEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsa4k {
    #[doc = "Allow RSA2048 and higher"]
    VALUE_0 = 0x0,
    #[doc = "RSA4096 only"]
    VALUE_1 = 0x01,
    #[doc = "RSA4096 only"]
    VALUE_2 = 0x02,
    #[doc = "RSA4096 only"]
    VALUE_3 = 0x03,
}
impl Rsa4k {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsa4k {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsa4k {
    #[inline(always)]
    fn from(val: u8) -> Rsa4k {
        Rsa4k::from_bits(val)
    }
}
impl From<Rsa4k> for u8 {
    #[inline(always)]
    fn from(val: Rsa4k) -> u8 {
        Rsa4k::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecBootEn {
    #[doc = "Plain image (internal flash with or without CRC)"]
    DISABLE = 0x0,
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    ENABLE = 0x01,
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    VALUE_2 = 0x02,
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    VALUE_3 = 0x03,
}
impl SecBootEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecBootEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecBootEn {
    #[inline(always)]
    fn from(val: u8) -> SecBootEn {
        SecBootEn::from_bits(val)
    }
}
impl From<SecBootEn> for u8 {
    #[inline(always)]
    fn from(val: SecBootEn) -> u8 {
        SecBootEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SkipBootSeed {
    #[doc = "Enable BOOT_SEED"]
    ENABLE = 0x0,
    #[doc = "Disable BOOT_SEED"]
    DISABLE = 0x01,
    #[doc = "Disable BOOT_SEED"]
    VALUE_2 = 0x02,
    #[doc = "Disable BOOT_SEED"]
    VALUE_3 = 0x03,
}
impl SkipBootSeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SkipBootSeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SkipBootSeed {
    #[inline(always)]
    fn from(val: u8) -> SkipBootSeed {
        SkipBootSeed::from_bits(val)
    }
}
impl From<SkipBootSeed> for u8 {
    #[inline(always)]
    fn from(val: SkipBootSeed) -> u8 {
        SkipBootSeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SkipDice {
    #[doc = "Enable DICE"]
    ENABLE = 0x0,
    #[doc = "Disable DICE"]
    DISABLE = 0x01,
    #[doc = "Disable DICE"]
    VALUE_2 = 0x02,
    #[doc = "Disable DICE"]
    VALUE_3 = 0x03,
}
impl SkipDice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SkipDice {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SkipDice {
    #[inline(always)]
    fn from(val: u8) -> SkipDice {
        SkipDice::from_bits(val)
    }
}
impl From<SkipDice> for u8 {
    #[inline(always)]
    fn from(val: SkipDice) -> u8 {
        SkipDice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TzmImageType {
    #[doc = "TZ-M image mode is taken from application image header"]
    VALUE_0 = 0x0,
    #[doc = "TZ-M disabled image, boots to non-secure mode"]
    VALUE_1 = 0x01,
    #[doc = "TZ-M enabled image, boots to secure mode"]
    VALUE_2 = 0x02,
    #[doc = "TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header"]
    VALUE_3 = 0x03,
}
impl TzmImageType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TzmImageType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TzmImageType {
    #[inline(always)]
    fn from(val: u8) -> TzmImageType {
        TzmImageType::from_bits(val)
    }
}
impl From<TzmImageType> for u8 {
    #[inline(always)]
    fn from(val: TzmImageType) -> u8 {
        TzmImageType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtal16mhzCapabankTrimTrimValid {
    #[doc = "Capa Bank trimmings not valid. Default trimmings value are used"]
    NOT_TRIM = 0x0,
    #[doc = "Capa Bank trimmings valid"]
    VALID = 0x01,
}
impl Xtal16mhzCapabankTrimTrimValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtal16mhzCapabankTrimTrimValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtal16mhzCapabankTrimTrimValid {
    #[inline(always)]
    fn from(val: u8) -> Xtal16mhzCapabankTrimTrimValid {
        Xtal16mhzCapabankTrimTrimValid::from_bits(val)
    }
}
impl From<Xtal16mhzCapabankTrimTrimValid> for u8 {
    #[inline(always)]
    fn from(val: Xtal16mhzCapabankTrimTrimValid) -> u8 {
        Xtal16mhzCapabankTrimTrimValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtal32khzCapabankTrimTrimValid {
    #[doc = "Capa Bank trimmings not valid. Default trimmings value are used"]
    NOT_TRIM = 0x0,
    #[doc = "Capa Bank trimmings valid"]
    VALID = 0x01,
}
impl Xtal32khzCapabankTrimTrimValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtal32khzCapabankTrimTrimValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtal32khzCapabankTrimTrimValid {
    #[inline(always)]
    fn from(val: u8) -> Xtal32khzCapabankTrimTrimValid {
        Xtal32khzCapabankTrimTrimValid::from_bits(val)
    }
}
impl From<Xtal32khzCapabankTrimTrimValid> for u8 {
    #[inline(always)]
    fn from(val: Xtal32khzCapabankTrimTrimValid) -> u8 {
        Xtal32khzCapabankTrimTrimValid::to_bits(val)
    }
}
