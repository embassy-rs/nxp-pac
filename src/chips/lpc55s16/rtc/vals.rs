#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alarm1hz {
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH = 0x0,
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    MATCH = 0x01,
}
impl Alarm1hz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alarm1hz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alarm1hz {
    #[inline(always)]
    fn from(val: u8) -> Alarm1hz {
        Alarm1hz::from_bits(val)
    }
}
impl From<Alarm1hz> for u8 {
    #[inline(always)]
    fn from(val: Alarm1hz) -> u8 {
        Alarm1hz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlarmdpdEn {
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE = 0x0,
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    ENABLE = 0x01,
}
impl AlarmdpdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlarmdpdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlarmdpdEn {
    #[inline(always)]
    fn from(val: u8) -> AlarmdpdEn {
        AlarmdpdEn::from_bits(val)
    }
}
impl From<AlarmdpdEn> for u8 {
    #[inline(always)]
    fn from(val: AlarmdpdEn) -> u8 {
        AlarmdpdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc1khzEn {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE = 0x0,
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    ENABLE = 0x01,
}
impl Rtc1khzEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc1khzEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc1khzEn {
    #[inline(always)]
    fn from(val: u8) -> Rtc1khzEn {
        Rtc1khzEn::from_bits(val)
    }
}
impl From<Rtc1khzEn> for u8 {
    #[inline(always)]
    fn from(val: Rtc1khzEn) -> u8 {
        Rtc1khzEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcEn {
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    DISABLE = 0x0,
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    ENABLE = 0x01,
}
impl RtcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcEn {
    #[inline(always)]
    fn from(val: u8) -> RtcEn {
        RtcEn::from_bits(val)
    }
}
impl From<RtcEn> for u8 {
    #[inline(always)]
    fn from(val: RtcEn) -> u8 {
        RtcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcOscBypass {
    #[doc = "The RTC Oscillator operates normally as a crystal oscillator with the crystal connected between the RTC_XTALIN and RTC_XTALOUT pins."]
    USED = 0x0,
    #[doc = "The RTC Oscillator is in bypass mode. In this mode a clock can be directly input into the RTC_XTALIN pin."]
    BYPASS = 0x01,
}
impl RtcOscBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcOscBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcOscBypass {
    #[inline(always)]
    fn from(val: u8) -> RtcOscBypass {
        RtcOscBypass::from_bits(val)
    }
}
impl From<RtcOscBypass> for u8 {
    #[inline(always)]
    fn from(val: RtcOscBypass) -> u8 {
        RtcOscBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcOscPd {
    #[doc = "See RTC_OSC_BYPASS"]
    POWER_UP = 0x0,
    #[doc = "RTC oscillator is powered-down."]
    POWERED_DOWN = 0x01,
}
impl RtcOscPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcOscPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcOscPd {
    #[inline(always)]
    fn from(val: u8) -> RtcOscPd {
        RtcOscPd::from_bits(val)
    }
}
impl From<RtcOscPd> for u8 {
    #[inline(always)]
    fn from(val: RtcOscPd) -> u8 {
        RtcOscPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcSubsecEna {
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'."]
    POWER_UP = 0x0,
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting commences on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) is set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep power-down mode."]
    POWERED_DOWN = 0x01,
}
impl RtcSubsecEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcSubsecEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcSubsecEna {
    #[inline(always)]
    fn from(val: u8) -> RtcSubsecEna {
        RtcSubsecEna::from_bits(val)
    }
}
impl From<RtcSubsecEna> for u8 {
    #[inline(always)]
    fn from(val: RtcSubsecEna) -> u8 {
        RtcSubsecEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swreset {
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NOT_IN_RESET = 0x0,
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    IN_RESET = 0x01,
}
impl Swreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swreset {
    #[inline(always)]
    fn from(val: u8) -> Swreset {
        Swreset::from_bits(val)
    }
}
impl From<Swreset> for u8 {
    #[inline(always)]
    fn from(val: Swreset) -> u8 {
        Swreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wake1khz {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN = 0x0,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIMEOUT = 0x01,
}
impl Wake1khz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wake1khz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wake1khz {
    #[inline(always)]
    fn from(val: u8) -> Wake1khz {
        Wake1khz::from_bits(val)
    }
}
impl From<Wake1khz> for u8 {
    #[inline(always)]
    fn from(val: Wake1khz) -> u8 {
        Wake1khz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakedpdEn {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE = 0x0,
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    ENABLE = 0x01,
}
impl WakedpdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakedpdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakedpdEn {
    #[inline(always)]
    fn from(val: u8) -> WakedpdEn {
        WakedpdEn::from_bits(val)
    }
}
impl From<WakedpdEn> for u8 {
    #[inline(always)]
    fn from(val: WakedpdEn) -> u8 {
        WakedpdEn::to_bits(val)
    }
}
