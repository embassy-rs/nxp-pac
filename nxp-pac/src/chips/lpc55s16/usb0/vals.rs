#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrCode {
    #[doc = "No error."]
    NoError = 0x0,
    #[doc = "PID encoding error."]
    PidEncodingError = 0x01,
    #[doc = "PID unknown."]
    PidUnknown = 0x02,
    #[doc = "Packet unexpected."]
    PacketUnexpected = 0x03,
    #[doc = "Token CRC error."]
    TokenCrcError = 0x04,
    #[doc = "Data CRC error."]
    DataCrcError = 0x05,
    #[doc = "Time out."]
    Timeout = 0x06,
    #[doc = "Babble."]
    Babble = 0x07,
    #[doc = "Truncated EOP."]
    TruncatedEop = 0x08,
    #[doc = "Sent/Received NAK."]
    SentReceivedNak = 0x09,
    #[doc = "Sent Stall."]
    SentStall = 0x0a,
    #[doc = "Overrun."]
    Overrun = 0x0b,
    #[doc = "Sent empty packet."]
    SentEmptyPacket = 0x0c,
    #[doc = "Bitstuff error."]
    BitstuffError = 0x0d,
    #[doc = "Sync error."]
    SyncError = 0x0e,
    #[doc = "Wrong data toggle."]
    WrongDataToggle = 0x0f,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceNeedclk {
    #[doc = "USB_NEEDCLK has normal function."]
    Normal = 0x0,
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    AlwaysOn = 0x01,
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
