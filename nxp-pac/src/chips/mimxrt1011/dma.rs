#[doc = "DMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Error Status."]
    #[inline(always)]
    pub const fn es(self) -> crate::common::Reg<regs::Es, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Enable Request."]
    #[inline(always)]
    pub const fn erq(self) -> crate::common::Reg<regs::Erq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Enable Error Interrupt."]
    #[inline(always)]
    pub const fn eei(self) -> crate::common::Reg<regs::Eei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Clear Enable Error Interrupt."]
    #[inline(always)]
    pub const fn ceei(self) -> crate::common::Reg<regs::Ceei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Set Enable Error Interrupt."]
    #[inline(always)]
    pub const fn seei(self) -> crate::common::Reg<regs::Seei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "Clear Enable Request."]
    #[inline(always)]
    pub const fn cerq(self) -> crate::common::Reg<regs::Cerq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Set Enable Request."]
    #[inline(always)]
    pub const fn serq(self) -> crate::common::Reg<regs::Serq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "Clear DONE Status Bit."]
    #[inline(always)]
    pub const fn cdne(self) -> crate::common::Reg<regs::Cdne, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Set START Bit."]
    #[inline(always)]
    pub const fn ssrt(self) -> crate::common::Reg<regs::Ssrt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "Clear Error."]
    #[inline(always)]
    pub const fn cerr(self) -> crate::common::Reg<regs::Cerr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Clear Interrupt Request."]
    #[inline(always)]
    pub const fn cint(self) -> crate::common::Reg<regs::Cint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "Interrupt Request."]
    #[inline(always)]
    pub const fn int(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Error."]
    #[inline(always)]
    pub const fn err(self) -> crate::common::Reg<regs::Err, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Hardware Request Status."]
    #[inline(always)]
    pub const fn hrs(self) -> crate::common::Reg<regs::Hrs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Enable Asynchronous Request in Stop."]
    #[inline(always)]
    pub const fn ears(self) -> crate::common::Reg<regs::Ears, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri3(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri2(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0101usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri1(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri0(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0103usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri7(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri6(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0105usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri5(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri4(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0107usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri11(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri10(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0109usize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri9(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri8(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010busize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri15(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri14(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010dusize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri13(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010eusize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn dchpri12(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010fusize) as _) }
    }
    #[doc = "Array of registers: TCD_ATTR, TCD_BITER_ELINKNO, TCD_BITER_ELINKYES, TCD_CITER_ELINKNO, TCD_CITER_ELINKYES, TCD_CSR, TCD_DADDR, TCD_DLASTSGA, TCD_DOFF, TCD_NBYTES_MLNO, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SADDR, TCD_SLAST, TCD_SOFF."]
    #[inline(always)]
    pub const fn tcd(self, n: usize) -> Tcd {
        assert!(n < 16usize);
        unsafe { Tcd::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 32usize) as _) }
    }
}
#[doc = "Array of registers: TCD_ATTR, TCD_BITER_ELINKNO, TCD_BITER_ELINKYES, TCD_CITER_ELINKNO, TCD_CITER_ELINKYES, TCD_CSR, TCD_DADDR, TCD_DLASTSGA, TCD_DOFF, TCD_NBYTES_MLNO, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SADDR, TCD_SLAST, TCD_SOFF."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd {
    ptr: *mut u8,
}
unsafe impl Send for Tcd {}
unsafe impl Sync for Tcd {}
impl Tcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TCD Source Address."]
    #[inline(always)]
    pub const fn tcd_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset."]
    #[inline(always)]
    pub const fn tcd_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "TCD Transfer Attributes."]
    #[inline(always)]
    pub const fn tcd_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)."]
    #[inline(always)]
    pub const fn tcd_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)."]
    #[inline(always)]
    pub const fn tcd_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)."]
    #[inline(always)]
    pub const fn tcd_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment."]
    #[inline(always)]
    pub const fn tcd_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TCD Destination Address."]
    #[inline(always)]
    pub const fn tcd_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset."]
    #[inline(always)]
    pub const fn tcd_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)."]
    #[inline(always)]
    pub const fn tcd_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)."]
    #[inline(always)]
    pub const fn tcd_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address."]
    #[inline(always)]
    pub const fn tcd_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TCD Control and Status."]
    #[inline(always)]
    pub const fn tcd_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)."]
    #[inline(always)]
    pub const fn tcd_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)."]
    #[inline(always)]
    pub const fn tcd_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
}
pub mod regs;
pub mod vals;
