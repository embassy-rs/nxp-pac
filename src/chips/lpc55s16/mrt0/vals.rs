#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gflag0 {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT = 0x0,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT = 0x01,
}
impl Gflag0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gflag0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gflag0 {
    #[inline(always)]
    fn from(val: u8) -> Gflag0 {
        Gflag0::from_bits(val)
    }
}
impl From<Gflag0> for u8 {
    #[inline(always)]
    fn from(val: Gflag0) -> u8 {
        Gflag0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inten {
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    DISABLED = 0x0,
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    ENABLED = 0x01,
}
impl Inten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inten {
    #[inline(always)]
    fn from(val: u8) -> Inten {
        Inten::from_bits(val)
    }
}
impl From<Inten> for u8 {
    #[inline(always)]
    fn from(val: Inten) -> u8 {
        Inten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intflag {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT = 0x0,
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT = 0x01,
}
impl Intflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intflag {
    #[inline(always)]
    fn from(val: u8) -> Intflag {
        Intflag::from_bits(val)
    }
}
impl From<Intflag> for u8 {
    #[inline(always)]
    fn from(val: Intflag) -> u8 {
        Intflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inuse {
    #[doc = "This channel is not in use."]
    NO = 0x0,
    #[doc = "This channel is in use."]
    YES = 0x01,
}
impl Inuse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inuse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inuse {
    #[inline(always)]
    fn from(val: u8) -> Inuse {
        Inuse::from_bits(val)
    }
}
impl From<Inuse> for u8 {
    #[inline(always)]
    fn from(val: Inuse) -> u8 {
        Inuse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Load {
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD = 0x0,
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD = 0x01,
}
impl Load {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Load {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Load {
    #[inline(always)]
    fn from(val: u8) -> Load {
        Load::from_bits(val)
    }
}
impl From<Load> for u8 {
    #[inline(always)]
    fn from(val: Load) -> u8 {
        Load::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Repeat interrupt mode."]
    REPEAT_INTERRUPT_MODE = 0x0,
    #[doc = "One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_MODE = 0x01,
    #[doc = "One-shot stall mode."]
    ONE_SHOT_STALL_MODE = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Multitask {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE = 0x0,
    #[doc = "Multi-task mode."]
    MULTI_TASK_MODE = 0x01,
}
impl Multitask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Multitask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Multitask {
    #[inline(always)]
    fn from(val: u8) -> Multitask {
        Multitask::from_bits(val)
    }
}
impl From<Multitask> for u8 {
    #[inline(always)]
    fn from(val: Multitask) -> u8 {
        Multitask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Run {
    #[doc = "Idle state. TIMERn is stopped."]
    IDLE_STATE = 0x0,
    #[doc = "Running. TIMERn is running."]
    RUNNING = 0x01,
}
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
