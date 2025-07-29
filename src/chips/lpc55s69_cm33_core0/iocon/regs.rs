#[doc = "Digital I/O control for port 0 pins PIO0_0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio00(pub u32);
impl Pio00 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio00Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio00Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio00Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio00Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio00Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio00Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio00Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio00Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio00Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio00Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio00Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio00Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio00Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio00Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio00Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio00Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio00Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio00Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio00 {
    #[inline(always)]
    fn default() -> Pio00 {
        Pio00(0)
    }
}
impl core::fmt::Debug for Pio00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio00")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio00 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio01(pub u32);
impl Pio01 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio01Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio01Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio01Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio01Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio01Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio01Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio01Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio01Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio01Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio01Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio01Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio01Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio01Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio01Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio01Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio01 {
    #[inline(always)]
    fn default() -> Pio01 {
        Pio01(0)
    }
}
impl core::fmt::Debug for Pio01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio01")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio01 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio010(pub u32);
impl Pio010 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio010Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio010Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio010Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio010Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio010Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio010Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio010Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio010Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio010Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio010Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio010Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio010Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio010Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio010Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio010Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio010Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio010Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio010Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio010 {
    #[inline(always)]
    fn default() -> Pio010 {
        Pio010(0)
    }
}
impl core::fmt::Debug for Pio010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio010")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio010 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio010 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio011(pub u32);
impl Pio011 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio011Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio011Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio011Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio011Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio011Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio011Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio011Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio011Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio011Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio011Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio011Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio011Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio011Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio011Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio011Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio011Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio011Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio011Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio011 {
    #[inline(always)]
    fn default() -> Pio011 {
        Pio011(0)
    }
}
impl core::fmt::Debug for Pio011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio011")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio011 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio011 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio012(pub u32);
impl Pio012 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio012Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio012Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio012Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio012Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio012Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio012Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio012Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio012Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio012Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio012Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio012Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio012Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio012Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio012Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio012Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio012Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio012Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio012Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio012 {
    #[inline(always)]
    fn default() -> Pio012 {
        Pio012(0)
    }
}
impl core::fmt::Debug for Pio012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio012")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio012 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio012 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio013(pub u32);
impl Pio013 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio013Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio013Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio013Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio013Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio013Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio013Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio013Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio013Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio013Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio013Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio013Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio013Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio013Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio013Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio013Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ssel(&self) -> super::vals::Pio013Ssel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pio013Ssel::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_ssel(&mut self, val: super::vals::Pio013Ssel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filteroff(&self) -> super::vals::Pio013Filteroff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pio013Filteroff::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_filteroff(&mut self, val: super::vals::Pio013Filteroff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ecs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn egp(&self) -> super::vals::Pio013Egp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pio013Egp::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_egp(&mut self, val: super::vals::Pio013Egp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cfilter(&self) -> super::vals::Pio013I2cfilter {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pio013I2cfilter::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_i2cfilter(&mut self, val: super::vals::Pio013I2cfilter) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pio013 {
    #[inline(always)]
    fn default() -> Pio013 {
        Pio013(0)
    }
}
impl core::fmt::Debug for Pio013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio013")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("ssel", &self.ssel())
            .field("filteroff", &self.filteroff())
            .field("ecs", &self.ecs())
            .field("egp", &self.egp())
            .field("i2cfilter", &self.i2cfilter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio013 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio013 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, ssel: {:?}, filteroff: {:?}, ecs: {=bool:?}, egp: {:?}, i2cfilter: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.ssel(),
            self.filteroff(),
            self.ecs(),
            self.egp(),
            self.i2cfilter()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio014(pub u32);
impl Pio014 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio014Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio014Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio014Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio014Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio014Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio014Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio014Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio014Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio014Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio014Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio014Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio014Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio014Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio014Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio014Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ssel(&self) -> super::vals::Pio014Ssel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pio014Ssel::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_ssel(&mut self, val: super::vals::Pio014Ssel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filteroff(&self) -> super::vals::Pio014Filteroff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pio014Filteroff::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_filteroff(&mut self, val: super::vals::Pio014Filteroff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ecs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn egp(&self) -> super::vals::Pio014Egp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pio014Egp::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_egp(&mut self, val: super::vals::Pio014Egp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cfilter(&self) -> super::vals::Pio014I2cfilter {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pio014I2cfilter::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_i2cfilter(&mut self, val: super::vals::Pio014I2cfilter) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pio014 {
    #[inline(always)]
    fn default() -> Pio014 {
        Pio014(0)
    }
}
impl core::fmt::Debug for Pio014 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio014")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("ssel", &self.ssel())
            .field("filteroff", &self.filteroff())
            .field("ecs", &self.ecs())
            .field("egp", &self.egp())
            .field("i2cfilter", &self.i2cfilter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio014 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio014 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, ssel: {:?}, filteroff: {:?}, ecs: {=bool:?}, egp: {:?}, i2cfilter: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.ssel(),
            self.filteroff(),
            self.ecs(),
            self.egp(),
            self.i2cfilter()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio015(pub u32);
impl Pio015 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio015Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio015Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio015Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio015Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio015Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio015Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio015Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio015Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio015Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio015Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio015Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio015Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio015Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio015Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio015Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio015Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio015Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio015Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio015 {
    #[inline(always)]
    fn default() -> Pio015 {
        Pio015(0)
    }
}
impl core::fmt::Debug for Pio015 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio015")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio015 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio015 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio016(pub u32);
impl Pio016 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio016Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio016Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio016Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio016Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio016Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio016Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio016Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio016Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio016Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio016Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio016Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio016Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio016Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio016Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio016Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio016Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio016Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio016Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio016 {
    #[inline(always)]
    fn default() -> Pio016 {
        Pio016(0)
    }
}
impl core::fmt::Debug for Pio016 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio016")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio016 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio016 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio017(pub u32);
impl Pio017 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio017Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio017Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio017Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio017Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio017Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio017Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio017Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio017Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio017Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio017Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio017Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio017Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio017Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio017Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio017Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio017 {
    #[inline(always)]
    fn default() -> Pio017 {
        Pio017(0)
    }
}
impl core::fmt::Debug for Pio017 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio017")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio017 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio017 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio018(pub u32);
impl Pio018 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio018Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio018Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio018Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio018Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio018Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio018Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio018Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio018Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio018Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio018Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio018Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio018Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio018Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio018Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio018Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio018Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio018Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio018Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio018 {
    #[inline(always)]
    fn default() -> Pio018 {
        Pio018(0)
    }
}
impl core::fmt::Debug for Pio018 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio018")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio018 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio018 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio019(pub u32);
impl Pio019 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio019Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio019Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio019Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio019Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio019Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio019Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio019Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio019Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio019Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio019Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio019Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio019Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio019Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio019Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio019Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio019 {
    #[inline(always)]
    fn default() -> Pio019 {
        Pio019(0)
    }
}
impl core::fmt::Debug for Pio019 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio019")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio019 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio019 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio02(pub u32);
impl Pio02 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio02Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio02Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio02Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio02Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio02Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio02Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio02Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio02Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio02Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio02Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio02Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio02Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio02Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio02Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio02Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio02 {
    #[inline(always)]
    fn default() -> Pio02 {
        Pio02(0)
    }
}
impl core::fmt::Debug for Pio02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio02")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio02 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio020(pub u32);
impl Pio020 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio020Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio020Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio020Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio020Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio020Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio020Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio020Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio020Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio020Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio020Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio020Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio020Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio020Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio020Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio020Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio020 {
    #[inline(always)]
    fn default() -> Pio020 {
        Pio020(0)
    }
}
impl core::fmt::Debug for Pio020 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio020")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio020 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio020 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio021(pub u32);
impl Pio021 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio021Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio021Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio021Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio021Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio021Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio021Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio021Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio021Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio021Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio021Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio021Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio021Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio021Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio021Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio021Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio021 {
    #[inline(always)]
    fn default() -> Pio021 {
        Pio021(0)
    }
}
impl core::fmt::Debug for Pio021 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio021")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio021 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio021 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio022(pub u32);
impl Pio022 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio022Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio022Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio022Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio022Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio022Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio022Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio022Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio022Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio022Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio022Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio022Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio022Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio022Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio022Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio022Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio022 {
    #[inline(always)]
    fn default() -> Pio022 {
        Pio022(0)
    }
}
impl core::fmt::Debug for Pio022 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio022")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio022 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio022 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio023(pub u32);
impl Pio023 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio023Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio023Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio023Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio023Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio023Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio023Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio023Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio023Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio023Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio023Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio023Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio023Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio023Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio023Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio023Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio023Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio023Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio023Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio023 {
    #[inline(always)]
    fn default() -> Pio023 {
        Pio023(0)
    }
}
impl core::fmt::Debug for Pio023 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio023")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio023 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio023 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio024(pub u32);
impl Pio024 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio024Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio024Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio024Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio024Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio024Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio024Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio024Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio024Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio024Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio024Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio024Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio024Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio024Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio024Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio024Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio024 {
    #[inline(always)]
    fn default() -> Pio024 {
        Pio024(0)
    }
}
impl core::fmt::Debug for Pio024 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio024")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio024 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio024 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio025(pub u32);
impl Pio025 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio025Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio025Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio025Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio025Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio025Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio025Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio025Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio025Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio025Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio025Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio025Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio025Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio025Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio025Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio025Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio025 {
    #[inline(always)]
    fn default() -> Pio025 {
        Pio025(0)
    }
}
impl core::fmt::Debug for Pio025 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio025")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio025 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio025 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio026(pub u32);
impl Pio026 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio026Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio026Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio026Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio026Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio026Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio026Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio026Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio026Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio026Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio026Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio026Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio026Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio026Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio026Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio026Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio026 {
    #[inline(always)]
    fn default() -> Pio026 {
        Pio026(0)
    }
}
impl core::fmt::Debug for Pio026 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio026")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio026 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio026 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio027(pub u32);
impl Pio027 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio027Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio027Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio027Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio027Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio027Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio027Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio027Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio027Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio027Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio027Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio027Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio027Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio027Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio027Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio027Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio027 {
    #[inline(always)]
    fn default() -> Pio027 {
        Pio027(0)
    }
}
impl core::fmt::Debug for Pio027 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio027")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio027 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio027 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio028(pub u32);
impl Pio028 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio028Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio028Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio028Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio028Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio028Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio028Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio028Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio028Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio028Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio028Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio028Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio028Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio028Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio028Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio028Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio028 {
    #[inline(always)]
    fn default() -> Pio028 {
        Pio028(0)
    }
}
impl core::fmt::Debug for Pio028 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio028")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio028 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio028 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio029(pub u32);
impl Pio029 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio029Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio029Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio029Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio029Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio029Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio029Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio029Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio029Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio029Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio029Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio029Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio029Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio029Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio029Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio029Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio029 {
    #[inline(always)]
    fn default() -> Pio029 {
        Pio029(0)
    }
}
impl core::fmt::Debug for Pio029 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio029")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio029 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio029 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio03(pub u32);
impl Pio03 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio03Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio03Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio03Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio03Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio03Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio03Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio03Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio03Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio03Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio03Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio03Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio03Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio03Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio03Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio03Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio03 {
    #[inline(always)]
    fn default() -> Pio03 {
        Pio03(0)
    }
}
impl core::fmt::Debug for Pio03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio03")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio03 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio030(pub u32);
impl Pio030 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio030Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio030Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio030Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio030Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio030Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio030Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio030Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio030Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio030Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio030Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio030Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio030Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio030Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio030Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio030Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio030 {
    #[inline(always)]
    fn default() -> Pio030 {
        Pio030(0)
    }
}
impl core::fmt::Debug for Pio030 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio030")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio030 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio030 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio031(pub u32);
impl Pio031 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio031Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio031Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio031Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio031Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio031Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio031Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio031Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio031Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio031Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio031Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio031Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio031Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio031Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio031Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio031Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio031Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio031Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio031Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio031 {
    #[inline(always)]
    fn default() -> Pio031 {
        Pio031(0)
    }
}
impl core::fmt::Debug for Pio031 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio031")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio031 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio031 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio04(pub u32);
impl Pio04 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio04Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio04Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio04Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio04Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio04Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio04Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio04Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio04Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio04Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio04Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio04Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio04Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio04Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio04Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio04Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio04 {
    #[inline(always)]
    fn default() -> Pio04 {
        Pio04(0)
    }
}
impl core::fmt::Debug for Pio04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio04")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio04 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio04 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio05(pub u32);
impl Pio05 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio05Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio05Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio05Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio05Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio05Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio05Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio05Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio05Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio05Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio05Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio05Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio05Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio05Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio05Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio05Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio05 {
    #[inline(always)]
    fn default() -> Pio05 {
        Pio05(0)
    }
}
impl core::fmt::Debug for Pio05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio05")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio05 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio05 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio06(pub u32);
impl Pio06 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio06Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio06Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio06Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio06Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio06Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio06Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio06Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio06Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio06Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio06Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio06Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio06Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio06Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio06Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio06Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio06 {
    #[inline(always)]
    fn default() -> Pio06 {
        Pio06(0)
    }
}
impl core::fmt::Debug for Pio06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio06")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio06 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio06 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio07(pub u32);
impl Pio07 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio07Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio07Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio07Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio07Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio07Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio07Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio07Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio07Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio07Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio07Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio07Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio07Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio07Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio07Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio07Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio07 {
    #[inline(always)]
    fn default() -> Pio07 {
        Pio07(0)
    }
}
impl core::fmt::Debug for Pio07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio07")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio07 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio07 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio08(pub u32);
impl Pio08 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio08Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio08Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio08Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio08Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio08Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio08Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio08Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio08Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio08Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio08Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio08Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio08Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio08Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio08Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio08Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio08 {
    #[inline(always)]
    fn default() -> Pio08 {
        Pio08(0)
    }
}
impl core::fmt::Debug for Pio08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio08")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio08 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio08 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio09(pub u32);
impl Pio09 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio09Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio09Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio09Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio09Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio09Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio09Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio09Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio09Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio09Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio09Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio09Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio09Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio09Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio09Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio09Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio09Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio09Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio09Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio09 {
    #[inline(always)]
    fn default() -> Pio09 {
        Pio09(0)
    }
}
impl core::fmt::Debug for Pio09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio09")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio09 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio09 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio10(pub u32);
impl Pio10 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio10Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio10Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio10Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio10Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio10Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio10Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio10Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio10Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio10Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio10Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio10Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio10Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio10Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio10Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio10Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio10Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio10Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio10Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio10 {
    #[inline(always)]
    fn default() -> Pio10 {
        Pio10(0)
    }
}
impl core::fmt::Debug for Pio10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio10")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio10 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio11(pub u32);
impl Pio11 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio11Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio11Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio11Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio11Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio11Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio11Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio11Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio11Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio11Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio11Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio11Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio11Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio11Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio11Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio11Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio11 {
    #[inline(always)]
    fn default() -> Pio11 {
        Pio11(0)
    }
}
impl core::fmt::Debug for Pio11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio11")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio11 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio110(pub u32);
impl Pio110 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio110Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio110Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio110Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio110Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio110Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio110Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio110Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio110Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio110Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio110Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio110Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio110Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio110Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio110Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio110Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio110 {
    #[inline(always)]
    fn default() -> Pio110 {
        Pio110(0)
    }
}
impl core::fmt::Debug for Pio110 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio110")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio110 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio110 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio111(pub u32);
impl Pio111 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio111Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio111Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio111Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio111Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio111Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio111Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio111Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio111Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio111Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio111Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio111Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio111Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio111Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio111Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio111Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio111 {
    #[inline(always)]
    fn default() -> Pio111 {
        Pio111(0)
    }
}
impl core::fmt::Debug for Pio111 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio111")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio111 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio111 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio112(pub u32);
impl Pio112 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio112Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio112Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio112Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio112Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio112Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio112Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio112Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio112Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio112Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio112Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio112Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio112Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio112Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio112Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio112Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio112 {
    #[inline(always)]
    fn default() -> Pio112 {
        Pio112(0)
    }
}
impl core::fmt::Debug for Pio112 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio112")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio112 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio112 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio113(pub u32);
impl Pio113 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio113Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio113Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio113Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio113Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio113Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio113Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio113Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio113Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio113Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio113Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio113Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio113Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio113Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio113Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio113Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio113 {
    #[inline(always)]
    fn default() -> Pio113 {
        Pio113(0)
    }
}
impl core::fmt::Debug for Pio113 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio113")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio113 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio113 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio114(pub u32);
impl Pio114 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio114Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio114Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio114Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio114Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio114Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio114Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio114Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio114Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio114Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio114Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio114Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio114Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio114Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio114Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio114Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio114Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio114Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio114Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio114 {
    #[inline(always)]
    fn default() -> Pio114 {
        Pio114(0)
    }
}
impl core::fmt::Debug for Pio114 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio114")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio114 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio114 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio115(pub u32);
impl Pio115 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio115Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio115Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio115Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio115Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio115Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio115Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio115Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio115Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio115Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio115Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio115Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio115Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio115Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio115Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio115Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio115 {
    #[inline(always)]
    fn default() -> Pio115 {
        Pio115(0)
    }
}
impl core::fmt::Debug for Pio115 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio115")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio115 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio115 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio116(pub u32);
impl Pio116 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio116Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio116Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio116Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio116Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio116Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio116Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio116Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio116Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio116Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio116Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio116Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio116Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio116Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio116Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio116Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio116 {
    #[inline(always)]
    fn default() -> Pio116 {
        Pio116(0)
    }
}
impl core::fmt::Debug for Pio116 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio116")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio116 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio116 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio117(pub u32);
impl Pio117 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio117Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio117Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio117Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio117Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio117Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio117Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio117Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio117Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio117Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio117Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio117Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio117Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio117Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio117Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio117Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio117 {
    #[inline(always)]
    fn default() -> Pio117 {
        Pio117(0)
    }
}
impl core::fmt::Debug for Pio117 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio117")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio117 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio117 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio118(pub u32);
impl Pio118 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio118Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio118Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio118Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio118Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio118Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio118Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio118Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio118Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio118Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio118Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio118Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio118Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio118Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio118Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio118Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio118 {
    #[inline(always)]
    fn default() -> Pio118 {
        Pio118(0)
    }
}
impl core::fmt::Debug for Pio118 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio118")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio118 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio118 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio119(pub u32);
impl Pio119 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio119Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio119Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio119Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio119Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio119Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio119Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio119Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio119Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio119Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio119Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio119Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio119Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio119Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio119Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio119Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio119Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio119Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio119Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio119 {
    #[inline(always)]
    fn default() -> Pio119 {
        Pio119(0)
    }
}
impl core::fmt::Debug for Pio119 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio119")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio119 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio119 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio12(pub u32);
impl Pio12 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio12Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio12Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio12Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio12Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio12Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio12Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio12Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio12Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio12Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio12Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio12Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio12Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio12Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio12Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio12Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio12 {
    #[inline(always)]
    fn default() -> Pio12 {
        Pio12(0)
    }
}
impl core::fmt::Debug for Pio12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio12")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio12 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio120(pub u32);
impl Pio120 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio120Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio120Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio120Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio120Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio120Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio120Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio120Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio120Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio120Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio120Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio120Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio120Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio120Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio120Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio120Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio120 {
    #[inline(always)]
    fn default() -> Pio120 {
        Pio120(0)
    }
}
impl core::fmt::Debug for Pio120 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio120")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio120 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio120 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio121(pub u32);
impl Pio121 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio121Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio121Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio121Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio121Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio121Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio121Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio121Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio121Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio121Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio121Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio121Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio121Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio121Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio121Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio121Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio121 {
    #[inline(always)]
    fn default() -> Pio121 {
        Pio121(0)
    }
}
impl core::fmt::Debug for Pio121 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio121")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio121 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio121 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio122(pub u32);
impl Pio122 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio122Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio122Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio122Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio122Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio122Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio122Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio122Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio122Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio122Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio122Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio122Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio122Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio122Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio122Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio122Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio122 {
    #[inline(always)]
    fn default() -> Pio122 {
        Pio122(0)
    }
}
impl core::fmt::Debug for Pio122 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio122")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio122 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio122 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio123(pub u32);
impl Pio123 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio123Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio123Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio123Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio123Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio123Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio123Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio123Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio123Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio123Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio123Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio123Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio123Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio123Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio123Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio123Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio123 {
    #[inline(always)]
    fn default() -> Pio123 {
        Pio123(0)
    }
}
impl core::fmt::Debug for Pio123 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio123")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio123 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio123 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio124(pub u32);
impl Pio124 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio124Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio124Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio124Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio124Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio124Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio124Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio124Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio124Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio124Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio124Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio124Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio124Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio124Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio124Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio124Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio124 {
    #[inline(always)]
    fn default() -> Pio124 {
        Pio124(0)
    }
}
impl core::fmt::Debug for Pio124 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio124")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio124 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio124 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio125(pub u32);
impl Pio125 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio125Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio125Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio125Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio125Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio125Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio125Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio125Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio125Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio125Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio125Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio125Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio125Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio125Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio125Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio125Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio125 {
    #[inline(always)]
    fn default() -> Pio125 {
        Pio125(0)
    }
}
impl core::fmt::Debug for Pio125 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio125")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio125 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio125 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio126(pub u32);
impl Pio126 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio126Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio126Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio126Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio126Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio126Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio126Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio126Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio126Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio126Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio126Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio126Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio126Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio126Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio126Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio126Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio126 {
    #[inline(always)]
    fn default() -> Pio126 {
        Pio126(0)
    }
}
impl core::fmt::Debug for Pio126 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio126")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio126 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio126 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio127(pub u32);
impl Pio127 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio127Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio127Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio127Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio127Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio127Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio127Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio127Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio127Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio127Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio127Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio127Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio127Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio127Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio127Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio127Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio127 {
    #[inline(always)]
    fn default() -> Pio127 {
        Pio127(0)
    }
}
impl core::fmt::Debug for Pio127 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio127")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio127 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio127 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio128(pub u32);
impl Pio128 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio128Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio128Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio128Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio128Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio128Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio128Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio128Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio128Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio128Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio128Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio128Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio128Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio128Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio128Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio128Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio128 {
    #[inline(always)]
    fn default() -> Pio128 {
        Pio128(0)
    }
}
impl core::fmt::Debug for Pio128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio128")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio128 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio128 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio129(pub u32);
impl Pio129 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio129Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio129Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio129Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio129Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio129Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio129Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio129Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio129Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio129Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio129Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio129Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio129Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio129Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio129Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio129Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio129 {
    #[inline(always)]
    fn default() -> Pio129 {
        Pio129(0)
    }
}
impl core::fmt::Debug for Pio129 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio129")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio129 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio129 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio13(pub u32);
impl Pio13 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio13Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio13Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio13Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio13Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio13Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio13Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio13Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio13Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio13Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio13Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio13Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio13Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio13Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio13Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio13Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio13 {
    #[inline(always)]
    fn default() -> Pio13 {
        Pio13(0)
    }
}
impl core::fmt::Debug for Pio13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio13")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio13 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio130(pub u32);
impl Pio130 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio130Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio130Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio130Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio130Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio130Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio130Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio130Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio130Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio130Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio130Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio130Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio130Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio130Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio130Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio130Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio130 {
    #[inline(always)]
    fn default() -> Pio130 {
        Pio130(0)
    }
}
impl core::fmt::Debug for Pio130 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio130")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio130 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio130 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio131(pub u32);
impl Pio131 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio131Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio131Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio131Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio131Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio131Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio131Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio131Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio131Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio131Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio131Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio131Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio131Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio131Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio131Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio131Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio131 {
    #[inline(always)]
    fn default() -> Pio131 {
        Pio131(0)
    }
}
impl core::fmt::Debug for Pio131 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio131")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio131 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio131 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio14(pub u32);
impl Pio14 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio14Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio14Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio14Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio14Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio14Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio14Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio14Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio14Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio14Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio14Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio14Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio14Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio14Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio14Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio14Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio14 {
    #[inline(always)]
    fn default() -> Pio14 {
        Pio14(0)
    }
}
impl core::fmt::Debug for Pio14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio14")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio14 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio15(pub u32);
impl Pio15 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio15Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio15Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio15Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio15Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio15Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio15Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio15Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio15Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio15Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio15Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio15Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio15Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio15Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio15Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio15Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio15 {
    #[inline(always)]
    fn default() -> Pio15 {
        Pio15(0)
    }
}
impl core::fmt::Debug for Pio15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio15")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio15 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio16(pub u32);
impl Pio16 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio16Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio16Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio16Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio16Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio16Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio16Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio16Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio16Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio16Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio16Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio16Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio16Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio16Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio16Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio16Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio16 {
    #[inline(always)]
    fn default() -> Pio16 {
        Pio16(0)
    }
}
impl core::fmt::Debug for Pio16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio16")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio16 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio17(pub u32);
impl Pio17 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio17Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio17Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio17Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio17Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio17Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio17Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio17Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio17Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio17Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio17Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio17Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio17Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio17Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio17Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio17Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pio17 {
    #[inline(always)]
    fn default() -> Pio17 {
        Pio17(0)
    }
}
impl core::fmt::Debug for Pio17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio17")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio17 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio18(pub u32);
impl Pio18 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio18Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio18Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio18Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio18Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio18Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio18Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio18Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio18Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio18Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio18Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio18Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio18Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio18Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio18Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio18Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio18Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio18Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio18Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio18 {
    #[inline(always)]
    fn default() -> Pio18 {
        Pio18(0)
    }
}
impl core::fmt::Debug for Pio18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio18")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio18 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio19(pub u32);
impl Pio19 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::Pio19Func {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pio19Func::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::Pio19Func) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pio19Mode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pio19Mode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Pio19Mode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::Pio19Slew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pio19Slew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::Pio19Slew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::Pio19Digimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pio19Digimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::Pio19Digimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::Pio19Od {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pio19Od::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::Pio19Od) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::Pio19Asw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pio19Asw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::Pio19Asw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Pio19 {
    #[inline(always)]
    fn default() -> Pio19 {
        Pio19(0)
    }
}
impl core::fmt::Debug for Pio19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio19")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio19 {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw()
        )
    }
}
