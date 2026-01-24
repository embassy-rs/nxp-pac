#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpha {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE = 0x0,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpol {
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    LOW = 0x0,
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    HIGH = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
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
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    DISABLED = 0x0,
    #[doc = "Enabled. The SPI is enabled for operation."]
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
pub enum Eof {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    NOT_EOF = 0x0,
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    EOF = 0x01,
}
impl Eof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eof {
    #[inline(always)]
    fn from(val: u8) -> Eof {
        Eof::from_bits(val)
    }
}
impl From<Eof> for u8 {
    #[inline(always)]
    fn from(val: Eof) -> u8 {
        Eof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eot {
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    NOT_DEASSERTED = 0x0,
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    DEASSERTED = 0x01,
}
impl Eot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eot {
    #[inline(always)]
    fn from(val: u8) -> Eot {
        Eot::from_bits(val)
    }
}
impl From<Eot> for u8 {
    #[inline(always)]
    fn from(val: Eot) -> u8 {
        Eot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loop {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Enabled."]
    ENABLED = 0x01,
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
pub enum Lsbf {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD = 0x0,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE = 0x01,
}
impl Lsbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsbf {
    #[inline(always)]
    fn from(val: u8) -> Lsbf {
        Lsbf::from_bits(val)
    }
}
impl From<Lsbf> for u8 {
    #[inline(always)]
    fn from(val: Lsbf) -> u8 {
        Lsbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE = 0x0,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstidleen {
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    DISABLED = 0x0,
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    ENABLED = 0x01,
}
impl Mstidleen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstidleen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstidleen {
    #[inline(always)]
    fn from(val: u8) -> Mstidleen {
        Mstidleen::from_bits(val)
    }
}
impl From<Mstidleen> for u8 {
    #[inline(always)]
    fn from(val: Mstidleen) -> u8 {
        Mstidleen::to_bits(val)
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
pub enum Rxignore {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ = 0x0,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE = 0x01,
}
impl Rxignore {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxignore {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxignore {
    #[inline(always)]
    fn from(val: u8) -> Rxignore {
        Rxignore::from_bits(val)
    }
}
impl From<Rxignore> for u8 {
    #[inline(always)]
    fn from(val: Rxignore) -> u8 {
        Rxignore::to_bits(val)
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
pub enum Spol0 {
    #[doc = "Low. The SSEL0 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL0 pin is active high."]
    HIGH = 0x01,
}
impl Spol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol0 {
    #[inline(always)]
    fn from(val: u8) -> Spol0 {
        Spol0::from_bits(val)
    }
}
impl From<Spol0> for u8 {
    #[inline(always)]
    fn from(val: Spol0) -> u8 {
        Spol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol1 {
    #[doc = "Low. The SSEL1 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL1 pin is active high."]
    HIGH = 0x01,
}
impl Spol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol1 {
    #[inline(always)]
    fn from(val: u8) -> Spol1 {
        Spol1::from_bits(val)
    }
}
impl From<Spol1> for u8 {
    #[inline(always)]
    fn from(val: Spol1) -> u8 {
        Spol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol2 {
    #[doc = "Low. The SSEL2 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL2 pin is active high."]
    HIGH = 0x01,
}
impl Spol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol2 {
    #[inline(always)]
    fn from(val: u8) -> Spol2 {
        Spol2::from_bits(val)
    }
}
impl From<Spol2> for u8 {
    #[inline(always)]
    fn from(val: Spol2) -> u8 {
        Spol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol3 {
    #[doc = "Low. The SSEL3 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL3 pin is active high."]
    HIGH = 0x01,
}
impl Spol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol3 {
    #[inline(always)]
    fn from(val: u8) -> Spol3 {
        Spol3::from_bits(val)
    }
}
impl From<Spol3> for u8 {
    #[inline(always)]
    fn from(val: Spol3) -> u8 {
        Spol3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssaen {
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    DISABLED = 0x0,
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    ENABLED = 0x01,
}
impl Ssaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssaen {
    #[inline(always)]
    fn from(val: u8) -> Ssaen {
        Ssaen::from_bits(val)
    }
}
impl From<Ssaen> for u8 {
    #[inline(always)]
    fn from(val: Ssaen) -> u8 {
        Ssaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssden {
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    DISABLED = 0x0,
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    ENABLED = 0x01,
}
impl Ssden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssden {
    #[inline(always)]
    fn from(val: u8) -> Ssden {
        Ssden::from_bits(val)
    }
}
impl From<Ssden> for u8 {
    #[inline(always)]
    fn from(val: Ssden) -> u8 {
        Ssden::to_bits(val)
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
pub enum Txignore {
    #[doc = "Write transmit data. Transmit data must be written for each data exchange between master and slave. In slave mode, an underrun error occurs if transmit data is not provided before needed in a data frame."]
    WRITE = 0x0,
    #[doc = "Ignore transmit data. Data can be received without transmitting data (after FIFOWR has been initialized to set TXIGNORE). No transmitter flags are generated. When configured with TXIGNORE =1, the slave will set the data to always 0."]
    IGNORE = 0x01,
}
impl Txignore {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txignore {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txignore {
    #[inline(always)]
    fn from(val: u8) -> Txignore {
        Txignore::from_bits(val)
    }
}
impl From<Txignore> for u8 {
    #[inline(always)]
    fn from(val: Txignore) -> u8 {
        Txignore::to_bits(val)
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
pub enum Txssel0N {
    #[doc = "SSEL0 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL0 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel0N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel0N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel0N {
    #[inline(always)]
    fn from(val: u8) -> Txssel0N {
        Txssel0N::from_bits(val)
    }
}
impl From<Txssel0N> for u8 {
    #[inline(always)]
    fn from(val: Txssel0N) -> u8 {
        Txssel0N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel1N {
    #[doc = "SSEL1 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL1 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel1N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel1N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel1N {
    #[inline(always)]
    fn from(val: u8) -> Txssel1N {
        Txssel1N::from_bits(val)
    }
}
impl From<Txssel1N> for u8 {
    #[inline(always)]
    fn from(val: Txssel1N) -> u8 {
        Txssel1N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel2N {
    #[doc = "SSEL2 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL2 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel2N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel2N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel2N {
    #[inline(always)]
    fn from(val: u8) -> Txssel2N {
        Txssel2N::from_bits(val)
    }
}
impl From<Txssel2N> for u8 {
    #[inline(always)]
    fn from(val: Txssel2N) -> u8 {
        Txssel2N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel3N {
    #[doc = "SSEL3 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL3 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel3N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel3N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel3N {
    #[inline(always)]
    fn from(val: u8) -> Txssel3N {
        Txssel3N::from_bits(val)
    }
}
impl From<Txssel3N> for u8 {
    #[inline(always)]
    fn from(val: Txssel3N) -> u8 {
        Txssel3N::to_bits(val)
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
