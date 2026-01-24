#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Header(pub u32);
impl Header {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn entries(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_entries(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn sub_type(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_sub_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn identifier(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_identifier(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Header {
    #[inline(always)]
    fn default() -> Header {
        Header(0)
    }
}
impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("entries", &self.entries())
            .field("sub_type", &self.sub_type())
            .field("type_", &self.type_())
            .field("identifier", &self.identifier())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Header {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Header {{ entries: {=u8:?}, sub_type: {=u8:?}, type_: {=u8:?}, identifier: {=u8:?} }}",
            self.entries(),
            self.sub_type(),
            self.type_(),
            self.identifier()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Patch(pub u32);
impl Patch {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn patch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_patch(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Patch {
    #[inline(always)]
    fn default() -> Patch {
        Patch(0)
    }
}
impl core::fmt::Debug for Patch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Patch")
            .field("patch", &self.patch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Patch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Patch {{ patch: {=u32:?} }}", self.patch())
    }
}
