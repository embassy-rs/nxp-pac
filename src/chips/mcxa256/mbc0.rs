#[doc = "TRDC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0 {
    ptr: *mut u8,
}
unsafe impl Send for Mbc0 {}
unsafe impl Sync for Mbc0 {}
impl Mbc0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem0_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem0Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem1_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem1Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem2_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem2Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem3_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem3Glbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac0(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac1(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac2(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac3(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac4(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac5(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac6(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac7(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w8(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w9(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w10(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w11(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w12(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w13(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w14(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w15(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem1BlkCfgW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem1BlkCfgW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem2BlkCfgW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
