#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addrdet {
    #[doc = "Disabled. The USART presents all incoming data."]
    DISABLED = 0x0,
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    ENABLED = 0x01,
}
impl Addrdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addrdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addrdet {
    #[inline(always)]
    fn from(val: u8) -> Addrdet {
        Addrdet::from_bits(val)
    }
}
impl From<Addrdet> for u8 {
    #[inline(always)]
    fn from(val: Addrdet) -> u8 {
        Addrdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autoaddr {
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    DISABLED = 0x0,
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    ENABLED = 0x01,
}
impl Autoaddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autoaddr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autoaddr {
    #[inline(always)]
    fn from(val: u8) -> Autoaddr {
        Autoaddr::from_bits(val)
    }
}
impl From<Autoaddr> for u8 {
    #[inline(always)]
    fn from(val: Autoaddr) -> u8 {
        Autoaddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autobaud {
    #[doc = "Disabled. USART is in normal operating mode."]
    DISABLED = 0x0,
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    ENABLED = 0x01,
}
impl Autobaud {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autobaud {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autobaud {
    #[inline(always)]
    fn from(val: u8) -> Autobaud {
        Autobaud::from_bits(val)
    }
}
impl From<Autobaud> for u8 {
    #[inline(always)]
    fn from(val: Autobaud) -> u8 {
        Autobaud::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER = 0x0,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK = 0x01,
}
impl Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc {
    #[inline(always)]
    fn from(val: u8) -> Cc {
        Cc::from_bits(val)
    }
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(val: Cc) -> u8 {
        Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkpol {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE = 0x0,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE = 0x01,
}
impl Clkpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkpol {
    #[inline(always)]
    fn from(val: u8) -> Clkpol {
        Clkpol::from_bits(val)
    }
}
impl From<Clkpol> for u8 {
    #[inline(always)]
    fn from(val: Clkpol) -> u8 {
        Clkpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrcconrx {
    #[doc = "No effect. No effect on the CC bit."]
    NO_EFFECT = 0x0,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR = 0x01,
}
impl Clrcconrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrcconrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrcconrx {
    #[inline(always)]
    fn from(val: u8) -> Clrcconrx {
        Clrcconrx::from_bits(val)
    }
}
impl From<Clrcconrx> for u8 {
    #[inline(always)]
    fn from(val: Clrcconrx) -> u8 {
        Clrcconrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctsen {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    DISABLED = 0x0,
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    ENABLED = 0x01,
}
impl Ctsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctsen {
    #[inline(always)]
    fn from(val: u8) -> Ctsen {
        Ctsen::from_bits(val)
    }
}
impl From<Ctsen> for u8 {
    #[inline(always)]
    fn from(val: Ctsen) -> u8 {
        Ctsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datalen {
    #[doc = "7 bit Data length."]
    BIT_7 = 0x0,
    #[doc = "8 bit Data length."]
    BIT_8 = 0x01,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    BIT_9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Datalen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datalen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datalen {
    #[inline(always)]
    fn from(val: u8) -> Datalen {
        Datalen::from_bits(val)
    }
}
impl From<Datalen> for u8 {
    #[inline(always)]
    fn from(val: Datalen) -> u8 {
        Datalen::to_bits(val)
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
pub enum Enable {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    DISABLED = 0x0,
    #[doc = "Enabled. The USART is enabled for operation."]
    ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
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
pub enum Linmode {
    #[doc = "Disabled. Break detect and generate is configured for normal operation."]
    DISABLED = 0x0,
    #[doc = "Enabled. Break detect and generate is configured for LIN bus operation."]
    ENABLED = 0x01,
}
impl Linmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linmode {
    #[inline(always)]
    fn from(val: u8) -> Linmode {
        Linmode::from_bits(val)
    }
}
impl From<Linmode> for u8 {
    #[inline(always)]
    fn from(val: Linmode) -> u8 {
        Linmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loop {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK = 0x01,
}
impl Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loop {
    #[inline(always)]
    fn from(val: u8) -> Loop {
        Loop::from_bits(val)
    }
}
impl From<Loop> for u8 {
    #[inline(always)]
    fn from(val: Loop) -> u8 {
        Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode32k {
    #[doc = "Disabled. USART uses standard clocking."]
    DISABLED = 0x0,
    #[doc = "Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    ENABLED = 0x01,
}
impl Mode32k {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode32k {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode32k {
    #[inline(always)]
    fn from(val: u8) -> Mode32k {
        Mode32k::from_bits(val)
    }
}
impl From<Mode32k> for u8 {
    #[inline(always)]
    fn from(val: Mode32k) -> u8 {
        Mode32k::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oepol {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    LOW = 0x0,
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    HIGH = 0x01,
}
impl Oepol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oepol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oepol {
    #[inline(always)]
    fn from(val: u8) -> Oepol {
        Oepol::from_bits(val)
    }
}
impl From<Oepol> for u8 {
    #[inline(always)]
    fn from(val: Oepol) -> u8 {
        Oepol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oesel {
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    STANDARD = 0x0,
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    RS_485 = 0x01,
}
impl Oesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oesel {
    #[inline(always)]
    fn from(val: u8) -> Oesel {
        Oesel::from_bits(val)
    }
}
impl From<Oesel> for u8 {
    #[inline(always)]
    fn from(val: Oesel) -> u8 {
        Oesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oeta {
    #[doc = "Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    DISABLED = 0x0,
    #[doc = "Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    ENABLED = 0x01,
}
impl Oeta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oeta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oeta {
    #[inline(always)]
    fn from(val: u8) -> Oeta {
        Oeta::from_bits(val)
    }
}
impl From<Oeta> for u8 {
    #[inline(always)]
    fn from(val: Oeta) -> u8 {
        Oeta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paritysel {
    #[doc = "No parity."]
    NO_PARITY = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY = 0x02,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY = 0x03,
}
impl Paritysel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paritysel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paritysel {
    #[inline(always)]
    fn from(val: u8) -> Paritysel {
        Paritysel::from_bits(val)
    }
}
impl From<Paritysel> for u8 {
    #[inline(always)]
    fn from(val: Paritysel) -> u8 {
        Paritysel::to_bits(val)
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
pub enum Rxpol {
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0x0,
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 0x01,
}
impl Rxpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpol {
    #[inline(always)]
    fn from(val: u8) -> Rxpol {
        Rxpol::from_bits(val)
    }
}
impl From<Rxpol> for u8 {
    #[inline(always)]
    fn from(val: Rxpol) -> u8 {
        Rxpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stoplen {
    #[doc = "1 stop bit."]
    BIT_1 = 0x0,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    BITS_2 = 0x01,
}
impl Stoplen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stoplen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stoplen {
    #[inline(always)]
    fn from(val: u8) -> Stoplen {
        Stoplen::from_bits(val)
    }
}
impl From<Stoplen> for u8 {
    #[inline(always)]
    fn from(val: Stoplen) -> u8 {
        Stoplen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syncen {
    #[doc = "Asynchronous mode."]
    ASYNCHRONOUS_MODE = 0x0,
    #[doc = "Synchronous mode."]
    SYNCHRONOUS_MODE = 0x01,
}
impl Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syncen {
    #[inline(always)]
    fn from(val: u8) -> Syncen {
        Syncen::from_bits(val)
    }
}
impl From<Syncen> for u8 {
    #[inline(always)]
    fn from(val: Syncen) -> u8 {
        Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syncmst {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE = 0x0,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    MASTER = 0x01,
}
impl Syncmst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syncmst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syncmst {
    #[inline(always)]
    fn from(val: u8) -> Syncmst {
        Syncmst::from_bits(val)
    }
}
impl From<Syncmst> for u8 {
    #[inline(always)]
    fn from(val: Syncmst) -> u8 {
        Syncmst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txbrken {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS = 0x01,
}
impl Txbrken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txbrken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txbrken {
    #[inline(always)]
    fn from(val: u8) -> Txbrken {
        Txbrken::from_bits(val)
    }
}
impl From<Txbrken> for u8 {
    #[inline(always)]
    fn from(val: Txbrken) -> u8 {
        Txbrken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdis {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    ENABLED = 0x0,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED = 0x01,
}
impl Txdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdis {
    #[inline(always)]
    fn from(val: u8) -> Txdis {
        Txdis::from_bits(val)
    }
}
impl From<Txdis> for u8 {
    #[inline(always)]
    fn from(val: Txdis) -> u8 {
        Txdis::to_bits(val)
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
pub enum Txpol {
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0x0,
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 0x01,
}
impl Txpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpol {
    #[inline(always)]
    fn from(val: u8) -> Txpol {
        Txpol::from_bits(val)
    }
}
impl From<Txpol> for u8 {
    #[inline(always)]
    fn from(val: Txpol) -> u8 {
        Txpol::to_bits(val)
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
