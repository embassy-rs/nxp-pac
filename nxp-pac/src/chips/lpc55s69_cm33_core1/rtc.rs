#[doc = "Real-Time Clock (RTC)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC match register."]
    #[inline(always)]
    pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC counter register."]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "High-resolution/wake-up timer control register."]
    #[inline(always)]
    pub const fn wake(self) -> crate::common::Reg<regs::Wake, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Sub-second counter register."]
    #[inline(always)]
    pub const fn subsec(self) -> crate::common::Reg<regs::Subsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg0(self) -> crate::common::Reg<regs::Gpreg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg1(self) -> crate::common::Reg<regs::Gpreg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg2(self) -> crate::common::Reg<regs::Gpreg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg3(self) -> crate::common::Reg<regs::Gpreg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg4(self) -> crate::common::Reg<regs::Gpreg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg5(self) -> crate::common::Reg<regs::Gpreg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg6(self) -> crate::common::Reg<regs::Gpreg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn gpreg7(self) -> crate::common::Reg<regs::Gpreg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
