#[doc = "Calibration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib0(pub u32);
impl Calib0 {
    #[doc = "Calibration of NMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver"]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib0 {
    #[inline(always)]
    fn default() -> Calib0 {
        Calib0(0)
    }
}
impl core::fmt::Debug for Calib0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib0")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib0 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Calibration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib1(pub u32);
impl Calib1 {
    #[doc = "Calibration of NMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver"]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib1 {
    #[inline(always)]
    fn default() -> Calib1 {
        Calib1(0)
    }
}
impl core::fmt::Debug for Calib1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib1")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib1 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Port Voltage Range"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "Port Voltage Range"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config {{ range: {:?} }}", self.range())
    }
}
#[doc = "Global Pin Control High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpchr(pub u32);
impl Gpchr {
    #[doc = "Global Pin Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data"]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe16(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe16(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe17(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe17(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe18(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe18(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe19(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe19(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe20(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe20(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe21(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe21(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe22(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe22(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe23(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe23(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe24(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe24(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe25(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe25(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe26(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe26(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe27(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe27(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe28(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe28(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe29(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe29(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe30(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe30(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe31(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe31(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpchr {
    #[inline(always)]
    fn default() -> Gpchr {
        Gpchr(0)
    }
}
impl core::fmt::Debug for Gpchr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpchr")
            .field("gpwd", &self.gpwd())
            .field("gpwe16", &self.gpwe16())
            .field("gpwe17", &self.gpwe17())
            .field("gpwe18", &self.gpwe18())
            .field("gpwe19", &self.gpwe19())
            .field("gpwe20", &self.gpwe20())
            .field("gpwe21", &self.gpwe21())
            .field("gpwe22", &self.gpwe22())
            .field("gpwe23", &self.gpwe23())
            .field("gpwe24", &self.gpwe24())
            .field("gpwe25", &self.gpwe25())
            .field("gpwe26", &self.gpwe26())
            .field("gpwe27", &self.gpwe27())
            .field("gpwe28", &self.gpwe28())
            .field("gpwe29", &self.gpwe29())
            .field("gpwe30", &self.gpwe30())
            .field("gpwe31", &self.gpwe31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpchr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpchr {{ gpwd: {=u16:?}, gpwe16: {:?}, gpwe17: {:?}, gpwe18: {:?}, gpwe19: {:?}, gpwe20: {:?}, gpwe21: {:?}, gpwe22: {:?}, gpwe23: {:?}, gpwe24: {:?}, gpwe25: {:?}, gpwe26: {:?}, gpwe27: {:?}, gpwe28: {:?}, gpwe29: {:?}, gpwe30: {:?}, gpwe31: {:?} }}",
            self.gpwd(),
            self.gpwe16(),
            self.gpwe17(),
            self.gpwe18(),
            self.gpwe19(),
            self.gpwe20(),
            self.gpwe21(),
            self.gpwe22(),
            self.gpwe23(),
            self.gpwe24(),
            self.gpwe25(),
            self.gpwe26(),
            self.gpwe27(),
            self.gpwe28(),
            self.gpwe29(),
            self.gpwe30(),
            self.gpwe31()
        )
    }
}
#[doc = "Global Pin Control Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr(pub u32);
impl Gpclr {
    #[doc = "Global Pin Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data"]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe0(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe0(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe1(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe1(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe2(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe2(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe3(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe3(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe4(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe4(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe5(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe5(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe6(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe6(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe7(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe7(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe8(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe8(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe9(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe9(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe10(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe10(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe11(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe11(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe12(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe12(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe13(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe13(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe14(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe14(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe15(&self) -> super::vals::Gpwe {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe15(&mut self, val: super::vals::Gpwe) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpclr {
    #[inline(always)]
    fn default() -> Gpclr {
        Gpclr(0)
    }
}
impl core::fmt::Debug for Gpclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpclr")
            .field("gpwd", &self.gpwd())
            .field("gpwe0", &self.gpwe0())
            .field("gpwe1", &self.gpwe1())
            .field("gpwe2", &self.gpwe2())
            .field("gpwe3", &self.gpwe3())
            .field("gpwe4", &self.gpwe4())
            .field("gpwe5", &self.gpwe5())
            .field("gpwe6", &self.gpwe6())
            .field("gpwe7", &self.gpwe7())
            .field("gpwe8", &self.gpwe8())
            .field("gpwe9", &self.gpwe9())
            .field("gpwe10", &self.gpwe10())
            .field("gpwe11", &self.gpwe11())
            .field("gpwe12", &self.gpwe12())
            .field("gpwe13", &self.gpwe13())
            .field("gpwe14", &self.gpwe14())
            .field("gpwe15", &self.gpwe15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpclr {{ gpwd: {=u16:?}, gpwe0: {:?}, gpwe1: {:?}, gpwe2: {:?}, gpwe3: {:?}, gpwe4: {:?}, gpwe5: {:?}, gpwe6: {:?}, gpwe7: {:?}, gpwe8: {:?}, gpwe9: {:?}, gpwe10: {:?}, gpwe11: {:?}, gpwe12: {:?}, gpwe13: {:?}, gpwe14: {:?}, gpwe15: {:?} }}",
            self.gpwd(),
            self.gpwe0(),
            self.gpwe1(),
            self.gpwe2(),
            self.gpwe3(),
            self.gpwe4(),
            self.gpwe5(),
            self.gpwe6(),
            self.gpwe7(),
            self.gpwe8(),
            self.gpwe9(),
            self.gpwe10(),
            self.gpwe11(),
            self.gpwe12(),
            self.gpwe13(),
            self.gpwe14(),
            self.gpwe15()
        )
    }
}
#[doc = "Pin Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        Pcr(0)
    }
}
impl core::fmt::Debug for Pcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
