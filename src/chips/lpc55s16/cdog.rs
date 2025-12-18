#[doc = "CDOG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdog {
    ptr: *mut u8,
}
unsafe impl Send for Cdog {}
unsafe impl Sync for Cdog {}
impl Cdog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Instruction timer reload"]
    #[inline(always)]
    pub const fn reload(self) -> crate::common::Reg<regs::Reload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "The INSTRUCTION TIMER itself"]
    #[inline(always)]
    pub const fn instruction_timer(
        self,
    ) -> crate::common::Reg<regs::InstructionTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Also known as SEC_CNT"]
    #[inline(always)]
    pub const fn secure_counter(
        self,
    ) -> crate::common::Reg<regs::SecureCounter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status register (1 of 2)"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "STATUS register (2 of 2)"]
    #[inline(always)]
    pub const fn status2(self) -> crate::common::Reg<regs::Status2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Hardware flags"]
    #[inline(always)]
    pub const fn flags(self) -> crate::common::Reg<regs::Flags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Persistent (Ad. Hoc., quasi-NV) data storage"]
    #[inline(always)]
    pub const fn persistent(self) -> crate::common::Reg<regs::Persistent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Write address for issuing the START command."]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Write address for issuing the STOP command."]
    #[inline(always)]
    pub const fn stop(self) -> crate::common::Reg<regs::Stop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Write address for issuing the RESTART command."]
    #[inline(always)]
    pub const fn restart(self) -> crate::common::Reg<regs::Restart, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Write address for issuing the ADD command."]
    #[inline(always)]
    pub const fn add(self) -> crate::common::Reg<regs::Add, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Write address for issuing the ADD1 command."]
    #[inline(always)]
    pub const fn add1(self) -> crate::common::Reg<regs::Add1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Write address for issuing the ADD16 command."]
    #[inline(always)]
    pub const fn add16(self) -> crate::common::Reg<regs::Add16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Write address for issuing the ADD16 command."]
    #[inline(always)]
    pub const fn add256(self) -> crate::common::Reg<regs::Add256, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Write address for issuing the SUB command."]
    #[inline(always)]
    pub const fn sub(self) -> crate::common::Reg<regs::Sub, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Write address for issuing the SUB1 command."]
    #[inline(always)]
    pub const fn sub1(self) -> crate::common::Reg<regs::Sub1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Write address for issuing the SUB16 command."]
    #[inline(always)]
    pub const fn sub16(self) -> crate::common::Reg<regs::Sub16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Write address for issuing the SUB256 command."]
    #[inline(always)]
    pub const fn sub256(self) -> crate::common::Reg<regs::Sub256, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs;
