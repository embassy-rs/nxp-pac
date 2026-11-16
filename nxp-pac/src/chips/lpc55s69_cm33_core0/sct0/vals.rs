#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bidir {
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    Up = 0x0,
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UpDown = 0x01,
}
impl Bidir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bidir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bidir {
    #[inline(always)]
    fn from(val: u8) -> Bidir {
        Bidir::from_bits(val)
    }
}
impl From<Bidir> for u8 {
    #[inline(always)]
    fn from(val: Bidir) -> u8 {
        Bidir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cksel {
    #[doc = "Rising edges on input 0."]
    Input0RisingEdges = 0x0,
    #[doc = "Falling edges on input 0."]
    Input0FallingEdge = 0x01,
    #[doc = "Rising edges on input 1."]
    Input1RisingEdges = 0x02,
    #[doc = "Falling edges on input 1."]
    Input1FallingEdge = 0x03,
    #[doc = "Rising edges on input 2."]
    Input2RisingEdges = 0x04,
    #[doc = "Falling edges on input 2."]
    Input2FallingEdge = 0x05,
    #[doc = "Rising edges on input 3."]
    Input3RisingEdges = 0x06,
    #[doc = "Falling edges on input 3."]
    Input3FallingEdge = 0x07,
    #[doc = "Rising edges on input 4."]
    Input4RisingEdges = 0x08,
    #[doc = "Falling edges on input 4."]
    Input4FallingEdge = 0x09,
    #[doc = "Rising edges on input 5."]
    Input5RisingEdges = 0x0a,
    #[doc = "Falling edges on input 5."]
    Input5FallingEdge = 0x0b,
    #[doc = "Rising edges on input 6."]
    Input6RisingEdges = 0x0c,
    #[doc = "Falling edges on input 6."]
    Input6FallingEdge = 0x0d,
    #[doc = "Rising edges on input 7."]
    Input7RisingEdges = 0x0e,
    #[doc = "Falling edges on input 7."]
    Input7FallingEdge = 0x0f,
}
impl Cksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cksel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cksel {
    #[inline(always)]
    fn from(val: u8) -> Cksel {
        Cksel::from_bits(val)
    }
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(val: Cksel) -> u8 {
        Cksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkmode {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SystemClockMode = 0x0,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SampledSystemClockMode = 0x01,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SctInputClockMode = 0x02,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    AsynchronousMode = 0x03,
}
impl Clkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkmode {
    #[inline(always)]
    fn from(val: u8) -> Clkmode {
        Clkmode::from_bits(val)
    }
}
impl From<Clkmode> for u8 {
    #[inline(always)]
    fn from(val: Clkmode) -> u8 {
        Clkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Combmode {
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    Or = 0x0,
    #[doc = "MATCH. Uses the specified match only."]
    Match = 0x01,
    #[doc = "IO. Uses the specified I/O condition only."]
    Io = 0x02,
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    And = 0x03,
}
impl Combmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Combmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Combmode {
    #[inline(always)]
    fn from(val: u8) -> Combmode {
        Combmode::from_bits(val)
    }
}
impl From<Combmode> for u8 {
    #[inline(always)]
    fn from(val: Combmode) -> u8 {
        Combmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    DirectionIndependent = 0x0,
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    CountingUp = 0x01,
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    CountingDown = 0x02,
    _RESERVED_3 = 0x03,
}
impl Direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Direction {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Direction {
    #[inline(always)]
    fn from(val: u8) -> Direction {
        Direction::from_bits(val)
    }
}
impl From<Direction> for u8 {
    #[inline(always)]
    fn from(val: Direction) -> u8 {
        Direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hevent {
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    LCounter = 0x0,
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    HCounter = 0x01,
}
impl Hevent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hevent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hevent {
    #[inline(always)]
    fn from(val: u8) -> Hevent {
        Hevent::from_bits(val)
    }
}
impl From<Hevent> for u8 {
    #[inline(always)]
    fn from(val: Hevent) -> u8 {
        Hevent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iocond {
    #[doc = "LOW."]
    Low = 0x0,
    #[doc = "Rise."]
    Rise = 0x01,
    #[doc = "Fall."]
    Fall = 0x02,
    #[doc = "HIGH."]
    High = 0x03,
}
impl Iocond {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iocond {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iocond {
    #[inline(always)]
    fn from(val: u8) -> Iocond {
        Iocond::from_bits(val)
    }
}
impl From<Iocond> for u8 {
    #[inline(always)]
    fn from(val: Iocond) -> u8 {
        Iocond::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ores {
    #[doc = "No change."]
    NoChange = 0x0,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    Set = 0x01,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    Clear = 0x02,
    #[doc = "Toggle output."]
    ToggleOutput = 0x03,
}
impl Ores {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ores {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ores {
    #[inline(always)]
    fn from(val: u8) -> Ores {
        Ores::from_bits(val)
    }
}
impl From<Ores> for u8 {
    #[inline(always)]
    fn from(val: Ores) -> u8 {
        Ores::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outsel {
    #[doc = "Selects the inputs selected by IOSEL."]
    Input = 0x0,
    #[doc = "Selects the outputs selected by IOSEL."]
    Output = 0x01,
}
impl Outsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outsel {
    #[inline(always)]
    fn from(val: u8) -> Outsel {
        Outsel::from_bits(val)
    }
}
impl From<Outsel> for u8 {
    #[inline(always)]
    fn from(val: Outsel) -> u8 {
        Outsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    Independent = 0x0,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    LReversed = 0x01,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    HReversed = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr {
    #[inline(always)]
    fn from(val: u8) -> Setclr {
        Setclr::from_bits(val)
    }
}
impl From<Setclr> for u8 {
    #[inline(always)]
    fn from(val: Setclr) -> u8 {
        Setclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stateld {
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    Add = 0x0,
    #[doc = "STATEV value is loaded into STATE."]
    Load = 0x01,
}
impl Stateld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stateld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stateld {
    #[inline(always)]
    fn from(val: u8) -> Stateld {
        Stateld::from_bits(val)
    }
}
impl From<Stateld> for u8 {
    #[inline(always)]
    fn from(val: Stateld) -> u8 {
        Stateld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unify {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DualCounter = 0x0,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UnifiedCounter = 0x01,
}
impl Unify {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unify {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unify {
    #[inline(always)]
    fn from(val: u8) -> Unify {
        Unify::from_bits(val)
    }
}
impl From<Unify> for u8 {
    #[inline(always)]
    fn from(val: Unify) -> u8 {
        Unify::to_bits(val)
    }
}
