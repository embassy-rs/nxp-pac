#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg1Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    DualChannel = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SingleChannel = 0x01,
}
impl Cfg1Onechannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg1Onechannel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg1Onechannel {
    #[inline(always)]
    fn from(val: u8) -> Cfg1Onechannel {
        Cfg1Onechannel::from_bits(val)
    }
}
impl From<Cfg1Onechannel> for u8 {
    #[inline(always)]
    fn from(val: Cfg1Onechannel) -> u8 {
        Cfg1Onechannel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datapause {
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    Normal = 0x0,
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    Pause = 0x01,
}
impl Datapause {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datapause {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datapause {
    #[inline(always)]
    fn from(val: u8) -> Datapause {
        Datapause::from_bits(val)
    }
}
impl From<Datapause> for u8 {
    #[inline(always)]
    fn from(val: Datapause) -> u8 {
        Datapause::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Leftjust {
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RightJustified = 0x0,
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LeftJustified = 0x01,
}
impl Leftjust {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Leftjust {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Leftjust {
    #[inline(always)]
    fn from(val: u8) -> Leftjust {
        Leftjust::from_bits(val)
    }
}
impl From<Leftjust> for u8 {
    #[inline(always)]
    fn from(val: Leftjust) -> u8 {
        Leftjust::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lr {
    #[doc = "Left channel."]
    LeftChannel = 0x0,
    #[doc = "Right channel."]
    RightChannel = 0x01,
}
impl Lr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lr {
    #[inline(always)]
    fn from(val: u8) -> Lr {
        Lr::from_bits(val)
    }
}
impl From<Lr> for u8 {
    #[inline(always)]
    fn from(val: Lr) -> u8 {
        Lr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    ClassicMode = 0x0,
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DspModeWs50Dutycycle = 0x01,
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DspModeWs1Clock = 0x02,
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DspModeWs1Data = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstslvcfg {
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NormalSlaveMode = 0x0,
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WsSyncMaster = 0x01,
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MasterUsingSck = 0x02,
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NormalMaster = 0x03,
}
impl Mstslvcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstslvcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstslvcfg {
    #[inline(always)]
    fn from(val: u8) -> Mstslvcfg {
        Mstslvcfg::from_bits(val)
    }
}
impl From<Mstslvcfg> for u8 {
    #[inline(always)]
    fn from(val: Mstslvcfg) -> u8 {
        Mstslvcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pack48 {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    Bit24 = 0x0,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    Bit3216 = 0x01,
}
impl Pack48 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pack48 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pack48 {
    #[inline(always)]
    fn from(val: u8) -> Pack48 {
        Pack48::from_bits(val)
    }
}
impl From<Pack48> for u8 {
    #[inline(always)]
    fn from(val: Pack48) -> u8 {
        Pack48::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paircount {
    #[doc = "1 I2S channel pairs in this flexcomm."]
    Pairs1 = 0x0,
    #[doc = "2 I2S channel pairs in this flexcomm."]
    Pairs2 = 0x01,
    #[doc = "3 I2S channel pairs in this flexcomm."]
    Pairs3 = 0x02,
    #[doc = "4 I2S channel pairs in this flexcomm."]
    Pairs4 = 0x03,
}
impl Paircount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paircount {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paircount {
    #[inline(always)]
    fn from(val: u8) -> Paircount {
        Paircount::from_bits(val)
    }
}
impl From<Paircount> for u8 {
    #[inline(always)]
    fn from(val: Paircount) -> u8 {
        Paircount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcfg11Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    Stereo = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair."]
    Single = 0x01,
}
impl Pcfg11Onechannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcfg11Onechannel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcfg11Onechannel {
    #[inline(always)]
    fn from(val: u8) -> Pcfg11Onechannel {
        Pcfg11Onechannel::from_bits(val)
    }
}
impl From<Pcfg11Onechannel> for u8 {
    #[inline(always)]
    fn from(val: Pcfg11Onechannel) -> u8 {
        Pcfg11Onechannel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcfg12Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    Stereo = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair."]
    Single = 0x01,
}
impl Pcfg12Onechannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcfg12Onechannel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcfg12Onechannel {
    #[inline(always)]
    fn from(val: u8) -> Pcfg12Onechannel {
        Pcfg12Onechannel::from_bits(val)
    }
}
impl From<Pcfg12Onechannel> for u8 {
    #[inline(always)]
    fn from(val: Pcfg12Onechannel) -> u8 {
        Pcfg12Onechannel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcfg13Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    Stereo = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair."]
    Single = 0x01,
}
impl Pcfg13Onechannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcfg13Onechannel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcfg13Onechannel {
    #[inline(always)]
    fn from(val: u8) -> Pcfg13Onechannel {
        Pcfg13Onechannel::from_bits(val)
    }
}
impl From<Pcfg13Onechannel> for u8 {
    #[inline(always)]
    fn from(val: Pcfg13Onechannel) -> u8 {
        Pcfg13Onechannel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pstat1Busy {
    #[doc = "The transmitter/receiver for this channel pair is currently idle."]
    Idle = 0x0,
    #[doc = "The transmitter/receiver for this channel pair is currently processing data."]
    Processing = 0x01,
}
impl Pstat1Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pstat1Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pstat1Busy {
    #[inline(always)]
    fn from(val: u8) -> Pstat1Busy {
        Pstat1Busy::from_bits(val)
    }
}
impl From<Pstat1Busy> for u8 {
    #[inline(always)]
    fn from(val: Pstat1Busy) -> u8 {
        Pstat1Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pstat1Datapaused {
    #[doc = "Data is not currently paused."]
    Pause = 0x0,
    #[doc = "A data pause has been requested and is now in force."]
    Force = 0x01,
}
impl Pstat1Datapaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pstat1Datapaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pstat1Datapaused {
    #[inline(always)]
    fn from(val: u8) -> Pstat1Datapaused {
        Pstat1Datapaused::from_bits(val)
    }
}
impl From<Pstat1Datapaused> for u8 {
    #[inline(always)]
    fn from(val: Pstat1Datapaused) -> u8 {
        Pstat1Datapaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pstat2Busy {
    #[doc = "The transmitter/receiver for this channel pair is currently idle."]
    Idle = 0x0,
    #[doc = "The transmitter/receiver for this channel pair is currently processing data."]
    Processing = 0x01,
}
impl Pstat2Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pstat2Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pstat2Busy {
    #[inline(always)]
    fn from(val: u8) -> Pstat2Busy {
        Pstat2Busy::from_bits(val)
    }
}
impl From<Pstat2Busy> for u8 {
    #[inline(always)]
    fn from(val: Pstat2Busy) -> u8 {
        Pstat2Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pstat2Datapaused {
    #[doc = "Data is not currently paused."]
    Pause = 0x0,
    #[doc = "A data pause has been requested and is now in force."]
    Force = 0x01,
}
impl Pstat2Datapaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pstat2Datapaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pstat2Datapaused {
    #[inline(always)]
    fn from(val: u8) -> Pstat2Datapaused {
        Pstat2Datapaused::from_bits(val)
    }
}
impl From<Pstat2Datapaused> for u8 {
    #[inline(always)]
    fn from(val: Pstat2Datapaused) -> u8 {
        Pstat2Datapaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pstat3Busy {
    #[doc = "The transmitter/receiver for this channel pair is currently idle."]
    Idle = 0x0,
    #[doc = "The transmitter/receiver for this channel pair is currently processing data."]
    Processing = 0x01,
}
impl Pstat3Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pstat3Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pstat3Busy {
    #[inline(always)]
    fn from(val: u8) -> Pstat3Busy {
        Pstat3Busy::from_bits(val)
    }
}
impl From<Pstat3Busy> for u8 {
    #[inline(always)]
    fn from(val: Pstat3Busy) -> u8 {
        Pstat3Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pstat3Datapaused {
    #[doc = "Data is not currently paused."]
    Pause = 0x0,
    #[doc = "A data pause has been requested and is now in force."]
    Force = 0x01,
}
impl Pstat3Datapaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pstat3Datapaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pstat3Datapaused {
    #[inline(always)]
    fn from(val: u8) -> Pstat3Datapaused {
        Pstat3Datapaused::from_bits(val)
    }
}
impl From<Pstat3Datapaused> for u8 {
    #[inline(always)]
    fn from(val: Pstat3Datapaused) -> u8 {
        Pstat3Datapaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rightlow {
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RightHigh = 0x0,
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RightLow = 0x01,
}
impl Rightlow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rightlow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rightlow {
    #[inline(always)]
    fn from(val: u8) -> Rightlow {
        Rightlow::from_bits(val)
    }
}
impl From<Rightlow> for u8 {
    #[inline(always)]
    fn from(val: Rightlow) -> u8 {
        Rightlow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SckPol {
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FallingEdge = 0x0,
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RisingEdge = 0x01,
}
impl SckPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SckPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SckPol {
    #[inline(always)]
    fn from(val: u8) -> SckPol {
        SckPol::from_bits(val)
    }
}
impl From<SckPol> for u8 {
    #[inline(always)]
    fn from(val: SckPol) -> u8 {
        SckPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatBusy {
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    Idle = 0x0,
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    Busy = 0x01,
}
impl StatBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatBusy {
    #[inline(always)]
    fn from(val: u8) -> StatBusy {
        StatBusy::from_bits(val)
    }
}
impl From<StatBusy> for u8 {
    #[inline(always)]
    fn from(val: StatBusy) -> u8 {
        StatBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txi2se0 {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LastValue = 0x0,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    Zero = 0x01,
}
impl Txi2se0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txi2se0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txi2se0 {
    #[inline(always)]
    fn from(val: u8) -> Txi2se0 {
        Txi2se0::from_bits(val)
    }
}
impl From<Txi2se0> for u8 {
    #[inline(always)]
    fn from(val: Txi2se0) -> u8 {
        Txi2se0::to_bits(val)
    }
}
