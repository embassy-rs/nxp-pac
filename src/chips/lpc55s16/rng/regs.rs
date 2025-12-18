#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CounterCfg(pub u32);
impl CounterCfg {
    #[doc = "00: disabled 01: update once."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "00: disabled 01: update once."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects the internal clock on which to compute statistics."]
    #[must_use]
    #[inline(always)]
    pub const fn clock_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub const fn set_clock_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[must_use]
    #[inline(always)]
    pub const fn shift4x(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub const fn set_shift4x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
}
impl Default for CounterCfg {
    #[inline(always)]
    fn default() -> CounterCfg {
        CounterCfg(0)
    }
}
impl core::fmt::Debug for CounterCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CounterCfg")
            .field("mode", &self.mode())
            .field("clock_sel", &self.clock_sel())
            .field("shift4x", &self.shift4x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CounterCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CounterCfg {{ mode: {=u8:?}, clock_sel: {=u8:?}, shift4x: {=u8:?} }}",
            self.mode(),
            self.clock_sel(),
            self.shift4x()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CounterVal(pub u32);
impl CounterVal {
    #[doc = "Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_ratio(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[inline(always)]
    pub const fn set_clk_ratio(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[must_use]
    #[inline(always)]
    pub const fn refresh_cnt(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x3fff;
        val as u16
    }
    #[doc = "Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[inline(always)]
    pub const fn set_refresh_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
    }
}
impl Default for CounterVal {
    #[inline(always)]
    fn default() -> CounterVal {
        CounterVal(0)
    }
}
impl core::fmt::Debug for CounterVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CounterVal")
            .field("clk_ratio", &self.clk_ratio())
            .field("refresh_cnt", &self.refresh_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CounterVal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CounterVal {{ clk_ratio: {=u8:?}, refresh_cnt: {=u16:?} }}",
            self.clk_ratio(),
            self.refresh_cnt()
        )
    }
}
#[doc = "This register contains a random 32 bit number which is pre-computed"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EncryptedNumber(pub u32);
impl EncryptedNumber {
    #[doc = "This register contains a random 32 bit number which is pre-computed."]
    #[must_use]
    #[inline(always)]
    pub const fn encrypted_number(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub const fn set_encrypted_number(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EncryptedNumber {
    #[inline(always)]
    fn default() -> EncryptedNumber {
        EncryptedNumber(0)
    }
}
impl core::fmt::Debug for EncryptedNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EncryptedNumber")
            .field("encrypted_number", &self.encrypted_number())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EncryptedNumber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EncryptedNumber {{ encrypted_number: {=u32:?} }}",
            self.encrypted_number()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EntropyInject(pub u32);
impl EntropyInject {
    #[doc = "Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[must_use]
    #[inline(always)]
    pub const fn entropy(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[inline(always)]
    pub const fn set_entropy(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EntropyInject {
    #[inline(always)]
    fn default() -> EntropyInject {
        EntropyInject(0)
    }
}
impl core::fmt::Debug for EntropyInject {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EntropyInject")
            .field("entropy", &self.entropy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EntropyInject {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EntropyInject {{ entropy: {=u32:?} }}", self.entropy())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCfg(pub u32);
impl MiscCfg {
    #[doc = "If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_reseed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub const fn set_aes_reseed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set this bit to re-seed AES."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_dt_cfg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to re-seed AES."]
    #[inline(always)]
    pub const fn set_aes_dt_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MiscCfg {
    #[inline(always)]
    fn default() -> MiscCfg {
        MiscCfg(0)
    }
}
impl core::fmt::Debug for MiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCfg")
            .field("aes_reseed", &self.aes_reseed())
            .field("aes_dt_cfg", &self.aes_dt_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCfg {{ aes_reseed: {=bool:?}, aes_dt_cfg: {=bool:?} }}",
            self.aes_reseed(),
            self.aes_dt_cfg()
        )
    }
}
#[doc = "IP identifier"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moduleid(pub u32);
impl Moduleid {
    #[doc = "Aperture i."]
    #[must_use]
    #[inline(always)]
    pub const fn aperture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture i."]
    #[inline(always)]
    pub const fn set_aperture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn min_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision i."]
    #[inline(always)]
    pub const fn set_min_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn maj_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision i."]
    #[inline(always)]
    pub const fn set_maj_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Identifier."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Moduleid {
    #[inline(always)]
    fn default() -> Moduleid {
        Moduleid(0)
    }
}
impl core::fmt::Debug for Moduleid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moduleid")
            .field("aperture", &self.aperture())
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moduleid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Moduleid {{ aperture: {=u8:?}, min_rev: {=u8:?}, maj_rev: {=u8:?}, id: {=u16:?} }}",
            self.aperture(),
            self.min_rev(),
            self.maj_rev(),
            self.id()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OnlineTestCfg(pub u32);
impl OnlineTestCfg {
    #[doc = "0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[must_use]
    #[inline(always)]
    pub const fn activate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub const fn set_activate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[must_use]
    #[inline(always)]
    pub const fn data_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub const fn set_data_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
}
impl Default for OnlineTestCfg {
    #[inline(always)]
    fn default() -> OnlineTestCfg {
        OnlineTestCfg(0)
    }
}
impl core::fmt::Debug for OnlineTestCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OnlineTestCfg")
            .field("activate", &self.activate())
            .field("data_sel", &self.data_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OnlineTestCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OnlineTestCfg {{ activate: {=bool:?}, data_sel: {=u8:?} }}",
            self.activate(),
            self.data_sel()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OnlineTestVal(pub u32);
impl OnlineTestVal {
    #[doc = "This value is updated as described in field 'activate'."]
    #[must_use]
    #[inline(always)]
    pub const fn live_chi_squared(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "This value is updated as described in field 'activate'."]
    #[inline(always)]
    pub const fn set_live_chi_squared(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[must_use]
    #[inline(always)]
    pub const fn min_chi_squared(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[inline(always)]
    pub const fn set_min_chi_squared(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[must_use]
    #[inline(always)]
    pub const fn max_chi_squared(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[inline(always)]
    pub const fn set_max_chi_squared(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for OnlineTestVal {
    #[inline(always)]
    fn default() -> OnlineTestVal {
        OnlineTestVal(0)
    }
}
impl core::fmt::Debug for OnlineTestVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OnlineTestVal")
            .field("live_chi_squared", &self.live_chi_squared())
            .field("min_chi_squared", &self.min_chi_squared())
            .field("max_chi_squared", &self.max_chi_squared())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OnlineTestVal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OnlineTestVal {{ live_chi_squared: {=u8:?}, min_chi_squared: {=u8:?}, max_chi_squared: {=u8:?} }}",
            self.live_chi_squared(),
            self.min_chi_squared(),
            self.max_chi_squared()
        )
    }
}
#[doc = "Powerdown mode (standard but certainly useless here)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerdown(pub u32);
impl Powerdown {
    #[doc = "Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[must_use]
    #[inline(always)]
    pub const fn soft_reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub const fn set_soft_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[must_use]
    #[inline(always)]
    pub const fn force_soft_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub const fn set_force_soft_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When set all accesses to standard registers are blocked."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Powerdown {
    #[inline(always)]
    fn default() -> Powerdown {
        Powerdown(0)
    }
}
impl core::fmt::Debug for Powerdown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Powerdown")
            .field("soft_reset", &self.soft_reset())
            .field("force_soft_reset", &self.force_soft_reset())
            .field("powerdown", &self.powerdown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Powerdown {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Powerdown {{ soft_reset: {=bool:?}, force_soft_reset: {=bool:?}, powerdown: {=bool:?} }}",
            self.soft_reset(),
            self.force_soft_reset(),
            self.powerdown()
        )
    }
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandomNumber(pub u32);
impl RandomNumber {
    #[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[must_use]
    #[inline(always)]
    pub const fn random_number(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline(always)]
    pub const fn set_random_number(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandomNumber {
    #[inline(always)]
    fn default() -> RandomNumber {
        RandomNumber(0)
    }
}
impl core::fmt::Debug for RandomNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandomNumber")
            .field("random_number", &self.random_number())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandomNumber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RandomNumber {{ random_number: {=u32:?} }}",
            self.random_number()
        )
    }
}
