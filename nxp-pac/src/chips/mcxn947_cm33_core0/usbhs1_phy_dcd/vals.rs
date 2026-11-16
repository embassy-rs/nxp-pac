#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "The sequence is not running."]
    SeqNotRunning = 0x0,
    #[doc = "The sequence is running."]
    SeqRunning = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bc12 {
    #[doc = "Compatible with BC1.1."]
    Bc11 = 0x0,
    #[doc = "Compatible with BC1.2 (default)."]
    Bc12 = 0x01,
}
impl Bc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bc12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bc12 {
    #[inline(always)]
    fn from(val: u8) -> Bc12 {
        Bc12::from_bits(val)
    }
}
impl From<Bc12> for u8 {
    #[inline(always)]
    fn from(val: Bc12) -> u8 {
        Bc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CheckDm {
    _RESERVED_0 = 0x0,
    #[doc = "1 ms - 15 ms."]
    Ms1 = 0x01,
    #[doc = "1 ms - 15 ms."]
    Ms2 = 0x02,
    #[doc = "1 ms - 15 ms."]
    Ms3 = 0x03,
    #[doc = "1 ms - 15 ms."]
    Ms4 = 0x04,
    #[doc = "1 ms - 15 ms."]
    Ms5 = 0x05,
    #[doc = "1 ms - 15 ms."]
    Ms6 = 0x06,
    #[doc = "1 ms - 15 ms."]
    Ms7 = 0x07,
    #[doc = "1 ms - 15 ms."]
    Ms8 = 0x08,
    #[doc = "1 ms - 15 ms."]
    Ms9 = 0x09,
    #[doc = "1 ms - 15 ms."]
    Ms10 = 0x0a,
    #[doc = "1 ms - 15 ms."]
    Ms11 = 0x0b,
    #[doc = "1 ms - 15 ms."]
    Ms12 = 0x0c,
    #[doc = "1 ms - 15 ms."]
    Ms13 = 0x0d,
    #[doc = "1 ms - 15 ms."]
    Ms14 = 0x0e,
    #[doc = "1 ms - 15 ms."]
    Ms15 = 0x0f,
}
impl CheckDm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CheckDm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CheckDm {
    #[inline(always)]
    fn from(val: u8) -> CheckDm {
        CheckDm::from_bits(val)
    }
}
impl From<CheckDm> for u8 {
    #[inline(always)]
    fn from(val: CheckDm) -> u8 {
        CheckDm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockUnit {
    #[doc = "kHz Speed (between 4 kHz and 1023 kHz)."]
    KhzClk = 0x0,
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)."]
    MhzClk = 0x01,
}
impl ClockUnit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockUnit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockUnit {
    #[inline(always)]
    fn from(val: u8) -> ClockUnit {
        ClockUnit::from_bits(val)
    }
}
impl From<ClockUnit> for u8 {
    #[inline(always)]
    fn from(val: ClockUnit) -> u8 {
        ClockUnit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iack {
    #[doc = "Do not clear the interrupt."]
    IntNoclear = 0x0,
    #[doc = "Clear the IF field (interrupt flag)."]
    IntClear = 0x01,
}
impl Iack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iack {
    #[inline(always)]
    fn from(val: u8) -> Iack {
        Iack::from_bits(val)
    }
}
impl From<Iack> for u8 {
    #[inline(always)]
    fn from(val: Iack) -> u8 {
        Iack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie {
    #[doc = "Disable interrupts to the system."]
    DisInt = 0x0,
    #[doc = "Enable interrupts to the system."]
    EnInt = 0x01,
}
impl Ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie {
    #[inline(always)]
    fn from(val: u8) -> Ie {
        Ie::from_bits(val)
    }
}
impl From<Ie> for u8 {
    #[inline(always)]
    fn from(val: Ie) -> u8 {
        Ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum If {
    #[doc = "No interrupt is pending."]
    IntPend = 0x0,
    #[doc = "An interrupt is pending."]
    IntNopend = 0x01,
}
impl If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for If {
    #[inline(always)]
    fn from(val: u8) -> If {
        If::from_bits(val)
    }
}
impl From<If> for u8 {
    #[inline(always)]
    fn from(val: If) -> u8 {
        If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "No overrides. Field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)."]
    NoOverride = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    PriDetOverride = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enables VDM_SRC voltage source only."]
    CdpAdvert = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqRes {
    #[doc = "No results to report."]
    NoResult = 0x0,
    #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    ConnSdp = 0x01,
    #[doc = "Attached to a charging port. The exact meaning depends on the STATUS\\[SEQ_STAT\\] field (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)."]
    ConnCp = 0x02,
    #[doc = "Attached to a DCP."]
    ConnDcp = 0x03,
}
impl SeqRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqRes {
    #[inline(always)]
    fn from(val: u8) -> SeqRes {
        SeqRes::from_bits(val)
    }
}
impl From<SeqRes> for u8 {
    #[inline(always)]
    fn from(val: SeqRes) -> u8 {
        SeqRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqStat {
    #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    NoDataPinConn = 0x0,
    #[doc = "Data pin contact detection is complete."]
    DataPinConn = 0x01,
    #[doc = "Charging port detection is complete."]
    CpDetDone = 0x02,
    #[doc = "Charger type detection is complete."]
    CtDetDone = 0x03,
}
impl SeqStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqStat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqStat {
    #[inline(always)]
    fn from(val: u8) -> SeqStat {
        SeqStat::from_bits(val)
    }
}
impl From<SeqStat> for u8 {
    #[inline(always)]
    fn from(val: SeqStat) -> u8 {
        SeqStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sr {
    #[doc = "Do not perform a software reset."]
    NoReset = 0x0,
    #[doc = "Perform a software reset."]
    SwReset = 0x01,
}
impl Sr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sr {
    #[inline(always)]
    fn from(val: u8) -> Sr {
        Sr::from_bits(val)
    }
}
impl From<Sr> for u8 {
    #[inline(always)]
    fn from(val: Sr) -> u8 {
        Sr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TdcdDbnc(u16);
impl TdcdDbnc {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1018: Self = Self(0x03fa);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1019: Self = Self(0x03fb);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1020: Self = Self(0x03fc);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1021: Self = Self(0x03fd);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1022: Self = Self(0x03fe);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1023: Self = Self(0x03ff);
}
impl TdcdDbnc {
    pub const fn from_bits(val: u16) -> TdcdDbnc {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TdcdDbnc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            0x03fa => f.write_str("Ms1018"),
            0x03fb => f.write_str("Ms1019"),
            0x03fc => f.write_str("Ms1020"),
            0x03fd => f.write_str("Ms1021"),
            0x03fe => f.write_str("Ms1022"),
            0x03ff => f.write_str("Ms1023"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TdcdDbnc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            0x03fa => defmt::write!(f, "Ms1018"),
            0x03fb => defmt::write!(f, "Ms1019"),
            0x03fc => defmt::write!(f, "Ms1020"),
            0x03fd => defmt::write!(f, "Ms1021"),
            0x03fe => defmt::write!(f, "Ms1022"),
            0x03ff => defmt::write!(f, "Ms1023"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TdcdDbnc {
    #[inline(always)]
    fn from(val: u16) -> TdcdDbnc {
        TdcdDbnc::from_bits(val)
    }
}
impl From<TdcdDbnc> for u16 {
    #[inline(always)]
    fn from(val: TdcdDbnc) -> u16 {
        TdcdDbnc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TseqInit(u16);
impl TseqInit {
    #[doc = "0 ms - 1023 ms."]
    pub const Ms0: Self = Self(0x0);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1018: Self = Self(0x03fa);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1019: Self = Self(0x03fb);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1020: Self = Self(0x03fc);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1021: Self = Self(0x03fd);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1022: Self = Self(0x03fe);
    #[doc = "0 ms - 1023 ms."]
    pub const Ms1023: Self = Self(0x03ff);
}
impl TseqInit {
    pub const fn from_bits(val: u16) -> TseqInit {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TseqInit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Ms0"),
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x03fa => f.write_str("Ms1018"),
            0x03fb => f.write_str("Ms1019"),
            0x03fc => f.write_str("Ms1020"),
            0x03fd => f.write_str("Ms1021"),
            0x03fe => f.write_str("Ms1022"),
            0x03ff => f.write_str("Ms1023"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TseqInit {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ms0"),
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x03fa => defmt::write!(f, "Ms1018"),
            0x03fb => defmt::write!(f, "Ms1019"),
            0x03fc => defmt::write!(f, "Ms1020"),
            0x03fd => defmt::write!(f, "Ms1021"),
            0x03fe => defmt::write!(f, "Ms1022"),
            0x03ff => defmt::write!(f, "Ms1023"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TseqInit {
    #[inline(always)]
    fn from(val: u16) -> TseqInit {
        TseqInit::from_bits(val)
    }
}
impl From<TseqInit> for u16 {
    #[inline(always)]
    fn from(val: TseqInit) -> u16 {
        TseqInit::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdmsrcOn(u16);
impl TvdmsrcOn {
    #[doc = "0 ms - 40 ms."]
    pub const Ms0: Self = Self(0x0);
    #[doc = "0 ms - 40 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "0 ms - 40 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "0 ms - 40 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "0 ms - 40 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "0 ms - 40 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "0 ms - 40 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "0 ms - 40 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "0 ms - 40 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "0 ms - 40 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "0 ms - 40 ms."]
    pub const Ms35: Self = Self(0x23);
    #[doc = "0 ms - 40 ms."]
    pub const Ms36: Self = Self(0x24);
    #[doc = "0 ms - 40 ms."]
    pub const Ms37: Self = Self(0x25);
    #[doc = "0 ms - 40 ms."]
    pub const Ms38: Self = Self(0x26);
    #[doc = "0 ms - 40 ms."]
    pub const Ms39: Self = Self(0x27);
    #[doc = "0 ms - 40 ms."]
    pub const Ms40: Self = Self(0x28);
}
impl TvdmsrcOn {
    pub const fn from_bits(val: u16) -> TvdmsrcOn {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TvdmsrcOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Ms0"),
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x23 => f.write_str("Ms35"),
            0x24 => f.write_str("Ms36"),
            0x25 => f.write_str("Ms37"),
            0x26 => f.write_str("Ms38"),
            0x27 => f.write_str("Ms39"),
            0x28 => f.write_str("Ms40"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdmsrcOn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ms0"),
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x23 => defmt::write!(f, "Ms35"),
            0x24 => defmt::write!(f, "Ms36"),
            0x25 => defmt::write!(f, "Ms37"),
            0x26 => defmt::write!(f, "Ms38"),
            0x27 => defmt::write!(f, "Ms39"),
            0x28 => defmt::write!(f, "Ms40"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdmsrcOn {
    #[inline(always)]
    fn from(val: u16) -> TvdmsrcOn {
        TvdmsrcOn::from_bits(val)
    }
}
impl From<TvdmsrcOn> for u16 {
    #[inline(always)]
    fn from(val: TvdmsrcOn) -> u16 {
        TvdmsrcOn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdpsrcCon(u16);
impl TvdpsrcCon {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1018: Self = Self(0x03fa);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1019: Self = Self(0x03fb);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1020: Self = Self(0x03fc);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1021: Self = Self(0x03fd);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1022: Self = Self(0x03fe);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1023: Self = Self(0x03ff);
}
impl TvdpsrcCon {
    pub const fn from_bits(val: u16) -> TvdpsrcCon {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TvdpsrcCon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            0x03fa => f.write_str("Ms1018"),
            0x03fb => f.write_str("Ms1019"),
            0x03fc => f.write_str("Ms1020"),
            0x03fd => f.write_str("Ms1021"),
            0x03fe => f.write_str("Ms1022"),
            0x03ff => f.write_str("Ms1023"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdpsrcCon {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            0x03fa => defmt::write!(f, "Ms1018"),
            0x03fb => defmt::write!(f, "Ms1019"),
            0x03fc => defmt::write!(f, "Ms1020"),
            0x03fd => defmt::write!(f, "Ms1021"),
            0x03fe => defmt::write!(f, "Ms1022"),
            0x03ff => defmt::write!(f, "Ms1023"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdpsrcCon {
    #[inline(always)]
    fn from(val: u16) -> TvdpsrcCon {
        TvdpsrcCon::from_bits(val)
    }
}
impl From<TvdpsrcCon> for u16 {
    #[inline(always)]
    fn from(val: TvdpsrcCon) -> u16 {
        TvdpsrcCon::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdpsrcOn(u16);
impl TvdpsrcOn {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1018: Self = Self(0x03fa);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1019: Self = Self(0x03fb);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1020: Self = Self(0x03fc);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1021: Self = Self(0x03fd);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1022: Self = Self(0x03fe);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1023: Self = Self(0x03ff);
}
impl TvdpsrcOn {
    pub const fn from_bits(val: u16) -> TvdpsrcOn {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TvdpsrcOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            0x03fa => f.write_str("Ms1018"),
            0x03fb => f.write_str("Ms1019"),
            0x03fc => f.write_str("Ms1020"),
            0x03fd => f.write_str("Ms1021"),
            0x03fe => f.write_str("Ms1022"),
            0x03ff => f.write_str("Ms1023"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdpsrcOn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            0x03fa => defmt::write!(f, "Ms1018"),
            0x03fb => defmt::write!(f, "Ms1019"),
            0x03fc => defmt::write!(f, "Ms1020"),
            0x03fd => defmt::write!(f, "Ms1021"),
            0x03fe => defmt::write!(f, "Ms1022"),
            0x03ff => defmt::write!(f, "Ms1023"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdpsrcOn {
    #[inline(always)]
    fn from(val: u16) -> TvdpsrcOn {
        TvdpsrcOn::from_bits(val)
    }
}
impl From<TvdpsrcOn> for u16 {
    #[inline(always)]
    fn from(val: TvdpsrcOn) -> u16 {
        TvdpsrcOn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TwaitAfterPrd(u16);
impl TwaitAfterPrd {
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms10: Self = Self(0x0a);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1018: Self = Self(0x03fa);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1019: Self = Self(0x03fb);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1020: Self = Self(0x03fc);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1021: Self = Self(0x03fd);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1022: Self = Self(0x03fe);
    #[doc = "1 ms - 1023 ms."]
    pub const Ms1023: Self = Self(0x03ff);
}
impl TwaitAfterPrd {
    pub const fn from_bits(val: u16) -> TwaitAfterPrd {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TwaitAfterPrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ms1"),
            0x02 => f.write_str("Ms2"),
            0x03 => f.write_str("Ms3"),
            0x04 => f.write_str("Ms4"),
            0x05 => f.write_str("Ms5"),
            0x06 => f.write_str("Ms6"),
            0x07 => f.write_str("Ms7"),
            0x08 => f.write_str("Ms8"),
            0x09 => f.write_str("Ms9"),
            0x0a => f.write_str("Ms10"),
            0x03fa => f.write_str("Ms1018"),
            0x03fb => f.write_str("Ms1019"),
            0x03fc => f.write_str("Ms1020"),
            0x03fd => f.write_str("Ms1021"),
            0x03fe => f.write_str("Ms1022"),
            0x03ff => f.write_str("Ms1023"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TwaitAfterPrd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ms1"),
            0x02 => defmt::write!(f, "Ms2"),
            0x03 => defmt::write!(f, "Ms3"),
            0x04 => defmt::write!(f, "Ms4"),
            0x05 => defmt::write!(f, "Ms5"),
            0x06 => defmt::write!(f, "Ms6"),
            0x07 => defmt::write!(f, "Ms7"),
            0x08 => defmt::write!(f, "Ms8"),
            0x09 => defmt::write!(f, "Ms9"),
            0x0a => defmt::write!(f, "Ms10"),
            0x03fa => defmt::write!(f, "Ms1018"),
            0x03fb => defmt::write!(f, "Ms1019"),
            0x03fc => defmt::write!(f, "Ms1020"),
            0x03fd => defmt::write!(f, "Ms1021"),
            0x03fe => defmt::write!(f, "Ms1022"),
            0x03ff => defmt::write!(f, "Ms1023"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TwaitAfterPrd {
    #[inline(always)]
    fn from(val: u16) -> TwaitAfterPrd {
        TwaitAfterPrd::from_bits(val)
    }
}
impl From<TwaitAfterPrd> for u16 {
    #[inline(always)]
    fn from(val: TwaitAfterPrd) -> u16 {
        TwaitAfterPrd::to_bits(val)
    }
}
