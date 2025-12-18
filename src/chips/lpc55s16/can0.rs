#[doc = "Controller Area Network Flexible Data (CAN FD)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0 {
    ptr: *mut u8,
}
unsafe impl Send for Can0 {}
unsafe impl Sync for Can0 {}
impl Can0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data Bit Timing Prescaler Register"]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Test Register"]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CC Control Register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Timestamp Counter Configuration"]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Timestamp Counter Value"]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Timeout Counter Configuration"]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Timeout Counter Value"]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::Tocv, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Transmitter Delay Compensator Register"]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Interrupt Line Select"]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt Line Enable"]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Global Filter Configuration"]
    #[inline(always)]
    pub const fn gfc(self) -> crate::common::Reg<regs::Gfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Standard ID Filter Configuration"]
    #[inline(always)]
    pub const fn sidfc(self) -> crate::common::Reg<regs::Sidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Extended ID Filter Configuration"]
    #[inline(always)]
    pub const fn xidfc(self) -> crate::common::Reg<regs::Xidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Extended ID AND Mask"]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "High Priority Message Status"]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::Hpms, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "New Data 1"]
    #[inline(always)]
    pub const fn ndat1(self) -> crate::common::Reg<regs::Ndat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "New Data 2"]
    #[inline(always)]
    pub const fn ndat2(self) -> crate::common::Reg<regs::Ndat2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Rx FIFO 0 Configuration"]
    #[inline(always)]
    pub const fn rxf0c(self) -> crate::common::Reg<regs::Rxf0c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Rx FIFO 0 Status"]
    #[inline(always)]
    pub const fn rxf0s(self) -> crate::common::Reg<regs::Rxf0s, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Rx FIFO 0 Acknowledge"]
    #[inline(always)]
    pub const fn rxf0a(self) -> crate::common::Reg<regs::Rxf0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Rx Buffer Configuration"]
    #[inline(always)]
    pub const fn rxbc(self) -> crate::common::Reg<regs::Rxbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Rx FIFO 1 Configuration"]
    #[inline(always)]
    pub const fn rxf1c(self) -> crate::common::Reg<regs::Rxf1c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Rx FIFO 1 Status"]
    #[inline(always)]
    pub const fn rxf1s(self) -> crate::common::Reg<regs::Rxf1s, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Rx FIFO 1 Acknowledge"]
    #[inline(always)]
    pub const fn rxf1a(self) -> crate::common::Reg<regs::Rxf1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Rx Buffer and FIFO Element Size Configuration"]
    #[inline(always)]
    pub const fn rxesc(self) -> crate::common::Reg<regs::Rxesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Tx Buffer Configuration"]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Tx FIFO/Queue Status"]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::Txfqs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Tx Buffer Element Size Configuration"]
    #[inline(always)]
    pub const fn txesc(self) -> crate::common::Reg<regs::Txesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Tx Buffer Request Pending"]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::Txbrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Tx Buffer Add Request"]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Request"]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Tx Buffer Transmission Occurred"]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::Txbto, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Finished"]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::Txbcf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Tx Buffer Transmission Interrupt Enable"]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Tx Event FIFO Configuration"]
    #[inline(always)]
    pub const fn txefc(self) -> crate::common::Reg<regs::Txefc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Tx Event FIFO Status"]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::Txefs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Tx Event FIFO Acknowledge"]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "CAN Message RAM Base Address"]
    #[inline(always)]
    pub const fn mrba(self) -> crate::common::Reg<regs::Mrba, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "External Timestamp Counter Configuration"]
    #[inline(always)]
    pub const fn etscc(self) -> crate::common::Reg<regs::Etscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "External Timestamp Counter Value"]
    #[inline(always)]
    pub const fn etscv(self) -> crate::common::Reg<regs::Etscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
}
pub mod regs;
