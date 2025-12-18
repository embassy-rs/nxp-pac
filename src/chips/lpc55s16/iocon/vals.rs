#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio00Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio00Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio00Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio00Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio00Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio00Invert {
        Pio00Invert::from_bits(val)
    }
}
impl From<Pio00Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio00Invert) -> u8 {
        Pio00Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio010Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio010Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio010Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio010Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio010Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio010Invert {
        Pio010Invert::from_bits(val)
    }
}
impl From<Pio010Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio010Invert) -> u8 {
        Pio010Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio011Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio011Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio011Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio011Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio011Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio011Invert {
        Pio011Invert::from_bits(val)
    }
}
impl From<Pio011Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio011Invert) -> u8 {
        Pio011Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio012Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio012Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio012Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio012Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio012Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio012Invert {
        Pio012Invert::from_bits(val)
    }
}
impl From<Pio012Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio012Invert) -> u8 {
        Pio012Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Ecs {
    #[doc = "Disabled. IO is in open drain cell."]
    DISABLED = 0x0,
    #[doc = "Enabled. Pull resistor is conencted."]
    ENABLED = 0x01,
}
impl Pio013Ecs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Ecs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Ecs {
    #[inline(always)]
    fn from(val: u8) -> Pio013Ecs {
        Pio013Ecs::from_bits(val)
    }
}
impl From<Pio013Ecs> for u8 {
    #[inline(always)]
    fn from(val: Pio013Ecs) -> u8 {
        Pio013Ecs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio013Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio013Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio013Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio013Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio013Invert {
        Pio013Invert::from_bits(val)
    }
}
impl From<Pio013Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio013Invert) -> u8 {
        Pio013Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Ecs {
    #[doc = "Disabled. IO is in open drain cell."]
    DISABLED = 0x0,
    #[doc = "Enabled. Pull resistor is conencted."]
    ENABLED = 0x01,
}
impl Pio014Ecs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Ecs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Ecs {
    #[inline(always)]
    fn from(val: u8) -> Pio014Ecs {
        Pio014Ecs::from_bits(val)
    }
}
impl From<Pio014Ecs> for u8 {
    #[inline(always)]
    fn from(val: Pio014Ecs) -> u8 {
        Pio014Ecs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio014Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio014Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio014Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio014Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio014Invert {
        Pio014Invert::from_bits(val)
    }
}
impl From<Pio014Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio014Invert) -> u8 {
        Pio014Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio015Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio015Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio015Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio015Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio015Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio015Invert {
        Pio015Invert::from_bits(val)
    }
}
impl From<Pio015Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio015Invert) -> u8 {
        Pio015Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio016Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio016Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio016Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio016Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio016Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio016Invert {
        Pio016Invert::from_bits(val)
    }
}
impl From<Pio016Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio016Invert) -> u8 {
        Pio016Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio017Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio017Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio017Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio017Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio017Invert {
        Pio017Invert::from_bits(val)
    }
}
impl From<Pio017Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio017Invert) -> u8 {
        Pio017Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio018Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio018Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio018Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio018Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio018Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio018Invert {
        Pio018Invert::from_bits(val)
    }
}
impl From<Pio018Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio018Invert) -> u8 {
        Pio018Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio019Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio019Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio019Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio019Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio019Invert {
        Pio019Invert::from_bits(val)
    }
}
impl From<Pio019Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio019Invert) -> u8 {
        Pio019Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio01Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio01Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio01Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio01Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio01Invert {
        Pio01Invert::from_bits(val)
    }
}
impl From<Pio01Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio01Invert) -> u8 {
        Pio01Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio020Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio020Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio020Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio020Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio020Invert {
        Pio020Invert::from_bits(val)
    }
}
impl From<Pio020Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio020Invert) -> u8 {
        Pio020Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio021Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio021Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio021Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio021Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio021Invert {
        Pio021Invert::from_bits(val)
    }
}
impl From<Pio021Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio021Invert) -> u8 {
        Pio021Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio022Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio022Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio022Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio022Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio022Invert {
        Pio022Invert::from_bits(val)
    }
}
impl From<Pio022Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio022Invert) -> u8 {
        Pio022Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio023Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio023Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio023Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio023Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio023Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio023Invert {
        Pio023Invert::from_bits(val)
    }
}
impl From<Pio023Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio023Invert) -> u8 {
        Pio023Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio024Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio024Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio024Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio024Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio024Invert {
        Pio024Invert::from_bits(val)
    }
}
impl From<Pio024Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio024Invert) -> u8 {
        Pio024Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio025Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio025Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio025Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio025Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio025Invert {
        Pio025Invert::from_bits(val)
    }
}
impl From<Pio025Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio025Invert) -> u8 {
        Pio025Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio026Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio026Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio026Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio026Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio026Invert {
        Pio026Invert::from_bits(val)
    }
}
impl From<Pio026Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio026Invert) -> u8 {
        Pio026Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio027Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio027Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio027Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio027Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio027Invert {
        Pio027Invert::from_bits(val)
    }
}
impl From<Pio027Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio027Invert) -> u8 {
        Pio027Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio028Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio028Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio028Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio028Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio028Invert {
        Pio028Invert::from_bits(val)
    }
}
impl From<Pio028Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio028Invert) -> u8 {
        Pio028Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio029Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio029Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio029Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio029Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio029Invert {
        Pio029Invert::from_bits(val)
    }
}
impl From<Pio029Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio029Invert) -> u8 {
        Pio029Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio02Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio02Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio02Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio02Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio02Invert {
        Pio02Invert::from_bits(val)
    }
}
impl From<Pio02Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio02Invert) -> u8 {
        Pio02Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio030Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio030Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio030Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio030Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio030Invert {
        Pio030Invert::from_bits(val)
    }
}
impl From<Pio030Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio030Invert) -> u8 {
        Pio030Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio031Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio031Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio031Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio031Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio031Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio031Invert {
        Pio031Invert::from_bits(val)
    }
}
impl From<Pio031Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio031Invert) -> u8 {
        Pio031Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio03Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio03Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio03Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio03Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio03Invert {
        Pio03Invert::from_bits(val)
    }
}
impl From<Pio03Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio03Invert) -> u8 {
        Pio03Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio04Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio04Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio04Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio04Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio04Invert {
        Pio04Invert::from_bits(val)
    }
}
impl From<Pio04Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio04Invert) -> u8 {
        Pio04Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio05Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio05Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio05Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio05Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio05Invert {
        Pio05Invert::from_bits(val)
    }
}
impl From<Pio05Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio05Invert) -> u8 {
        Pio05Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio06Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio06Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio06Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio06Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio06Invert {
        Pio06Invert::from_bits(val)
    }
}
impl From<Pio06Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio06Invert) -> u8 {
        Pio06Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio07Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio07Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio07Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio07Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio07Invert {
        Pio07Invert::from_bits(val)
    }
}
impl From<Pio07Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio07Invert) -> u8 {
        Pio07Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio08Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio08Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio08Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio08Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio08Invert {
        Pio08Invert::from_bits(val)
    }
}
impl From<Pio08Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio08Invert) -> u8 {
        Pio08Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio09Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio09Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio09Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio09Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio09Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio09Invert {
        Pio09Invert::from_bits(val)
    }
}
impl From<Pio09Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio09Invert) -> u8 {
        Pio09Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio10Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio10Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio10Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio10Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio10Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio10Invert {
        Pio10Invert::from_bits(val)
    }
}
impl From<Pio10Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio10Invert) -> u8 {
        Pio10Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio110Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio110Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio110Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio110Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio110Invert {
        Pio110Invert::from_bits(val)
    }
}
impl From<Pio110Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio110Invert) -> u8 {
        Pio110Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio111Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio111Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio111Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio111Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio111Invert {
        Pio111Invert::from_bits(val)
    }
}
impl From<Pio111Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio111Invert) -> u8 {
        Pio111Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio112Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio112Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio112Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio112Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio112Invert {
        Pio112Invert::from_bits(val)
    }
}
impl From<Pio112Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio112Invert) -> u8 {
        Pio112Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio113Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio113Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio113Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio113Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio113Invert {
        Pio113Invert::from_bits(val)
    }
}
impl From<Pio113Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio113Invert) -> u8 {
        Pio113Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio114Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio114Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio114Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio114Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio114Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio114Invert {
        Pio114Invert::from_bits(val)
    }
}
impl From<Pio114Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio114Invert) -> u8 {
        Pio114Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio115Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio115Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio115Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio115Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio115Invert {
        Pio115Invert::from_bits(val)
    }
}
impl From<Pio115Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio115Invert) -> u8 {
        Pio115Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio116Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio116Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio116Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio116Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio116Invert {
        Pio116Invert::from_bits(val)
    }
}
impl From<Pio116Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio116Invert) -> u8 {
        Pio116Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio117Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio117Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio117Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio117Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio117Invert {
        Pio117Invert::from_bits(val)
    }
}
impl From<Pio117Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio117Invert) -> u8 {
        Pio117Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio118Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio118Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio118Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio118Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio118Invert {
        Pio118Invert::from_bits(val)
    }
}
impl From<Pio118Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio118Invert) -> u8 {
        Pio118Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio119Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio119Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio119Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio119Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio119Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio119Invert {
        Pio119Invert::from_bits(val)
    }
}
impl From<Pio119Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio119Invert) -> u8 {
        Pio119Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio11Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio11Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio11Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio11Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio11Invert {
        Pio11Invert::from_bits(val)
    }
}
impl From<Pio11Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio11Invert) -> u8 {
        Pio11Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio120Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio120Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio120Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio120Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio120Invert {
        Pio120Invert::from_bits(val)
    }
}
impl From<Pio120Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio120Invert) -> u8 {
        Pio120Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio121Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio121Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio121Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio121Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio121Invert {
        Pio121Invert::from_bits(val)
    }
}
impl From<Pio121Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio121Invert) -> u8 {
        Pio121Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio122Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio122Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio122Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio122Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio122Invert {
        Pio122Invert::from_bits(val)
    }
}
impl From<Pio122Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio122Invert) -> u8 {
        Pio122Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio123Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio123Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio123Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio123Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio123Invert {
        Pio123Invert::from_bits(val)
    }
}
impl From<Pio123Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio123Invert) -> u8 {
        Pio123Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio124Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio124Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio124Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio124Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio124Invert {
        Pio124Invert::from_bits(val)
    }
}
impl From<Pio124Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio124Invert) -> u8 {
        Pio124Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio125Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio125Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio125Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio125Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio125Invert {
        Pio125Invert::from_bits(val)
    }
}
impl From<Pio125Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio125Invert) -> u8 {
        Pio125Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio126Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio126Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio126Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio126Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio126Invert {
        Pio126Invert::from_bits(val)
    }
}
impl From<Pio126Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio126Invert) -> u8 {
        Pio126Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio127Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio127Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio127Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio127Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio127Invert {
        Pio127Invert::from_bits(val)
    }
}
impl From<Pio127Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio127Invert) -> u8 {
        Pio127Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio128Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio128Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio128Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio128Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio128Invert {
        Pio128Invert::from_bits(val)
    }
}
impl From<Pio128Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio128Invert) -> u8 {
        Pio128Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio129Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio129Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio129Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio129Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio129Invert {
        Pio129Invert::from_bits(val)
    }
}
impl From<Pio129Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio129Invert) -> u8 {
        Pio129Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio12Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio12Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio12Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio12Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio12Invert {
        Pio12Invert::from_bits(val)
    }
}
impl From<Pio12Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio12Invert) -> u8 {
        Pio12Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio130Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio130Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio130Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio130Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio130Invert {
        Pio130Invert::from_bits(val)
    }
}
impl From<Pio130Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio130Invert) -> u8 {
        Pio130Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio131Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio131Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio131Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio131Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio131Invert {
        Pio131Invert::from_bits(val)
    }
}
impl From<Pio131Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio131Invert) -> u8 {
        Pio131Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio13Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio13Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio13Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio13Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio13Invert {
        Pio13Invert::from_bits(val)
    }
}
impl From<Pio13Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio13Invert) -> u8 {
        Pio13Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio14Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio14Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio14Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio14Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio14Invert {
        Pio14Invert::from_bits(val)
    }
}
impl From<Pio14Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio14Invert) -> u8 {
        Pio14Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio15Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio15Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio15Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio15Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio15Invert {
        Pio15Invert::from_bits(val)
    }
}
impl From<Pio15Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio15Invert) -> u8 {
        Pio15Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio16Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio16Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio16Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio16Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio16Invert {
        Pio16Invert::from_bits(val)
    }
}
impl From<Pio16Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio16Invert) -> u8 {
        Pio16Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio17Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio17Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio17Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio17Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio17Invert {
        Pio17Invert::from_bits(val)
    }
}
impl From<Pio17Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio17Invert) -> u8 {
        Pio17Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio18Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio18Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio18Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio18Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio18Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio18Invert {
        Pio18Invert::from_bits(val)
    }
}
impl From<Pio18Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio18Invert) -> u8 {
        Pio18Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pio19Asw {
    #[doc = "Analog switch is open. (disable)"]
    DISABLE = 0x0,
    #[doc = "Analog switch is closed. (enable)"]
    ENABLE = 0x01,
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
pub enum Pio19Invert {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED = 0x01,
}
impl Pio19Invert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pio19Invert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pio19Invert {
    #[inline(always)]
    fn from(val: u8) -> Pio19Invert {
        Pio19Invert::from_bits(val)
    }
}
impl From<Pio19Invert> for u8 {
    #[inline(always)]
    fn from(val: Pio19Invert) -> u8 {
        Pio19Invert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioDigimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PioDigimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioDigimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioDigimode {
    #[inline(always)]
    fn from(val: u8) -> PioDigimode {
        PioDigimode::from_bits(val)
    }
}
impl From<PioDigimode> for u8 {
    #[inline(always)]
    fn from(val: PioDigimode) -> u8 {
        PioDigimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioEgp {
    #[doc = "I2C mode."]
    I2C_MODE = 0x0,
    #[doc = "GPIO mode."]
    GPIO_MODE = 0x01,
}
impl PioEgp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioEgp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioEgp {
    #[inline(always)]
    fn from(val: u8) -> PioEgp {
        PioEgp::from_bits(val)
    }
}
impl From<PioEgp> for u8 {
    #[inline(always)]
    fn from(val: PioEgp) -> u8 {
        PioEgp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioFilteroff {
    #[doc = "Filter enabled."]
    ENABLED = 0x0,
    #[doc = "Filter disabled."]
    DISABLED = 0x01,
}
impl PioFilteroff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioFilteroff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioFilteroff {
    #[inline(always)]
    fn from(val: u8) -> PioFilteroff {
        PioFilteroff::from_bits(val)
    }
}
impl From<PioFilteroff> for u8 {
    #[inline(always)]
    fn from(val: PioFilteroff) -> u8 {
        PioFilteroff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioFunc {
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
impl PioFunc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioFunc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioFunc {
    #[inline(always)]
    fn from(val: u8) -> PioFunc {
        PioFunc::from_bits(val)
    }
}
impl From<PioFunc> for u8 {
    #[inline(always)]
    fn from(val: PioFunc) -> u8 {
        PioFunc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioI2cfilter {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE = 0x0,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    STANDARD_MODE = 0x01,
}
impl PioI2cfilter {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioI2cfilter {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioI2cfilter {
    #[inline(always)]
    fn from(val: u8) -> PioI2cfilter {
        PioI2cfilter::from_bits(val)
    }
}
impl From<PioI2cfilter> for u8 {
    #[inline(always)]
    fn from(val: PioI2cfilter) -> u8 {
        PioI2cfilter::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioMode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PioMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioMode {
    #[inline(always)]
    fn from(val: u8) -> PioMode {
        PioMode::from_bits(val)
    }
}
impl From<PioMode> for u8 {
    #[inline(always)]
    fn from(val: PioMode) -> u8 {
        PioMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioOd {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PioOd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioOd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioOd {
    #[inline(always)]
    fn from(val: u8) -> PioOd {
        PioOd::from_bits(val)
    }
}
impl From<PioOd> for u8 {
    #[inline(always)]
    fn from(val: PioOd) -> u8 {
        PioOd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioSlew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PioSlew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioSlew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioSlew {
    #[inline(always)]
    fn from(val: u8) -> PioSlew {
        PioSlew::from_bits(val)
    }
}
impl From<PioSlew> for u8 {
    #[inline(always)]
    fn from(val: PioSlew) -> u8 {
        PioSlew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioSsel {
    #[doc = "3V3 Signaling in I2C Mode."]
    SEL3V3 = 0x0,
    #[doc = "1V8 Signaling in I2C Mode."]
    SEL1V8 = 0x01,
}
impl PioSsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioSsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioSsel {
    #[inline(always)]
    fn from(val: u8) -> PioSsel {
        PioSsel::from_bits(val)
    }
}
impl From<PioSsel> for u8 {
    #[inline(always)]
    fn from(val: PioSsel) -> u8 {
        PioSsel::to_bits(val)
    }
}
