#[doc = "PUF SRAM Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PufSramCtrl {
    ptr: *mut u8,
}
unsafe impl Send for PufSramCtrl {}
unsafe impl Sync for PufSramCtrl {}
impl PufSramCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration Register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn int_clr_enable(self) -> crate::common::Reg<regs::IntClrEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn int_set_enable(self) -> crate::common::Reg<regs::IntSetEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_enable(self) -> crate::common::Reg<regs::IntEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn int_clr_status(self) -> crate::common::Reg<regs::IntClrStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Interrupt Status set"]
    #[inline(always)]
    pub const fn int_set_status(self) -> crate::common::Reg<regs::IntSetStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
}
pub mod regs;
