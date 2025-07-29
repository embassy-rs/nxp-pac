#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio00Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio00Asw {
        Pio00Asw::from_bits(val)
    }
}
impl From<Pio00Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio00Asw) -> u8 {
        Pio00Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio00Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio00Digimode {
        Pio00Digimode::from_bits(val)
    }
}
impl From<Pio00Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio00Digimode) -> u8 {
        Pio00Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio00Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Func {
    #[inline(always)]
    fn from(val: u8) -> Pio00Func {
        Pio00Func::from_bits(val)
    }
}
impl From<Pio00Func> for u8 {
    #[inline(always)]
    fn from(val: Pio00Func) -> u8 {
        Pio00Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio00Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio00Mode {
        Pio00Mode::from_bits(val)
    }
}
impl From<Pio00Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio00Mode) -> u8 {
        Pio00Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio00Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Od {
    #[inline(always)]
    fn from(val: u8) -> Pio00Od {
        Pio00Od::from_bits(val)
    }
}
impl From<Pio00Od> for u8 {
    #[inline(always)]
    fn from(val: Pio00Od) -> u8 {
        Pio00Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio00Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio00Slew {
        Pio00Slew::from_bits(val)
    }
}
impl From<Pio00Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio00Slew) -> u8 {
        Pio00Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio010Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio010Asw {
        Pio010Asw::from_bits(val)
    }
}
impl From<Pio010Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio010Asw) -> u8 {
        Pio010Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio010Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio010Digimode {
        Pio010Digimode::from_bits(val)
    }
}
impl From<Pio010Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio010Digimode) -> u8 {
        Pio010Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio010Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Func {
    #[inline(always)]
    fn from(val: u8) -> Pio010Func {
        Pio010Func::from_bits(val)
    }
}
impl From<Pio010Func> for u8 {
    #[inline(always)]
    fn from(val: Pio010Func) -> u8 {
        Pio010Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio010Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio010Mode {
        Pio010Mode::from_bits(val)
    }
}
impl From<Pio010Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio010Mode) -> u8 {
        Pio010Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio010Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Od {
    #[inline(always)]
    fn from(val: u8) -> Pio010Od {
        Pio010Od::from_bits(val)
    }
}
impl From<Pio010Od> for u8 {
    #[inline(always)]
    fn from(val: Pio010Od) -> u8 {
        Pio010Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio010Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio010Slew {
        Pio010Slew::from_bits(val)
    }
}
impl From<Pio010Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio010Slew) -> u8 {
        Pio010Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio011Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio011Asw {
        Pio011Asw::from_bits(val)
    }
}
impl From<Pio011Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio011Asw) -> u8 {
        Pio011Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio011Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio011Digimode {
        Pio011Digimode::from_bits(val)
    }
}
impl From<Pio011Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio011Digimode) -> u8 {
        Pio011Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio011Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Func {
    #[inline(always)]
    fn from(val: u8) -> Pio011Func {
        Pio011Func::from_bits(val)
    }
}
impl From<Pio011Func> for u8 {
    #[inline(always)]
    fn from(val: Pio011Func) -> u8 {
        Pio011Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio011Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio011Mode {
        Pio011Mode::from_bits(val)
    }
}
impl From<Pio011Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio011Mode) -> u8 {
        Pio011Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio011Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Od {
    #[inline(always)]
    fn from(val: u8) -> Pio011Od {
        Pio011Od::from_bits(val)
    }
}
impl From<Pio011Od> for u8 {
    #[inline(always)]
    fn from(val: Pio011Od) -> u8 {
        Pio011Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio011Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio011Slew {
        Pio011Slew::from_bits(val)
    }
}
impl From<Pio011Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio011Slew) -> u8 {
        Pio011Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio012Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio012Asw {
        Pio012Asw::from_bits(val)
    }
}
impl From<Pio012Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio012Asw) -> u8 {
        Pio012Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio012Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio012Digimode {
        Pio012Digimode::from_bits(val)
    }
}
impl From<Pio012Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio012Digimode) -> u8 {
        Pio012Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio012Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Func {
    #[inline(always)]
    fn from(val: u8) -> Pio012Func {
        Pio012Func::from_bits(val)
    }
}
impl From<Pio012Func> for u8 {
    #[inline(always)]
    fn from(val: Pio012Func) -> u8 {
        Pio012Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio012Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio012Mode {
        Pio012Mode::from_bits(val)
    }
}
impl From<Pio012Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio012Mode) -> u8 {
        Pio012Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio012Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Od {
    #[inline(always)]
    fn from(val: u8) -> Pio012Od {
        Pio012Od::from_bits(val)
    }
}
impl From<Pio012Od> for u8 {
    #[inline(always)]
    fn from(val: Pio012Od) -> u8 {
        Pio012Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio012Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio012Slew {
        Pio012Slew::from_bits(val)
    }
}
impl From<Pio012Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio012Slew) -> u8 {
        Pio012Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio013Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio013Digimode {
        Pio013Digimode::from_bits(val)
    }
}
impl From<Pio013Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio013Digimode) -> u8 {
        Pio013Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Egp {
    #[doc = "I2C mode."]
    I2C_MODE = 0x0,
    #[doc = "GPIO mode."]
    GPIO_MODE = 0x01,
}
impl Pio013Egp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Egp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Egp {
    #[inline(always)]
    fn from(val: u8) -> Pio013Egp {
        Pio013Egp::from_bits(val)
    }
}
impl From<Pio013Egp> for u8 {
    #[inline(always)]
    fn from(val: Pio013Egp) -> u8 {
        Pio013Egp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Filteroff {
    #[doc = "Filter enabled."]
    ENABLED = 0x0,
    #[doc = "Filter disabled."]
    DISABLED = 0x01,
}
impl Pio013Filteroff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Filteroff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Filteroff {
    #[inline(always)]
    fn from(val: u8) -> Pio013Filteroff {
        Pio013Filteroff::from_bits(val)
    }
}
impl From<Pio013Filteroff> for u8 {
    #[inline(always)]
    fn from(val: Pio013Filteroff) -> u8 {
        Pio013Filteroff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio013Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Func {
    #[inline(always)]
    fn from(val: u8) -> Pio013Func {
        Pio013Func::from_bits(val)
    }
}
impl From<Pio013Func> for u8 {
    #[inline(always)]
    fn from(val: Pio013Func) -> u8 {
        Pio013Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013I2cfilter {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE = 0x0,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    STANDARD_MODE = 0x01,
}
impl Pio013I2cfilter {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013I2cfilter {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013I2cfilter {
    #[inline(always)]
    fn from(val: u8) -> Pio013I2cfilter {
        Pio013I2cfilter::from_bits(val)
    }
}
impl From<Pio013I2cfilter> for u8 {
    #[inline(always)]
    fn from(val: Pio013I2cfilter) -> u8 {
        Pio013I2cfilter::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio013Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio013Mode {
        Pio013Mode::from_bits(val)
    }
}
impl From<Pio013Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio013Mode) -> u8 {
        Pio013Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio013Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Od {
    #[inline(always)]
    fn from(val: u8) -> Pio013Od {
        Pio013Od::from_bits(val)
    }
}
impl From<Pio013Od> for u8 {
    #[inline(always)]
    fn from(val: Pio013Od) -> u8 {
        Pio013Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio013Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio013Slew {
        Pio013Slew::from_bits(val)
    }
}
impl From<Pio013Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio013Slew) -> u8 {
        Pio013Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Ssel {
    #[doc = "3V3 Signaling in I2C Mode."]
    SEL3V3 = 0x0,
    #[doc = "1V8 Signaling in I2C Mode."]
    SEL1V8 = 0x01,
}
impl Pio013Ssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Ssel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Ssel {
    #[inline(always)]
    fn from(val: u8) -> Pio013Ssel {
        Pio013Ssel::from_bits(val)
    }
}
impl From<Pio013Ssel> for u8 {
    #[inline(always)]
    fn from(val: Pio013Ssel) -> u8 {
        Pio013Ssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio014Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio014Digimode {
        Pio014Digimode::from_bits(val)
    }
}
impl From<Pio014Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio014Digimode) -> u8 {
        Pio014Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Egp {
    #[doc = "I2C mode."]
    I2C_MODE = 0x0,
    #[doc = "GPIO mode."]
    GPIO_MODE = 0x01,
}
impl Pio014Egp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Egp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Egp {
    #[inline(always)]
    fn from(val: u8) -> Pio014Egp {
        Pio014Egp::from_bits(val)
    }
}
impl From<Pio014Egp> for u8 {
    #[inline(always)]
    fn from(val: Pio014Egp) -> u8 {
        Pio014Egp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Filteroff {
    #[doc = "Filter enabled."]
    ENABLED = 0x0,
    #[doc = "Filter disabled."]
    DISABLED = 0x01,
}
impl Pio014Filteroff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Filteroff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Filteroff {
    #[inline(always)]
    fn from(val: u8) -> Pio014Filteroff {
        Pio014Filteroff::from_bits(val)
    }
}
impl From<Pio014Filteroff> for u8 {
    #[inline(always)]
    fn from(val: Pio014Filteroff) -> u8 {
        Pio014Filteroff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio014Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Func {
    #[inline(always)]
    fn from(val: u8) -> Pio014Func {
        Pio014Func::from_bits(val)
    }
}
impl From<Pio014Func> for u8 {
    #[inline(always)]
    fn from(val: Pio014Func) -> u8 {
        Pio014Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014I2cfilter {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE = 0x0,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    STANDARD_MODE = 0x01,
}
impl Pio014I2cfilter {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014I2cfilter {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014I2cfilter {
    #[inline(always)]
    fn from(val: u8) -> Pio014I2cfilter {
        Pio014I2cfilter::from_bits(val)
    }
}
impl From<Pio014I2cfilter> for u8 {
    #[inline(always)]
    fn from(val: Pio014I2cfilter) -> u8 {
        Pio014I2cfilter::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio014Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio014Mode {
        Pio014Mode::from_bits(val)
    }
}
impl From<Pio014Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio014Mode) -> u8 {
        Pio014Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio014Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Od {
    #[inline(always)]
    fn from(val: u8) -> Pio014Od {
        Pio014Od::from_bits(val)
    }
}
impl From<Pio014Od> for u8 {
    #[inline(always)]
    fn from(val: Pio014Od) -> u8 {
        Pio014Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio014Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio014Slew {
        Pio014Slew::from_bits(val)
    }
}
impl From<Pio014Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio014Slew) -> u8 {
        Pio014Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Ssel {
    #[doc = "3V3 Signaling in I2C Mode."]
    SEL3V3 = 0x0,
    #[doc = "1V8 Signaling in I2C Mode."]
    SEL1V8 = 0x01,
}
impl Pio014Ssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Ssel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Ssel {
    #[inline(always)]
    fn from(val: u8) -> Pio014Ssel {
        Pio014Ssel::from_bits(val)
    }
}
impl From<Pio014Ssel> for u8 {
    #[inline(always)]
    fn from(val: Pio014Ssel) -> u8 {
        Pio014Ssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio015Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio015Asw {
        Pio015Asw::from_bits(val)
    }
}
impl From<Pio015Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio015Asw) -> u8 {
        Pio015Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio015Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio015Digimode {
        Pio015Digimode::from_bits(val)
    }
}
impl From<Pio015Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio015Digimode) -> u8 {
        Pio015Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio015Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Func {
    #[inline(always)]
    fn from(val: u8) -> Pio015Func {
        Pio015Func::from_bits(val)
    }
}
impl From<Pio015Func> for u8 {
    #[inline(always)]
    fn from(val: Pio015Func) -> u8 {
        Pio015Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio015Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio015Mode {
        Pio015Mode::from_bits(val)
    }
}
impl From<Pio015Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio015Mode) -> u8 {
        Pio015Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio015Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Od {
    #[inline(always)]
    fn from(val: u8) -> Pio015Od {
        Pio015Od::from_bits(val)
    }
}
impl From<Pio015Od> for u8 {
    #[inline(always)]
    fn from(val: Pio015Od) -> u8 {
        Pio015Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio015Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio015Slew {
        Pio015Slew::from_bits(val)
    }
}
impl From<Pio015Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio015Slew) -> u8 {
        Pio015Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio016Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio016Asw {
        Pio016Asw::from_bits(val)
    }
}
impl From<Pio016Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio016Asw) -> u8 {
        Pio016Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio016Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio016Digimode {
        Pio016Digimode::from_bits(val)
    }
}
impl From<Pio016Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio016Digimode) -> u8 {
        Pio016Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio016Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Func {
    #[inline(always)]
    fn from(val: u8) -> Pio016Func {
        Pio016Func::from_bits(val)
    }
}
impl From<Pio016Func> for u8 {
    #[inline(always)]
    fn from(val: Pio016Func) -> u8 {
        Pio016Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio016Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio016Mode {
        Pio016Mode::from_bits(val)
    }
}
impl From<Pio016Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio016Mode) -> u8 {
        Pio016Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio016Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Od {
    #[inline(always)]
    fn from(val: u8) -> Pio016Od {
        Pio016Od::from_bits(val)
    }
}
impl From<Pio016Od> for u8 {
    #[inline(always)]
    fn from(val: Pio016Od) -> u8 {
        Pio016Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio016Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio016Slew {
        Pio016Slew::from_bits(val)
    }
}
impl From<Pio016Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio016Slew) -> u8 {
        Pio016Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio017Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio017Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio017Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio017Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio017Digimode {
        Pio017Digimode::from_bits(val)
    }
}
impl From<Pio017Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio017Digimode) -> u8 {
        Pio017Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio017Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio017Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio017Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio017Func {
    #[inline(always)]
    fn from(val: u8) -> Pio017Func {
        Pio017Func::from_bits(val)
    }
}
impl From<Pio017Func> for u8 {
    #[inline(always)]
    fn from(val: Pio017Func) -> u8 {
        Pio017Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio017Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio017Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio017Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio017Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio017Mode {
        Pio017Mode::from_bits(val)
    }
}
impl From<Pio017Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio017Mode) -> u8 {
        Pio017Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio017Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio017Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio017Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio017Od {
    #[inline(always)]
    fn from(val: u8) -> Pio017Od {
        Pio017Od::from_bits(val)
    }
}
impl From<Pio017Od> for u8 {
    #[inline(always)]
    fn from(val: Pio017Od) -> u8 {
        Pio017Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio017Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio017Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio017Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio017Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio017Slew {
        Pio017Slew::from_bits(val)
    }
}
impl From<Pio017Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio017Slew) -> u8 {
        Pio017Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio018Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio018Asw {
        Pio018Asw::from_bits(val)
    }
}
impl From<Pio018Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio018Asw) -> u8 {
        Pio018Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio018Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio018Digimode {
        Pio018Digimode::from_bits(val)
    }
}
impl From<Pio018Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio018Digimode) -> u8 {
        Pio018Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio018Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Func {
    #[inline(always)]
    fn from(val: u8) -> Pio018Func {
        Pio018Func::from_bits(val)
    }
}
impl From<Pio018Func> for u8 {
    #[inline(always)]
    fn from(val: Pio018Func) -> u8 {
        Pio018Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio018Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio018Mode {
        Pio018Mode::from_bits(val)
    }
}
impl From<Pio018Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio018Mode) -> u8 {
        Pio018Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio018Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Od {
    #[inline(always)]
    fn from(val: u8) -> Pio018Od {
        Pio018Od::from_bits(val)
    }
}
impl From<Pio018Od> for u8 {
    #[inline(always)]
    fn from(val: Pio018Od) -> u8 {
        Pio018Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio018Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio018Slew {
        Pio018Slew::from_bits(val)
    }
}
impl From<Pio018Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio018Slew) -> u8 {
        Pio018Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio019Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio019Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio019Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio019Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio019Digimode {
        Pio019Digimode::from_bits(val)
    }
}
impl From<Pio019Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio019Digimode) -> u8 {
        Pio019Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio019Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio019Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio019Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio019Func {
    #[inline(always)]
    fn from(val: u8) -> Pio019Func {
        Pio019Func::from_bits(val)
    }
}
impl From<Pio019Func> for u8 {
    #[inline(always)]
    fn from(val: Pio019Func) -> u8 {
        Pio019Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio019Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio019Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio019Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio019Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio019Mode {
        Pio019Mode::from_bits(val)
    }
}
impl From<Pio019Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio019Mode) -> u8 {
        Pio019Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio019Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio019Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio019Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio019Od {
    #[inline(always)]
    fn from(val: u8) -> Pio019Od {
        Pio019Od::from_bits(val)
    }
}
impl From<Pio019Od> for u8 {
    #[inline(always)]
    fn from(val: Pio019Od) -> u8 {
        Pio019Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio019Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio019Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio019Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio019Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio019Slew {
        Pio019Slew::from_bits(val)
    }
}
impl From<Pio019Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio019Slew) -> u8 {
        Pio019Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio01Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio01Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio01Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio01Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio01Digimode {
        Pio01Digimode::from_bits(val)
    }
}
impl From<Pio01Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio01Digimode) -> u8 {
        Pio01Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio01Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio01Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio01Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio01Func {
    #[inline(always)]
    fn from(val: u8) -> Pio01Func {
        Pio01Func::from_bits(val)
    }
}
impl From<Pio01Func> for u8 {
    #[inline(always)]
    fn from(val: Pio01Func) -> u8 {
        Pio01Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio01Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio01Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio01Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio01Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio01Mode {
        Pio01Mode::from_bits(val)
    }
}
impl From<Pio01Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio01Mode) -> u8 {
        Pio01Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio01Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio01Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio01Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio01Od {
    #[inline(always)]
    fn from(val: u8) -> Pio01Od {
        Pio01Od::from_bits(val)
    }
}
impl From<Pio01Od> for u8 {
    #[inline(always)]
    fn from(val: Pio01Od) -> u8 {
        Pio01Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio01Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio01Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio01Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio01Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio01Slew {
        Pio01Slew::from_bits(val)
    }
}
impl From<Pio01Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio01Slew) -> u8 {
        Pio01Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio020Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio020Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio020Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio020Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio020Digimode {
        Pio020Digimode::from_bits(val)
    }
}
impl From<Pio020Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio020Digimode) -> u8 {
        Pio020Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio020Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio020Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio020Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio020Func {
    #[inline(always)]
    fn from(val: u8) -> Pio020Func {
        Pio020Func::from_bits(val)
    }
}
impl From<Pio020Func> for u8 {
    #[inline(always)]
    fn from(val: Pio020Func) -> u8 {
        Pio020Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio020Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio020Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio020Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio020Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio020Mode {
        Pio020Mode::from_bits(val)
    }
}
impl From<Pio020Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio020Mode) -> u8 {
        Pio020Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio020Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio020Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio020Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio020Od {
    #[inline(always)]
    fn from(val: u8) -> Pio020Od {
        Pio020Od::from_bits(val)
    }
}
impl From<Pio020Od> for u8 {
    #[inline(always)]
    fn from(val: Pio020Od) -> u8 {
        Pio020Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio020Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio020Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio020Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio020Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio020Slew {
        Pio020Slew::from_bits(val)
    }
}
impl From<Pio020Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio020Slew) -> u8 {
        Pio020Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio021Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio021Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio021Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio021Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio021Digimode {
        Pio021Digimode::from_bits(val)
    }
}
impl From<Pio021Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio021Digimode) -> u8 {
        Pio021Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio021Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio021Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio021Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio021Func {
    #[inline(always)]
    fn from(val: u8) -> Pio021Func {
        Pio021Func::from_bits(val)
    }
}
impl From<Pio021Func> for u8 {
    #[inline(always)]
    fn from(val: Pio021Func) -> u8 {
        Pio021Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio021Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio021Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio021Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio021Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio021Mode {
        Pio021Mode::from_bits(val)
    }
}
impl From<Pio021Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio021Mode) -> u8 {
        Pio021Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio021Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio021Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio021Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio021Od {
    #[inline(always)]
    fn from(val: u8) -> Pio021Od {
        Pio021Od::from_bits(val)
    }
}
impl From<Pio021Od> for u8 {
    #[inline(always)]
    fn from(val: Pio021Od) -> u8 {
        Pio021Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio021Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio021Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio021Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio021Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio021Slew {
        Pio021Slew::from_bits(val)
    }
}
impl From<Pio021Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio021Slew) -> u8 {
        Pio021Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio022Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio022Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio022Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio022Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio022Digimode {
        Pio022Digimode::from_bits(val)
    }
}
impl From<Pio022Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio022Digimode) -> u8 {
        Pio022Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio022Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio022Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio022Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio022Func {
    #[inline(always)]
    fn from(val: u8) -> Pio022Func {
        Pio022Func::from_bits(val)
    }
}
impl From<Pio022Func> for u8 {
    #[inline(always)]
    fn from(val: Pio022Func) -> u8 {
        Pio022Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio022Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio022Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio022Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio022Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio022Mode {
        Pio022Mode::from_bits(val)
    }
}
impl From<Pio022Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio022Mode) -> u8 {
        Pio022Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio022Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio022Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio022Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio022Od {
    #[inline(always)]
    fn from(val: u8) -> Pio022Od {
        Pio022Od::from_bits(val)
    }
}
impl From<Pio022Od> for u8 {
    #[inline(always)]
    fn from(val: Pio022Od) -> u8 {
        Pio022Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio022Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio022Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio022Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio022Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio022Slew {
        Pio022Slew::from_bits(val)
    }
}
impl From<Pio022Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio022Slew) -> u8 {
        Pio022Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio023Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio023Asw {
        Pio023Asw::from_bits(val)
    }
}
impl From<Pio023Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio023Asw) -> u8 {
        Pio023Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio023Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio023Digimode {
        Pio023Digimode::from_bits(val)
    }
}
impl From<Pio023Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio023Digimode) -> u8 {
        Pio023Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio023Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Func {
    #[inline(always)]
    fn from(val: u8) -> Pio023Func {
        Pio023Func::from_bits(val)
    }
}
impl From<Pio023Func> for u8 {
    #[inline(always)]
    fn from(val: Pio023Func) -> u8 {
        Pio023Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio023Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio023Mode {
        Pio023Mode::from_bits(val)
    }
}
impl From<Pio023Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio023Mode) -> u8 {
        Pio023Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio023Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Od {
    #[inline(always)]
    fn from(val: u8) -> Pio023Od {
        Pio023Od::from_bits(val)
    }
}
impl From<Pio023Od> for u8 {
    #[inline(always)]
    fn from(val: Pio023Od) -> u8 {
        Pio023Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio023Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio023Slew {
        Pio023Slew::from_bits(val)
    }
}
impl From<Pio023Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio023Slew) -> u8 {
        Pio023Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio024Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio024Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio024Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio024Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio024Digimode {
        Pio024Digimode::from_bits(val)
    }
}
impl From<Pio024Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio024Digimode) -> u8 {
        Pio024Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio024Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio024Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio024Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio024Func {
    #[inline(always)]
    fn from(val: u8) -> Pio024Func {
        Pio024Func::from_bits(val)
    }
}
impl From<Pio024Func> for u8 {
    #[inline(always)]
    fn from(val: Pio024Func) -> u8 {
        Pio024Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio024Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio024Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio024Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio024Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio024Mode {
        Pio024Mode::from_bits(val)
    }
}
impl From<Pio024Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio024Mode) -> u8 {
        Pio024Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio024Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio024Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio024Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio024Od {
    #[inline(always)]
    fn from(val: u8) -> Pio024Od {
        Pio024Od::from_bits(val)
    }
}
impl From<Pio024Od> for u8 {
    #[inline(always)]
    fn from(val: Pio024Od) -> u8 {
        Pio024Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio024Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio024Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio024Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio024Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio024Slew {
        Pio024Slew::from_bits(val)
    }
}
impl From<Pio024Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio024Slew) -> u8 {
        Pio024Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio025Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio025Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio025Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio025Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio025Digimode {
        Pio025Digimode::from_bits(val)
    }
}
impl From<Pio025Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio025Digimode) -> u8 {
        Pio025Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio025Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio025Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio025Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio025Func {
    #[inline(always)]
    fn from(val: u8) -> Pio025Func {
        Pio025Func::from_bits(val)
    }
}
impl From<Pio025Func> for u8 {
    #[inline(always)]
    fn from(val: Pio025Func) -> u8 {
        Pio025Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio025Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio025Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio025Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio025Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio025Mode {
        Pio025Mode::from_bits(val)
    }
}
impl From<Pio025Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio025Mode) -> u8 {
        Pio025Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio025Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio025Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio025Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio025Od {
    #[inline(always)]
    fn from(val: u8) -> Pio025Od {
        Pio025Od::from_bits(val)
    }
}
impl From<Pio025Od> for u8 {
    #[inline(always)]
    fn from(val: Pio025Od) -> u8 {
        Pio025Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio025Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio025Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio025Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio025Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio025Slew {
        Pio025Slew::from_bits(val)
    }
}
impl From<Pio025Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio025Slew) -> u8 {
        Pio025Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio026Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio026Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio026Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio026Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio026Digimode {
        Pio026Digimode::from_bits(val)
    }
}
impl From<Pio026Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio026Digimode) -> u8 {
        Pio026Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio026Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio026Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio026Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio026Func {
    #[inline(always)]
    fn from(val: u8) -> Pio026Func {
        Pio026Func::from_bits(val)
    }
}
impl From<Pio026Func> for u8 {
    #[inline(always)]
    fn from(val: Pio026Func) -> u8 {
        Pio026Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio026Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio026Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio026Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio026Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio026Mode {
        Pio026Mode::from_bits(val)
    }
}
impl From<Pio026Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio026Mode) -> u8 {
        Pio026Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio026Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio026Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio026Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio026Od {
    #[inline(always)]
    fn from(val: u8) -> Pio026Od {
        Pio026Od::from_bits(val)
    }
}
impl From<Pio026Od> for u8 {
    #[inline(always)]
    fn from(val: Pio026Od) -> u8 {
        Pio026Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio026Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio026Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio026Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio026Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio026Slew {
        Pio026Slew::from_bits(val)
    }
}
impl From<Pio026Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio026Slew) -> u8 {
        Pio026Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio027Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio027Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio027Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio027Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio027Digimode {
        Pio027Digimode::from_bits(val)
    }
}
impl From<Pio027Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio027Digimode) -> u8 {
        Pio027Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio027Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio027Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio027Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio027Func {
    #[inline(always)]
    fn from(val: u8) -> Pio027Func {
        Pio027Func::from_bits(val)
    }
}
impl From<Pio027Func> for u8 {
    #[inline(always)]
    fn from(val: Pio027Func) -> u8 {
        Pio027Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio027Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio027Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio027Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio027Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio027Mode {
        Pio027Mode::from_bits(val)
    }
}
impl From<Pio027Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio027Mode) -> u8 {
        Pio027Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio027Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio027Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio027Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio027Od {
    #[inline(always)]
    fn from(val: u8) -> Pio027Od {
        Pio027Od::from_bits(val)
    }
}
impl From<Pio027Od> for u8 {
    #[inline(always)]
    fn from(val: Pio027Od) -> u8 {
        Pio027Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio027Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio027Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio027Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio027Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio027Slew {
        Pio027Slew::from_bits(val)
    }
}
impl From<Pio027Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio027Slew) -> u8 {
        Pio027Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio028Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio028Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio028Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio028Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio028Digimode {
        Pio028Digimode::from_bits(val)
    }
}
impl From<Pio028Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio028Digimode) -> u8 {
        Pio028Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio028Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio028Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio028Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio028Func {
    #[inline(always)]
    fn from(val: u8) -> Pio028Func {
        Pio028Func::from_bits(val)
    }
}
impl From<Pio028Func> for u8 {
    #[inline(always)]
    fn from(val: Pio028Func) -> u8 {
        Pio028Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio028Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio028Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio028Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio028Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio028Mode {
        Pio028Mode::from_bits(val)
    }
}
impl From<Pio028Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio028Mode) -> u8 {
        Pio028Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio028Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio028Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio028Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio028Od {
    #[inline(always)]
    fn from(val: u8) -> Pio028Od {
        Pio028Od::from_bits(val)
    }
}
impl From<Pio028Od> for u8 {
    #[inline(always)]
    fn from(val: Pio028Od) -> u8 {
        Pio028Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio028Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio028Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio028Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio028Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio028Slew {
        Pio028Slew::from_bits(val)
    }
}
impl From<Pio028Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio028Slew) -> u8 {
        Pio028Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio029Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio029Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio029Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio029Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio029Digimode {
        Pio029Digimode::from_bits(val)
    }
}
impl From<Pio029Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio029Digimode) -> u8 {
        Pio029Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio029Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio029Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio029Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio029Func {
    #[inline(always)]
    fn from(val: u8) -> Pio029Func {
        Pio029Func::from_bits(val)
    }
}
impl From<Pio029Func> for u8 {
    #[inline(always)]
    fn from(val: Pio029Func) -> u8 {
        Pio029Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio029Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio029Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio029Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio029Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio029Mode {
        Pio029Mode::from_bits(val)
    }
}
impl From<Pio029Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio029Mode) -> u8 {
        Pio029Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio029Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio029Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio029Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio029Od {
    #[inline(always)]
    fn from(val: u8) -> Pio029Od {
        Pio029Od::from_bits(val)
    }
}
impl From<Pio029Od> for u8 {
    #[inline(always)]
    fn from(val: Pio029Od) -> u8 {
        Pio029Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio029Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio029Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio029Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio029Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio029Slew {
        Pio029Slew::from_bits(val)
    }
}
impl From<Pio029Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio029Slew) -> u8 {
        Pio029Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio02Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio02Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio02Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio02Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio02Digimode {
        Pio02Digimode::from_bits(val)
    }
}
impl From<Pio02Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio02Digimode) -> u8 {
        Pio02Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio02Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio02Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio02Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio02Func {
    #[inline(always)]
    fn from(val: u8) -> Pio02Func {
        Pio02Func::from_bits(val)
    }
}
impl From<Pio02Func> for u8 {
    #[inline(always)]
    fn from(val: Pio02Func) -> u8 {
        Pio02Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio02Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio02Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio02Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio02Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio02Mode {
        Pio02Mode::from_bits(val)
    }
}
impl From<Pio02Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio02Mode) -> u8 {
        Pio02Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio02Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio02Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio02Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio02Od {
    #[inline(always)]
    fn from(val: u8) -> Pio02Od {
        Pio02Od::from_bits(val)
    }
}
impl From<Pio02Od> for u8 {
    #[inline(always)]
    fn from(val: Pio02Od) -> u8 {
        Pio02Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio02Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio02Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio02Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio02Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio02Slew {
        Pio02Slew::from_bits(val)
    }
}
impl From<Pio02Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio02Slew) -> u8 {
        Pio02Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio030Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio030Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio030Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio030Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio030Digimode {
        Pio030Digimode::from_bits(val)
    }
}
impl From<Pio030Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio030Digimode) -> u8 {
        Pio030Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio030Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio030Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio030Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio030Func {
    #[inline(always)]
    fn from(val: u8) -> Pio030Func {
        Pio030Func::from_bits(val)
    }
}
impl From<Pio030Func> for u8 {
    #[inline(always)]
    fn from(val: Pio030Func) -> u8 {
        Pio030Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio030Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio030Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio030Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio030Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio030Mode {
        Pio030Mode::from_bits(val)
    }
}
impl From<Pio030Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio030Mode) -> u8 {
        Pio030Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio030Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio030Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio030Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio030Od {
    #[inline(always)]
    fn from(val: u8) -> Pio030Od {
        Pio030Od::from_bits(val)
    }
}
impl From<Pio030Od> for u8 {
    #[inline(always)]
    fn from(val: Pio030Od) -> u8 {
        Pio030Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio030Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio030Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio030Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio030Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio030Slew {
        Pio030Slew::from_bits(val)
    }
}
impl From<Pio030Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio030Slew) -> u8 {
        Pio030Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio031Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio031Asw {
        Pio031Asw::from_bits(val)
    }
}
impl From<Pio031Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio031Asw) -> u8 {
        Pio031Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio031Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio031Digimode {
        Pio031Digimode::from_bits(val)
    }
}
impl From<Pio031Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio031Digimode) -> u8 {
        Pio031Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio031Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Func {
    #[inline(always)]
    fn from(val: u8) -> Pio031Func {
        Pio031Func::from_bits(val)
    }
}
impl From<Pio031Func> for u8 {
    #[inline(always)]
    fn from(val: Pio031Func) -> u8 {
        Pio031Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio031Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio031Mode {
        Pio031Mode::from_bits(val)
    }
}
impl From<Pio031Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio031Mode) -> u8 {
        Pio031Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio031Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Od {
    #[inline(always)]
    fn from(val: u8) -> Pio031Od {
        Pio031Od::from_bits(val)
    }
}
impl From<Pio031Od> for u8 {
    #[inline(always)]
    fn from(val: Pio031Od) -> u8 {
        Pio031Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio031Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio031Slew {
        Pio031Slew::from_bits(val)
    }
}
impl From<Pio031Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio031Slew) -> u8 {
        Pio031Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio03Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio03Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio03Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio03Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio03Digimode {
        Pio03Digimode::from_bits(val)
    }
}
impl From<Pio03Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio03Digimode) -> u8 {
        Pio03Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio03Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio03Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio03Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio03Func {
    #[inline(always)]
    fn from(val: u8) -> Pio03Func {
        Pio03Func::from_bits(val)
    }
}
impl From<Pio03Func> for u8 {
    #[inline(always)]
    fn from(val: Pio03Func) -> u8 {
        Pio03Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio03Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio03Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio03Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio03Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio03Mode {
        Pio03Mode::from_bits(val)
    }
}
impl From<Pio03Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio03Mode) -> u8 {
        Pio03Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio03Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio03Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio03Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio03Od {
    #[inline(always)]
    fn from(val: u8) -> Pio03Od {
        Pio03Od::from_bits(val)
    }
}
impl From<Pio03Od> for u8 {
    #[inline(always)]
    fn from(val: Pio03Od) -> u8 {
        Pio03Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio03Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio03Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio03Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio03Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio03Slew {
        Pio03Slew::from_bits(val)
    }
}
impl From<Pio03Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio03Slew) -> u8 {
        Pio03Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio04Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio04Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio04Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio04Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio04Digimode {
        Pio04Digimode::from_bits(val)
    }
}
impl From<Pio04Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio04Digimode) -> u8 {
        Pio04Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio04Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio04Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio04Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio04Func {
    #[inline(always)]
    fn from(val: u8) -> Pio04Func {
        Pio04Func::from_bits(val)
    }
}
impl From<Pio04Func> for u8 {
    #[inline(always)]
    fn from(val: Pio04Func) -> u8 {
        Pio04Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio04Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio04Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio04Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio04Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio04Mode {
        Pio04Mode::from_bits(val)
    }
}
impl From<Pio04Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio04Mode) -> u8 {
        Pio04Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio04Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio04Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio04Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio04Od {
    #[inline(always)]
    fn from(val: u8) -> Pio04Od {
        Pio04Od::from_bits(val)
    }
}
impl From<Pio04Od> for u8 {
    #[inline(always)]
    fn from(val: Pio04Od) -> u8 {
        Pio04Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio04Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio04Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio04Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio04Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio04Slew {
        Pio04Slew::from_bits(val)
    }
}
impl From<Pio04Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio04Slew) -> u8 {
        Pio04Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio05Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio05Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio05Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio05Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio05Digimode {
        Pio05Digimode::from_bits(val)
    }
}
impl From<Pio05Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio05Digimode) -> u8 {
        Pio05Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio05Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio05Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio05Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio05Func {
    #[inline(always)]
    fn from(val: u8) -> Pio05Func {
        Pio05Func::from_bits(val)
    }
}
impl From<Pio05Func> for u8 {
    #[inline(always)]
    fn from(val: Pio05Func) -> u8 {
        Pio05Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio05Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio05Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio05Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio05Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio05Mode {
        Pio05Mode::from_bits(val)
    }
}
impl From<Pio05Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio05Mode) -> u8 {
        Pio05Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio05Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio05Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio05Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio05Od {
    #[inline(always)]
    fn from(val: u8) -> Pio05Od {
        Pio05Od::from_bits(val)
    }
}
impl From<Pio05Od> for u8 {
    #[inline(always)]
    fn from(val: Pio05Od) -> u8 {
        Pio05Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio05Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio05Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio05Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio05Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio05Slew {
        Pio05Slew::from_bits(val)
    }
}
impl From<Pio05Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio05Slew) -> u8 {
        Pio05Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio06Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio06Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio06Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio06Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio06Digimode {
        Pio06Digimode::from_bits(val)
    }
}
impl From<Pio06Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio06Digimode) -> u8 {
        Pio06Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio06Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio06Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio06Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio06Func {
    #[inline(always)]
    fn from(val: u8) -> Pio06Func {
        Pio06Func::from_bits(val)
    }
}
impl From<Pio06Func> for u8 {
    #[inline(always)]
    fn from(val: Pio06Func) -> u8 {
        Pio06Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio06Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio06Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio06Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio06Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio06Mode {
        Pio06Mode::from_bits(val)
    }
}
impl From<Pio06Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio06Mode) -> u8 {
        Pio06Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio06Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio06Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio06Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio06Od {
    #[inline(always)]
    fn from(val: u8) -> Pio06Od {
        Pio06Od::from_bits(val)
    }
}
impl From<Pio06Od> for u8 {
    #[inline(always)]
    fn from(val: Pio06Od) -> u8 {
        Pio06Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio06Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio06Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio06Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio06Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio06Slew {
        Pio06Slew::from_bits(val)
    }
}
impl From<Pio06Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio06Slew) -> u8 {
        Pio06Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio07Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio07Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio07Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio07Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio07Digimode {
        Pio07Digimode::from_bits(val)
    }
}
impl From<Pio07Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio07Digimode) -> u8 {
        Pio07Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio07Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio07Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio07Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio07Func {
    #[inline(always)]
    fn from(val: u8) -> Pio07Func {
        Pio07Func::from_bits(val)
    }
}
impl From<Pio07Func> for u8 {
    #[inline(always)]
    fn from(val: Pio07Func) -> u8 {
        Pio07Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio07Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio07Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio07Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio07Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio07Mode {
        Pio07Mode::from_bits(val)
    }
}
impl From<Pio07Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio07Mode) -> u8 {
        Pio07Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio07Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio07Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio07Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio07Od {
    #[inline(always)]
    fn from(val: u8) -> Pio07Od {
        Pio07Od::from_bits(val)
    }
}
impl From<Pio07Od> for u8 {
    #[inline(always)]
    fn from(val: Pio07Od) -> u8 {
        Pio07Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio07Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio07Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio07Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio07Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio07Slew {
        Pio07Slew::from_bits(val)
    }
}
impl From<Pio07Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio07Slew) -> u8 {
        Pio07Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio08Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio08Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio08Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio08Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio08Digimode {
        Pio08Digimode::from_bits(val)
    }
}
impl From<Pio08Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio08Digimode) -> u8 {
        Pio08Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio08Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio08Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio08Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio08Func {
    #[inline(always)]
    fn from(val: u8) -> Pio08Func {
        Pio08Func::from_bits(val)
    }
}
impl From<Pio08Func> for u8 {
    #[inline(always)]
    fn from(val: Pio08Func) -> u8 {
        Pio08Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio08Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio08Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio08Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio08Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio08Mode {
        Pio08Mode::from_bits(val)
    }
}
impl From<Pio08Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio08Mode) -> u8 {
        Pio08Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio08Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio08Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio08Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio08Od {
    #[inline(always)]
    fn from(val: u8) -> Pio08Od {
        Pio08Od::from_bits(val)
    }
}
impl From<Pio08Od> for u8 {
    #[inline(always)]
    fn from(val: Pio08Od) -> u8 {
        Pio08Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio08Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio08Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio08Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio08Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio08Slew {
        Pio08Slew::from_bits(val)
    }
}
impl From<Pio08Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio08Slew) -> u8 {
        Pio08Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio09Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio09Asw {
        Pio09Asw::from_bits(val)
    }
}
impl From<Pio09Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio09Asw) -> u8 {
        Pio09Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio09Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio09Digimode {
        Pio09Digimode::from_bits(val)
    }
}
impl From<Pio09Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio09Digimode) -> u8 {
        Pio09Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio09Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Func {
    #[inline(always)]
    fn from(val: u8) -> Pio09Func {
        Pio09Func::from_bits(val)
    }
}
impl From<Pio09Func> for u8 {
    #[inline(always)]
    fn from(val: Pio09Func) -> u8 {
        Pio09Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio09Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio09Mode {
        Pio09Mode::from_bits(val)
    }
}
impl From<Pio09Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio09Mode) -> u8 {
        Pio09Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio09Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Od {
    #[inline(always)]
    fn from(val: u8) -> Pio09Od {
        Pio09Od::from_bits(val)
    }
}
impl From<Pio09Od> for u8 {
    #[inline(always)]
    fn from(val: Pio09Od) -> u8 {
        Pio09Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio09Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio09Slew {
        Pio09Slew::from_bits(val)
    }
}
impl From<Pio09Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio09Slew) -> u8 {
        Pio09Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio10Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio10Asw {
        Pio10Asw::from_bits(val)
    }
}
impl From<Pio10Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio10Asw) -> u8 {
        Pio10Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio10Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio10Digimode {
        Pio10Digimode::from_bits(val)
    }
}
impl From<Pio10Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio10Digimode) -> u8 {
        Pio10Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio10Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Func {
    #[inline(always)]
    fn from(val: u8) -> Pio10Func {
        Pio10Func::from_bits(val)
    }
}
impl From<Pio10Func> for u8 {
    #[inline(always)]
    fn from(val: Pio10Func) -> u8 {
        Pio10Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio10Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio10Mode {
        Pio10Mode::from_bits(val)
    }
}
impl From<Pio10Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio10Mode) -> u8 {
        Pio10Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio10Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Od {
    #[inline(always)]
    fn from(val: u8) -> Pio10Od {
        Pio10Od::from_bits(val)
    }
}
impl From<Pio10Od> for u8 {
    #[inline(always)]
    fn from(val: Pio10Od) -> u8 {
        Pio10Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio10Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio10Slew {
        Pio10Slew::from_bits(val)
    }
}
impl From<Pio10Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio10Slew) -> u8 {
        Pio10Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio110Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio110Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio110Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio110Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio110Digimode {
        Pio110Digimode::from_bits(val)
    }
}
impl From<Pio110Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio110Digimode) -> u8 {
        Pio110Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio110Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio110Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio110Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio110Func {
    #[inline(always)]
    fn from(val: u8) -> Pio110Func {
        Pio110Func::from_bits(val)
    }
}
impl From<Pio110Func> for u8 {
    #[inline(always)]
    fn from(val: Pio110Func) -> u8 {
        Pio110Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio110Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio110Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio110Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio110Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio110Mode {
        Pio110Mode::from_bits(val)
    }
}
impl From<Pio110Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio110Mode) -> u8 {
        Pio110Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio110Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio110Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio110Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio110Od {
    #[inline(always)]
    fn from(val: u8) -> Pio110Od {
        Pio110Od::from_bits(val)
    }
}
impl From<Pio110Od> for u8 {
    #[inline(always)]
    fn from(val: Pio110Od) -> u8 {
        Pio110Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio110Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio110Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio110Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio110Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio110Slew {
        Pio110Slew::from_bits(val)
    }
}
impl From<Pio110Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio110Slew) -> u8 {
        Pio110Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio111Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio111Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio111Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio111Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio111Digimode {
        Pio111Digimode::from_bits(val)
    }
}
impl From<Pio111Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio111Digimode) -> u8 {
        Pio111Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio111Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio111Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio111Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio111Func {
    #[inline(always)]
    fn from(val: u8) -> Pio111Func {
        Pio111Func::from_bits(val)
    }
}
impl From<Pio111Func> for u8 {
    #[inline(always)]
    fn from(val: Pio111Func) -> u8 {
        Pio111Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio111Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio111Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio111Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio111Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio111Mode {
        Pio111Mode::from_bits(val)
    }
}
impl From<Pio111Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio111Mode) -> u8 {
        Pio111Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio111Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio111Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio111Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio111Od {
    #[inline(always)]
    fn from(val: u8) -> Pio111Od {
        Pio111Od::from_bits(val)
    }
}
impl From<Pio111Od> for u8 {
    #[inline(always)]
    fn from(val: Pio111Od) -> u8 {
        Pio111Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio111Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio111Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio111Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio111Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio111Slew {
        Pio111Slew::from_bits(val)
    }
}
impl From<Pio111Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio111Slew) -> u8 {
        Pio111Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio112Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio112Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio112Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio112Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio112Digimode {
        Pio112Digimode::from_bits(val)
    }
}
impl From<Pio112Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio112Digimode) -> u8 {
        Pio112Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio112Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio112Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio112Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio112Func {
    #[inline(always)]
    fn from(val: u8) -> Pio112Func {
        Pio112Func::from_bits(val)
    }
}
impl From<Pio112Func> for u8 {
    #[inline(always)]
    fn from(val: Pio112Func) -> u8 {
        Pio112Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio112Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio112Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio112Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio112Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio112Mode {
        Pio112Mode::from_bits(val)
    }
}
impl From<Pio112Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio112Mode) -> u8 {
        Pio112Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio112Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio112Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio112Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio112Od {
    #[inline(always)]
    fn from(val: u8) -> Pio112Od {
        Pio112Od::from_bits(val)
    }
}
impl From<Pio112Od> for u8 {
    #[inline(always)]
    fn from(val: Pio112Od) -> u8 {
        Pio112Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio112Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio112Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio112Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio112Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio112Slew {
        Pio112Slew::from_bits(val)
    }
}
impl From<Pio112Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio112Slew) -> u8 {
        Pio112Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio113Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio113Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio113Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio113Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio113Digimode {
        Pio113Digimode::from_bits(val)
    }
}
impl From<Pio113Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio113Digimode) -> u8 {
        Pio113Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio113Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio113Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio113Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio113Func {
    #[inline(always)]
    fn from(val: u8) -> Pio113Func {
        Pio113Func::from_bits(val)
    }
}
impl From<Pio113Func> for u8 {
    #[inline(always)]
    fn from(val: Pio113Func) -> u8 {
        Pio113Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio113Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio113Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio113Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio113Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio113Mode {
        Pio113Mode::from_bits(val)
    }
}
impl From<Pio113Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio113Mode) -> u8 {
        Pio113Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio113Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio113Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio113Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio113Od {
    #[inline(always)]
    fn from(val: u8) -> Pio113Od {
        Pio113Od::from_bits(val)
    }
}
impl From<Pio113Od> for u8 {
    #[inline(always)]
    fn from(val: Pio113Od) -> u8 {
        Pio113Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio113Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio113Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio113Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio113Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio113Slew {
        Pio113Slew::from_bits(val)
    }
}
impl From<Pio113Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio113Slew) -> u8 {
        Pio113Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio114Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio114Asw {
        Pio114Asw::from_bits(val)
    }
}
impl From<Pio114Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio114Asw) -> u8 {
        Pio114Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio114Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio114Digimode {
        Pio114Digimode::from_bits(val)
    }
}
impl From<Pio114Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio114Digimode) -> u8 {
        Pio114Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio114Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Func {
    #[inline(always)]
    fn from(val: u8) -> Pio114Func {
        Pio114Func::from_bits(val)
    }
}
impl From<Pio114Func> for u8 {
    #[inline(always)]
    fn from(val: Pio114Func) -> u8 {
        Pio114Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio114Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio114Mode {
        Pio114Mode::from_bits(val)
    }
}
impl From<Pio114Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio114Mode) -> u8 {
        Pio114Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio114Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Od {
    #[inline(always)]
    fn from(val: u8) -> Pio114Od {
        Pio114Od::from_bits(val)
    }
}
impl From<Pio114Od> for u8 {
    #[inline(always)]
    fn from(val: Pio114Od) -> u8 {
        Pio114Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio114Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio114Slew {
        Pio114Slew::from_bits(val)
    }
}
impl From<Pio114Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio114Slew) -> u8 {
        Pio114Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio115Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio115Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio115Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio115Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio115Digimode {
        Pio115Digimode::from_bits(val)
    }
}
impl From<Pio115Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio115Digimode) -> u8 {
        Pio115Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio115Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio115Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio115Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio115Func {
    #[inline(always)]
    fn from(val: u8) -> Pio115Func {
        Pio115Func::from_bits(val)
    }
}
impl From<Pio115Func> for u8 {
    #[inline(always)]
    fn from(val: Pio115Func) -> u8 {
        Pio115Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio115Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio115Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio115Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio115Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio115Mode {
        Pio115Mode::from_bits(val)
    }
}
impl From<Pio115Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio115Mode) -> u8 {
        Pio115Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio115Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio115Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio115Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio115Od {
    #[inline(always)]
    fn from(val: u8) -> Pio115Od {
        Pio115Od::from_bits(val)
    }
}
impl From<Pio115Od> for u8 {
    #[inline(always)]
    fn from(val: Pio115Od) -> u8 {
        Pio115Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio115Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio115Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio115Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio115Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio115Slew {
        Pio115Slew::from_bits(val)
    }
}
impl From<Pio115Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio115Slew) -> u8 {
        Pio115Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio116Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio116Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio116Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio116Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio116Digimode {
        Pio116Digimode::from_bits(val)
    }
}
impl From<Pio116Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio116Digimode) -> u8 {
        Pio116Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio116Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio116Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio116Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio116Func {
    #[inline(always)]
    fn from(val: u8) -> Pio116Func {
        Pio116Func::from_bits(val)
    }
}
impl From<Pio116Func> for u8 {
    #[inline(always)]
    fn from(val: Pio116Func) -> u8 {
        Pio116Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio116Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio116Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio116Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio116Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio116Mode {
        Pio116Mode::from_bits(val)
    }
}
impl From<Pio116Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio116Mode) -> u8 {
        Pio116Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio116Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio116Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio116Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio116Od {
    #[inline(always)]
    fn from(val: u8) -> Pio116Od {
        Pio116Od::from_bits(val)
    }
}
impl From<Pio116Od> for u8 {
    #[inline(always)]
    fn from(val: Pio116Od) -> u8 {
        Pio116Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio116Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio116Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio116Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio116Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio116Slew {
        Pio116Slew::from_bits(val)
    }
}
impl From<Pio116Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio116Slew) -> u8 {
        Pio116Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio117Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio117Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio117Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio117Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio117Digimode {
        Pio117Digimode::from_bits(val)
    }
}
impl From<Pio117Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio117Digimode) -> u8 {
        Pio117Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio117Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio117Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio117Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio117Func {
    #[inline(always)]
    fn from(val: u8) -> Pio117Func {
        Pio117Func::from_bits(val)
    }
}
impl From<Pio117Func> for u8 {
    #[inline(always)]
    fn from(val: Pio117Func) -> u8 {
        Pio117Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio117Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio117Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio117Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio117Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio117Mode {
        Pio117Mode::from_bits(val)
    }
}
impl From<Pio117Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio117Mode) -> u8 {
        Pio117Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio117Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio117Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio117Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio117Od {
    #[inline(always)]
    fn from(val: u8) -> Pio117Od {
        Pio117Od::from_bits(val)
    }
}
impl From<Pio117Od> for u8 {
    #[inline(always)]
    fn from(val: Pio117Od) -> u8 {
        Pio117Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio117Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio117Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio117Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio117Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio117Slew {
        Pio117Slew::from_bits(val)
    }
}
impl From<Pio117Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio117Slew) -> u8 {
        Pio117Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio118Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio118Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio118Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio118Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio118Digimode {
        Pio118Digimode::from_bits(val)
    }
}
impl From<Pio118Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio118Digimode) -> u8 {
        Pio118Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio118Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio118Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio118Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio118Func {
    #[inline(always)]
    fn from(val: u8) -> Pio118Func {
        Pio118Func::from_bits(val)
    }
}
impl From<Pio118Func> for u8 {
    #[inline(always)]
    fn from(val: Pio118Func) -> u8 {
        Pio118Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio118Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio118Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio118Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio118Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio118Mode {
        Pio118Mode::from_bits(val)
    }
}
impl From<Pio118Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio118Mode) -> u8 {
        Pio118Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio118Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio118Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio118Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio118Od {
    #[inline(always)]
    fn from(val: u8) -> Pio118Od {
        Pio118Od::from_bits(val)
    }
}
impl From<Pio118Od> for u8 {
    #[inline(always)]
    fn from(val: Pio118Od) -> u8 {
        Pio118Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio118Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio118Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio118Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio118Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio118Slew {
        Pio118Slew::from_bits(val)
    }
}
impl From<Pio118Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio118Slew) -> u8 {
        Pio118Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio119Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio119Asw {
        Pio119Asw::from_bits(val)
    }
}
impl From<Pio119Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio119Asw) -> u8 {
        Pio119Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio119Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio119Digimode {
        Pio119Digimode::from_bits(val)
    }
}
impl From<Pio119Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio119Digimode) -> u8 {
        Pio119Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio119Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Func {
    #[inline(always)]
    fn from(val: u8) -> Pio119Func {
        Pio119Func::from_bits(val)
    }
}
impl From<Pio119Func> for u8 {
    #[inline(always)]
    fn from(val: Pio119Func) -> u8 {
        Pio119Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio119Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio119Mode {
        Pio119Mode::from_bits(val)
    }
}
impl From<Pio119Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio119Mode) -> u8 {
        Pio119Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio119Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Od {
    #[inline(always)]
    fn from(val: u8) -> Pio119Od {
        Pio119Od::from_bits(val)
    }
}
impl From<Pio119Od> for u8 {
    #[inline(always)]
    fn from(val: Pio119Od) -> u8 {
        Pio119Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio119Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio119Slew {
        Pio119Slew::from_bits(val)
    }
}
impl From<Pio119Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio119Slew) -> u8 {
        Pio119Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio11Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio11Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio11Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio11Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio11Digimode {
        Pio11Digimode::from_bits(val)
    }
}
impl From<Pio11Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio11Digimode) -> u8 {
        Pio11Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio11Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio11Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio11Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio11Func {
    #[inline(always)]
    fn from(val: u8) -> Pio11Func {
        Pio11Func::from_bits(val)
    }
}
impl From<Pio11Func> for u8 {
    #[inline(always)]
    fn from(val: Pio11Func) -> u8 {
        Pio11Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio11Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio11Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio11Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio11Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio11Mode {
        Pio11Mode::from_bits(val)
    }
}
impl From<Pio11Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio11Mode) -> u8 {
        Pio11Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio11Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio11Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio11Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio11Od {
    #[inline(always)]
    fn from(val: u8) -> Pio11Od {
        Pio11Od::from_bits(val)
    }
}
impl From<Pio11Od> for u8 {
    #[inline(always)]
    fn from(val: Pio11Od) -> u8 {
        Pio11Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio11Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio11Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio11Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio11Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio11Slew {
        Pio11Slew::from_bits(val)
    }
}
impl From<Pio11Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio11Slew) -> u8 {
        Pio11Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio120Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio120Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio120Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio120Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio120Digimode {
        Pio120Digimode::from_bits(val)
    }
}
impl From<Pio120Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio120Digimode) -> u8 {
        Pio120Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio120Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio120Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio120Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio120Func {
    #[inline(always)]
    fn from(val: u8) -> Pio120Func {
        Pio120Func::from_bits(val)
    }
}
impl From<Pio120Func> for u8 {
    #[inline(always)]
    fn from(val: Pio120Func) -> u8 {
        Pio120Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio120Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio120Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio120Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio120Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio120Mode {
        Pio120Mode::from_bits(val)
    }
}
impl From<Pio120Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio120Mode) -> u8 {
        Pio120Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio120Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio120Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio120Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio120Od {
    #[inline(always)]
    fn from(val: u8) -> Pio120Od {
        Pio120Od::from_bits(val)
    }
}
impl From<Pio120Od> for u8 {
    #[inline(always)]
    fn from(val: Pio120Od) -> u8 {
        Pio120Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio120Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio120Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio120Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio120Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio120Slew {
        Pio120Slew::from_bits(val)
    }
}
impl From<Pio120Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio120Slew) -> u8 {
        Pio120Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio121Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio121Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio121Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio121Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio121Digimode {
        Pio121Digimode::from_bits(val)
    }
}
impl From<Pio121Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio121Digimode) -> u8 {
        Pio121Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio121Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio121Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio121Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio121Func {
    #[inline(always)]
    fn from(val: u8) -> Pio121Func {
        Pio121Func::from_bits(val)
    }
}
impl From<Pio121Func> for u8 {
    #[inline(always)]
    fn from(val: Pio121Func) -> u8 {
        Pio121Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio121Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio121Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio121Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio121Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio121Mode {
        Pio121Mode::from_bits(val)
    }
}
impl From<Pio121Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio121Mode) -> u8 {
        Pio121Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio121Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio121Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio121Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio121Od {
    #[inline(always)]
    fn from(val: u8) -> Pio121Od {
        Pio121Od::from_bits(val)
    }
}
impl From<Pio121Od> for u8 {
    #[inline(always)]
    fn from(val: Pio121Od) -> u8 {
        Pio121Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio121Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio121Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio121Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio121Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio121Slew {
        Pio121Slew::from_bits(val)
    }
}
impl From<Pio121Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio121Slew) -> u8 {
        Pio121Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio122Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio122Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio122Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio122Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio122Digimode {
        Pio122Digimode::from_bits(val)
    }
}
impl From<Pio122Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio122Digimode) -> u8 {
        Pio122Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio122Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio122Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio122Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio122Func {
    #[inline(always)]
    fn from(val: u8) -> Pio122Func {
        Pio122Func::from_bits(val)
    }
}
impl From<Pio122Func> for u8 {
    #[inline(always)]
    fn from(val: Pio122Func) -> u8 {
        Pio122Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio122Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio122Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio122Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio122Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio122Mode {
        Pio122Mode::from_bits(val)
    }
}
impl From<Pio122Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio122Mode) -> u8 {
        Pio122Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio122Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio122Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio122Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio122Od {
    #[inline(always)]
    fn from(val: u8) -> Pio122Od {
        Pio122Od::from_bits(val)
    }
}
impl From<Pio122Od> for u8 {
    #[inline(always)]
    fn from(val: Pio122Od) -> u8 {
        Pio122Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio122Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio122Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio122Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio122Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio122Slew {
        Pio122Slew::from_bits(val)
    }
}
impl From<Pio122Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio122Slew) -> u8 {
        Pio122Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio123Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio123Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio123Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio123Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio123Digimode {
        Pio123Digimode::from_bits(val)
    }
}
impl From<Pio123Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio123Digimode) -> u8 {
        Pio123Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio123Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio123Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio123Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio123Func {
    #[inline(always)]
    fn from(val: u8) -> Pio123Func {
        Pio123Func::from_bits(val)
    }
}
impl From<Pio123Func> for u8 {
    #[inline(always)]
    fn from(val: Pio123Func) -> u8 {
        Pio123Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio123Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio123Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio123Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio123Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio123Mode {
        Pio123Mode::from_bits(val)
    }
}
impl From<Pio123Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio123Mode) -> u8 {
        Pio123Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio123Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio123Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio123Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio123Od {
    #[inline(always)]
    fn from(val: u8) -> Pio123Od {
        Pio123Od::from_bits(val)
    }
}
impl From<Pio123Od> for u8 {
    #[inline(always)]
    fn from(val: Pio123Od) -> u8 {
        Pio123Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio123Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio123Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio123Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio123Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio123Slew {
        Pio123Slew::from_bits(val)
    }
}
impl From<Pio123Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio123Slew) -> u8 {
        Pio123Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio124Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio124Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio124Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio124Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio124Digimode {
        Pio124Digimode::from_bits(val)
    }
}
impl From<Pio124Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio124Digimode) -> u8 {
        Pio124Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio124Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio124Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio124Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio124Func {
    #[inline(always)]
    fn from(val: u8) -> Pio124Func {
        Pio124Func::from_bits(val)
    }
}
impl From<Pio124Func> for u8 {
    #[inline(always)]
    fn from(val: Pio124Func) -> u8 {
        Pio124Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio124Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio124Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio124Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio124Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio124Mode {
        Pio124Mode::from_bits(val)
    }
}
impl From<Pio124Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio124Mode) -> u8 {
        Pio124Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio124Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio124Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio124Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio124Od {
    #[inline(always)]
    fn from(val: u8) -> Pio124Od {
        Pio124Od::from_bits(val)
    }
}
impl From<Pio124Od> for u8 {
    #[inline(always)]
    fn from(val: Pio124Od) -> u8 {
        Pio124Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio124Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio124Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio124Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio124Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio124Slew {
        Pio124Slew::from_bits(val)
    }
}
impl From<Pio124Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio124Slew) -> u8 {
        Pio124Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio125Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio125Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio125Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio125Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio125Digimode {
        Pio125Digimode::from_bits(val)
    }
}
impl From<Pio125Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio125Digimode) -> u8 {
        Pio125Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio125Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio125Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio125Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio125Func {
    #[inline(always)]
    fn from(val: u8) -> Pio125Func {
        Pio125Func::from_bits(val)
    }
}
impl From<Pio125Func> for u8 {
    #[inline(always)]
    fn from(val: Pio125Func) -> u8 {
        Pio125Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio125Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio125Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio125Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio125Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio125Mode {
        Pio125Mode::from_bits(val)
    }
}
impl From<Pio125Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio125Mode) -> u8 {
        Pio125Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio125Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio125Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio125Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio125Od {
    #[inline(always)]
    fn from(val: u8) -> Pio125Od {
        Pio125Od::from_bits(val)
    }
}
impl From<Pio125Od> for u8 {
    #[inline(always)]
    fn from(val: Pio125Od) -> u8 {
        Pio125Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio125Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio125Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio125Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio125Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio125Slew {
        Pio125Slew::from_bits(val)
    }
}
impl From<Pio125Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio125Slew) -> u8 {
        Pio125Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio126Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio126Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio126Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio126Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio126Digimode {
        Pio126Digimode::from_bits(val)
    }
}
impl From<Pio126Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio126Digimode) -> u8 {
        Pio126Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio126Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio126Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio126Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio126Func {
    #[inline(always)]
    fn from(val: u8) -> Pio126Func {
        Pio126Func::from_bits(val)
    }
}
impl From<Pio126Func> for u8 {
    #[inline(always)]
    fn from(val: Pio126Func) -> u8 {
        Pio126Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio126Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio126Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio126Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio126Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio126Mode {
        Pio126Mode::from_bits(val)
    }
}
impl From<Pio126Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio126Mode) -> u8 {
        Pio126Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio126Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio126Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio126Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio126Od {
    #[inline(always)]
    fn from(val: u8) -> Pio126Od {
        Pio126Od::from_bits(val)
    }
}
impl From<Pio126Od> for u8 {
    #[inline(always)]
    fn from(val: Pio126Od) -> u8 {
        Pio126Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio126Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio126Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio126Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio126Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio126Slew {
        Pio126Slew::from_bits(val)
    }
}
impl From<Pio126Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio126Slew) -> u8 {
        Pio126Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio127Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio127Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio127Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio127Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio127Digimode {
        Pio127Digimode::from_bits(val)
    }
}
impl From<Pio127Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio127Digimode) -> u8 {
        Pio127Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio127Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio127Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio127Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio127Func {
    #[inline(always)]
    fn from(val: u8) -> Pio127Func {
        Pio127Func::from_bits(val)
    }
}
impl From<Pio127Func> for u8 {
    #[inline(always)]
    fn from(val: Pio127Func) -> u8 {
        Pio127Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio127Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio127Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio127Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio127Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio127Mode {
        Pio127Mode::from_bits(val)
    }
}
impl From<Pio127Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio127Mode) -> u8 {
        Pio127Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio127Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio127Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio127Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio127Od {
    #[inline(always)]
    fn from(val: u8) -> Pio127Od {
        Pio127Od::from_bits(val)
    }
}
impl From<Pio127Od> for u8 {
    #[inline(always)]
    fn from(val: Pio127Od) -> u8 {
        Pio127Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio127Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio127Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio127Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio127Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio127Slew {
        Pio127Slew::from_bits(val)
    }
}
impl From<Pio127Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio127Slew) -> u8 {
        Pio127Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio128Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio128Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio128Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio128Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio128Digimode {
        Pio128Digimode::from_bits(val)
    }
}
impl From<Pio128Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio128Digimode) -> u8 {
        Pio128Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio128Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio128Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio128Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio128Func {
    #[inline(always)]
    fn from(val: u8) -> Pio128Func {
        Pio128Func::from_bits(val)
    }
}
impl From<Pio128Func> for u8 {
    #[inline(always)]
    fn from(val: Pio128Func) -> u8 {
        Pio128Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio128Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio128Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio128Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio128Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio128Mode {
        Pio128Mode::from_bits(val)
    }
}
impl From<Pio128Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio128Mode) -> u8 {
        Pio128Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio128Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio128Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio128Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio128Od {
    #[inline(always)]
    fn from(val: u8) -> Pio128Od {
        Pio128Od::from_bits(val)
    }
}
impl From<Pio128Od> for u8 {
    #[inline(always)]
    fn from(val: Pio128Od) -> u8 {
        Pio128Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio128Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio128Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio128Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio128Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio128Slew {
        Pio128Slew::from_bits(val)
    }
}
impl From<Pio128Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio128Slew) -> u8 {
        Pio128Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio129Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio129Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio129Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio129Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio129Digimode {
        Pio129Digimode::from_bits(val)
    }
}
impl From<Pio129Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio129Digimode) -> u8 {
        Pio129Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio129Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio129Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio129Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio129Func {
    #[inline(always)]
    fn from(val: u8) -> Pio129Func {
        Pio129Func::from_bits(val)
    }
}
impl From<Pio129Func> for u8 {
    #[inline(always)]
    fn from(val: Pio129Func) -> u8 {
        Pio129Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio129Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio129Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio129Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio129Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio129Mode {
        Pio129Mode::from_bits(val)
    }
}
impl From<Pio129Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio129Mode) -> u8 {
        Pio129Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio129Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio129Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio129Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio129Od {
    #[inline(always)]
    fn from(val: u8) -> Pio129Od {
        Pio129Od::from_bits(val)
    }
}
impl From<Pio129Od> for u8 {
    #[inline(always)]
    fn from(val: Pio129Od) -> u8 {
        Pio129Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio129Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio129Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio129Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio129Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio129Slew {
        Pio129Slew::from_bits(val)
    }
}
impl From<Pio129Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio129Slew) -> u8 {
        Pio129Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio12Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio12Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio12Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio12Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio12Digimode {
        Pio12Digimode::from_bits(val)
    }
}
impl From<Pio12Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio12Digimode) -> u8 {
        Pio12Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio12Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio12Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio12Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio12Func {
    #[inline(always)]
    fn from(val: u8) -> Pio12Func {
        Pio12Func::from_bits(val)
    }
}
impl From<Pio12Func> for u8 {
    #[inline(always)]
    fn from(val: Pio12Func) -> u8 {
        Pio12Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio12Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio12Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio12Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio12Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio12Mode {
        Pio12Mode::from_bits(val)
    }
}
impl From<Pio12Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio12Mode) -> u8 {
        Pio12Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio12Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio12Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio12Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio12Od {
    #[inline(always)]
    fn from(val: u8) -> Pio12Od {
        Pio12Od::from_bits(val)
    }
}
impl From<Pio12Od> for u8 {
    #[inline(always)]
    fn from(val: Pio12Od) -> u8 {
        Pio12Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio12Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio12Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio12Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio12Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio12Slew {
        Pio12Slew::from_bits(val)
    }
}
impl From<Pio12Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio12Slew) -> u8 {
        Pio12Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio130Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio130Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio130Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio130Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio130Digimode {
        Pio130Digimode::from_bits(val)
    }
}
impl From<Pio130Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio130Digimode) -> u8 {
        Pio130Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio130Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio130Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio130Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio130Func {
    #[inline(always)]
    fn from(val: u8) -> Pio130Func {
        Pio130Func::from_bits(val)
    }
}
impl From<Pio130Func> for u8 {
    #[inline(always)]
    fn from(val: Pio130Func) -> u8 {
        Pio130Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio130Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio130Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio130Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio130Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio130Mode {
        Pio130Mode::from_bits(val)
    }
}
impl From<Pio130Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio130Mode) -> u8 {
        Pio130Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio130Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio130Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio130Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio130Od {
    #[inline(always)]
    fn from(val: u8) -> Pio130Od {
        Pio130Od::from_bits(val)
    }
}
impl From<Pio130Od> for u8 {
    #[inline(always)]
    fn from(val: Pio130Od) -> u8 {
        Pio130Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio130Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio130Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio130Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio130Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio130Slew {
        Pio130Slew::from_bits(val)
    }
}
impl From<Pio130Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio130Slew) -> u8 {
        Pio130Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio131Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio131Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio131Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio131Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio131Digimode {
        Pio131Digimode::from_bits(val)
    }
}
impl From<Pio131Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio131Digimode) -> u8 {
        Pio131Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio131Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio131Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio131Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio131Func {
    #[inline(always)]
    fn from(val: u8) -> Pio131Func {
        Pio131Func::from_bits(val)
    }
}
impl From<Pio131Func> for u8 {
    #[inline(always)]
    fn from(val: Pio131Func) -> u8 {
        Pio131Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio131Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio131Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio131Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio131Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio131Mode {
        Pio131Mode::from_bits(val)
    }
}
impl From<Pio131Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio131Mode) -> u8 {
        Pio131Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio131Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio131Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio131Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio131Od {
    #[inline(always)]
    fn from(val: u8) -> Pio131Od {
        Pio131Od::from_bits(val)
    }
}
impl From<Pio131Od> for u8 {
    #[inline(always)]
    fn from(val: Pio131Od) -> u8 {
        Pio131Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio131Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio131Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio131Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio131Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio131Slew {
        Pio131Slew::from_bits(val)
    }
}
impl From<Pio131Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio131Slew) -> u8 {
        Pio131Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio13Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio13Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio13Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio13Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio13Digimode {
        Pio13Digimode::from_bits(val)
    }
}
impl From<Pio13Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio13Digimode) -> u8 {
        Pio13Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio13Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio13Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio13Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio13Func {
    #[inline(always)]
    fn from(val: u8) -> Pio13Func {
        Pio13Func::from_bits(val)
    }
}
impl From<Pio13Func> for u8 {
    #[inline(always)]
    fn from(val: Pio13Func) -> u8 {
        Pio13Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio13Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio13Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio13Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio13Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio13Mode {
        Pio13Mode::from_bits(val)
    }
}
impl From<Pio13Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio13Mode) -> u8 {
        Pio13Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio13Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio13Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio13Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio13Od {
    #[inline(always)]
    fn from(val: u8) -> Pio13Od {
        Pio13Od::from_bits(val)
    }
}
impl From<Pio13Od> for u8 {
    #[inline(always)]
    fn from(val: Pio13Od) -> u8 {
        Pio13Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio13Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio13Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio13Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio13Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio13Slew {
        Pio13Slew::from_bits(val)
    }
}
impl From<Pio13Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio13Slew) -> u8 {
        Pio13Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio14Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio14Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio14Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio14Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio14Digimode {
        Pio14Digimode::from_bits(val)
    }
}
impl From<Pio14Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio14Digimode) -> u8 {
        Pio14Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio14Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio14Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio14Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio14Func {
    #[inline(always)]
    fn from(val: u8) -> Pio14Func {
        Pio14Func::from_bits(val)
    }
}
impl From<Pio14Func> for u8 {
    #[inline(always)]
    fn from(val: Pio14Func) -> u8 {
        Pio14Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio14Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio14Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio14Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio14Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio14Mode {
        Pio14Mode::from_bits(val)
    }
}
impl From<Pio14Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio14Mode) -> u8 {
        Pio14Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio14Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio14Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio14Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio14Od {
    #[inline(always)]
    fn from(val: u8) -> Pio14Od {
        Pio14Od::from_bits(val)
    }
}
impl From<Pio14Od> for u8 {
    #[inline(always)]
    fn from(val: Pio14Od) -> u8 {
        Pio14Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio14Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio14Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio14Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio14Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio14Slew {
        Pio14Slew::from_bits(val)
    }
}
impl From<Pio14Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio14Slew) -> u8 {
        Pio14Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio15Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio15Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio15Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio15Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio15Digimode {
        Pio15Digimode::from_bits(val)
    }
}
impl From<Pio15Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio15Digimode) -> u8 {
        Pio15Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio15Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio15Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio15Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio15Func {
    #[inline(always)]
    fn from(val: u8) -> Pio15Func {
        Pio15Func::from_bits(val)
    }
}
impl From<Pio15Func> for u8 {
    #[inline(always)]
    fn from(val: Pio15Func) -> u8 {
        Pio15Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio15Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio15Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio15Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio15Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio15Mode {
        Pio15Mode::from_bits(val)
    }
}
impl From<Pio15Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio15Mode) -> u8 {
        Pio15Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio15Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio15Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio15Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio15Od {
    #[inline(always)]
    fn from(val: u8) -> Pio15Od {
        Pio15Od::from_bits(val)
    }
}
impl From<Pio15Od> for u8 {
    #[inline(always)]
    fn from(val: Pio15Od) -> u8 {
        Pio15Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio15Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio15Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio15Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio15Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio15Slew {
        Pio15Slew::from_bits(val)
    }
}
impl From<Pio15Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio15Slew) -> u8 {
        Pio15Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio16Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio16Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio16Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio16Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio16Digimode {
        Pio16Digimode::from_bits(val)
    }
}
impl From<Pio16Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio16Digimode) -> u8 {
        Pio16Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio16Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio16Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio16Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio16Func {
    #[inline(always)]
    fn from(val: u8) -> Pio16Func {
        Pio16Func::from_bits(val)
    }
}
impl From<Pio16Func> for u8 {
    #[inline(always)]
    fn from(val: Pio16Func) -> u8 {
        Pio16Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio16Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio16Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio16Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio16Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio16Mode {
        Pio16Mode::from_bits(val)
    }
}
impl From<Pio16Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio16Mode) -> u8 {
        Pio16Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio16Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio16Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio16Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio16Od {
    #[inline(always)]
    fn from(val: u8) -> Pio16Od {
        Pio16Od::from_bits(val)
    }
}
impl From<Pio16Od> for u8 {
    #[inline(always)]
    fn from(val: Pio16Od) -> u8 {
        Pio16Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio16Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio16Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio16Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio16Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio16Slew {
        Pio16Slew::from_bits(val)
    }
}
impl From<Pio16Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio16Slew) -> u8 {
        Pio16Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio17Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio17Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio17Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio17Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio17Digimode {
        Pio17Digimode::from_bits(val)
    }
}
impl From<Pio17Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio17Digimode) -> u8 {
        Pio17Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio17Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio17Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio17Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio17Func {
    #[inline(always)]
    fn from(val: u8) -> Pio17Func {
        Pio17Func::from_bits(val)
    }
}
impl From<Pio17Func> for u8 {
    #[inline(always)]
    fn from(val: Pio17Func) -> u8 {
        Pio17Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio17Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio17Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio17Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio17Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio17Mode {
        Pio17Mode::from_bits(val)
    }
}
impl From<Pio17Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio17Mode) -> u8 {
        Pio17Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio17Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio17Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio17Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio17Od {
    #[inline(always)]
    fn from(val: u8) -> Pio17Od {
        Pio17Od::from_bits(val)
    }
}
impl From<Pio17Od> for u8 {
    #[inline(always)]
    fn from(val: Pio17Od) -> u8 {
        Pio17Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio17Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio17Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio17Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio17Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio17Slew {
        Pio17Slew::from_bits(val)
    }
}
impl From<Pio17Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio17Slew) -> u8 {
        Pio17Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio18Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio18Asw {
        Pio18Asw::from_bits(val)
    }
}
impl From<Pio18Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio18Asw) -> u8 {
        Pio18Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio18Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio18Digimode {
        Pio18Digimode::from_bits(val)
    }
}
impl From<Pio18Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio18Digimode) -> u8 {
        Pio18Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio18Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Func {
    #[inline(always)]
    fn from(val: u8) -> Pio18Func {
        Pio18Func::from_bits(val)
    }
}
impl From<Pio18Func> for u8 {
    #[inline(always)]
    fn from(val: Pio18Func) -> u8 {
        Pio18Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio18Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio18Mode {
        Pio18Mode::from_bits(val)
    }
}
impl From<Pio18Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio18Mode) -> u8 {
        Pio18Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio18Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Od {
    #[inline(always)]
    fn from(val: u8) -> Pio18Od {
        Pio18Od::from_bits(val)
    }
}
impl From<Pio18Od> for u8 {
    #[inline(always)]
    fn from(val: Pio18Od) -> u8 {
        Pio18Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio18Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio18Slew {
        Pio18Slew::from_bits(val)
    }
}
impl From<Pio18Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio18Slew) -> u8 {
        Pio18Slew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Asw {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)"]
    VALUE1 = 0x01,
}
impl Pio19Asw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Asw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Asw {
    #[inline(always)]
    fn from(val: u8) -> Pio19Asw {
        Pio19Asw::from_bits(val)
    }
}
impl From<Pio19Asw> for u8 {
    #[inline(always)]
    fn from(val: Pio19Asw) -> u8 {
        Pio19Asw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Digimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl Pio19Digimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Digimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Digimode {
    #[inline(always)]
    fn from(val: u8) -> Pio19Digimode {
        Pio19Digimode::from_bits(val)
    }
}
impl From<Pio19Digimode> for u8 {
    #[inline(always)]
    fn from(val: Pio19Digimode) -> u8 {
        Pio19Digimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Func {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pio19Func {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Func {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Func {
    #[inline(always)]
    fn from(val: u8) -> Pio19Func {
        Pio19Func::from_bits(val)
    }
}
impl From<Pio19Func> for u8 {
    #[inline(always)]
    fn from(val: Pio19Func) -> u8 {
        Pio19Func::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Mode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Pio19Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Mode {
    #[inline(always)]
    fn from(val: u8) -> Pio19Mode {
        Pio19Mode::from_bits(val)
    }
}
impl From<Pio19Mode> for u8 {
    #[inline(always)]
    fn from(val: Pio19Mode) -> u8 {
        Pio19Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Od {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl Pio19Od {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Od {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Od {
    #[inline(always)]
    fn from(val: u8) -> Pio19Od {
        Pio19Od::from_bits(val)
    }
}
impl From<Pio19Od> for u8 {
    #[inline(always)]
    fn from(val: Pio19Od) -> u8 {
        Pio19Od::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Slew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl Pio19Slew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Slew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Slew {
    #[inline(always)]
    fn from(val: u8) -> Pio19Slew {
        Pio19Slew::from_bits(val)
    }
}
impl From<Pio19Slew> for u8 {
    #[inline(always)]
    fn from(val: Pio19Slew) -> u8 {
        Pio19Slew::to_bits(val)
    }
}
