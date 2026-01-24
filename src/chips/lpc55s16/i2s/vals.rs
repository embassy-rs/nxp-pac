#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg1Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    DUAL_CHANNEL = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SINGLE_CHANNEL = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datapause {
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    NORMAL = 0x0,
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    PAUSE = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmarx {
    #[doc = "DMA is not used for the receive function."]
    DISABLED = 0x0,
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED = 0x01,
}
impl Dmarx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmarx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmarx {
    #[inline(always)]
    fn from(val: u8) -> Dmarx {
        Dmarx::from_bits(val)
    }
}
impl From<Dmarx> for u8 {
    #[inline(always)]
    fn from(val: Dmarx) -> u8 {
        Dmarx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmatx {
    #[doc = "DMA is not used for the transmit function."]
    DISABLED = 0x0,
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED = 0x01,
}
impl Dmatx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmatx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmatx {
    #[inline(always)]
    fn from(val: u8) -> Dmatx {
        Dmatx::from_bits(val)
    }
}
impl From<Dmatx> for u8 {
    #[inline(always)]
    fn from(val: Dmatx) -> u8 {
        Dmatx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enablerx {
    #[doc = "The receive FIFO is not enabled."]
    DISABLED = 0x0,
    #[doc = "The receive FIFO is enabled."]
    ENABLED = 0x01,
}
impl Enablerx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enablerx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enablerx {
    #[inline(always)]
    fn from(val: u8) -> Enablerx {
        Enablerx::from_bits(val)
    }
}
impl From<Enablerx> for u8 {
    #[inline(always)]
    fn from(val: Enablerx) -> u8 {
        Enablerx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enabletx {
    #[doc = "The transmit FIFO is not enabled."]
    DISABLED = 0x0,
    #[doc = "The transmit FIFO is enabled."]
    ENABLED = 0x01,
}
impl Enabletx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enabletx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enabletx {
    #[inline(always)]
    fn from(val: u8) -> Enabletx {
        Enabletx::from_bits(val)
    }
}
impl From<Enabletx> for u8 {
    #[inline(always)]
    fn from(val: Enabletx) -> u8 {
        Enabletx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Leftjust {
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RIGHT_JUSTIFIED = 0x0,
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LEFT_JUSTIFIED = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lr {
    #[doc = "Left channel."]
    LEFT_CHANNEL = 0x0,
    #[doc = "Right channel."]
    RIGHT_CHANNEL = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mainenable {
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    DISABLED = 0x0,
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    ENABLED = 0x01,
}
impl Mainenable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mainenable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mainenable {
    #[inline(always)]
    fn from(val: u8) -> Mainenable {
        Mainenable::from_bits(val)
    }
}
impl From<Mainenable> for u8 {
    #[inline(always)]
    fn from(val: Mainenable) -> u8 {
        Mainenable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    CLASSIC_MODE = 0x0,
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DSP_MODE_WS_50_DUTYCYCLE = 0x01,
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_CLOCK = 0x02,
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_DATA = 0x03,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstslvcfg {
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NORMAL_SLAVE_MODE = 0x0,
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WS_SYNC_MASTER = 0x01,
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MASTER_USING_SCK = 0x02,
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NORMAL_MASTER = 0x03,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pack48 {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    BIT_24 = 0x0,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    BIT_32_16 = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paircount {
    #[doc = "1 I2S channel pairs in this flexcomm"]
    PAIRS_1 = 0x0,
    #[doc = "2 I2S channel pairs in this flexcomm"]
    PAIRS_2 = 0x01,
    #[doc = "3 I2S channel pairs in this flexcomm"]
    PAIRS_3 = 0x02,
    #[doc = "4 I2S channel pairs in this flexcomm"]
    PAIRS_4 = 0x03,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pairenable {
    #[doc = "This I2S channel pair is disabled."]
    DISABLE = 0x0,
    #[doc = "This I2S channel pair is enabled."]
    ENABLE = 0x01,
}
impl Pairenable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pairenable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pairenable {
    #[inline(always)]
    fn from(val: u8) -> Pairenable {
        Pairenable::from_bits(val)
    }
}
impl From<Pairenable> for u8 {
    #[inline(always)]
    fn from(val: Pairenable) -> u8 {
        Pairenable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcfg1Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    STEREO = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair."]
    SINGLE = 0x01,
}
impl Pcfg1Onechannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcfg1Onechannel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcfg1Onechannel {
    #[inline(always)]
    fn from(val: u8) -> Pcfg1Onechannel {
        Pcfg1Onechannel::from_bits(val)
    }
}
impl From<Pcfg1Onechannel> for u8 {
    #[inline(always)]
    fn from(val: Pcfg1Onechannel) -> u8 {
        Pcfg1Onechannel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PstatBusy {
    #[doc = "The transmitter/receiver for this channel pair is currently idle."]
    IDLE = 0x0,
    #[doc = "The transmitter/receiver for this channel pair is currently processing data."]
    PROCESSING = 0x01,
}
impl PstatBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PstatBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PstatBusy {
    #[inline(always)]
    fn from(val: u8) -> PstatBusy {
        PstatBusy::from_bits(val)
    }
}
impl From<PstatBusy> for u8 {
    #[inline(always)]
    fn from(val: PstatBusy) -> u8 {
        PstatBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PstatDatapaused {
    #[doc = "Data is not currently paused."]
    PAUSE = 0x0,
    #[doc = "A data pause has been requested and is now in force."]
    FORCE = 0x01,
}
impl PstatDatapaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PstatDatapaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PstatDatapaused {
    #[inline(always)]
    fn from(val: u8) -> PstatDatapaused {
        PstatDatapaused::from_bits(val)
    }
}
impl From<PstatDatapaused> for u8 {
    #[inline(always)]
    fn from(val: PstatDatapaused) -> u8 {
        PstatDatapaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rightlow {
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RIGHT_HIGH = 0x0,
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RIGHT_LOW = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxerr {
    #[doc = "No interrupt will be generated for a receive error."]
    DISABLED = 0x0,
    #[doc = "An interrupt will be generated when a receive error occurs."]
    ENABLED = 0x01,
}
impl Rxerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxerr {
    #[inline(always)]
    fn from(val: u8) -> Rxerr {
        Rxerr::from_bits(val)
    }
}
impl From<Rxerr> for u8 {
    #[inline(always)]
    fn from(val: Rxerr) -> u8 {
        Rxerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxlvl {
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    DISABLED = 0x0,
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    ENABLED = 0x01,
}
impl Rxlvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxlvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxlvl {
    #[inline(always)]
    fn from(val: u8) -> Rxlvl {
        Rxlvl::from_bits(val)
    }
}
impl From<Rxlvl> for u8 {
    #[inline(always)]
    fn from(val: Rxlvl) -> u8 {
        Rxlvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxlvlena {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0x0,
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED = 0x01,
}
impl Rxlvlena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxlvlena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxlvlena {
    #[inline(always)]
    fn from(val: u8) -> Rxlvlena {
        Rxlvlena::from_bits(val)
    }
}
impl From<Rxlvlena> for u8 {
    #[inline(always)]
    fn from(val: Rxlvlena) -> u8 {
        Rxlvlena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SckPol {
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FALLING_EDGE = 0x0,
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RISING_EDGE = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvfrmerr {
    #[doc = "No error has been recorded."]
    NO_ERROR = 0x0,
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    ERROR = 0x01,
}
impl Slvfrmerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvfrmerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvfrmerr {
    #[inline(always)]
    fn from(val: u8) -> Slvfrmerr {
        Slvfrmerr::from_bits(val)
    }
}
impl From<Slvfrmerr> for u8 {
    #[inline(always)]
    fn from(val: Slvfrmerr) -> u8 {
        Slvfrmerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatBusy {
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    IDLE = 0x0,
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    BUSY = 0x01,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatDatapaused {
    #[doc = "Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description of the DATAPAUSE control bit in the CFG1 register."]
    NOT_PAUSED = 0x0,
    #[doc = "A data pause has been requested and is now in force."]
    PAUSED = 0x01,
}
impl StatDatapaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatDatapaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatDatapaused {
    #[inline(always)]
    fn from(val: u8) -> StatDatapaused {
        StatDatapaused::from_bits(val)
    }
}
impl From<StatDatapaused> for u8 {
    #[inline(always)]
    fn from(val: StatDatapaused) -> u8 {
        StatDatapaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txerr {
    #[doc = "No interrupt will be generated for a transmit error."]
    DISABLED = 0x0,
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    ENABLED = 0x01,
}
impl Txerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txerr {
    #[inline(always)]
    fn from(val: u8) -> Txerr {
        Txerr::from_bits(val)
    }
}
impl From<Txerr> for u8 {
    #[inline(always)]
    fn from(val: Txerr) -> u8 {
        Txerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txi2se0 {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LAST_VALUE = 0x0,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    ZERO = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txlvl {
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    DISABLED = 0x0,
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    ENABLED = 0x01,
}
impl Txlvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txlvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txlvl {
    #[inline(always)]
    fn from(val: u8) -> Txlvl {
        Txlvl::from_bits(val)
    }
}
impl From<Txlvl> for u8 {
    #[inline(always)]
    fn from(val: Txlvl) -> u8 {
        Txlvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txlvlena {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0x0,
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED = 0x01,
}
impl Txlvlena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txlvlena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txlvlena {
    #[inline(always)]
    fn from(val: u8) -> Txlvlena {
        Txlvlena::from_bits(val)
    }
}
impl From<Txlvlena> for u8 {
    #[inline(always)]
    fn from(val: Txlvlena) -> u8 {
        Txlvlena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakerx {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED = 0x0,
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED = 0x01,
}
impl Wakerx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakerx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakerx {
    #[inline(always)]
    fn from(val: u8) -> Wakerx {
        Wakerx::from_bits(val)
    }
}
impl From<Wakerx> for u8 {
    #[inline(always)]
    fn from(val: Wakerx) -> u8 {
        Wakerx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Waketx {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED = 0x0,
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED = 0x01,
}
impl Waketx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Waketx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Waketx {
    #[inline(always)]
    fn from(val: u8) -> Waketx {
        Waketx::from_bits(val)
    }
}
impl From<Waketx> for u8 {
    #[inline(always)]
    fn from(val: Waketx) -> u8 {
        Waketx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WsPol {
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    NOT_INVERTED = 0x0,
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    INVERTED = 0x01,
}
impl WsPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WsPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WsPol {
    #[inline(always)]
    fn from(val: u8) -> WsPol {
        WsPol::from_bits(val)
    }
}
impl From<WsPol> for u8 {
    #[inline(always)]
    fn from(val: WsPol) -> u8 {
        WsPol::to_bits(val)
    }
}
