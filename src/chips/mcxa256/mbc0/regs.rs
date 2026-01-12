#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW0(pub u32);
impl Mbc0Dom0Mem0BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW0 {
        Mbc0Dom0Mem0BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW0")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW0 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW1(pub u32);
impl Mbc0Dom0Mem0BlkCfgW1 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW1 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW1 {
        Mbc0Dom0Mem0BlkCfgW1(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW1")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW1 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW10(pub u32);
impl Mbc0Dom0Mem0BlkCfgW10 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW10Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW10Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW10Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW10 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW10 {
        Mbc0Dom0Mem0BlkCfgW10(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW10")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW10 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW11(pub u32);
impl Mbc0Dom0Mem0BlkCfgW11 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW11Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW11Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW11Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW11 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW11 {
        Mbc0Dom0Mem0BlkCfgW11(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW11")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW11 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW12(pub u32);
impl Mbc0Dom0Mem0BlkCfgW12 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW12Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW12Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW12Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW12 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW12 {
        Mbc0Dom0Mem0BlkCfgW12(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW12")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW12 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW13(pub u32);
impl Mbc0Dom0Mem0BlkCfgW13 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW13Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW13Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW13Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW13 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW13 {
        Mbc0Dom0Mem0BlkCfgW13(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW13")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW13 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW14(pub u32);
impl Mbc0Dom0Mem0BlkCfgW14 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW14Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW14Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW14Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW14 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW14 {
        Mbc0Dom0Mem0BlkCfgW14(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW14")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW14 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW15(pub u32);
impl Mbc0Dom0Mem0BlkCfgW15 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW15Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW15Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW15Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW15 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW15 {
        Mbc0Dom0Mem0BlkCfgW15(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW15")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW15 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW2(pub u32);
impl Mbc0Dom0Mem0BlkCfgW2 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW2 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW2 {
        Mbc0Dom0Mem0BlkCfgW2(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW2")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW2 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW3(pub u32);
impl Mbc0Dom0Mem0BlkCfgW3 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW3 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW3 {
        Mbc0Dom0Mem0BlkCfgW3(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW3")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW3 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW4(pub u32);
impl Mbc0Dom0Mem0BlkCfgW4 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW4 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW4 {
        Mbc0Dom0Mem0BlkCfgW4(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW4")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW4 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW5(pub u32);
impl Mbc0Dom0Mem0BlkCfgW5 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW5 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW5 {
        Mbc0Dom0Mem0BlkCfgW5(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW5")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW5 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW6(pub u32);
impl Mbc0Dom0Mem0BlkCfgW6 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW6 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW6 {
        Mbc0Dom0Mem0BlkCfgW6(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW6")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW6 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW7(pub u32);
impl Mbc0Dom0Mem0BlkCfgW7 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW7 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW7 {
        Mbc0Dom0Mem0BlkCfgW7(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW7")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW7 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW8(pub u32);
impl Mbc0Dom0Mem0BlkCfgW8 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW8Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW8Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW8Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW8 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW8 {
        Mbc0Dom0Mem0BlkCfgW8(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW8")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW8 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW9(pub u32);
impl Mbc0Dom0Mem0BlkCfgW9 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW9Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW9Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW9Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW9 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW9 {
        Mbc0Dom0Mem0BlkCfgW9(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW9")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW9 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem1BlkCfgW0(pub u32);
impl Mbc0Dom0Mem1BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem1BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem1BlkCfgW0 {
        Mbc0Dom0Mem1BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem1BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem1BlkCfgW0")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem1BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem1BlkCfgW0 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem1BlkCfgW1(pub u32);
impl Mbc0Dom0Mem1BlkCfgW1 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW1Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW1Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW1Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem1BlkCfgW1 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem1BlkCfgW1 {
        Mbc0Dom0Mem1BlkCfgW1(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem1BlkCfgW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem1BlkCfgW1")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem1BlkCfgW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem1BlkCfgW1 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem2BlkCfgW0(pub u32);
impl Mbc0Dom0Mem2BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem2BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem2BlkCfgW0 {
        Mbc0Dom0Mem2BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem2BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem2BlkCfgW0")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem2BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem2BlkCfgW0 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem0Glbcfg(pub u32);
impl Mbc0Mem0Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem0Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem0Glbcfg {
        Mbc0Mem0Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem0Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem0Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem0Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem0Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem1Glbcfg(pub u32);
impl Mbc0Mem1Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem1Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem1Glbcfg {
        Mbc0Mem1Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem1Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem1Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem1Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem1Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem2Glbcfg(pub u32);
impl Mbc0Mem2Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem2Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem2Glbcfg {
        Mbc0Mem2Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem2Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem2Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem2Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem2Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem3Glbcfg(pub u32);
impl Mbc0Mem3Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Clear Error"]
    #[must_use]
    #[inline(always)]
    pub const fn clre(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Clear Error"]
    #[inline(always)]
    pub const fn set_clre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Mbc0Mem3Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem3Glbcfg {
        Mbc0Mem3Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem3Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem3Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .field("clre", &self.clre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem3Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem3Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?}, clre: {=u8:?} }}",
            self.nblks(),
            self.size_log2(),
            self.clre()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac0(pub u32);
impl Mbc0MemnGlbac0 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mbc0MemnGlbac0 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac0 {
        Mbc0MemnGlbac0(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac0")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac0 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac1(pub u32);
impl Mbc0MemnGlbac1 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac1 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac1 {
        Mbc0MemnGlbac1(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac1")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac1 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac2(pub u32);
impl Mbc0MemnGlbac2 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac2 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac2 {
        Mbc0MemnGlbac2(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac2")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac2 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac3(pub u32);
impl Mbc0MemnGlbac3 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac3 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac3 {
        Mbc0MemnGlbac3(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac3")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac3 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac4(pub u32);
impl Mbc0MemnGlbac4 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac4 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac4 {
        Mbc0MemnGlbac4(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac4")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac4 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac5(pub u32);
impl Mbc0MemnGlbac5 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac5 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac5 {
        Mbc0MemnGlbac5(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac5")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac5 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac6(pub u32);
impl Mbc0MemnGlbac6 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac6 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac6 {
        Mbc0MemnGlbac6(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac6")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac6 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac7(pub u32);
impl Mbc0MemnGlbac7 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac7 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac7 {
        Mbc0MemnGlbac7(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac7")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac7 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
