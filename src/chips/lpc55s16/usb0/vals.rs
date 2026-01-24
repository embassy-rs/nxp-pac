#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrCode {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "PID encoding error"]
    PID_ENCODING_ERROR = 0x01,
    #[doc = "PID unknown"]
    PID_UNKNOWN = 0x02,
    #[doc = "Packet unexpected"]
    PACKET_UNEXPECTED = 0x03,
    #[doc = "Token CRC error"]
    TOKEN_CRC_ERROR = 0x04,
    #[doc = "Data CRC error"]
    DATA_CRC_ERROR = 0x05,
    #[doc = "Time out"]
    TIMEOUT = 0x06,
    #[doc = "Babble"]
    BABBLE = 0x07,
    #[doc = "Truncated EOP"]
    TRUNCATED_EOP = 0x08,
    #[doc = "Sent/Received NAK"]
    SENT_RECEIVED_NAK = 0x09,
    #[doc = "Sent Stall"]
    SENT_STALL = 0x0a,
    #[doc = "Overrun"]
    OVERRUN = 0x0b,
    #[doc = "Sent empty packet"]
    SENT_EMPTY_PACKET = 0x0c,
    #[doc = "Bitstuff error"]
    BITSTUFF_ERROR = 0x0d,
    #[doc = "Sync error"]
    SYNC_ERROR = 0x0e,
    #[doc = "Wrong data toggle"]
    WRONG_DATA_TOGGLE = 0x0f,
}
impl ErrCode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrCode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrCode {
    #[inline(always)]
    fn from(val: u8) -> ErrCode {
        ErrCode::from_bits(val)
    }
}
impl From<ErrCode> for u8 {
    #[inline(always)]
    fn from(val: ErrCode) -> u8 {
        ErrCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceNeedclk {
    #[doc = "USB_NEEDCLK has normal function."]
    NORMAL = 0x0,
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    ALWAYS_ON = 0x01,
}
impl ForceNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceNeedclk {
    #[inline(always)]
    fn from(val: u8) -> ForceNeedclk {
        ForceNeedclk::from_bits(val)
    }
}
impl From<ForceNeedclk> for u8 {
    #[inline(always)]
    fn from(val: ForceNeedclk) -> u8 {
        ForceNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntonnakAi {
    #[doc = "Only acknowledged packets generate an interrupt"]
    DISABLED = 0x0,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 0x01,
}
impl IntonnakAi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntonnakAi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntonnakAi {
    #[inline(always)]
    fn from(val: u8) -> IntonnakAi {
        IntonnakAi::from_bits(val)
    }
}
impl From<IntonnakAi> for u8 {
    #[inline(always)]
    fn from(val: IntonnakAi) -> u8 {
        IntonnakAi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntonnakAo {
    #[doc = "Only acknowledged packets generate an interrupt"]
    DISABLED = 0x0,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 0x01,
}
impl IntonnakAo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntonnakAo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntonnakAo {
    #[inline(always)]
    fn from(val: u8) -> IntonnakAo {
        IntonnakAo::from_bits(val)
    }
}
impl From<IntonnakAo> for u8 {
    #[inline(always)]
    fn from(val: IntonnakAo) -> u8 {
        IntonnakAo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntonnakCi {
    #[doc = "Only acknowledged packets generate an interrupt"]
    DISABLED = 0x0,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 0x01,
}
impl IntonnakCi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntonnakCi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntonnakCi {
    #[inline(always)]
    fn from(val: u8) -> IntonnakCi {
        IntonnakCi::from_bits(val)
    }
}
impl From<IntonnakCi> for u8 {
    #[inline(always)]
    fn from(val: IntonnakCi) -> u8 {
        IntonnakCi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntonnakCo {
    #[doc = "Only acknowledged packets generate an interrupt"]
    DISABLED = 0x0,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 0x01,
}
impl IntonnakCo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntonnakCo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntonnakCo {
    #[inline(always)]
    fn from(val: u8) -> IntonnakCo {
        IntonnakCo::from_bits(val)
    }
}
impl From<IntonnakCo> for u8 {
    #[inline(always)]
    fn from(val: IntonnakCo) -> u8 {
        IntonnakCo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmSup {
    #[doc = "LPM not supported."]
    NO = 0x0,
    #[doc = "LPM supported."]
    YES = 0x01,
}
impl LpmSup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmSup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmSup {
    #[inline(always)]
    fn from(val: u8) -> LpmSup {
        LpmSup::from_bits(val)
    }
}
impl From<LpmSup> for u8 {
    #[inline(always)]
    fn from(val: LpmSup) -> u8 {
        LpmSup::to_bits(val)
    }
}
