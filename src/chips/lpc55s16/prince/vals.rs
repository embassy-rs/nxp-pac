#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Encryption of writes to the flash controller DATAW* registers is disabled."]
    DISABLED = 0x0,
    #[doc = "Encryption of writes to the flash controller DATAW* registers is enabled. Reading of PRINCE-encrypted flash regions is disabled."]
    ENABLED = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errstat {
    #[doc = "No PRINCE error."]
    NO_ERROR = 0x0,
    #[doc = "Error. A read of a PRINCE-encrypted region was attempted while ENC_ENABLE.EN=1."]
    ERROR = 0x01,
}
impl Errstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errstat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errstat {
    #[inline(always)]
    fn from(val: u8) -> Errstat {
        Errstat::from_bits(val)
    }
}
impl From<Errstat> for u8 {
    #[inline(always)]
    fn from(val: Errstat) -> u8 {
        Errstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockmask {
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    DISABLED = 0x0,
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    ENABLED = 0x01,
}
impl Lockmask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockmask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockmask {
    #[inline(always)]
    fn from(val: u8) -> Lockmask {
        Lockmask::from_bits(val)
    }
}
impl From<Lockmask> for u8 {
    #[inline(always)]
    fn from(val: Lockmask) -> u8 {
        Lockmask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockreg0 {
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    DISABLED = 0x0,
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    ENABLED = 0x01,
}
impl Lockreg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockreg0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockreg0 {
    #[inline(always)]
    fn from(val: u8) -> Lockreg0 {
        Lockreg0::from_bits(val)
    }
}
impl From<Lockreg0> for u8 {
    #[inline(always)]
    fn from(val: Lockreg0) -> u8 {
        Lockreg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockreg1 {
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    DISABLED = 0x0,
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    ENABLED = 0x01,
}
impl Lockreg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockreg1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockreg1 {
    #[inline(always)]
    fn from(val: u8) -> Lockreg1 {
        Lockreg1::from_bits(val)
    }
}
impl From<Lockreg1> for u8 {
    #[inline(always)]
    fn from(val: Lockreg1) -> u8 {
        Lockreg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockreg2 {
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    DISABLED = 0x0,
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    ENABLED = 0x01,
}
impl Lockreg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockreg2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockreg2 {
    #[inline(always)]
    fn from(val: u8) -> Lockreg2 {
        Lockreg2::from_bits(val)
    }
}
impl From<Lockreg2> for u8 {
    #[inline(always)]
    fn from(val: Lockreg2) -> u8 {
        Lockreg2::to_bits(val)
    }
}
