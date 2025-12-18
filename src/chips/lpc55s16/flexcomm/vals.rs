#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cpresent {
    #[doc = "This Flexcomm does not include the I2C function."]
    NOT_PRESENT = 0x0,
    #[doc = "This Flexcomm includes the I2C function."]
    PRESENT = 0x01,
}
impl I2cpresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cpresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cpresent {
    #[inline(always)]
    fn from(val: u8) -> I2cpresent {
        I2cpresent::from_bits(val)
    }
}
impl From<I2cpresent> for u8 {
    #[inline(always)]
    fn from(val: I2cpresent) -> u8 {
        I2cpresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2spresent {
    #[doc = "This Flexcomm does not include the I2S function."]
    NOT_PRESENT = 0x0,
    #[doc = "This Flexcomm includes the I2S function."]
    PRESENT = 0x01,
}
impl I2spresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2spresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2spresent {
    #[inline(always)]
    fn from(val: u8) -> I2spresent {
        I2spresent::from_bits(val)
    }
}
impl From<I2spresent> for u8 {
    #[inline(always)]
    fn from(val: I2spresent) -> u8 {
        I2spresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lock {
    #[doc = "Peripheral select can be changed by software."]
    UNLOCKED = 0x0,
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED = 0x01,
}
impl Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Persel {
    #[doc = "No peripheral selected."]
    NO_PERIPH_SELECTED = 0x0,
    #[doc = "USART function selected."]
    USART = 0x01,
    #[doc = "SPI function selected."]
    SPI = 0x02,
    #[doc = "I2C function selected."]
    I2C = 0x03,
    #[doc = "I2S transmit function selected."]
    I2S_TRANSMIT = 0x04,
    #[doc = "I2S receive function selected."]
    I2S_RECEIVE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Persel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Persel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Persel {
    #[inline(always)]
    fn from(val: u8) -> Persel {
        Persel::from_bits(val)
    }
}
impl From<Persel> for u8 {
    #[inline(always)]
    fn from(val: Persel) -> u8 {
        Persel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spipresent {
    #[doc = "This Flexcomm does not include the SPI function."]
    NOT_PRESENT = 0x0,
    #[doc = "This Flexcomm includes the SPI function."]
    PRESENT = 0x01,
}
impl Spipresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spipresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spipresent {
    #[inline(always)]
    fn from(val: u8) -> Spipresent {
        Spipresent::from_bits(val)
    }
}
impl From<Spipresent> for u8 {
    #[inline(always)]
    fn from(val: Spipresent) -> u8 {
        Spipresent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usartpresent {
    #[doc = "This Flexcomm does not include the USART function."]
    NOT_PRESENT = 0x0,
    #[doc = "This Flexcomm includes the USART function."]
    PRESENT = 0x01,
}
impl Usartpresent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usartpresent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usartpresent {
    #[inline(always)]
    fn from(val: u8) -> Usartpresent {
        Usartpresent::from_bits(val)
    }
}
impl From<Usartpresent> for u8 {
    #[inline(always)]
    fn from(val: Usartpresent) -> u8 {
        Usartpresent::to_bits(val)
    }
}
