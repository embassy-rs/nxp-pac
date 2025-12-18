#[doc = "FLASH_ROMPATCH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashRompatch {
    ptr: *mut u8,
}
unsafe impl Send for FlashRompatch {}
unsafe impl Sync for FlashRompatch {}
impl FlashRompatch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn header(self) -> crate::common::Reg<regs::Header, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn patch(self, n: usize) -> crate::common::Reg<regs::Patch, crate::common::RW> {
        assert!(n < 255usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _) }
    }
}
pub mod regs;
