#[doc = "I/O pin configuration (IOCON)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocon {
    ptr: *mut u8,
}
unsafe impl Send for Iocon {}
unsafe impl Sync for Iocon {}
impl Iocon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_0"]
    #[inline(always)]
    pub const fn pio0_0(self) -> crate::common::Reg<regs::Pio00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_1"]
    #[inline(always)]
    pub const fn pio0_1(self) -> crate::common::Reg<regs::Pio01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_2"]
    #[inline(always)]
    pub const fn pio0_2(self) -> crate::common::Reg<regs::Pio02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_3"]
    #[inline(always)]
    pub const fn pio0_3(self) -> crate::common::Reg<regs::Pio03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_4"]
    #[inline(always)]
    pub const fn pio0_4(self) -> crate::common::Reg<regs::Pio04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_5"]
    #[inline(always)]
    pub const fn pio0_5(self) -> crate::common::Reg<regs::Pio05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_6"]
    #[inline(always)]
    pub const fn pio0_6(self) -> crate::common::Reg<regs::Pio06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_7"]
    #[inline(always)]
    pub const fn pio0_7(self) -> crate::common::Reg<regs::Pio07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_8"]
    #[inline(always)]
    pub const fn pio0_8(self) -> crate::common::Reg<regs::Pio08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_9"]
    #[inline(always)]
    pub const fn pio0_9(self) -> crate::common::Reg<regs::Pio09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_10"]
    #[inline(always)]
    pub const fn pio0_10(self) -> crate::common::Reg<regs::Pio010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_11"]
    #[inline(always)]
    pub const fn pio0_11(self) -> crate::common::Reg<regs::Pio011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_12"]
    #[inline(always)]
    pub const fn pio0_12(self) -> crate::common::Reg<regs::Pio012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_13"]
    #[inline(always)]
    pub const fn pio0_13(self) -> crate::common::Reg<regs::Pio013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_14"]
    #[inline(always)]
    pub const fn pio0_14(self) -> crate::common::Reg<regs::Pio014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_15"]
    #[inline(always)]
    pub const fn pio0_15(self) -> crate::common::Reg<regs::Pio015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_16"]
    #[inline(always)]
    pub const fn pio0_16(self) -> crate::common::Reg<regs::Pio016, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_17"]
    #[inline(always)]
    pub const fn pio0_17(self) -> crate::common::Reg<regs::Pio017, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_18"]
    #[inline(always)]
    pub const fn pio0_18(self) -> crate::common::Reg<regs::Pio018, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_19"]
    #[inline(always)]
    pub const fn pio0_19(self) -> crate::common::Reg<regs::Pio019, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_20"]
    #[inline(always)]
    pub const fn pio0_20(self) -> crate::common::Reg<regs::Pio020, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_21"]
    #[inline(always)]
    pub const fn pio0_21(self) -> crate::common::Reg<regs::Pio021, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_22"]
    #[inline(always)]
    pub const fn pio0_22(self) -> crate::common::Reg<regs::Pio022, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_23"]
    #[inline(always)]
    pub const fn pio0_23(self) -> crate::common::Reg<regs::Pio023, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_24"]
    #[inline(always)]
    pub const fn pio0_24(self) -> crate::common::Reg<regs::Pio024, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_25"]
    #[inline(always)]
    pub const fn pio0_25(self) -> crate::common::Reg<regs::Pio025, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_26"]
    #[inline(always)]
    pub const fn pio0_26(self) -> crate::common::Reg<regs::Pio026, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_27"]
    #[inline(always)]
    pub const fn pio0_27(self) -> crate::common::Reg<regs::Pio027, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_28"]
    #[inline(always)]
    pub const fn pio0_28(self) -> crate::common::Reg<regs::Pio028, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_29"]
    #[inline(always)]
    pub const fn pio0_29(self) -> crate::common::Reg<regs::Pio029, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_30"]
    #[inline(always)]
    pub const fn pio0_30(self) -> crate::common::Reg<regs::Pio030, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_31"]
    #[inline(always)]
    pub const fn pio0_31(self) -> crate::common::Reg<regs::Pio031, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_0"]
    #[inline(always)]
    pub const fn pio1_0(self) -> crate::common::Reg<regs::Pio10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_1"]
    #[inline(always)]
    pub const fn pio1_1(self) -> crate::common::Reg<regs::Pio11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_2"]
    #[inline(always)]
    pub const fn pio1_2(self) -> crate::common::Reg<regs::Pio12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_3"]
    #[inline(always)]
    pub const fn pio1_3(self) -> crate::common::Reg<regs::Pio13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_4"]
    #[inline(always)]
    pub const fn pio1_4(self) -> crate::common::Reg<regs::Pio14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_5"]
    #[inline(always)]
    pub const fn pio1_5(self) -> crate::common::Reg<regs::Pio15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_6"]
    #[inline(always)]
    pub const fn pio1_6(self) -> crate::common::Reg<regs::Pio16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_7"]
    #[inline(always)]
    pub const fn pio1_7(self) -> crate::common::Reg<regs::Pio17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_8"]
    #[inline(always)]
    pub const fn pio1_8(self) -> crate::common::Reg<regs::Pio18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_9"]
    #[inline(always)]
    pub const fn pio1_9(self) -> crate::common::Reg<regs::Pio19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_10"]
    #[inline(always)]
    pub const fn pio1_10(self) -> crate::common::Reg<regs::Pio110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_11"]
    #[inline(always)]
    pub const fn pio1_11(self) -> crate::common::Reg<regs::Pio111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_12"]
    #[inline(always)]
    pub const fn pio1_12(self) -> crate::common::Reg<regs::Pio112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_13"]
    #[inline(always)]
    pub const fn pio1_13(self) -> crate::common::Reg<regs::Pio113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_14"]
    #[inline(always)]
    pub const fn pio1_14(self) -> crate::common::Reg<regs::Pio114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_15"]
    #[inline(always)]
    pub const fn pio1_15(self) -> crate::common::Reg<regs::Pio115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_16"]
    #[inline(always)]
    pub const fn pio1_16(self) -> crate::common::Reg<regs::Pio116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_17"]
    #[inline(always)]
    pub const fn pio1_17(self) -> crate::common::Reg<regs::Pio117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_18"]
    #[inline(always)]
    pub const fn pio1_18(self) -> crate::common::Reg<regs::Pio118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_19"]
    #[inline(always)]
    pub const fn pio1_19(self) -> crate::common::Reg<regs::Pio119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_20"]
    #[inline(always)]
    pub const fn pio1_20(self) -> crate::common::Reg<regs::Pio120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_21"]
    #[inline(always)]
    pub const fn pio1_21(self) -> crate::common::Reg<regs::Pio121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_22"]
    #[inline(always)]
    pub const fn pio1_22(self) -> crate::common::Reg<regs::Pio122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_23"]
    #[inline(always)]
    pub const fn pio1_23(self) -> crate::common::Reg<regs::Pio123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_24"]
    #[inline(always)]
    pub const fn pio1_24(self) -> crate::common::Reg<regs::Pio124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_25"]
    #[inline(always)]
    pub const fn pio1_25(self) -> crate::common::Reg<regs::Pio125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_26"]
    #[inline(always)]
    pub const fn pio1_26(self) -> crate::common::Reg<regs::Pio126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_27"]
    #[inline(always)]
    pub const fn pio1_27(self) -> crate::common::Reg<regs::Pio127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_28"]
    #[inline(always)]
    pub const fn pio1_28(self) -> crate::common::Reg<regs::Pio128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_29"]
    #[inline(always)]
    pub const fn pio1_29(self) -> crate::common::Reg<regs::Pio129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_30"]
    #[inline(always)]
    pub const fn pio1_30(self) -> crate::common::Reg<regs::Pio130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_31"]
    #[inline(always)]
    pub const fn pio1_31(self) -> crate::common::Reg<regs::Pio131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
