#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbient0(pub u32);
impl AuxBiasCurveAmbient0 {
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveAmbient0 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbient0 {
        AuxBiasCurveAmbient0(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbient0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbient0")
            .field("vref1vcurvetrim_0", &self.vref1vcurvetrim_0())
            .field("vref1vcurvetrim_1", &self.vref1vcurvetrim_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbient0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbient0 {{ vref1vcurvetrim_0: {=u16:?}, vref1vcurvetrim_1: {=u16:?} }}",
            self.vref1vcurvetrim_0(),
            self.vref1vcurvetrim_1()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbient1(pub u32);
impl AuxBiasCurveAmbient1 {
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveAmbient1 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbient1 {
        AuxBiasCurveAmbient1(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbient1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbient1")
            .field("vref1vcurvetrim_2", &self.vref1vcurvetrim_2())
            .field("vref1vcurvetrim_3", &self.vref1vcurvetrim_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbient1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbient1 {{ vref1vcurvetrim_2: {=u16:?}, vref1vcurvetrim_3: {=u16:?} }}",
            self.vref1vcurvetrim_2(),
            self.vref1vcurvetrim_3()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbient2(pub u32);
impl AuxBiasCurveAmbient2 {
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveAmbient2 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbient2 {
        AuxBiasCurveAmbient2(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbient2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbient2")
            .field("vref1vcurvetrim_4", &self.vref1vcurvetrim_4())
            .field("vref1vcurvetrim_5", &self.vref1vcurvetrim_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbient2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbient2 {{ vref1vcurvetrim_4: {=u16:?}, vref1vcurvetrim_5: {=u16:?} }}",
            self.vref1vcurvetrim_4(),
            self.vref1vcurvetrim_5()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbient3(pub u32);
impl AuxBiasCurveAmbient3 {
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveAmbient3 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbient3 {
        AuxBiasCurveAmbient3(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbient3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbient3")
            .field("vref1vcurvetrim_6", &self.vref1vcurvetrim_6())
            .field("vref1vcurvetrim_7", &self.vref1vcurvetrim_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbient3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbient3 {{ vref1vcurvetrim_6: {=u16:?}, vref1vcurvetrim_7: {=u16:?} }}",
            self.vref1vcurvetrim_6(),
            self.vref1vcurvetrim_7()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbientArray0(pub u32);
impl AuxBiasCurveAmbientArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveAmbientArray0 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbientArray0 {
        AuxBiasCurveAmbientArray0(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbientArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbientArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbientArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbientArray0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbientArray1(pub u32);
impl AuxBiasCurveAmbientArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveAmbientArray1 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbientArray1 {
        AuxBiasCurveAmbientArray1(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbientArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbientArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbientArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbientArray1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbientArray2(pub u32);
impl AuxBiasCurveAmbientArray2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveAmbientArray2 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbientArray2 {
        AuxBiasCurveAmbientArray2(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbientArray2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbientArray2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbientArray2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbientArray2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveAmbientArray3(pub u32);
impl AuxBiasCurveAmbientArray3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveAmbientArray3 {
    #[inline(always)]
    fn default() -> AuxBiasCurveAmbientArray3 {
        AuxBiasCurveAmbientArray3(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveAmbientArray3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveAmbientArray3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveAmbientArray3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveAmbientArray3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTemp0(pub u32);
impl AuxBiasCurveTemp0 {
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveTemp0 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTemp0 {
        AuxBiasCurveTemp0(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTemp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTemp0")
            .field("vref1vcurvetrim_0", &self.vref1vcurvetrim_0())
            .field("vref1vcurvetrim_1", &self.vref1vcurvetrim_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTemp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTemp0 {{ vref1vcurvetrim_0: {=u16:?}, vref1vcurvetrim_1: {=u16:?} }}",
            self.vref1vcurvetrim_0(),
            self.vref1vcurvetrim_1()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTemp1(pub u32);
impl AuxBiasCurveTemp1 {
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveTemp1 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTemp1 {
        AuxBiasCurveTemp1(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTemp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTemp1")
            .field("vref1vcurvetrim_2", &self.vref1vcurvetrim_2())
            .field("vref1vcurvetrim_3", &self.vref1vcurvetrim_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTemp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTemp1 {{ vref1vcurvetrim_2: {=u16:?}, vref1vcurvetrim_3: {=u16:?} }}",
            self.vref1vcurvetrim_2(),
            self.vref1vcurvetrim_3()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTemp2(pub u32);
impl AuxBiasCurveTemp2 {
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveTemp2 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTemp2 {
        AuxBiasCurveTemp2(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTemp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTemp2")
            .field("vref1vcurvetrim_4", &self.vref1vcurvetrim_4())
            .field("vref1vcurvetrim_5", &self.vref1vcurvetrim_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTemp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTemp2 {{ vref1vcurvetrim_4: {=u16:?}, vref1vcurvetrim_5: {=u16:?} }}",
            self.vref1vcurvetrim_4(),
            self.vref1vcurvetrim_5()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTemp3(pub u32);
impl AuxBiasCurveTemp3 {
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim_7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)"]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AuxBiasCurveTemp3 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTemp3 {
        AuxBiasCurveTemp3(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTemp3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTemp3")
            .field("vref1vcurvetrim_6", &self.vref1vcurvetrim_6())
            .field("vref1vcurvetrim_7", &self.vref1vcurvetrim_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTemp3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTemp3 {{ vref1vcurvetrim_6: {=u16:?}, vref1vcurvetrim_7: {=u16:?} }}",
            self.vref1vcurvetrim_6(),
            self.vref1vcurvetrim_7()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTempArray0(pub u32);
impl AuxBiasCurveTempArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveTempArray0 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTempArray0 {
        AuxBiasCurveTempArray0(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTempArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTempArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTempArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTempArray0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTempArray1(pub u32);
impl AuxBiasCurveTempArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveTempArray1 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTempArray1 {
        AuxBiasCurveTempArray1(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTempArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTempArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTempArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTempArray1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTempArray2(pub u32);
impl AuxBiasCurveTempArray2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveTempArray2 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTempArray2 {
        AuxBiasCurveTempArray2(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTempArray2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTempArray2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTempArray2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTempArray2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBiasCurveTempArray3(pub u32);
impl AuxBiasCurveTempArray3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AuxBiasCurveTempArray3 {
    #[inline(always)]
    fn default() -> AuxBiasCurveTempArray3 {
        AuxBiasCurveTempArray3(0)
    }
}
impl core::fmt::Debug for AuxBiasCurveTempArray3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBiasCurveTempArray3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBiasCurveTempArray3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBiasCurveTempArray3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bod(pub u32);
impl Bod {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn bod_vbat_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_bod_vbat_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn bod_vbat_triglvl(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_bod_vbat_triglvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn bod_vbat_hyst(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_bod_vbat_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn bod_core_trim_valid(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_bod_core_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn bod_core_triglvl(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_bod_core_triglvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn bod_core_hyst(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_bod_core_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
}
impl Default for Bod {
    #[inline(always)]
    fn default() -> Bod {
        Bod(0)
    }
}
impl core::fmt::Debug for Bod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bod")
            .field("bod_vbat_trim_valid", &self.bod_vbat_trim_valid())
            .field("bod_vbat_triglvl", &self.bod_vbat_triglvl())
            .field("bod_vbat_hyst", &self.bod_vbat_hyst())
            .field("bod_core_trim_valid", &self.bod_core_trim_valid())
            .field("bod_core_triglvl", &self.bod_core_triglvl())
            .field("bod_core_hyst", &self.bod_core_hyst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bod {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bod {{ bod_vbat_trim_valid: {=bool:?}, bod_vbat_triglvl: {=u8:?}, bod_vbat_hyst: {=u8:?}, bod_core_trim_valid: {=bool:?}, bod_core_triglvl: {=u8:?}, bod_core_hyst: {=u8:?} }}",
            self.bod_vbat_trim_valid(),
            self.bod_vbat_triglvl(),
            self.bod_vbat_hyst(),
            self.bod_core_trim_valid(),
            self.bod_core_triglvl(),
            self.bod_core_hyst()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileHigh0(pub u32);
impl DcdcPowerProfileHigh0 {
    #[doc = "DCDC is trimed."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is trimed."]
    #[inline(always)]
    pub const fn set_dcdc_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn rc(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_rc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn icomp(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_icomp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn icenable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_icenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn tmos(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_tmos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn disableisense(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_disableisense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn slicingenable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_slicingenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn inductorclampenable(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_inductorclampenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_pwd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_vout_pwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DcdcPowerProfileHigh0 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileHigh0 {
        DcdcPowerProfileHigh0(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileHigh0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileHigh0")
            .field("dcdc_trim_valid", &self.dcdc_trim_valid())
            .field("rc", &self.rc())
            .field("icomp", &self.icomp())
            .field("isel", &self.isel())
            .field("icenable", &self.icenable())
            .field("tmos", &self.tmos())
            .field("disableisense", &self.disableisense())
            .field("vout", &self.vout())
            .field("slicingenable", &self.slicingenable())
            .field("inductorclampenable", &self.inductorclampenable())
            .field("vout_pwd", &self.vout_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileHigh0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileHigh0 {{ dcdc_trim_valid: {=bool:?}, rc: {=u8:?}, icomp: {=u8:?}, isel: {=u8:?}, icenable: {=bool:?}, tmos: {=u8:?}, disableisense: {=bool:?}, vout: {=u8:?}, slicingenable: {=bool:?}, inductorclampenable: {=bool:?}, vout_pwd: {=u8:?} }}",
            self.dcdc_trim_valid(),
            self.rc(),
            self.icomp(),
            self.isel(),
            self.icenable(),
            self.tmos(),
            self.disableisense(),
            self.vout(),
            self.slicingenable(),
            self.inductorclampenable(),
            self.vout_pwd()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileHigh1(pub u32);
impl DcdcPowerProfileHigh1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrimoffet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_rtrimoffet(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn rsensetrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_rsensetrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestenable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_dtestenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setcurve(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setcurve(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setdc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestsel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_dtestsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn iscaleenable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_iscaleenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn forcebypass(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_forcebypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn trimautocot(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_trimautocot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn forcefullcycle(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_forcefullcycle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn lcenable(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_lcenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn toff(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_toff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn toffenable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_toffenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DcdcPowerProfileHigh1 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileHigh1 {
        DcdcPowerProfileHigh1(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileHigh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileHigh1")
            .field("rtrimoffet", &self.rtrimoffet())
            .field("rsensetrim", &self.rsensetrim())
            .field("dtestenable", &self.dtestenable())
            .field("setcurve", &self.setcurve())
            .field("setdc", &self.setdc())
            .field("dtestsel", &self.dtestsel())
            .field("iscaleenable", &self.iscaleenable())
            .field("forcebypass", &self.forcebypass())
            .field("trimautocot", &self.trimautocot())
            .field("forcefullcycle", &self.forcefullcycle())
            .field("lcenable", &self.lcenable())
            .field("toff", &self.toff())
            .field("toffenable", &self.toffenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileHigh1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileHigh1 {{ rtrimoffet: {=u8:?}, rsensetrim: {=u8:?}, dtestenable: {=bool:?}, setcurve: {=u8:?}, setdc: {=u8:?}, dtestsel: {=u8:?}, iscaleenable: {=bool:?}, forcebypass: {=bool:?}, trimautocot: {=u8:?}, forcefullcycle: {=bool:?}, lcenable: {=bool:?}, toff: {=u8:?}, toffenable: {=bool:?} }}",
            self.rtrimoffet(),
            self.rsensetrim(),
            self.dtestenable(),
            self.setcurve(),
            self.setdc(),
            self.dtestsel(),
            self.iscaleenable(),
            self.forcebypass(),
            self.trimautocot(),
            self.forcefullcycle(),
            self.lcenable(),
            self.toff(),
            self.toffenable()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileHighArray0(pub u32);
impl DcdcPowerProfileHighArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DcdcPowerProfileHighArray0 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileHighArray0 {
        DcdcPowerProfileHighArray0(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileHighArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileHighArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileHighArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileHighArray0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileHighArray1(pub u32);
impl DcdcPowerProfileHighArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DcdcPowerProfileHighArray1 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileHighArray1 {
        DcdcPowerProfileHighArray1(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileHighArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileHighArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileHighArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileHighArray1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileLow0(pub u32);
impl DcdcPowerProfileLow0 {
    #[doc = "DCDC is trimed."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is trimed."]
    #[inline(always)]
    pub const fn set_dcdc_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn rc(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_rc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn icomp(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_icomp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn icenable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_icenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn tmos(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_tmos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn disableisense(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_disableisense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn slicingenable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_slicingenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn inductorclampenable(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_inductorclampenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_pwd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_vout_pwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DcdcPowerProfileLow0 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileLow0 {
        DcdcPowerProfileLow0(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileLow0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileLow0")
            .field("dcdc_trim_valid", &self.dcdc_trim_valid())
            .field("rc", &self.rc())
            .field("icomp", &self.icomp())
            .field("isel", &self.isel())
            .field("icenable", &self.icenable())
            .field("tmos", &self.tmos())
            .field("disableisense", &self.disableisense())
            .field("vout", &self.vout())
            .field("slicingenable", &self.slicingenable())
            .field("inductorclampenable", &self.inductorclampenable())
            .field("vout_pwd", &self.vout_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileLow0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileLow0 {{ dcdc_trim_valid: {=bool:?}, rc: {=u8:?}, icomp: {=u8:?}, isel: {=u8:?}, icenable: {=bool:?}, tmos: {=u8:?}, disableisense: {=bool:?}, vout: {=u8:?}, slicingenable: {=bool:?}, inductorclampenable: {=bool:?}, vout_pwd: {=u8:?} }}",
            self.dcdc_trim_valid(),
            self.rc(),
            self.icomp(),
            self.isel(),
            self.icenable(),
            self.tmos(),
            self.disableisense(),
            self.vout(),
            self.slicingenable(),
            self.inductorclampenable(),
            self.vout_pwd()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileLow1(pub u32);
impl DcdcPowerProfileLow1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrimoffet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_rtrimoffet(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn rsensetrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_rsensetrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestenable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_dtestenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setcurve(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setcurve(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setdc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestsel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_dtestsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn iscaleenable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_iscaleenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn forcebypass(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_forcebypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn trimautocot(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_trimautocot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn forcefullcycle(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_forcefullcycle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn lcenable(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_lcenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn toff(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_toff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn toffenable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_toffenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DcdcPowerProfileLow1 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileLow1 {
        DcdcPowerProfileLow1(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileLow1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileLow1")
            .field("rtrimoffet", &self.rtrimoffet())
            .field("rsensetrim", &self.rsensetrim())
            .field("dtestenable", &self.dtestenable())
            .field("setcurve", &self.setcurve())
            .field("setdc", &self.setdc())
            .field("dtestsel", &self.dtestsel())
            .field("iscaleenable", &self.iscaleenable())
            .field("forcebypass", &self.forcebypass())
            .field("trimautocot", &self.trimautocot())
            .field("forcefullcycle", &self.forcefullcycle())
            .field("lcenable", &self.lcenable())
            .field("toff", &self.toff())
            .field("toffenable", &self.toffenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileLow1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileLow1 {{ rtrimoffet: {=u8:?}, rsensetrim: {=u8:?}, dtestenable: {=bool:?}, setcurve: {=u8:?}, setdc: {=u8:?}, dtestsel: {=u8:?}, iscaleenable: {=bool:?}, forcebypass: {=bool:?}, trimautocot: {=u8:?}, forcefullcycle: {=bool:?}, lcenable: {=bool:?}, toff: {=u8:?}, toffenable: {=bool:?} }}",
            self.rtrimoffet(),
            self.rsensetrim(),
            self.dtestenable(),
            self.setcurve(),
            self.setdc(),
            self.dtestsel(),
            self.iscaleenable(),
            self.forcebypass(),
            self.trimautocot(),
            self.forcefullcycle(),
            self.lcenable(),
            self.toff(),
            self.toffenable()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileLowArray0(pub u32);
impl DcdcPowerProfileLowArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DcdcPowerProfileLowArray0 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileLowArray0 {
        DcdcPowerProfileLowArray0(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileLowArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileLowArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileLowArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileLowArray0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileLowArray1(pub u32);
impl DcdcPowerProfileLowArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DcdcPowerProfileLowArray1 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileLowArray1 {
        DcdcPowerProfileLowArray1(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileLowArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileLowArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileLowArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileLowArray1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileMedium0(pub u32);
impl DcdcPowerProfileMedium0 {
    #[doc = "DCDC is trimed."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is trimed."]
    #[inline(always)]
    pub const fn set_dcdc_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn rc(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_rc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn icomp(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_icomp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn icenable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_icenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn tmos(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_tmos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn disableisense(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_disableisense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn slicingenable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_slicingenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn inductorclampenable(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_inductorclampenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_pwd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_vout_pwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DcdcPowerProfileMedium0 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileMedium0 {
        DcdcPowerProfileMedium0(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileMedium0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileMedium0")
            .field("dcdc_trim_valid", &self.dcdc_trim_valid())
            .field("rc", &self.rc())
            .field("icomp", &self.icomp())
            .field("isel", &self.isel())
            .field("icenable", &self.icenable())
            .field("tmos", &self.tmos())
            .field("disableisense", &self.disableisense())
            .field("vout", &self.vout())
            .field("slicingenable", &self.slicingenable())
            .field("inductorclampenable", &self.inductorclampenable())
            .field("vout_pwd", &self.vout_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileMedium0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileMedium0 {{ dcdc_trim_valid: {=bool:?}, rc: {=u8:?}, icomp: {=u8:?}, isel: {=u8:?}, icenable: {=bool:?}, tmos: {=u8:?}, disableisense: {=bool:?}, vout: {=u8:?}, slicingenable: {=bool:?}, inductorclampenable: {=bool:?}, vout_pwd: {=u8:?} }}",
            self.dcdc_trim_valid(),
            self.rc(),
            self.icomp(),
            self.isel(),
            self.icenable(),
            self.tmos(),
            self.disableisense(),
            self.vout(),
            self.slicingenable(),
            self.inductorclampenable(),
            self.vout_pwd()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileMedium1(pub u32);
impl DcdcPowerProfileMedium1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrimoffet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_rtrimoffet(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn rsensetrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_rsensetrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestenable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_dtestenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setcurve(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setcurve(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setdc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestsel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_dtestsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn iscaleenable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_iscaleenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn forcebypass(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_forcebypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn trimautocot(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_trimautocot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn forcefullcycle(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_forcefullcycle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn lcenable(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_lcenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn toff(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_toff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn toffenable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_toffenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DcdcPowerProfileMedium1 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileMedium1 {
        DcdcPowerProfileMedium1(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileMedium1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileMedium1")
            .field("rtrimoffet", &self.rtrimoffet())
            .field("rsensetrim", &self.rsensetrim())
            .field("dtestenable", &self.dtestenable())
            .field("setcurve", &self.setcurve())
            .field("setdc", &self.setdc())
            .field("dtestsel", &self.dtestsel())
            .field("iscaleenable", &self.iscaleenable())
            .field("forcebypass", &self.forcebypass())
            .field("trimautocot", &self.trimautocot())
            .field("forcefullcycle", &self.forcefullcycle())
            .field("lcenable", &self.lcenable())
            .field("toff", &self.toff())
            .field("toffenable", &self.toffenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileMedium1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileMedium1 {{ rtrimoffet: {=u8:?}, rsensetrim: {=u8:?}, dtestenable: {=bool:?}, setcurve: {=u8:?}, setdc: {=u8:?}, dtestsel: {=u8:?}, iscaleenable: {=bool:?}, forcebypass: {=bool:?}, trimautocot: {=u8:?}, forcefullcycle: {=bool:?}, lcenable: {=bool:?}, toff: {=u8:?}, toffenable: {=bool:?} }}",
            self.rtrimoffet(),
            self.rsensetrim(),
            self.dtestenable(),
            self.setcurve(),
            self.setdc(),
            self.dtestsel(),
            self.iscaleenable(),
            self.forcebypass(),
            self.trimautocot(),
            self.forcefullcycle(),
            self.lcenable(),
            self.toff(),
            self.toffenable()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileMediumArray0(pub u32);
impl DcdcPowerProfileMediumArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DcdcPowerProfileMediumArray0 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileMediumArray0 {
        DcdcPowerProfileMediumArray0(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileMediumArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileMediumArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileMediumArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileMediumArray0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcPowerProfileMediumArray1(pub u32);
impl DcdcPowerProfileMediumArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DcdcPowerProfileMediumArray1 {
    #[inline(always)]
    fn default() -> DcdcPowerProfileMediumArray1 {
        DcdcPowerProfileMediumArray1(0)
    }
}
impl core::fmt::Debug for DcdcPowerProfileMediumArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcPowerProfileMediumArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcPowerProfileMediumArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcPowerProfileMediumArray1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
    #[inline(always)]
    pub const fn set_device_type_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_sec(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub const fn set_device_type_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pkg(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
    #[inline(always)]
    pub const fn set_device_type_pkg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of pins on the package."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Number of pins on the package."]
    #[inline(always)]
    pub const fn set_device_type_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type_num", &self.device_type_num())
            .field("device_type_sec", &self.device_type_sec())
            .field("device_type_pkg", &self.device_type_pkg())
            .field("device_type_pin", &self.device_type_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type_num: {=u16:?}, device_type_sec: {=bool:?}, device_type_pkg: {=u8:?}, device_type_pin: {=u8:?} }}",
            self.device_type_num(),
            self.device_type_sec(),
            self.device_type_pkg(),
            self.device_type_pin()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DisRomHiding(pub u32);
impl DisRomHiding {
    #[doc = "When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_rom_hiding(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[inline(always)]
    pub const fn set_dis_rom_hiding(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DisRomHiding {
    #[inline(always)]
    fn default() -> DisRomHiding {
        DisRomHiding(0)
    }
}
impl core::fmt::Debug for DisRomHiding {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DisRomHiding")
            .field("dis_rom_hiding", &self.dis_rom_hiding())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DisRomHiding {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DisRomHiding {{ dis_rom_hiding: {=u32:?} }}",
            self.dis_rom_hiding()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackup0(pub u32);
impl EcidBackup0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn coord_y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_coord_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn coord_x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_coord_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EcidBackup0 {
    #[inline(always)]
    fn default() -> EcidBackup0 {
        EcidBackup0(0)
    }
}
impl core::fmt::Debug for EcidBackup0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackup0")
            .field("coord_y", &self.coord_y())
            .field("coord_x", &self.coord_x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackup0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EcidBackup0 {{ coord_y: {=u16:?}, coord_x: {=u16:?} }}",
            self.coord_y(),
            self.coord_x()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackup1(pub u32);
impl EcidBackup1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn wafer(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_wafer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for EcidBackup1 {
    #[inline(always)]
    fn default() -> EcidBackup1 {
        EcidBackup1(0)
    }
}
impl core::fmt::Debug for EcidBackup1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackup1")
            .field("wafer", &self.wafer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackup1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackup1 {{ wafer: {=u8:?} }}", self.wafer())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackup2(pub u32);
impl EcidBackup2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lotid_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lotid_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EcidBackup2 {
    #[inline(always)]
    fn default() -> EcidBackup2 {
        EcidBackup2(0)
    }
}
impl core::fmt::Debug for EcidBackup2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackup2")
            .field("lotid_lsb", &self.lotid_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackup2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackup2 {{ lotid_lsb: {=u32:?} }}", self.lotid_lsb())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackup3(pub u32);
impl EcidBackup3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lotid_msb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lotid_msb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EcidBackup3 {
    #[inline(always)]
    fn default() -> EcidBackup3 {
        EcidBackup3(0)
    }
}
impl core::fmt::Debug for EcidBackup3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackup3")
            .field("lotid_msb", &self.lotid_msb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackup3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackup3 {{ lotid_msb: {=u32:?} }}", self.lotid_msb())
    }
}
#[doc = "ECID backup (the original is in page n-1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackupArray0(pub u32);
impl EcidBackupArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EcidBackupArray0 {
    #[inline(always)]
    fn default() -> EcidBackupArray0 {
        EcidBackupArray0(0)
    }
}
impl core::fmt::Debug for EcidBackupArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackupArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackupArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackupArray0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "ECID backup (the original is in page n-1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackupArray1(pub u32);
impl EcidBackupArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EcidBackupArray1 {
    #[inline(always)]
    fn default() -> EcidBackupArray1 {
        EcidBackupArray1(0)
    }
}
impl core::fmt::Debug for EcidBackupArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackupArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackupArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackupArray1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "ECID backup (the original is in page n-1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackupArray2(pub u32);
impl EcidBackupArray2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EcidBackupArray2 {
    #[inline(always)]
    fn default() -> EcidBackupArray2 {
        EcidBackupArray2(0)
    }
}
impl core::fmt::Debug for EcidBackupArray2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackupArray2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackupArray2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackupArray2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "ECID backup (the original is in page n-1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcidBackupArray3(pub u32);
impl EcidBackupArray3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EcidBackupArray3 {
    #[inline(always)]
    fn default() -> EcidBackupArray3 {
        EcidBackupArray3(0)
    }
}
impl core::fmt::Debug for EcidBackupArray3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcidBackupArray3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcidBackupArray3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EcidBackupArray3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchId0(pub u32);
impl FinalTestBatchId0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchId0 {
    #[inline(always)]
    fn default() -> FinalTestBatchId0 {
        FinalTestBatchId0(0)
    }
}
impl core::fmt::Debug for FinalTestBatchId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchId0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FinalTestBatchId0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchId1(pub u32);
impl FinalTestBatchId1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchId1 {
    #[inline(always)]
    fn default() -> FinalTestBatchId1 {
        FinalTestBatchId1(0)
    }
}
impl core::fmt::Debug for FinalTestBatchId1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchId1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchId1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FinalTestBatchId1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchId2(pub u32);
impl FinalTestBatchId2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchId2 {
    #[inline(always)]
    fn default() -> FinalTestBatchId2 {
        FinalTestBatchId2(0)
    }
}
impl core::fmt::Debug for FinalTestBatchId2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchId2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchId2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FinalTestBatchId2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchId3(pub u32);
impl FinalTestBatchId3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchId3 {
    #[inline(always)]
    fn default() -> FinalTestBatchId3 {
        FinalTestBatchId3(0)
    }
}
impl core::fmt::Debug for FinalTestBatchId3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchId3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchId3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FinalTestBatchId3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchIdArray0(pub u32);
impl FinalTestBatchIdArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchIdArray0 {
    #[inline(always)]
    fn default() -> FinalTestBatchIdArray0 {
        FinalTestBatchIdArray0(0)
    }
}
impl core::fmt::Debug for FinalTestBatchIdArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchIdArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchIdArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FinalTestBatchIdArray0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchIdArray1(pub u32);
impl FinalTestBatchIdArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchIdArray1 {
    #[inline(always)]
    fn default() -> FinalTestBatchIdArray1 {
        FinalTestBatchIdArray1(0)
    }
}
impl core::fmt::Debug for FinalTestBatchIdArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchIdArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchIdArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FinalTestBatchIdArray1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchIdArray2(pub u32);
impl FinalTestBatchIdArray2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchIdArray2 {
    #[inline(always)]
    fn default() -> FinalTestBatchIdArray2 {
        FinalTestBatchIdArray2(0)
    }
}
impl core::fmt::Debug for FinalTestBatchIdArray2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchIdArray2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchIdArray2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FinalTestBatchIdArray2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestBatchIdArray3(pub u32);
impl FinalTestBatchIdArray3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestBatchIdArray3 {
    #[inline(always)]
    fn default() -> FinalTestBatchIdArray3 {
        FinalTestBatchIdArray3(0)
    }
}
impl core::fmt::Debug for FinalTestBatchIdArray3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestBatchIdArray3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestBatchIdArray3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FinalTestBatchIdArray3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestDate(pub u32);
impl FinalTestDate {
    #[doc = "DATE \\[stored as : year*10000+month*100+day\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub const fn set_date(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestDate {
    #[inline(always)]
    fn default() -> FinalTestDate {
        FinalTestDate(0)
    }
}
impl core::fmt::Debug for FinalTestDate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestDate")
            .field("date", &self.date())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestDate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FinalTestDate {{ date: {=u32:?} }}", self.date())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestProgramVersion(pub u32);
impl FinalTestProgramVersion {
    #[doc = "PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn program_version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub const fn set_program_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestProgramVersion {
    #[inline(always)]
    fn default() -> FinalTestProgramVersion {
        FinalTestProgramVersion(0)
    }
}
impl core::fmt::Debug for FinalTestProgramVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestProgramVersion")
            .field("program_version", &self.program_version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestProgramVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FinalTestProgramVersion {{ program_version: {=u32:?} }}",
            self.program_version()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FinalTestTime(pub u32);
impl FinalTestTime {
    #[doc = "TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub const fn set_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FinalTestTime {
    #[inline(always)]
    fn default() -> FinalTestTime {
        FinalTestTime(0)
    }
}
impl core::fmt::Debug for FinalTestTime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FinalTestTime")
            .field("time", &self.time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FinalTestTime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FinalTestTime {{ time: {=u32:?} }}", self.time())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashsizecfg(pub u32);
impl Flashsizecfg {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_configuration(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flash_configuration(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Flashsizecfg {
    #[inline(always)]
    fn default() -> Flashsizecfg {
        Flashsizecfg(0)
    }
}
impl core::fmt::Debug for Flashsizecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flashsizecfg")
            .field("flash_configuration", &self.flash_configuration())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashsizecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flashsizecfg {{ flash_configuration: {=u32:?} }}",
            self.flash_configuration()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fro192mhz(pub u32);
impl Fro192mhz {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fro192m_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fro192m_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FRO192M_BIASTRIM\\[5:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn fro192m_biastrim(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "FRO192M_BIASTRIM\\[5:0\\]."]
    #[inline(always)]
    pub const fn set_fro192m_biastrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "FRO192M_TEMPTRIM\\[6:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn fro192m_temptrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "FRO192M_TEMPTRIM\\[6:0\\]."]
    #[inline(always)]
    pub const fn set_fro192m_temptrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "FRO192M_DACTRIM\\[7:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn fro192m_dactrim(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0xff;
        val as u8
    }
    #[doc = "FRO192M_DACTRIM\\[7:0\\]."]
    #[inline(always)]
    pub const fn set_fro192m_dactrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 17usize)) | (((val as u32) & 0xff) << 17usize);
    }
}
impl Default for Fro192mhz {
    #[inline(always)]
    fn default() -> Fro192mhz {
        Fro192mhz(0)
    }
}
impl core::fmt::Debug for Fro192mhz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fro192mhz")
            .field("fro192m_trim_valid", &self.fro192m_trim_valid())
            .field("fro192m_biastrim", &self.fro192m_biastrim())
            .field("fro192m_temptrim", &self.fro192m_temptrim())
            .field("fro192m_dactrim", &self.fro192m_dactrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fro192mhz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fro192mhz {{ fro192m_trim_valid: {=bool:?}, fro192m_biastrim: {=u8:?}, fro192m_temptrim: {=u8:?}, fro192m_dactrim: {=u8:?} }}",
            self.fro192m_trim_valid(),
            self.fro192m_biastrim(),
            self.fro192m_temptrim(),
            self.fro192m_dactrim()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fro1mhz(pub u32);
impl Fro1mhz {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fro1m_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fro1m_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frequency trimming bits."]
    #[must_use]
    #[inline(always)]
    pub const fn fro1m_freqsel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Frequency trimming bits."]
    #[inline(always)]
    pub const fn set_fro1m_freqsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Fro1mhz {
    #[inline(always)]
    fn default() -> Fro1mhz {
        Fro1mhz(0)
    }
}
impl core::fmt::Debug for Fro1mhz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fro1mhz")
            .field("fro1m_trim_valid", &self.fro1m_trim_valid())
            .field("fro1m_freqsel", &self.fro1m_freqsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fro1mhz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fro1mhz {{ fro1m_trim_valid: {=bool:?}, fro1m_freqsel: {=u8:?} }}",
            self.fro1m_trim_valid(),
            self.fro1m_freqsel()
        )
    }
}
#[doc = "GPO0 register 0 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo00(pub u32);
impl Gpo00 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fro_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fro_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fro32k_ntat(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fro32k_ntat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fro32k_ptat(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fro32k_ptat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fro32k_capcal(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fro32k_capcal(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gpo00 {
    #[inline(always)]
    fn default() -> Gpo00 {
        Gpo00(0)
    }
}
impl core::fmt::Debug for Gpo00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo00")
            .field("fro_trim_valid", &self.fro_trim_valid())
            .field("fro32k_ntat", &self.fro32k_ntat())
            .field("fro32k_ptat", &self.fro32k_ptat())
            .field("fro32k_capcal", &self.fro32k_capcal())
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo00 {{ fro_trim_valid: {=bool:?}, fro32k_ntat: {=u8:?}, fro32k_ptat: {=u8:?}, fro32k_capcal: {=u16:?}, field: {=u16:?} }}",
            self.fro_trim_valid(),
            self.fro32k_ntat(),
            self.fro32k_ptat(),
            self.fro32k_capcal(),
            self.field()
        )
    }
}
#[doc = "GPO0 register 1 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo01(pub u32);
impl Gpo01 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo01 {
    #[inline(always)]
    fn default() -> Gpo01 {
        Gpo01(0)
    }
}
impl core::fmt::Debug for Gpo01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo01")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo01 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO0 register 2 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo02(pub u32);
impl Gpo02 {
    #[doc = "00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz"]
    #[must_use]
    #[inline(always)]
    pub const fn system_speed_code(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz"]
    #[inline(always)]
    pub const fn set_system_speed_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_ctrl_opmode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
    #[inline(always)]
    pub const fn set_flash_ctrl_opmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Gpo02 {
    #[inline(always)]
    fn default() -> Gpo02 {
        Gpo02(0)
    }
}
impl core::fmt::Debug for Gpo02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo02")
            .field("system_speed_code", &self.system_speed_code())
            .field("flash_ctrl_opmode", &self.flash_ctrl_opmode())
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo02 {{ system_speed_code: {=u8:?}, flash_ctrl_opmode: {=u8:?}, field: {=u32:?} }}",
            self.system_speed_code(),
            self.flash_ctrl_opmode(),
            self.field()
        )
    }
}
#[doc = "GPO0 register 3 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo03(pub u32);
impl Gpo03 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo03 {
    #[inline(always)]
    fn default() -> Gpo03 {
        Gpo03(0)
    }
}
impl core::fmt::Debug for Gpo03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo03")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo03 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO0 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo0Array0(pub u32);
impl Gpo0Array0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo0Array0 {
    #[inline(always)]
    fn default() -> Gpo0Array0 {
        Gpo0Array0(0)
    }
}
impl core::fmt::Debug for Gpo0Array0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo0Array0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo0Array0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo0Array0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO0 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo0Array1(pub u32);
impl Gpo0Array1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo0Array1 {
    #[inline(always)]
    fn default() -> Gpo0Array1 {
        Gpo0Array1(0)
    }
}
impl core::fmt::Debug for Gpo0Array1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo0Array1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo0Array1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo0Array1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO0 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo0Array2(pub u32);
impl Gpo0Array2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo0Array2 {
    #[inline(always)]
    fn default() -> Gpo0Array2 {
        Gpo0Array2(0)
    }
}
impl core::fmt::Debug for Gpo0Array2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo0Array2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo0Array2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo0Array2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO0 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo0Array3(pub u32);
impl Gpo0Array3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo0Array3 {
    #[inline(always)]
    fn default() -> Gpo0Array3 {
        Gpo0Array3(0)
    }
}
impl core::fmt::Debug for Gpo0Array3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo0Array3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo0Array3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo0Array3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO1 register 0 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo10(pub u32);
impl Gpo10 {
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[must_use]
    #[inline(always)]
    pub const fn final_test_not_done(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub const fn set_final_test_not_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Device type number. (E.g : LPC5569 stored as 69 decimal)"]
    #[must_use]
    #[inline(always)]
    pub const fn partconfig(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x7f;
        val as u8
    }
    #[doc = "Device type number. (E.g : LPC5569 stored as 69 decimal)"]
    #[inline(always)]
    pub const fn set_partconfig(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_sec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub const fn set_device_type_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_size(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED"]
    #[inline(always)]
    pub const fn set_sram_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_security_extension_disable(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub const fn set_cpu0_security_extension_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "ROM Revision-Minor \\[3:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_revision_minor(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM Revision-Minor \\[3:0\\]"]
    #[inline(always)]
    pub const fn set_rom_revision_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "METAL REVISION ID\\[3:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn metal_revision_id(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "METAL REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub const fn set_metal_revision_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Gpo10 {
    #[inline(always)]
    fn default() -> Gpo10 {
        Gpo10(0)
    }
}
impl core::fmt::Debug for Gpo10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo10")
            .field("final_test_not_done", &self.final_test_not_done())
            .field("partconfig", &self.partconfig())
            .field("device_type_sec", &self.device_type_sec())
            .field("sram_size", &self.sram_size())
            .field(
                "cpu0_security_extension_disable",
                &self.cpu0_security_extension_disable(),
            )
            .field("field", &self.field())
            .field("rom_revision_minor", &self.rom_revision_minor())
            .field("metal_revision_id", &self.metal_revision_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo10 {{ final_test_not_done: {=u8:?}, partconfig: {=u8:?}, device_type_sec: {=bool:?}, sram_size: {=u8:?}, cpu0_security_extension_disable: {=u8:?}, field: {=u8:?}, rom_revision_minor: {=u8:?}, metal_revision_id: {=u8:?} }}",
            self.final_test_not_done(),
            self.partconfig(),
            self.device_type_sec(),
            self.sram_size(),
            self.cpu0_security_extension_disable(),
            self.field(),
            self.rom_revision_minor(),
            self.metal_revision_id()
        )
    }
}
#[doc = "GPO1 register 1 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo11(pub u32);
impl Gpo11 {
    #[doc = "ROM Patch Version \\[3:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_patch_version(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM Patch Version \\[3:0\\]"]
    #[inline(always)]
    pub const fn set_rom_patch_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CUSTOMER REVISION ID\\[3:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn customer_revision_id(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CUSTOMER REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub const fn set_customer_revision_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Gpo11 {
    #[inline(always)]
    fn default() -> Gpo11 {
        Gpo11(0)
    }
}
impl core::fmt::Debug for Gpo11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo11")
            .field("rom_patch_version", &self.rom_patch_version())
            .field("customer_revision_id", &self.customer_revision_id())
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo11 {{ rom_patch_version: {=u8:?}, customer_revision_id: {=u8:?}, field: {=u32:?} }}",
            self.rom_patch_version(),
            self.customer_revision_id(),
            self.field()
        )
    }
}
#[doc = "GPO1 register 2 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo12(pub u32);
impl Gpo12 {
    #[doc = "High Voltage Stress: 0=not done; 1=done."]
    #[must_use]
    #[inline(always)]
    pub const fn hvst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "High Voltage Stress: 0=not done; 1=done."]
    #[inline(always)]
    pub const fn set_hvst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Gpo12 {
    #[inline(always)]
    fn default() -> Gpo12 {
        Gpo12(0)
    }
}
impl core::fmt::Debug for Gpo12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo12")
            .field("hvst", &self.hvst())
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo12 {{ hvst: {=bool:?}, field: {=u32:?} }}",
            self.hvst(),
            self.field()
        )
    }
}
#[doc = "GPO1 register 3 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo13(pub u32);
impl Gpo13 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo13 {
    #[inline(always)]
    fn default() -> Gpo13 {
        Gpo13(0)
    }
}
impl core::fmt::Debug for Gpo13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo13 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO1 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo1Array0(pub u32);
impl Gpo1Array0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo1Array0 {
    #[inline(always)]
    fn default() -> Gpo1Array0 {
        Gpo1Array0(0)
    }
}
impl core::fmt::Debug for Gpo1Array0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo1Array0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo1Array0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo1Array0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO1 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo1Array1(pub u32);
impl Gpo1Array1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo1Array1 {
    #[inline(always)]
    fn default() -> Gpo1Array1 {
        Gpo1Array1(0)
    }
}
impl core::fmt::Debug for Gpo1Array1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo1Array1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo1Array1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo1Array1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO1 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo1Array2(pub u32);
impl Gpo1Array2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo1Array2 {
    #[inline(always)]
    fn default() -> Gpo1Array2 {
        Gpo1Array2(0)
    }
}
impl core::fmt::Debug for Gpo1Array2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo1Array2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo1Array2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo1Array2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO1 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo1Array3(pub u32);
impl Gpo1Array3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo1Array3 {
    #[inline(always)]
    fn default() -> Gpo1Array3 {
        Gpo1Array3(0)
    }
}
impl core::fmt::Debug for Gpo1Array3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo1Array3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo1Array3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo1Array3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 register 0 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo20(pub u32);
impl Gpo20 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usbhs_phy_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb_reg_env_tail_adj_vd(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_usb_reg_env_tail_adj_vd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_trim_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "(For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_size(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "(For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED"]
    #[inline(always)]
    pub const fn set_flash_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_security_extension_disable(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub const fn set_cpu0_security_extension_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Gpo20 {
    #[inline(always)]
    fn default() -> Gpo20 {
        Gpo20(0)
    }
}
impl core::fmt::Debug for Gpo20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo20")
            .field("usbhs_phy_trim_valid", &self.usbhs_phy_trim_valid())
            .field(
                "trim_usb_reg_env_tail_adj_vd",
                &self.trim_usb_reg_env_tail_adj_vd(),
            )
            .field("trim_usbphy_tx_d_cal", &self.trim_usbphy_tx_d_cal())
            .field("trim_usbphy_tx_cal45dp", &self.trim_usbphy_tx_cal45dp())
            .field("trim_usbphy_tx_cal45dn", &self.trim_usbphy_tx_cal45dn())
            .field("trim_usb2_refbias_tst", &self.trim_usb2_refbias_tst())
            .field("trim_usb2_refbias_vbgadj", &self.trim_usb2_refbias_vbgadj())
            .field("trim_pll_ctrl0_div_sel", &self.trim_pll_ctrl0_div_sel())
            .field("flash_size", &self.flash_size())
            .field(
                "cpu0_security_extension_disable",
                &self.cpu0_security_extension_disable(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo20 {{ usbhs_phy_trim_valid: {=bool:?}, trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dn: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?}, flash_size: {=u8:?}, cpu0_security_extension_disable: {=u8:?} }}",
            self.usbhs_phy_trim_valid(),
            self.trim_usb_reg_env_tail_adj_vd(),
            self.trim_usbphy_tx_d_cal(),
            self.trim_usbphy_tx_cal45dp(),
            self.trim_usbphy_tx_cal45dn(),
            self.trim_usb2_refbias_tst(),
            self.trim_usb2_refbias_vbgadj(),
            self.trim_pll_ctrl0_div_sel(),
            self.flash_size(),
            self.cpu0_security_extension_disable()
        )
    }
}
#[doc = "GPO2 register 1 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo21(pub u32);
impl Gpo21 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo21 {
    #[inline(always)]
    fn default() -> Gpo21 {
        Gpo21(0)
    }
}
impl core::fmt::Debug for Gpo21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo21")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo21 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 register 2 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo22(pub u32);
impl Gpo22 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo22 {
    #[inline(always)]
    fn default() -> Gpo22 {
        Gpo22(0)
    }
}
impl core::fmt::Debug for Gpo22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo22")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo22 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 register 3 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo23(pub u32);
impl Gpo23 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo23 {
    #[inline(always)]
    fn default() -> Gpo23 {
        Gpo23(0)
    }
}
impl core::fmt::Debug for Gpo23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo23")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo23 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo2Array0(pub u32);
impl Gpo2Array0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo2Array0 {
    #[inline(always)]
    fn default() -> Gpo2Array0 {
        Gpo2Array0(0)
    }
}
impl core::fmt::Debug for Gpo2Array0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo2Array0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo2Array0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo2Array0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo2Array1(pub u32);
impl Gpo2Array1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo2Array1 {
    #[inline(always)]
    fn default() -> Gpo2Array1 {
        Gpo2Array1(0)
    }
}
impl core::fmt::Debug for Gpo2Array1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo2Array1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo2Array1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo2Array1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo2Array2(pub u32);
impl Gpo2Array2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo2Array2 {
    #[inline(always)]
    fn default() -> Gpo2Array2 {
        Gpo2Array2(0)
    }
}
impl core::fmt::Debug for Gpo2Array2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo2Array2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo2Array2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo2Array2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO2 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo2Array3(pub u32);
impl Gpo2Array3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo2Array3 {
    #[inline(always)]
    fn default() -> Gpo2Array3 {
        Gpo2Array3(0)
    }
}
impl core::fmt::Debug for Gpo2Array3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo2Array3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo2Array3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo2Array3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 register 0 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo30(pub u32);
impl Gpo30 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn aux_bias_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_aux_bias_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn aux_bias_itrim(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_aux_bias_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn aux_bias_ptat_itrim(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_aux_bias_ptat_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn aux_bias_vref1_vtrim(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_aux_bias_vref1_vtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn aux_bias_vref1_vcurve_trim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_aux_bias_vref1_vcurve_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 19usize)) | (((val as u32) & 0x3f) << 19usize);
    }
    #[doc = "ModelNumber extension\\[2:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn modelnum_extension(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "ModelNumber extension\\[2:0\\]"]
    #[inline(always)]
    pub const fn set_modelnum_extension(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[must_use]
    #[inline(always)]
    pub const fn final_test_not_done(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub const fn set_final_test_not_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Gpo30 {
    #[inline(always)]
    fn default() -> Gpo30 {
        Gpo30(0)
    }
}
impl core::fmt::Debug for Gpo30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo30")
            .field("aux_bias_trim_valid", &self.aux_bias_trim_valid())
            .field("aux_bias_itrim", &self.aux_bias_itrim())
            .field("aux_bias_ptat_itrim", &self.aux_bias_ptat_itrim())
            .field("aux_bias_vref1_vtrim", &self.aux_bias_vref1_vtrim())
            .field(
                "aux_bias_vref1_vcurve_trim",
                &self.aux_bias_vref1_vcurve_trim(),
            )
            .field("field", &self.field())
            .field("modelnum_extension", &self.modelnum_extension())
            .field("final_test_not_done", &self.final_test_not_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpo30 {{ aux_bias_trim_valid: {=bool:?}, aux_bias_itrim: {=u8:?}, aux_bias_ptat_itrim: {=u8:?}, aux_bias_vref1_vtrim: {=u8:?}, aux_bias_vref1_vcurve_trim: {=u8:?}, field: {=u8:?}, modelnum_extension: {=u8:?}, final_test_not_done: {=u8:?} }}",
            self.aux_bias_trim_valid(),
            self.aux_bias_itrim(),
            self.aux_bias_ptat_itrim(),
            self.aux_bias_vref1_vtrim(),
            self.aux_bias_vref1_vcurve_trim(),
            self.field(),
            self.modelnum_extension(),
            self.final_test_not_done()
        )
    }
}
#[doc = "GPO3 register 1 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo31(pub u32);
impl Gpo31 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo31 {
    #[inline(always)]
    fn default() -> Gpo31 {
        Gpo31(0)
    }
}
impl core::fmt::Debug for Gpo31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo31")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo31 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 register 2 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo32(pub u32);
impl Gpo32 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo32 {
    #[inline(always)]
    fn default() -> Gpo32 {
        Gpo32(0)
    }
}
impl core::fmt::Debug for Gpo32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo32")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo32 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 register 3 description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo33(pub u32);
impl Gpo33 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo33 {
    #[inline(always)]
    fn default() -> Gpo33 {
        Gpo33(0)
    }
}
impl core::fmt::Debug for Gpo33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo33")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo33 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo3Array0(pub u32);
impl Gpo3Array0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo3Array0 {
    #[inline(always)]
    fn default() -> Gpo3Array0 {
        Gpo3Array0(0)
    }
}
impl core::fmt::Debug for Gpo3Array0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo3Array0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo3Array0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo3Array0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo3Array1(pub u32);
impl Gpo3Array1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo3Array1 {
    #[inline(always)]
    fn default() -> Gpo3Array1 {
        Gpo3Array1(0)
    }
}
impl core::fmt::Debug for Gpo3Array1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo3Array1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo3Array1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo3Array1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo3Array2(pub u32);
impl Gpo3Array2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo3Array2 {
    #[inline(always)]
    fn default() -> Gpo3Array2 {
        Gpo3Array2(0)
    }
}
impl core::fmt::Debug for Gpo3Array2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo3Array2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo3Array2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo3Array2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "GPO3 array description"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpo3Array3(pub u32);
impl Gpo3Array3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpo3Array3 {
    #[inline(always)]
    fn default() -> Gpo3Array3 {
        Gpo3Array3(0)
    }
}
impl core::fmt::Debug for Gpo3Array3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpo3Array3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpo3Array3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpo3Array3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksum0(pub u32);
impl GpoChecksum0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksum0 {
    #[inline(always)]
    fn default() -> GpoChecksum0 {
        GpoChecksum0(0)
    }
}
impl core::fmt::Debug for GpoChecksum0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksum0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksum0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksum0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksum1(pub u32);
impl GpoChecksum1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksum1 {
    #[inline(always)]
    fn default() -> GpoChecksum1 {
        GpoChecksum1(0)
    }
}
impl core::fmt::Debug for GpoChecksum1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksum1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksum1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksum1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksum2(pub u32);
impl GpoChecksum2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksum2 {
    #[inline(always)]
    fn default() -> GpoChecksum2 {
        GpoChecksum2(0)
    }
}
impl core::fmt::Debug for GpoChecksum2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksum2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksum2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksum2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksum3(pub u32);
impl GpoChecksum3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksum3 {
    #[inline(always)]
    fn default() -> GpoChecksum3 {
        GpoChecksum3(0)
    }
}
impl core::fmt::Debug for GpoChecksum3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksum3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksum3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksum3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksumArray0(pub u32);
impl GpoChecksumArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksumArray0 {
    #[inline(always)]
    fn default() -> GpoChecksumArray0 {
        GpoChecksumArray0(0)
    }
}
impl core::fmt::Debug for GpoChecksumArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksumArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksumArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksumArray0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksumArray1(pub u32);
impl GpoChecksumArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksumArray1 {
    #[inline(always)]
    fn default() -> GpoChecksumArray1 {
        GpoChecksumArray1(0)
    }
}
impl core::fmt::Debug for GpoChecksumArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksumArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksumArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksumArray1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksumArray2(pub u32);
impl GpoChecksumArray2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksumArray2 {
    #[inline(always)]
    fn default() -> GpoChecksumArray2 {
        GpoChecksumArray2(0)
    }
}
impl core::fmt::Debug for GpoChecksumArray2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksumArray2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksumArray2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksumArray2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpoChecksumArray3(pub u32);
impl GpoChecksumArray3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GpoChecksumArray3 {
    #[inline(always)]
    fn default() -> GpoChecksumArray3 {
        GpoChecksumArray3(0)
    }
}
impl core::fmt::Debug for GpoChecksumArray3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpoChecksumArray3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpoChecksumArray3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GpoChecksumArray3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoAo(pub u32);
impl LdoAo {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn active_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_active_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn active_trim(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_active_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dslp_trim_valid(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dslp_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dslp_trim(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dslp_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pdwn_trim_valid(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pdwn_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pdwn_trim(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pdwn_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dpdw_trim_valid(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dpdw_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dpdw_trim(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dpdw_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for LdoAo {
    #[inline(always)]
    fn default() -> LdoAo {
        LdoAo(0)
    }
}
impl core::fmt::Debug for LdoAo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LdoAo")
            .field("active_trim_valid", &self.active_trim_valid())
            .field("active_trim", &self.active_trim())
            .field("dslp_trim_valid", &self.dslp_trim_valid())
            .field("dslp_trim", &self.dslp_trim())
            .field("pdwn_trim_valid", &self.pdwn_trim_valid())
            .field("pdwn_trim", &self.pdwn_trim())
            .field("dpdw_trim_valid", &self.dpdw_trim_valid())
            .field("dpdw_trim", &self.dpdw_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LdoAo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LdoAo {{ active_trim_valid: {=bool:?}, active_trim: {=u8:?}, dslp_trim_valid: {=bool:?}, dslp_trim: {=u8:?}, pdwn_trim_valid: {=bool:?}, pdwn_trim: {=u8:?}, dpdw_trim_valid: {=bool:?}, dpdw_trim: {=u8:?} }}",
            self.active_trim_valid(),
            self.active_trim(),
            self.dslp_trim_valid(),
            self.dslp_trim(),
            self.pdwn_trim_valid(),
            self.pdwn_trim(),
            self.dpdw_trim_valid(),
            self.dpdw_trim()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NxpDeviceCertificate0(pub u32);
impl NxpDeviceCertificate0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NxpDeviceCertificate0 {
    #[inline(always)]
    fn default() -> NxpDeviceCertificate0 {
        NxpDeviceCertificate0(0)
    }
}
impl core::fmt::Debug for NxpDeviceCertificate0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NxpDeviceCertificate0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NxpDeviceCertificate0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NxpDeviceCertificate0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NxpDeviceCertificate1(pub u32);
impl NxpDeviceCertificate1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NxpDeviceCertificate1 {
    #[inline(always)]
    fn default() -> NxpDeviceCertificate1 {
        NxpDeviceCertificate1(0)
    }
}
impl core::fmt::Debug for NxpDeviceCertificate1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NxpDeviceCertificate1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NxpDeviceCertificate1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NxpDeviceCertificate1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NxpDeviceCertificate2(pub u32);
impl NxpDeviceCertificate2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NxpDeviceCertificate2 {
    #[inline(always)]
    fn default() -> NxpDeviceCertificate2 {
        NxpDeviceCertificate2(0)
    }
}
impl core::fmt::Debug for NxpDeviceCertificate2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NxpDeviceCertificate2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NxpDeviceCertificate2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NxpDeviceCertificate2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NxpDeviceCertificate3(pub u32);
impl NxpDeviceCertificate3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NxpDeviceCertificate3 {
    #[inline(always)]
    fn default() -> NxpDeviceCertificate3 {
        NxpDeviceCertificate3(0)
    }
}
impl core::fmt::Debug for NxpDeviceCertificate3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NxpDeviceCertificate3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NxpDeviceCertificate3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NxpDeviceCertificate3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NxpDevicePrivateKey(pub u32);
impl NxpDevicePrivateKey {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NxpDevicePrivateKey {
    #[inline(always)]
    fn default() -> NxpDevicePrivateKey {
        NxpDevicePrivateKey(0)
    }
}
impl core::fmt::Debug for NxpDevicePrivateKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NxpDevicePrivateKey")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NxpDevicePrivateKey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NxpDevicePrivateKey {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periphencfg(pub u32);
impl Periphencfg {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn peripheral_configuration(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_peripheral_configuration(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_cpu1_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Periphencfg {
    #[inline(always)]
    fn default() -> Periphencfg {
        Periphencfg(0)
    }
}
impl core::fmt::Debug for Periphencfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periphencfg")
            .field("peripheral_configuration", &self.peripheral_configuration())
            .field("cpu1_enable", &self.cpu1_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periphencfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Periphencfg {{ peripheral_configuration: {=u16:?}, cpu1_enable: {=bool:?} }}",
            self.peripheral_configuration(),
            self.cpu1_enable()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PufSram(pub u32);
impl PufSram {
    #[doc = "1: PUF_SRAM is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_sram_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: PUF_SRAM is valid."]
    #[inline(always)]
    pub const fn set_puf_sram_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF SRAM Controller operating mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller operating mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PUF SRAM Clock Gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ckgating(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub const fn set_ckgating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Source Biasing voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn smb(&self) -> super::vals::Smb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Smb::from_bits(val as u8)
    }
    #[doc = "Source Biasing voltage."]
    #[inline(always)]
    pub const fn set_smb(&mut self, val: super::vals::Smb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Read Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn rm(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Read Margin control settings."]
    #[inline(always)]
    pub const fn set_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Write Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn wm(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Write Margin control settings."]
    #[inline(always)]
    pub const fn set_wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Write read margin enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wrme(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write read margin enable."]
    #[inline(always)]
    pub const fn set_wrme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SRAM Read Assist Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn raen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Read Assist Enable"]
    #[inline(always)]
    pub const fn set_raen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SRAM Read Assist settings"]
    #[must_use]
    #[inline(always)]
    pub const fn ram(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "SRAM Read Assist settings"]
    #[inline(always)]
    pub const fn set_ram(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "SRAM Write Assist Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Write Assist Enable"]
    #[inline(always)]
    pub const fn set_waen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SRAM Write Assist settings"]
    #[must_use]
    #[inline(always)]
    pub const fn wam(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "SRAM Write Assist settings"]
    #[inline(always)]
    pub const fn set_wam(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[doc = "STBP"]
    #[must_use]
    #[inline(always)]
    pub const fn stbp(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "STBP"]
    #[inline(always)]
    pub const fn set_stbp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for PufSram {
    #[inline(always)]
    fn default() -> PufSram {
        PufSram(0)
    }
}
impl core::fmt::Debug for PufSram {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PufSram")
            .field("puf_sram_valid", &self.puf_sram_valid())
            .field("mode", &self.mode())
            .field("ckgating", &self.ckgating())
            .field("smb", &self.smb())
            .field("rm", &self.rm())
            .field("wm", &self.wm())
            .field("wrme", &self.wrme())
            .field("raen", &self.raen())
            .field("ram", &self.ram())
            .field("waen", &self.waen())
            .field("wam", &self.wam())
            .field("stbp", &self.stbp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PufSram {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PufSram {{ puf_sram_valid: {=bool:?}, mode: {=bool:?}, ckgating: {=bool:?}, smb: {:?}, rm: {=u8:?}, wm: {=u8:?}, wrme: {=bool:?}, raen: {=bool:?}, ram: {=u8:?}, waen: {=bool:?}, wam: {=u8:?}, stbp: {=bool:?} }}",
            self.puf_sram_valid(),
            self.mode(),
            self.ckgating(),
            self.smb(),
            self.rm(),
            self.wm(),
            self.wrme(),
            self.raen(),
            self.ram(),
            self.waen(),
            self.wam(),
            self.stbp()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor0Array0(pub u32);
impl PvtMonitor0Array0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PvtMonitor0Array0 {
    #[inline(always)]
    fn default() -> PvtMonitor0Array0 {
        PvtMonitor0Array0(0)
    }
}
impl core::fmt::Debug for PvtMonitor0Array0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor0Array0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor0Array0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PvtMonitor0Array0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor0Array1(pub u32);
impl PvtMonitor0Array1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PvtMonitor0Array1 {
    #[inline(always)]
    fn default() -> PvtMonitor0Array1 {
        PvtMonitor0Array1(0)
    }
}
impl core::fmt::Debug for PvtMonitor0Array1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor0Array1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor0Array1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PvtMonitor0Array1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor0Array2(pub u32);
impl PvtMonitor0Array2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PvtMonitor0Array2 {
    #[inline(always)]
    fn default() -> PvtMonitor0Array2 {
        PvtMonitor0Array2(0)
    }
}
impl core::fmt::Debug for PvtMonitor0Array2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor0Array2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor0Array2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PvtMonitor0Array2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor0DelaysLsb(pub u32);
impl PvtMonitor0DelaysLsb {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn delay_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_delay_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_1(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for PvtMonitor0DelaysLsb {
    #[inline(always)]
    fn default() -> PvtMonitor0DelaysLsb {
        PvtMonitor0DelaysLsb(0)
    }
}
impl core::fmt::Debug for PvtMonitor0DelaysLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor0DelaysLsb")
            .field("delay_valid", &self.delay_valid())
            .field("delay_0", &self.delay_0())
            .field("delay_1", &self.delay_1())
            .field("delay_2", &self.delay_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor0DelaysLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PvtMonitor0DelaysLsb {{ delay_valid: {=bool:?}, delay_0: {=u16:?}, delay_1: {=u16:?}, delay_2: {=u16:?} }}",
            self.delay_valid(),
            self.delay_0(),
            self.delay_1(),
            self.delay_2()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor0DelaysMsb(pub u32);
impl PvtMonitor0DelaysMsb {
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_4(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_5(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for PvtMonitor0DelaysMsb {
    #[inline(always)]
    fn default() -> PvtMonitor0DelaysMsb {
        PvtMonitor0DelaysMsb(0)
    }
}
impl core::fmt::Debug for PvtMonitor0DelaysMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor0DelaysMsb")
            .field("delay_3", &self.delay_3())
            .field("delay_4", &self.delay_4())
            .field("delay_5", &self.delay_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor0DelaysMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PvtMonitor0DelaysMsb {{ delay_3: {=u16:?}, delay_4: {=u16:?}, delay_5: {=u16:?} }}",
            self.delay_3(),
            self.delay_4(),
            self.delay_5()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor0Ringo(pub u32);
impl PvtMonitor0Ringo {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ringo_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_freq_hz(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ringo_freq_hz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for PvtMonitor0Ringo {
    #[inline(always)]
    fn default() -> PvtMonitor0Ringo {
        PvtMonitor0Ringo(0)
    }
}
impl core::fmt::Debug for PvtMonitor0Ringo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor0Ringo")
            .field("ringo_valid", &self.ringo_valid())
            .field("ringo_freq_hz", &self.ringo_freq_hz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor0Ringo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PvtMonitor0Ringo {{ ringo_valid: {=bool:?}, ringo_freq_hz: {=u32:?} }}",
            self.ringo_valid(),
            self.ringo_freq_hz()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor1Array0(pub u32);
impl PvtMonitor1Array0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PvtMonitor1Array0 {
    #[inline(always)]
    fn default() -> PvtMonitor1Array0 {
        PvtMonitor1Array0(0)
    }
}
impl core::fmt::Debug for PvtMonitor1Array0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor1Array0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor1Array0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PvtMonitor1Array0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor1Array1(pub u32);
impl PvtMonitor1Array1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PvtMonitor1Array1 {
    #[inline(always)]
    fn default() -> PvtMonitor1Array1 {
        PvtMonitor1Array1(0)
    }
}
impl core::fmt::Debug for PvtMonitor1Array1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor1Array1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor1Array1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PvtMonitor1Array1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor1Array2(pub u32);
impl PvtMonitor1Array2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PvtMonitor1Array2 {
    #[inline(always)]
    fn default() -> PvtMonitor1Array2 {
        PvtMonitor1Array2(0)
    }
}
impl core::fmt::Debug for PvtMonitor1Array2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor1Array2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor1Array2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PvtMonitor1Array2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor1DelaysLsb(pub u32);
impl PvtMonitor1DelaysLsb {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn delay_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_delay_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_1(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for PvtMonitor1DelaysLsb {
    #[inline(always)]
    fn default() -> PvtMonitor1DelaysLsb {
        PvtMonitor1DelaysLsb(0)
    }
}
impl core::fmt::Debug for PvtMonitor1DelaysLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor1DelaysLsb")
            .field("delay_valid", &self.delay_valid())
            .field("delay_0", &self.delay_0())
            .field("delay_1", &self.delay_1())
            .field("delay_2", &self.delay_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor1DelaysLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PvtMonitor1DelaysLsb {{ delay_valid: {=bool:?}, delay_0: {=u16:?}, delay_1: {=u16:?}, delay_2: {=u16:?} }}",
            self.delay_valid(),
            self.delay_0(),
            self.delay_1(),
            self.delay_2()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor1DelaysMsb(pub u32);
impl PvtMonitor1DelaysMsb {
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_4(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn delay_5(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_delay_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for PvtMonitor1DelaysMsb {
    #[inline(always)]
    fn default() -> PvtMonitor1DelaysMsb {
        PvtMonitor1DelaysMsb(0)
    }
}
impl core::fmt::Debug for PvtMonitor1DelaysMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor1DelaysMsb")
            .field("delay_3", &self.delay_3())
            .field("delay_4", &self.delay_4())
            .field("delay_5", &self.delay_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor1DelaysMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PvtMonitor1DelaysMsb {{ delay_3: {=u16:?}, delay_4: {=u16:?}, delay_5: {=u16:?} }}",
            self.delay_3(),
            self.delay_4(),
            self.delay_5()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PvtMonitor1Ringo(pub u32);
impl PvtMonitor1Ringo {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ringo_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_freq_hz(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ringo_freq_hz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for PvtMonitor1Ringo {
    #[inline(always)]
    fn default() -> PvtMonitor1Ringo {
        PvtMonitor1Ringo(0)
    }
}
impl core::fmt::Debug for PvtMonitor1Ringo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PvtMonitor1Ringo")
            .field("ringo_valid", &self.ringo_valid())
            .field("ringo_freq_hz", &self.ringo_freq_hz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PvtMonitor1Ringo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PvtMonitor1Ringo {{ ringo_valid: {=bool:?}, ringo_freq_hz: {=u32:?} }}",
            self.ringo_valid(),
            self.ringo_freq_hz()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramsizecfg(pub u32);
impl Ramsizecfg {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_configuration(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram_configuration(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ramsizecfg {
    #[inline(always)]
    fn default() -> Ramsizecfg {
        Ramsizecfg(0)
    }
}
impl core::fmt::Debug for Ramsizecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ramsizecfg")
            .field("sram_configuration", &self.sram_configuration())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ramsizecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ramsizecfg {{ sram_configuration: {=u32:?} }}",
            self.sram_configuration()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ringo0(pub u32);
impl Ringo0 {
    #[doc = "1: RINGO_0_CTRL is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_0_ctrl_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: RINGO_0_CTRL is valid."]
    #[inline(always)]
    pub const fn set_ringo_0_ctrl_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_0_ctrl(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub const fn set_ringo_0_ctrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Ringo0 {
    #[inline(always)]
    fn default() -> Ringo0 {
        Ringo0(0)
    }
}
impl core::fmt::Debug for Ringo0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ringo0")
            .field("ringo_0_ctrl_valid", &self.ringo_0_ctrl_valid())
            .field("ringo_0_ctrl", &self.ringo_0_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ringo0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ringo0 {{ ringo_0_ctrl_valid: {=bool:?}, ringo_0_ctrl: {=u32:?} }}",
            self.ringo_0_ctrl_valid(),
            self.ringo_0_ctrl()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ringo1(pub u32);
impl Ringo1 {
    #[doc = "1: RINGO_1_CTRL is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_1_ctrl_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: RINGO_1_CTRL is valid."]
    #[inline(always)]
    pub const fn set_ringo_1_ctrl_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_1_ctrl(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub const fn set_ringo_1_ctrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Ringo1 {
    #[inline(always)]
    fn default() -> Ringo1 {
        Ringo1(0)
    }
}
impl core::fmt::Debug for Ringo1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ringo1")
            .field("ringo_1_ctrl_valid", &self.ringo_1_ctrl_valid())
            .field("ringo_1_ctrl", &self.ringo_1_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ringo1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ringo1 {{ ringo_1_ctrl_valid: {=bool:?}, ringo_1_ctrl: {=u32:?} }}",
            self.ringo_1_ctrl_valid(),
            self.ringo_1_ctrl()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ringo2(pub u32);
impl Ringo2 {
    #[doc = "1: RINGO_2_CTRL is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_2_ctrl_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: RINGO_2_CTRL is valid."]
    #[inline(always)]
    pub const fn set_ringo_2_ctrl_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ringo_2_ctrl(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub const fn set_ringo_2_ctrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Ringo2 {
    #[inline(always)]
    fn default() -> Ringo2 {
        Ringo2(0)
    }
}
impl core::fmt::Debug for Ringo2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ringo2")
            .field("ringo_2_ctrl_valid", &self.ringo_2_ctrl_valid())
            .field("ringo_2_ctrl", &self.ringo_2_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ringo2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ringo2 {{ ringo_2_ctrl_valid: {=bool:?}, ringo_2_ctrl: {=u32:?} }}",
            self.ringo_2_ctrl_valid(),
            self.ringo_2_ctrl()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdioDelay(pub u32);
impl SdioDelay {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_0_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio_0_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDIO_0_DELAY (unit: 100 ps)."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_0_delay(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "SDIO_0_DELAY (unit: 100 ps)."]
    #[inline(always)]
    pub const fn set_sdio_0_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
}
impl Default for SdioDelay {
    #[inline(always)]
    fn default() -> SdioDelay {
        SdioDelay(0)
    }
}
impl core::fmt::Debug for SdioDelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SdioDelay")
            .field("sdio_0_valid", &self.sdio_0_valid())
            .field("sdio_0_delay", &self.sdio_0_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SdioDelay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SdioDelay {{ sdio_0_valid: {=bool:?}, sdio_0_delay: {=u16:?} }}",
            self.sdio_0_valid(),
            self.sdio_0_delay()
        )
    }
}
#[doc = "SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sha256Digest(pub u32);
impl Sha256Digest {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sha256Digest {
    #[inline(always)]
    fn default() -> Sha256Digest {
        Sha256Digest(0)
    }
}
impl core::fmt::Debug for Sha256Digest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sha256Digest")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sha256Digest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sha256Digest {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TempSensOffset(pub u32);
impl TempSensOffset {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OFFSET_x1024\\[30:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn offset_x1024(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "OFFSET_x1024\\[30:0\\]"]
    #[inline(always)]
    pub const fn set_offset_x1024(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TempSensOffset {
    #[inline(always)]
    fn default() -> TempSensOffset {
        TempSensOffset(0)
    }
}
impl core::fmt::Debug for TempSensOffset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TempSensOffset")
            .field("valid", &self.valid())
            .field("offset_x1024", &self.offset_x1024())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TempSensOffset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TempSensOffset {{ valid: {=bool:?}, offset_x1024: {=u32:?} }}",
            self.valid(),
            self.offset_x1024()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TempSensSlope(pub u32);
impl TempSensSlope {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SLOPE_x1024\\[30:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn slope_x1024(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "SLOPE_x1024\\[30:0\\]"]
    #[inline(always)]
    pub const fn set_slope_x1024(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TempSensSlope {
    #[inline(always)]
    fn default() -> TempSensSlope {
        TempSensSlope(0)
    }
}
impl core::fmt::Debug for TempSensSlope {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TempSensSlope")
            .field("valid", &self.valid())
            .field("slope_x1024", &self.slope_x1024())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TempSensSlope {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TempSensSlope {{ valid: {=bool:?}, slope_x1024: {=u32:?} }}",
            self.valid(),
            self.slope_x1024()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TempSensVbe1vbe8Ref1(pub u32);
impl TempSensVbe1vbe8Ref1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbe1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbe1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbe8(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbe8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TempSensVbe1vbe8Ref1 {
    #[inline(always)]
    fn default() -> TempSensVbe1vbe8Ref1 {
        TempSensVbe1vbe8Ref1(0)
    }
}
impl core::fmt::Debug for TempSensVbe1vbe8Ref1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TempSensVbe1vbe8Ref1")
            .field("vbe1", &self.vbe1())
            .field("vbe8", &self.vbe8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TempSensVbe1vbe8Ref1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TempSensVbe1vbe8Ref1 {{ vbe1: {=u16:?}, vbe8: {=u16:?} }}",
            self.vbe1(),
            self.vbe8()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TempSensVbe1vbe8Ref2(pub u32);
impl TempSensVbe1vbe8Ref2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbe1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbe1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbe8(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbe8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TempSensVbe1vbe8Ref2 {
    #[inline(always)]
    fn default() -> TempSensVbe1vbe8Ref2 {
        TempSensVbe1vbe8Ref2(0)
    }
}
impl core::fmt::Debug for TempSensVbe1vbe8Ref2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TempSensVbe1vbe8Ref2")
            .field("vbe1", &self.vbe1())
            .field("vbe8", &self.vbe8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TempSensVbe1vbe8Ref2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TempSensVbe1vbe8Ref2 {{ vbe1: {=u16:?}, vbe8: {=u16:?} }}",
            self.vbe1(),
            self.vbe8()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcfg(pub u32);
impl Usbcfg {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_ready_time_out_ms(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_ready_time_out_ms(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_speed(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED"]
    #[inline(always)]
    pub const fn set_usb_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_use_xo32m_capa_banks(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[inline(always)]
    pub const fn set_usb_use_xo32m_capa_banks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Usbcfg {
    #[inline(always)]
    fn default() -> Usbcfg {
        Usbcfg(0)
    }
}
impl core::fmt::Debug for Usbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbcfg")
            .field("xo32m_ready_time_out_ms", &self.xo32m_ready_time_out_ms())
            .field("usb_speed", &self.usb_speed())
            .field("usb_use_xo32m_capa_banks", &self.usb_use_xo32m_capa_banks())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbcfg {{ xo32m_ready_time_out_ms: {=u8:?}, usb_speed: {=u8:?}, usb_use_xo32m_capa_banks: {=bool:?} }}",
            self.xo32m_ready_time_out_ms(),
            self.usb_speed(),
            self.usb_use_xo32m_capa_banks()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uuid0(pub u32);
impl Uuid0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uuid0 {
    #[inline(always)]
    fn default() -> Uuid0 {
        Uuid0(0)
    }
}
impl core::fmt::Debug for Uuid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uuid0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uuid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uuid0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uuid1(pub u32);
impl Uuid1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uuid1 {
    #[inline(always)]
    fn default() -> Uuid1 {
        Uuid1(0)
    }
}
impl core::fmt::Debug for Uuid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uuid1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uuid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uuid1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uuid2(pub u32);
impl Uuid2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uuid2 {
    #[inline(always)]
    fn default() -> Uuid2 {
        Uuid2(0)
    }
}
impl core::fmt::Debug for Uuid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uuid2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uuid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uuid2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uuid3(pub u32);
impl Uuid3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uuid3 {
    #[inline(always)]
    fn default() -> Uuid3 {
        Uuid3(0)
    }
}
impl core::fmt::Debug for Uuid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uuid3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uuid3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uuid3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UuidArray0(pub u32);
impl UuidArray0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UuidArray0 {
    #[inline(always)]
    fn default() -> UuidArray0 {
        UuidArray0(0)
    }
}
impl core::fmt::Debug for UuidArray0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UuidArray0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UuidArray0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UuidArray0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UuidArray1(pub u32);
impl UuidArray1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UuidArray1 {
    #[inline(always)]
    fn default() -> UuidArray1 {
        UuidArray1(0)
    }
}
impl core::fmt::Debug for UuidArray1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UuidArray1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UuidArray1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UuidArray1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UuidArray2(pub u32);
impl UuidArray2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UuidArray2 {
    #[inline(always)]
    fn default() -> UuidArray2 {
        UuidArray2(0)
    }
}
impl core::fmt::Debug for UuidArray2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UuidArray2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UuidArray2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UuidArray2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UuidArray3(pub u32);
impl UuidArray3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UuidArray3 {
    #[inline(always)]
    fn default() -> UuidArray3 {
        UuidArray3(0)
    }
}
impl core::fmt::Debug for UuidArray3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UuidArray3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UuidArray3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UuidArray3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaferTest1Date(pub u32);
impl WaferTest1Date {
    #[doc = "WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn wt1_date(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub const fn set_wt1_date(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WaferTest1Date {
    #[inline(always)]
    fn default() -> WaferTest1Date {
        WaferTest1Date(0)
    }
}
impl core::fmt::Debug for WaferTest1Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WaferTest1Date")
            .field("wt1_date", &self.wt1_date())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WaferTest1Date {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WaferTest1Date {{ wt1_date: {=u32:?} }}",
            self.wt1_date()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaferTest1ProgramVersion(pub u32);
impl WaferTest1ProgramVersion {
    #[doc = "WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn wt1_program_version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub const fn set_wt1_program_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WaferTest1ProgramVersion {
    #[inline(always)]
    fn default() -> WaferTest1ProgramVersion {
        WaferTest1ProgramVersion(0)
    }
}
impl core::fmt::Debug for WaferTest1ProgramVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WaferTest1ProgramVersion")
            .field("wt1_program_version", &self.wt1_program_version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WaferTest1ProgramVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WaferTest1ProgramVersion {{ wt1_program_version: {=u32:?} }}",
            self.wt1_program_version()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaferTest1Time(pub u32);
impl WaferTest1Time {
    #[doc = "WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn wt1_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub const fn set_wt1_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WaferTest1Time {
    #[inline(always)]
    fn default() -> WaferTest1Time {
        WaferTest1Time(0)
    }
}
impl core::fmt::Debug for WaferTest1Time {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WaferTest1Time")
            .field("wt1_time", &self.wt1_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WaferTest1Time {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WaferTest1Time {{ wt1_time: {=u32:?} }}",
            self.wt1_time()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaferTest2Date(pub u32);
impl WaferTest2Date {
    #[doc = "WT2_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn wt2_date(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT2_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub const fn set_wt2_date(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WaferTest2Date {
    #[inline(always)]
    fn default() -> WaferTest2Date {
        WaferTest2Date(0)
    }
}
impl core::fmt::Debug for WaferTest2Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WaferTest2Date")
            .field("wt2_date", &self.wt2_date())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WaferTest2Date {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WaferTest2Date {{ wt2_date: {=u32:?} }}",
            self.wt2_date()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaferTest2ProgramVersion(pub u32);
impl WaferTest2ProgramVersion {
    #[doc = "WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn wt2_program_version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub const fn set_wt2_program_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WaferTest2ProgramVersion {
    #[inline(always)]
    fn default() -> WaferTest2ProgramVersion {
        WaferTest2ProgramVersion(0)
    }
}
impl core::fmt::Debug for WaferTest2ProgramVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WaferTest2ProgramVersion")
            .field("wt2_program_version", &self.wt2_program_version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WaferTest2ProgramVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WaferTest2ProgramVersion {{ wt2_program_version: {=u32:?} }}",
            self.wt2_program_version()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaferTest2Time(pub u32);
impl WaferTest2Time {
    #[doc = "WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn wt2_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub const fn set_wt2_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WaferTest2Time {
    #[inline(always)]
    fn default() -> WaferTest2Time {
        WaferTest2Time(0)
    }
}
impl core::fmt::Debug for WaferTest2Time {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WaferTest2Time")
            .field("wt2_time", &self.wt2_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WaferTest2Time {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WaferTest2Time {{ wt2_time: {=u32:?} }}",
            self.wt2_time()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xo32khz(pub u32);
impl Xo32khz {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32k_xin_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32k_xin_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32k_xin_capcal_6pf(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32k_xin_capcal_6pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32k_xin_capcal_8pf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32k_xin_capcal_8pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32k_xout_trim_valid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32k_xout_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32k_xout_capcal_6pf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32k_xout_capcal_6pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32k_xout_capcal_8pf(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32k_xout_capcal_8pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
}
impl Default for Xo32khz {
    #[inline(always)]
    fn default() -> Xo32khz {
        Xo32khz(0)
    }
}
impl core::fmt::Debug for Xo32khz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xo32khz")
            .field("xo32k_xin_trim_valid", &self.xo32k_xin_trim_valid())
            .field("xo32k_xin_capcal_6pf", &self.xo32k_xin_capcal_6pf())
            .field("xo32k_xin_capcal_8pf", &self.xo32k_xin_capcal_8pf())
            .field("xo32k_xout_trim_valid", &self.xo32k_xout_trim_valid())
            .field("xo32k_xout_capcal_6pf", &self.xo32k_xout_capcal_6pf())
            .field("xo32k_xout_capcal_8pf", &self.xo32k_xout_capcal_8pf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xo32khz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xo32khz {{ xo32k_xin_trim_valid: {=bool:?}, xo32k_xin_capcal_6pf: {=u8:?}, xo32k_xin_capcal_8pf: {=u8:?}, xo32k_xout_trim_valid: {=bool:?}, xo32k_xout_capcal_6pf: {=u8:?}, xo32k_xout_capcal_8pf: {=u8:?} }}",
            self.xo32k_xin_trim_valid(),
            self.xo32k_xin_capcal_6pf(),
            self.xo32k_xin_capcal_8pf(),
            self.xo32k_xout_trim_valid(),
            self.xo32k_xout_capcal_6pf(),
            self.xo32k_xout_capcal_8pf()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xo32mhz(pub u32);
impl Xo32mhz {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xin_trim_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xin_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xin_capcal_6pf(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xin_capcal_6pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xin_capcal_8pf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xin_capcal_8pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xout_trim_valid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xout_trim_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xout_capcal_6pf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xout_capcal_6pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xout_capcal_8pf(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xout_capcal_8pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xo_slave_status(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xo_slave_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn xo32m_xo_ac_buf_status(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_xo32m_xo_ac_buf_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Xo32mhz {
    #[inline(always)]
    fn default() -> Xo32mhz {
        Xo32mhz(0)
    }
}
impl core::fmt::Debug for Xo32mhz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xo32mhz")
            .field("xo32m_xin_trim_valid", &self.xo32m_xin_trim_valid())
            .field("xo32m_xin_capcal_6pf", &self.xo32m_xin_capcal_6pf())
            .field("xo32m_xin_capcal_8pf", &self.xo32m_xin_capcal_8pf())
            .field("xo32m_xout_trim_valid", &self.xo32m_xout_trim_valid())
            .field("xo32m_xout_capcal_6pf", &self.xo32m_xout_capcal_6pf())
            .field("xo32m_xout_capcal_8pf", &self.xo32m_xout_capcal_8pf())
            .field("xo32m_xo_slave_status", &self.xo32m_xo_slave_status())
            .field("xo32m_xo_ac_buf_status", &self.xo32m_xo_ac_buf_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xo32mhz {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xo32mhz {{ xo32m_xin_trim_valid: {=bool:?}, xo32m_xin_capcal_6pf: {=u8:?}, xo32m_xin_capcal_8pf: {=u8:?}, xo32m_xout_trim_valid: {=bool:?}, xo32m_xout_capcal_6pf: {=u8:?}, xo32m_xout_capcal_8pf: {=u8:?}, xo32m_xo_slave_status: {=bool:?}, xo32m_xo_ac_buf_status: {=bool:?} }}",
            self.xo32m_xin_trim_valid(),
            self.xo32m_xin_capcal_6pf(),
            self.xo32m_xin_capcal_8pf(),
            self.xo32m_xout_trim_valid(),
            self.xo32m_xout_capcal_6pf(),
            self.xo32m_xout_capcal_8pf(),
            self.xo32m_xo_slave_status(),
            self.xo32m_xo_ac_buf_status()
        )
    }
}
