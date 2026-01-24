#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bleed {
    #[doc = "Bleed current is disable."]
    DISABLE = 0x0,
    #[doc = "Bleed current is enable."]
    ENABLE = 0x01,
}
impl Bleed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bleed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bleed {
    #[inline(always)]
    fn from(val: u8) -> Bleed {
        Bleed::from_bits(val)
    }
}
impl From<Bleed> for u8 {
    #[inline(always)]
    fn from(val: Bleed) -> u8 {
        Bleed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodcoreresetenaSecure {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    ENABLE = 0x01,
    #[doc = "BOD Core reset is disable."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl BodcoreresetenaSecure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodcoreresetenaSecure {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodcoreresetenaSecure {
    #[inline(always)]
    fn from(val: u8) -> BodcoreresetenaSecure {
        BodcoreresetenaSecure::from_bits(val)
    }
}
impl From<BodcoreresetenaSecure> for u8 {
    #[inline(always)]
    fn from(val: BodcoreresetenaSecure) -> u8 {
        BodcoreresetenaSecure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodcoreresetenaSecureDp {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    ENABLE = 0x01,
    #[doc = "BOD Core reset is disable."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl BodcoreresetenaSecureDp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodcoreresetenaSecureDp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodcoreresetenaSecureDp {
    #[inline(always)]
    fn from(val: u8) -> BodcoreresetenaSecureDp {
        BodcoreresetenaSecureDp::from_bits(val)
    }
}
impl From<BodcoreresetenaSecureDp> for u8 {
    #[inline(always)]
    fn from(val: BodcoreresetenaSecureDp) -> u8 {
        BodcoreresetenaSecureDp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatHyst {
    #[doc = "25 mV."]
    HYST_25MV = 0x0,
    #[doc = "50 mV."]
    HYST_50MV = 0x01,
    #[doc = "75 mV."]
    HYST_75MV = 0x02,
    #[doc = "100 mV."]
    HYST_100MV = 0x03,
}
impl BodvbatHyst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatHyst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatHyst {
    #[inline(always)]
    fn from(val: u8) -> BodvbatHyst {
        BodvbatHyst::from_bits(val)
    }
}
impl From<BodvbatHyst> for u8 {
    #[inline(always)]
    fn from(val: BodvbatHyst) -> u8 {
        BodvbatHyst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatresetenaSecure {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    ENABLE = 0x01,
    #[doc = "BOD VBAT reset is disable."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl BodvbatresetenaSecure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatresetenaSecure {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatresetenaSecure {
    #[inline(always)]
    fn from(val: u8) -> BodvbatresetenaSecure {
        BodvbatresetenaSecure::from_bits(val)
    }
}
impl From<BodvbatresetenaSecure> for u8 {
    #[inline(always)]
    fn from(val: BodvbatresetenaSecure) -> u8 {
        BodvbatresetenaSecure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BodvbatresetenaSecureDp {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    ENABLE = 0x01,
    #[doc = "BOD VBAT reset is disable."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl BodvbatresetenaSecureDp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BodvbatresetenaSecureDp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BodvbatresetenaSecureDp {
    #[inline(always)]
    fn from(val: u8) -> BodvbatresetenaSecureDp {
        BodvbatresetenaSecureDp::from_bits(val)
    }
}
impl From<BodvbatresetenaSecureDp> for u8 {
    #[inline(always)]
    fn from(val: BodvbatresetenaSecureDp) -> u8 {
        BodvbatresetenaSecureDp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BoostEna {
    #[doc = "LDO AO Boost Mode is disable."]
    DISABLE = 0x0,
    #[doc = "LDO AO Boost Mode is enable."]
    ENABLE = 0x01,
}
impl BoostEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BoostEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BoostEna {
    #[inline(always)]
    fn from(val: u8) -> BoostEna {
        BoostEna::from_bits(val)
    }
}
impl From<BoostEna> for u8 {
    #[inline(always)]
    fn from(val: BoostEna) -> u8 {
        BoostEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BoostEnaPwd {
    #[doc = "LDO AO Boost Mode is disable."]
    DISABLE = 0x0,
    #[doc = "LDO AO Boost Mode is enable."]
    ENABLE = 0x01,
}
impl BoostEnaPwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BoostEnaPwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BoostEnaPwd {
    #[inline(always)]
    fn from(val: u8) -> BoostEnaPwd {
        BoostEnaPwd::from_bits(val)
    }
}
impl From<BoostEnaPwd> for u8 {
    #[inline(always)]
    fn from(val: BoostEnaPwd) -> u8 {
        BoostEnaPwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bootmode {
    #[doc = "Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    POWERUP = 0x0,
    #[doc = "Latest IC boot was from DEEP SLEEP low power mode.."]
    DEEPSLEEP = 0x01,
    #[doc = "Latest IC boot was from POWER DOWN low power mode.."]
    POWERDOWN = 0x02,
    #[doc = "Latest IC boot was from DEEP POWER DOWN low power mode.."]
    DEEPPOWERDOWN = 0x03,
}
impl Bootmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bootmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bootmode {
    #[inline(always)]
    fn from(val: u8) -> Bootmode {
        Bootmode::from_bits(val)
    }
}
impl From<Bootmode> for u8 {
    #[inline(always)]
    fn from(val: Bootmode) -> u8 {
        Bootmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Captestoscinsel {
    #[doc = "Oscillator output pin (osc_out)."]
    OSCOUT = 0x0,
    #[doc = "Oscillator input pin (osc_in)."]
    OSCIN = 0x01,
}
impl Captestoscinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Captestoscinsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Captestoscinsel {
    #[inline(always)]
    fn from(val: u8) -> Captestoscinsel {
        Captestoscinsel::from_bits(val)
    }
}
impl From<Captestoscinsel> for u8 {
    #[inline(always)]
    fn from(val: Captestoscinsel) -> u8 {
        Captestoscinsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capteststartsrcsel {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPSTART = 0x0,
    #[doc = "Sourced from calibration."]
    CALIB = 0x01,
}
impl Capteststartsrcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capteststartsrcsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capteststartsrcsel {
    #[inline(always)]
    fn from(val: u8) -> Capteststartsrcsel {
        Capteststartsrcsel::from_bits(val)
    }
}
impl From<Capteststartsrcsel> for u8 {
    #[inline(always)]
    fn from(val: Capteststartsrcsel) -> u8 {
        Capteststartsrcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CompHyst {
    #[doc = "Hysteresis is disable."]
    DISABLE = 0x0,
    #[doc = "Hysteresis is enable."]
    ENABLE = 0x01,
}
impl CompHyst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CompHyst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CompHyst {
    #[inline(always)]
    fn from(val: u8) -> CompHyst {
        CompHyst::from_bits(val)
    }
}
impl From<CompHyst> for u8 {
    #[inline(always)]
    fn from(val: CompHyst) -> u8 {
        CompHyst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableBleed {
    #[doc = "LDO_MEM bleed current is enabled."]
    BLEED_ENABLE = 0x0,
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    BLEED_DISABLE = 0x01,
}
impl DisableBleed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableBleed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableBleed {
    #[inline(always)]
    fn from(val: u8) -> DisableBleed {
        DisableBleed::from_bits(val)
    }
}
impl From<DisableBleed> for u8 {
    #[inline(always)]
    fn from(val: DisableBleed) -> u8 {
        DisableBleed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpdwakeupresetenable {
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    DISABLE = 0x0,
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    ENABLE = 0x01,
}
impl Dpdwakeupresetenable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpdwakeupresetenable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpdwakeupresetenable {
    #[inline(always)]
    fn from(val: u8) -> Dpdwakeupresetenable {
        Dpdwakeupresetenable::from_bits(val)
    }
}
impl From<Dpdwakeupresetenable> for u8 {
    #[inline(always)]
    fn from(val: Dpdwakeupresetenable) -> u8 {
        Dpdwakeupresetenable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fallingedgewakeup0 {
    #[doc = "Falling edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Falling edge detection is enable."]
    ENABLE = 0x01,
}
impl Fallingedgewakeup0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fallingedgewakeup0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fallingedgewakeup0 {
    #[inline(always)]
    fn from(val: u8) -> Fallingedgewakeup0 {
        Fallingedgewakeup0::from_bits(val)
    }
}
impl From<Fallingedgewakeup0> for u8 {
    #[inline(always)]
    fn from(val: Fallingedgewakeup0) -> u8 {
        Fallingedgewakeup0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fallingedgewakeup1 {
    #[doc = "Falling edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Falling edge detection is enable."]
    ENABLE = 0x01,
}
impl Fallingedgewakeup1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fallingedgewakeup1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fallingedgewakeup1 {
    #[inline(always)]
    fn from(val: u8) -> Fallingedgewakeup1 {
        Fallingedgewakeup1::from_bits(val)
    }
}
impl From<Fallingedgewakeup1> for u8 {
    #[inline(always)]
    fn from(val: Fallingedgewakeup1) -> u8 {
        Fallingedgewakeup1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fallingedgewakeup2 {
    #[doc = "Falling edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Falling edge detection is enable."]
    ENABLE = 0x01,
}
impl Fallingedgewakeup2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fallingedgewakeup2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fallingedgewakeup2 {
    #[inline(always)]
    fn from(val: u8) -> Fallingedgewakeup2 {
        Fallingedgewakeup2::from_bits(val)
    }
}
impl From<Fallingedgewakeup2> for u8 {
    #[inline(always)]
    fn from(val: Fallingedgewakeup2) -> u8 {
        Fallingedgewakeup2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fallingedgewakeup3 {
    #[doc = "Falling edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Falling edge detection is enable."]
    ENABLE = 0x01,
}
impl Fallingedgewakeup3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fallingedgewakeup3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fallingedgewakeup3 {
    #[inline(always)]
    fn from(val: u8) -> Fallingedgewakeup3 {
        Fallingedgewakeup3::from_bits(val)
    }
}
impl From<Fallingedgewakeup3> for u8 {
    #[inline(always)]
    fn from(val: Fallingedgewakeup3) -> u8 {
        Fallingedgewakeup3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FiltercgfClkdiv {
    #[doc = "Filter clock period duration equals 1 Analog Comparator clock period."]
    FILTER_1CLK_PERIOD = 0x0,
    #[doc = "Filter clock period duration equals 2 Analog Comparator clock period."]
    FILTER_2CLK_PERIOD = 0x01,
    #[doc = "Filter clock period duration equals 4 Analog Comparator clock period."]
    FILTER_4CLK_PERIOD = 0x02,
    #[doc = "Filter clock period duration equals 8 Analog Comparator clock period."]
    FILTER_8CLK_PERIOD = 0x03,
    #[doc = "Filter clock period duration equals 16 Analog Comparator clock period."]
    FILTER_16CLK_PERIOD = 0x04,
    #[doc = "Filter clock period duration equals 32 Analog Comparator clock period."]
    FILTER_32CLK_PERIOD = 0x05,
    #[doc = "Filter clock period duration equals 64 Analog Comparator clock period."]
    FILTER_64CLK_PERIOD = 0x06,
    #[doc = "Filter clock period duration equals 128 Analog Comparator clock period."]
    FILTER_128CLK_PERIOD = 0x07,
}
impl FiltercgfClkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltercgfClkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltercgfClkdiv {
    #[inline(always)]
    fn from(val: u8) -> FiltercgfClkdiv {
        FiltercgfClkdiv::from_bits(val)
    }
}
impl From<FiltercgfClkdiv> for u8 {
    #[inline(always)]
    fn from(val: FiltercgfClkdiv) -> u8 {
        FiltercgfClkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FiltercgfSamplemode {
    #[doc = "Bypass mode."]
    BYPASS = 0x0,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK = 0x01,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK = 0x02,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK = 0x03,
}
impl FiltercgfSamplemode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltercgfSamplemode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltercgfSamplemode {
    #[inline(always)]
    fn from(val: u8) -> FiltercgfSamplemode {
        FiltercgfSamplemode::from_bits(val)
    }
}
impl From<FiltercgfSamplemode> for u8 {
    #[inline(always)]
    fn from(val: FiltercgfSamplemode) -> u8 {
        FiltercgfSamplemode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsmmain {
    #[doc = "POWER UP : The IC is powering up."]
    FSMMAIN_POWERUP = 0x0,
    #[doc = "ACTIVE : Power up is completed. The IC is in normal functional operation mode."]
    FSMMAIN_ACTIVE = 0x01,
    #[doc = "POWER DOWN : the IC has entered POWER DOWN mode."]
    FSMMAIN_POWERDOWN = 0x02,
    #[doc = "DEEP SLEEP: The IC has entered DEEP SLEEP mode."]
    FSMMAIN_DEEPSLEEP = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "DEEP POWER DOWN : The IC entred DEEP POWER DOWN mode."]
    FSMMAIN_DEEPPOWERDOWN = 0x06,
    #[doc = "IC Structural TEST Mode : The IC has entered in IC Test mode."]
    FSMMAIN_DFT_ACTIVE = 0x07,
}
impl Fsmmain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsmmain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsmmain {
    #[inline(always)]
    fn from(val: u8) -> Fsmmain {
        Fsmmain::from_bits(val)
    }
}
impl From<Fsmmain> for u8 {
    #[inline(always)]
    fn from(val: Fsmmain) -> u8 {
        Fsmmain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hwwkup {
    #[doc = "Analog References fast wake-up feature is disabled in case of Hardware Pin reset."]
    DISABLE = 0x0,
    #[doc = "Analog References fast wake-up feature is enabled in case of Hardware Pin reset."]
    ENABLE = 0x01,
}
impl Hwwkup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hwwkup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hwwkup {
    #[inline(always)]
    fn from(val: u8) -> Hwwkup {
        Hwwkup::from_bits(val)
    }
}
impl From<Hwwkup> for u8 {
    #[inline(always)]
    fn from(val: Hwwkup) -> u8 {
        Hwwkup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldodeepsleepref {
    #[doc = "LDO DEEP Sleep uses Flash buffer biasing as reference."]
    FLASHBUFFER = 0x0,
    #[doc = "LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    BGP0P8V = 0x01,
}
impl Ldodeepsleepref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldodeepsleepref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldodeepsleepref {
    #[inline(always)]
    fn from(val: u8) -> Ldodeepsleepref {
        Ldodeepsleepref::from_bits(val)
    }
}
impl From<Ldodeepsleepref> for u8 {
    #[inline(always)]
    fn from(val: Ldodeepsleepref) -> u8 {
        Ldodeepsleepref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldomemhighzmode {
    #[doc = "LDO MEM High Z mode is disabled."]
    DISABLE = 0x0,
    #[doc = "LDO MEM High Z mode is enabled."]
    ENABLE = 0x01,
}
impl Ldomemhighzmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldomemhighzmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldomemhighzmode {
    #[inline(always)]
    fn from(val: u8) -> Ldomemhighzmode {
        Ldomemhighzmode::from_bits(val)
    }
}
impl From<Ldomemhighzmode> for u8 {
    #[inline(always)]
    fn from(val: Ldomemhighzmode) -> u8 {
        Ldomemhighzmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lowpower {
    #[doc = "High speed mode."]
    HIGHSPEED = 0x0,
    #[doc = "Low power mode (Low speed)."]
    LOWSPEED = 0x01,
}
impl Lowpower {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lowpower {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lowpower {
    #[inline(always)]
    fn from(val: u8) -> Lowpower {
        Lowpower::from_bits(val)
    }
}
impl From<Lowpower> for u8 {
    #[inline(always)]
    fn from(val: Lowpower) -> u8 {
        Lowpower::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpwkup {
    #[doc = "Analog References fast wake-up feature is disabled in case of wake-up from any Low power mode."]
    DISABLE = 0x0,
    #[doc = "Analog References fast wake-up feature is enabled in case of wake-up from any Low power mode."]
    ENABLE = 0x01,
}
impl Lpwkup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpwkup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpwkup {
    #[inline(always)]
    fn from(val: u8) -> Lpwkup {
        Lpwkup::from_bits(val)
    }
}
impl From<Lpwkup> for u8 {
    #[inline(always)]
    fn from(val: Lpwkup) -> u8 {
        Lpwkup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modewakeupiopad0 {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Modewakeupiopad0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modewakeupiopad0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modewakeupiopad0 {
    #[inline(always)]
    fn from(val: u8) -> Modewakeupiopad0 {
        Modewakeupiopad0::from_bits(val)
    }
}
impl From<Modewakeupiopad0> for u8 {
    #[inline(always)]
    fn from(val: Modewakeupiopad0) -> u8 {
        Modewakeupiopad0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modewakeupiopad1 {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Modewakeupiopad1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modewakeupiopad1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modewakeupiopad1 {
    #[inline(always)]
    fn from(val: u8) -> Modewakeupiopad1 {
        Modewakeupiopad1::from_bits(val)
    }
}
impl From<Modewakeupiopad1> for u8 {
    #[inline(always)]
    fn from(val: Modewakeupiopad1) -> u8 {
        Modewakeupiopad1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modewakeupiopad2 {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Modewakeupiopad2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modewakeupiopad2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modewakeupiopad2 {
    #[inline(always)]
    fn from(val: u8) -> Modewakeupiopad2 {
        Modewakeupiopad2::from_bits(val)
    }
}
impl From<Modewakeupiopad2> for u8 {
    #[inline(always)]
    fn from(val: Modewakeupiopad2) -> u8 {
        Modewakeupiopad2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modewakeupiopad3 {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl Modewakeupiopad3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Modewakeupiopad3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Modewakeupiopad3 {
    #[inline(always)]
    fn from(val: u8) -> Modewakeupiopad3 {
        Modewakeupiopad3::from_bits(val)
    }
}
impl From<Modewakeupiopad3> for u8 {
    #[inline(always)]
    fn from(val: Modewakeupiopad3) -> u8 {
        Modewakeupiopad3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nmux {
    #[doc = "VREF (See field VREFINPUT)."]
    VREF = 0x0,
    #[doc = "Pin P0_0."]
    CMP0_A = 0x01,
    #[doc = "Pin P0_9."]
    CMP0_B = 0x02,
    #[doc = "Pin P0_18."]
    CMP0_C = 0x03,
    #[doc = "Pin P1_14."]
    CMP0_D = 0x04,
    #[doc = "Pin P2_23."]
    CMP0_E = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nmux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nmux {
    #[inline(always)]
    fn from(val: u8) -> Nmux {
        Nmux::from_bits(val)
    }
}
impl From<Nmux> for u8 {
    #[inline(always)]
    fn from(val: Nmux) -> u8 {
        Nmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostimerclksel {
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0X0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X1 = 0x01,
    #[doc = "Main clock for OS timer."]
    ENUM_0X2 = 0x02,
    #[doc = "No clock."]
    ENUM_0X3 = 0x03,
}
impl Ostimerclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimerclksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimerclksel {
    #[inline(always)]
    fn from(val: u8) -> Ostimerclksel {
        Ostimerclksel::from_bits(val)
    }
}
impl From<Ostimerclksel> for u8 {
    #[inline(always)]
    fn from(val: Ostimerclksel) -> u8 {
        Ostimerclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenAuxbias {
    #[doc = "auxiliary biasing is powered."]
    POWEREDON = 0x0,
    #[doc = "auxiliary biasing is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenAuxbias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenAuxbias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenAuxbias {
    #[inline(always)]
    fn from(val: u8) -> PdenAuxbias {
        PdenAuxbias::from_bits(val)
    }
}
impl From<PdenAuxbias> for u8 {
    #[inline(always)]
    fn from(val: PdenAuxbias) -> u8 {
        PdenAuxbias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenBodvbat {
    #[doc = "BOD VBAT is powered."]
    POWEREDON = 0x0,
    #[doc = "BOD VBAT is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenBodvbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenBodvbat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenBodvbat {
    #[inline(always)]
    fn from(val: u8) -> PdenBodvbat {
        PdenBodvbat::from_bits(val)
    }
}
impl From<PdenBodvbat> for u8 {
    #[inline(always)]
    fn from(val: PdenBodvbat) -> u8 {
        PdenBodvbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenComp {
    #[doc = "Analog Comparator is powered."]
    POWEREDON = 0x0,
    #[doc = "Analog Comparator is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenComp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenComp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenComp {
    #[inline(always)]
    fn from(val: u8) -> PdenComp {
        PdenComp::from_bits(val)
    }
}
impl From<PdenComp> for u8 {
    #[inline(always)]
    fn from(val: PdenComp) -> u8 {
        PdenComp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenFro32k {
    #[doc = "FRO32KHz is powered."]
    POWEREDON = 0x0,
    #[doc = "FRO32KHz is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenFro32k {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenFro32k {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenFro32k {
    #[inline(always)]
    fn from(val: u8) -> PdenFro32k {
        PdenFro32k::from_bits(val)
    }
}
impl From<PdenFro32k> for u8 {
    #[inline(always)]
    fn from(val: PdenFro32k) -> u8 {
        PdenFro32k::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenLdousbhs {
    #[doc = "USB high speed LDO is powered."]
    POWEREDON = 0x0,
    #[doc = "USB high speed LDO is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenLdousbhs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenLdousbhs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenLdousbhs {
    #[inline(always)]
    fn from(val: u8) -> PdenLdousbhs {
        PdenLdousbhs::from_bits(val)
    }
}
impl From<PdenLdousbhs> for u8 {
    #[inline(always)]
    fn from(val: PdenLdousbhs) -> u8 {
        PdenLdousbhs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenLdoxo32m {
    #[doc = "High speed crystal LDO is powered."]
    POWEREDON = 0x0,
    #[doc = "High speed crystal LDO is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenLdoxo32m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenLdoxo32m {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenLdoxo32m {
    #[inline(always)]
    fn from(val: u8) -> PdenLdoxo32m {
        PdenLdoxo32m::from_bits(val)
    }
}
impl From<PdenLdoxo32m> for u8 {
    #[inline(always)]
    fn from(val: PdenLdoxo32m) -> u8 {
        PdenLdoxo32m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenPll0 {
    #[doc = "PLL0 is powered."]
    POWEREDON = 0x0,
    #[doc = "PLL0 is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenPll0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenPll0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenPll0 {
    #[inline(always)]
    fn from(val: u8) -> PdenPll0 {
        PdenPll0::from_bits(val)
    }
}
impl From<PdenPll0> for u8 {
    #[inline(always)]
    fn from(val: PdenPll0) -> u8 {
        PdenPll0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenPll0Sscg {
    #[doc = "PLL0 Sread spectrum module is powered."]
    POWEREDON = 0x0,
    #[doc = "PLL0 Sread spectrum module is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenPll0Sscg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenPll0Sscg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenPll0Sscg {
    #[inline(always)]
    fn from(val: u8) -> PdenPll0Sscg {
        PdenPll0Sscg::from_bits(val)
    }
}
impl From<PdenPll0Sscg> for u8 {
    #[inline(always)]
    fn from(val: PdenPll0Sscg) -> u8 {
        PdenPll0Sscg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenPll1 {
    #[doc = "PLL1 is powered."]
    POWEREDON = 0x0,
    #[doc = "PLL1 is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenPll1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenPll1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenPll1 {
    #[inline(always)]
    fn from(val: u8) -> PdenPll1 {
        PdenPll1::from_bits(val)
    }
}
impl From<PdenPll1> for u8 {
    #[inline(always)]
    fn from(val: PdenPll1) -> u8 {
        PdenPll1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenRng {
    #[doc = "TRNG clocks are powered."]
    POWEREDON = 0x0,
    #[doc = "TRNG clocks are powered down."]
    POWEREDOFF = 0x01,
}
impl PdenRng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenRng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenRng {
    #[inline(always)]
    fn from(val: u8) -> PdenRng {
        PdenRng::from_bits(val)
    }
}
impl From<PdenRng> for u8 {
    #[inline(always)]
    fn from(val: PdenRng) -> u8 {
        PdenRng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenUsbfsphy {
    #[doc = "USB Full Speed phy is powered."]
    POWEREDON = 0x0,
    #[doc = "USB Full Speed phy is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenUsbfsphy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenUsbfsphy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenUsbfsphy {
    #[inline(always)]
    fn from(val: u8) -> PdenUsbfsphy {
        PdenUsbfsphy::from_bits(val)
    }
}
impl From<PdenUsbfsphy> for u8 {
    #[inline(always)]
    fn from(val: PdenUsbfsphy) -> u8 {
        PdenUsbfsphy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenUsbhsphy {
    #[doc = "USB HS phy is powered."]
    POWEREDON = 0x0,
    #[doc = "USB HS phy is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenUsbhsphy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenUsbhsphy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenUsbhsphy {
    #[inline(always)]
    fn from(val: u8) -> PdenUsbhsphy {
        PdenUsbhsphy::from_bits(val)
    }
}
impl From<PdenUsbhsphy> for u8 {
    #[inline(always)]
    fn from(val: PdenUsbhsphy) -> u8 {
        PdenUsbhsphy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenXtal32k {
    #[doc = "Crystal 32KHz is powered."]
    POWEREDON = 0x0,
    #[doc = "Crystal 32KHz is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenXtal32k {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenXtal32k {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenXtal32k {
    #[inline(always)]
    fn from(val: u8) -> PdenXtal32k {
        PdenXtal32k::from_bits(val)
    }
}
impl From<PdenXtal32k> for u8 {
    #[inline(always)]
    fn from(val: PdenXtal32k) -> u8 {
        PdenXtal32k::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdenXtal32m {
    #[doc = "High speed crystal is powered."]
    POWEREDON = 0x0,
    #[doc = "High speed crystal is powered down."]
    POWEREDOFF = 0x01,
}
impl PdenXtal32m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdenXtal32m {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdenXtal32m {
    #[inline(always)]
    fn from(val: u8) -> PdenXtal32m {
        PdenXtal32m::from_bits(val)
    }
}
impl From<PdenXtal32m> for u8 {
    #[inline(always)]
    fn from(val: PdenXtal32m) -> u8 {
        PdenXtal32m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmux {
    #[doc = "VREF (See fiedl VREFINPUT)."]
    VREF = 0x0,
    #[doc = "Pin P0_0."]
    CMP0_A = 0x01,
    #[doc = "Pin P0_9."]
    CMP0_B = 0x02,
    #[doc = "Pin P0_18."]
    CMP0_C = 0x03,
    #[doc = "Pin P1_14."]
    CMP0_D = 0x04,
    #[doc = "Pin P2_23."]
    CMP0_E = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmux {
    #[inline(always)]
    fn from(val: u8) -> Pmux {
        Pmux::from_bits(val)
    }
}
impl From<Pmux> for u8 {
    #[inline(always)]
    fn from(val: Pmux) -> u8 {
        Pmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Risingedgewakeup0 {
    #[doc = "Rising edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Rising edge detection is enable."]
    ENABLE = 0x01,
}
impl Risingedgewakeup0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Risingedgewakeup0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Risingedgewakeup0 {
    #[inline(always)]
    fn from(val: u8) -> Risingedgewakeup0 {
        Risingedgewakeup0::from_bits(val)
    }
}
impl From<Risingedgewakeup0> for u8 {
    #[inline(always)]
    fn from(val: Risingedgewakeup0) -> u8 {
        Risingedgewakeup0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Risingedgewakeup1 {
    #[doc = "Rising edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Rising edge detection is enable."]
    ENABLE = 0x01,
}
impl Risingedgewakeup1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Risingedgewakeup1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Risingedgewakeup1 {
    #[inline(always)]
    fn from(val: u8) -> Risingedgewakeup1 {
        Risingedgewakeup1::from_bits(val)
    }
}
impl From<Risingedgewakeup1> for u8 {
    #[inline(always)]
    fn from(val: Risingedgewakeup1) -> u8 {
        Risingedgewakeup1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Risingedgewakeup2 {
    #[doc = "Rising edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Rising edge detection is enable."]
    ENABLE = 0x01,
}
impl Risingedgewakeup2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Risingedgewakeup2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Risingedgewakeup2 {
    #[inline(always)]
    fn from(val: u8) -> Risingedgewakeup2 {
        Risingedgewakeup2::from_bits(val)
    }
}
impl From<Risingedgewakeup2> for u8 {
    #[inline(always)]
    fn from(val: Risingedgewakeup2) -> u8 {
        Risingedgewakeup2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Risingedgewakeup3 {
    #[doc = "Rising edge detection is disable."]
    DISABLE = 0x0,
    #[doc = "Rising edge detection is enable."]
    ENABLE = 0x01,
}
impl Risingedgewakeup3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Risingedgewakeup3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Risingedgewakeup3 {
    #[inline(always)]
    fn from(val: u8) -> Risingedgewakeup3 {
        Risingedgewakeup3::from_bits(val)
    }
}
impl From<Risingedgewakeup3> for u8 {
    #[inline(always)]
    fn from(val: Risingedgewakeup3) -> u8 {
        Risingedgewakeup3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sel {
    #[doc = "FRO 32 KHz."]
    FRO32K = 0x0,
    #[doc = "XTAL 32KHz."]
    XTAL32K = 0x01,
}
impl Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sel {
    #[inline(always)]
    fn from(val: u8) -> Sel {
        Sel::from_bits(val)
    }
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(val: Sel) -> u8 {
        Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smb {
    #[doc = "Low leakage."]
    LOW = 0x0,
    #[doc = "Medium leakage."]
    MEDIUM = 0x01,
    #[doc = "Highest leakage."]
    HIGHEST = 0x02,
    #[doc = "Disable."]
    DISABLE = 0x03,
}
impl Smb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smb {
    #[inline(always)]
    fn from(val: u8) -> Smb {
        Smb::from_bits(val)
    }
}
impl From<Smb> for u8 {
    #[inline(always)]
    fn from(val: Smb) -> u8 {
        Smb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swrresetenable {
    #[doc = "Software reset is disable."]
    DISABLE = 0x0,
    #[doc = "Software reset is enable."]
    ENABLE = 0x01,
}
impl Swrresetenable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swrresetenable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swrresetenable {
    #[inline(always)]
    fn from(val: u8) -> Swrresetenable {
        Swrresetenable::from_bits(val)
    }
}
impl From<Swrresetenable> for u8 {
    #[inline(always)]
    fn from(val: Swrresetenable) -> u8 {
        Swrresetenable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Triglvl {
    #[doc = "1.00 V."]
    V_1P00 = 0x0,
    #[doc = "1.10 V."]
    V_1P10 = 0x01,
    #[doc = "1.20 V."]
    V_1P20 = 0x02,
    #[doc = "1.30 V."]
    V_1P30 = 0x03,
    #[doc = "1.40 V."]
    V_1P40 = 0x04,
    #[doc = "1.50 V."]
    V_1P50 = 0x05,
    #[doc = "1.60 V."]
    V_1P60 = 0x06,
    #[doc = "1.65 V."]
    V_1P65 = 0x07,
    #[doc = "1.70 V."]
    V_1P70 = 0x08,
    #[doc = "1.75 V."]
    V_1P75 = 0x09,
    #[doc = "1.80 V."]
    V_1P80 = 0x0a,
    #[doc = "1.90 V."]
    V_1P90 = 0x0b,
    #[doc = "2.00 V."]
    V_2P00 = 0x0c,
    #[doc = "2.10 V."]
    V_2P10 = 0x0d,
    #[doc = "2.20 V."]
    V_2P20 = 0x0e,
    #[doc = "2.30 V."]
    V_2P30 = 0x0f,
    #[doc = "2.40 V."]
    V_2P40 = 0x10,
    #[doc = "2.50 V."]
    V_2P50 = 0x11,
    #[doc = "2.60 V."]
    V_2P60 = 0x12,
    #[doc = "2.70 V."]
    V_2P70 = 0x13,
    #[doc = "2.806 V."]
    V_2P80 = 0x14,
    #[doc = "2.90 V."]
    V_2P90 = 0x15,
    #[doc = "3.00 V."]
    V_3P00 = 0x16,
    #[doc = "3.10 V."]
    V_3P10 = 0x17,
    #[doc = "3.20 V."]
    V_3P20 = 0x18,
    #[doc = "3.30 V."]
    V_3P30_2 = 0x19,
    #[doc = "3.30 V."]
    V_3P30_3 = 0x1a,
    #[doc = "3.30 V."]
    V_3P30_4 = 0x1b,
    #[doc = "3.30 V."]
    V_3P30_5 = 0x1c,
    #[doc = "3.30 V."]
    V_3P30_6 = 0x1d,
    #[doc = "3.30 V."]
    V_3P30_7 = 0x1e,
    #[doc = "3.30 V."]
    V_3P30_8 = 0x1f,
}
impl Triglvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Triglvl {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Triglvl {
    #[inline(always)]
    fn from(val: u8) -> Triglvl {
        Triglvl::from_bits(val)
    }
}
impl From<Triglvl> for u8 {
    #[inline(always)]
    fn from(val: Triglvl) -> u8 {
        Triglvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vadj {
    #[doc = "1.22 V."]
    V_1P220 = 0x0,
    #[doc = "0.7 V."]
    V_0P700 = 0x01,
    #[doc = "0.725 V."]
    V_0P725 = 0x02,
    #[doc = "0.75 V."]
    V_0P750 = 0x03,
    #[doc = "0.775 V."]
    V_0P775 = 0x04,
    #[doc = "0.8 V."]
    V_0P800 = 0x05,
    #[doc = "0.825 V."]
    V_0P825 = 0x06,
    #[doc = "0.85 V."]
    V_0P850 = 0x07,
    #[doc = "0.875 V."]
    V_0P875 = 0x08,
    #[doc = "0.9 V."]
    V_0P900 = 0x09,
    #[doc = "0.96 V."]
    V_0P960 = 0x0a,
    #[doc = "0.97 V."]
    V_0P970 = 0x0b,
    #[doc = "0.98 V."]
    V_0P980 = 0x0c,
    #[doc = "0.99 V."]
    V_0P990 = 0x0d,
    #[doc = "1 V."]
    V_1P000 = 0x0e,
    #[doc = "1.01 V."]
    V_1P010 = 0x0f,
    #[doc = "1.02 V."]
    V_1P020 = 0x10,
    #[doc = "1.03 V."]
    V_1P030 = 0x11,
    #[doc = "1.04 V."]
    V_1P040 = 0x12,
    #[doc = "1.05 V."]
    V_1P050 = 0x13,
    #[doc = "1.06 V."]
    V_1P060 = 0x14,
    #[doc = "1.07 V."]
    V_1P070 = 0x15,
    #[doc = "1.08 V."]
    V_1P080 = 0x16,
    #[doc = "1.09 V."]
    V_1P090 = 0x17,
    #[doc = "1.1 V."]
    V_1P100 = 0x18,
    #[doc = "1.11 V."]
    V_1P110 = 0x19,
    #[doc = "1.12 V."]
    V_1P120 = 0x1a,
    #[doc = "1.13 V."]
    V_1P130 = 0x1b,
    #[doc = "1.14 V."]
    V_1P140 = 0x1c,
    #[doc = "1.15 V."]
    V_1P150 = 0x1d,
    #[doc = "1.16 V."]
    V_1P160 = 0x1e,
    #[doc = "1.22 V."]
    V_1P220_1 = 0x1f,
}
impl Vadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vadj {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vadj {
    #[inline(always)]
    fn from(val: u8) -> Vadj {
        Vadj::from_bits(val)
    }
}
impl From<Vadj> for u8 {
    #[inline(always)]
    fn from(val: Vadj) -> u8 {
        Vadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vout {
    #[doc = "0.95 V."]
    V_DCDC_0P950 = 0x0,
    #[doc = "0.975 V."]
    V_DCDC_0P975 = 0x01,
    #[doc = "1 V."]
    V_DCDC_1P000 = 0x02,
    #[doc = "1.025 V."]
    V_DCDC_1P025 = 0x03,
    #[doc = "1.05 V."]
    V_DCDC_1P050 = 0x04,
    #[doc = "1.075 V."]
    V_DCDC_1P075 = 0x05,
    #[doc = "1.1 V."]
    V_DCDC_1P100 = 0x06,
    #[doc = "1.125 V."]
    V_DCDC_1P125 = 0x07,
    #[doc = "1.15 V."]
    V_DCDC_1P150 = 0x08,
    #[doc = "1.175 V."]
    V_DCDC_1P175 = 0x09,
    #[doc = "1.2 V."]
    V_DCDC_1P200 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Vout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vout {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vout {
    #[inline(always)]
    fn from(val: u8) -> Vout {
        Vout::from_bits(val)
    }
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(val: Vout) -> u8 {
        Vout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrefinput {
    #[doc = "Select internal VREF."]
    INTERNALREF = 0x0,
    #[doc = "Select VDDA."]
    VDDA = 0x01,
}
impl Vrefinput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrefinput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrefinput {
    #[inline(always)]
    fn from(val: u8) -> Vrefinput {
        Vrefinput::from_bits(val)
    }
}
impl From<Vrefinput> for u8 {
    #[inline(always)]
    fn from(val: Vrefinput) -> u8 {
        Vrefinput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakeup0 {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 0."]
    NOEVENT = 0x0,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 0."]
    EVENT = 0x01,
}
impl Wakeup0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeup0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeup0 {
    #[inline(always)]
    fn from(val: u8) -> Wakeup0 {
        Wakeup0::from_bits(val)
    }
}
impl From<Wakeup0> for u8 {
    #[inline(always)]
    fn from(val: Wakeup0) -> u8 {
        Wakeup0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakeup1 {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    NOEVENT = 0x0,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    EVENT = 0x01,
}
impl Wakeup1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeup1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeup1 {
    #[inline(always)]
    fn from(val: u8) -> Wakeup1 {
        Wakeup1::from_bits(val)
    }
}
impl From<Wakeup1> for u8 {
    #[inline(always)]
    fn from(val: Wakeup1) -> u8 {
        Wakeup1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakeup2 {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    NOEVENT = 0x0,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    EVENT = 0x01,
}
impl Wakeup2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeup2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeup2 {
    #[inline(always)]
    fn from(val: u8) -> Wakeup2 {
        Wakeup2::from_bits(val)
    }
}
impl From<Wakeup2> for u8 {
    #[inline(always)]
    fn from(val: Wakeup2) -> u8 {
        Wakeup2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakeup3 {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    NOEVENT = 0x0,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    EVENT = 0x01,
}
impl Wakeup3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeup3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeup3 {
    #[inline(always)]
    fn from(val: u8) -> Wakeup3 {
        Wakeup3::from_bits(val)
    }
}
impl From<Wakeup3> for u8 {
    #[inline(always)]
    fn from(val: Wakeup3) -> u8 {
        Wakeup3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeupioEnableCtrl {
    #[doc = "WAKEUP IO PAD mode control comes from IOCON."]
    DISABLE = 0x0,
    #[doc = "WAKEUP IO PAD mode control comes from MODEWAKEUPIOPAD (bits 12 to 19)."]
    ENABLE = 0x01,
}
impl WakeupioEnableCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeupioEnableCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeupioEnableCtrl {
    #[inline(always)]
    fn from(val: u8) -> WakeupioEnableCtrl {
        WakeupioEnableCtrl::from_bits(val)
    }
}
impl From<WakeupioEnableCtrl> for u8 {
    #[inline(always)]
    fn from(val: WakeupioEnableCtrl) -> u8 {
        WakeupioEnableCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeupioRstn {
    #[doc = "Bloc is reset."]
    ASSERTED = 0x0,
    #[doc = "Bloc is not reset."]
    RELEASED = 0x01,
}
impl WakeupioRstn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeupioRstn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeupioRstn {
    #[inline(always)]
    fn from(val: u8) -> WakeupioRstn {
        WakeupioRstn::from_bits(val)
    }
}
impl From<WakeupioRstn> for u8 {
    #[inline(always)]
    fn from(val: WakeupioRstn) -> u8 {
        WakeupioRstn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtal32koscfailure {
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared."]
    NOFAIL = 0x0,
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared."]
    FAILURE = 0x01,
}
impl Xtal32koscfailure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtal32koscfailure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtal32koscfailure {
    #[inline(always)]
    fn from(val: u8) -> Xtal32koscfailure {
        Xtal32koscfailure::from_bits(val)
    }
}
impl From<Xtal32koscfailure> for u8 {
    #[inline(always)]
    fn from(val: Xtal32koscfailure) -> u8 {
        Xtal32koscfailure::to_bits(val)
    }
}
