#[doc = "CC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cccr(pub u32);
impl Cccr {
    #[doc = "Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Configuration change enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cce(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration change enable."]
    #[inline(always)]
    pub const fn set_cce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Restricted operational mode."]
    #[must_use]
    #[inline(always)]
    pub const fn asm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Restricted operational mode."]
    #[inline(always)]
    pub const fn set_asm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Stop Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn csa(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Acknowledge."]
    #[inline(always)]
    pub const fn set_csa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Stop Request."]
    #[must_use]
    #[inline(always)]
    pub const fn csr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Request."]
    #[inline(always)]
    pub const fn set_csr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bus monitoring mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mon(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bus monitoring mode."]
    #[inline(always)]
    pub const fn set_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable automatic retransmission."]
    #[must_use]
    #[inline(always)]
    pub const fn dar(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disable automatic retransmission."]
    #[inline(always)]
    pub const fn set_dar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Test mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Test mode enable."]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CAN FD operation enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fdoe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CAN FD operation enable."]
    #[inline(always)]
    pub const fn set_fdoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When CAN FD operation is disabled, this bit is not evaluated."]
    #[must_use]
    #[inline(always)]
    pub const fn brse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When CAN FD operation is disabled, this bit is not evaluated."]
    #[inline(always)]
    pub const fn set_brse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protocol exception handling disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pxhd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol exception handling disable."]
    #[inline(always)]
    pub const fn set_pxhd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Edge filtering during bus integration."]
    #[must_use]
    #[inline(always)]
    pub const fn efbi(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Edge filtering during bus integration."]
    #[inline(always)]
    pub const fn set_efbi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Transmit pause."]
    #[must_use]
    #[inline(always)]
    pub const fn txp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit pause."]
    #[inline(always)]
    pub const fn set_txp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Non ISO operation."]
    #[must_use]
    #[inline(always)]
    pub const fn niso(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Non ISO operation."]
    #[inline(always)]
    pub const fn set_niso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cccr {
    #[inline(always)]
    fn default() -> Cccr {
        Cccr(0)
    }
}
impl core::fmt::Debug for Cccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cccr")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cccr {{ init: {=bool:?}, cce: {=bool:?}, asm: {=bool:?}, csa: {=bool:?}, csr: {=bool:?}, mon: {=bool:?}, dar: {=bool:?}, test: {=bool:?}, fdoe: {=bool:?}, brse: {=bool:?}, pxhd: {=bool:?}, efbi: {=bool:?}, txp: {=bool:?}, niso: {=bool:?} }}",
            self.init(),
            self.cce(),
            self.asm(),
            self.csa(),
            self.csr(),
            self.mon(),
            self.dar(),
            self.test(),
            self.fdoe(),
            self.brse(),
            self.pxhd(),
            self.efbi(),
            self.txp(),
            self.niso()
        )
    }
}
#[doc = "Data Bit Timing Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbtp(pub u32);
impl Dbtp {
    #[doc = "Data (re)synchronization jump width."]
    #[must_use]
    #[inline(always)]
    pub const fn dsjw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data (re)synchronization jump width."]
    #[inline(always)]
    pub const fn set_dsjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data time segment after sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn dtseg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data time segment after sample point."]
    #[inline(always)]
    pub const fn set_dtseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data time segment before sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn dtseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Data time segment before sample point."]
    #[inline(always)]
    pub const fn set_dtseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Data bit rate prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn dbrp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data bit rate prescaler."]
    #[inline(always)]
    pub const fn set_dbrp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Transmitter delay compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn tdc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter delay compensation."]
    #[inline(always)]
    pub const fn set_tdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Dbtp {
    #[inline(always)]
    fn default() -> Dbtp {
        Dbtp(0)
    }
}
impl core::fmt::Debug for Dbtp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbtp")
            .field("dsjw", &self.dsjw())
            .field("dtseg2", &self.dtseg2())
            .field("dtseg1", &self.dtseg1())
            .field("dbrp", &self.dbrp())
            .field("tdc", &self.tdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbtp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbtp {{ dsjw: {=u8:?}, dtseg2: {=u8:?}, dtseg1: {=u8:?}, dbrp: {=u8:?}, tdc: {=bool:?} }}",
            self.dsjw(),
            self.dtseg2(),
            self.dtseg1(),
            self.dbrp(),
            self.tdc()
        )
    }
}
#[doc = "Error Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Transmit error counter."]
    #[must_use]
    #[inline(always)]
    pub const fn tec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit error counter."]
    #[inline(always)]
    pub const fn set_tec(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive error counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive error counter."]
    #[inline(always)]
    pub const fn set_rec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Receive error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive error passive."]
    #[inline(always)]
    pub const fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CAN error logging."]
    #[must_use]
    #[inline(always)]
    pub const fn cel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CAN error logging."]
    #[inline(always)]
    pub const fn set_cel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
impl core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr")
            .field("tec", &self.tec())
            .field("rec", &self.rec())
            .field("rp", &self.rp())
            .field("cel", &self.cel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecr {{ tec: {=u8:?}, rec: {=u8:?}, rp: {=bool:?}, cel: {=u8:?} }}",
            self.tec(),
            self.rec(),
            self.rp(),
            self.cel()
        )
    }
}
#[doc = "External Timestamp Counter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etscc(pub u32);
impl Etscc {
    #[doc = "External timestamp prescaler value."]
    #[must_use]
    #[inline(always)]
    pub const fn etcp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "External timestamp prescaler value."]
    #[inline(always)]
    pub const fn set_etcp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "External timestamp counter enable."]
    #[must_use]
    #[inline(always)]
    pub const fn etce(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "External timestamp counter enable."]
    #[inline(always)]
    pub const fn set_etce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Etscc {
    #[inline(always)]
    fn default() -> Etscc {
        Etscc(0)
    }
}
impl core::fmt::Debug for Etscc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etscc")
            .field("etcp", &self.etcp())
            .field("etce", &self.etce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etscc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etscc {{ etcp: {=u16:?}, etce: {=bool:?} }}",
            self.etcp(),
            self.etce()
        )
    }
}
#[doc = "External Timestamp Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etscv(pub u32);
impl Etscv {
    #[doc = "External timestamp counter."]
    #[must_use]
    #[inline(always)]
    pub const fn etsc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "External timestamp counter."]
    #[inline(always)]
    pub const fn set_etsc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Etscv {
    #[inline(always)]
    fn default() -> Etscv {
        Etscv(0)
    }
}
impl core::fmt::Debug for Etscv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etscv").field("etsc", &self.etsc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etscv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etscv {{ etsc: {=u16:?} }}", self.etsc())
    }
}
#[doc = "Global Filter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfc(pub u32);
impl Gfc {
    #[doc = "Reject remote frames extended."]
    #[must_use]
    #[inline(always)]
    pub const fn rrfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reject remote frames extended."]
    #[inline(always)]
    pub const fn set_rrfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reject remote frames standard."]
    #[must_use]
    #[inline(always)]
    pub const fn rrfs(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reject remote frames standard."]
    #[inline(always)]
    pub const fn set_rrfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Accept non-matching frames extended."]
    #[must_use]
    #[inline(always)]
    pub const fn anfe(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Accept non-matching frames extended."]
    #[inline(always)]
    pub const fn set_anfe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Accept non-matching frames standard."]
    #[must_use]
    #[inline(always)]
    pub const fn anfs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Accept non-matching frames standard."]
    #[inline(always)]
    pub const fn set_anfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Gfc {
    #[inline(always)]
    fn default() -> Gfc {
        Gfc(0)
    }
}
impl core::fmt::Debug for Gfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gfc")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gfc {{ rrfe: {=bool:?}, rrfs: {=bool:?}, anfe: {=u8:?}, anfs: {=u8:?} }}",
            self.rrfe(),
            self.rrfs(),
            self.anfe(),
            self.anfs()
        )
    }
}
#[doc = "High Priority Message Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hpms(pub u32);
impl Hpms {
    #[doc = "Buffer index."]
    #[must_use]
    #[inline(always)]
    pub const fn bidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Buffer index."]
    #[inline(always)]
    pub const fn set_bidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Message storage indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn msi(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Message storage indicator."]
    #[inline(always)]
    pub const fn set_msi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Filter index."]
    #[must_use]
    #[inline(always)]
    pub const fn fidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Filter index."]
    #[inline(always)]
    pub const fn set_fidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Filter list."]
    #[must_use]
    #[inline(always)]
    pub const fn flst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Filter list."]
    #[inline(always)]
    pub const fn set_flst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Hpms {
    #[inline(always)]
    fn default() -> Hpms {
        Hpms(0)
    }
}
impl core::fmt::Debug for Hpms {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hpms")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hpms {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hpms {{ bidx: {=u8:?}, msi: {=u8:?}, fidx: {=u8:?}, flst: {=bool:?} }}",
            self.bidx(),
            self.msi(),
            self.fidx(),
            self.flst()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie(pub u32);
impl Ie {
    #[doc = "Rx FIFO 0 new message interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0ne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 new message interrupt enable."]
    #[inline(always)]
    pub const fn set_rf0ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0we(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt enable."]
    #[inline(always)]
    pub const fn set_rf0we(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full interrupt enable."]
    #[inline(always)]
    pub const fn set_rf0fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 message lost interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0le(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost interrupt enable."]
    #[inline(always)]
    pub const fn set_rf0le(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 new message interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1ne(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 new message interrupt enable."]
    #[inline(always)]
    pub const fn set_rf1ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1we(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt enable."]
    #[inline(always)]
    pub const fn set_rf1we(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1fe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full interrupt enable."]
    #[inline(always)]
    pub const fn set_rf1fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 message lost interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1le(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost interrupt enable."]
    #[inline(always)]
    pub const fn set_rf1le(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High priority message interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hpme(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High priority message interrupt enable."]
    #[inline(always)]
    pub const fn set_hpme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission completed interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission completed interrupt enable."]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission cancellation finished interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcfe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission cancellation finished interrupt enable."]
    #[inline(always)]
    pub const fn set_tcfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO empty interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tfee(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO empty interrupt enable."]
    #[inline(always)]
    pub const fn set_tfee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO new entry interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tefne(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO new entry interrupt enable."]
    #[inline(always)]
    pub const fn set_tefne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx event FIFO watermark reached interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tefwe(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO watermark reached interrupt enable."]
    #[inline(always)]
    pub const fn set_tefwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx event FIFO full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn teffe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO full interrupt enable."]
    #[inline(always)]
    pub const fn set_teffe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx event FIFO element lost interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tefle(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost interrupt enable."]
    #[inline(always)]
    pub const fn set_tefle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp wraparound interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tswe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp wraparound interrupt enable."]
    #[inline(always)]
    pub const fn set_tswe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM access failure interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mrafe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM access failure interrupt enable."]
    #[inline(always)]
    pub const fn set_mrafe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout occurred interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tooe(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout occurred interrupt enable."]
    #[inline(always)]
    pub const fn set_tooe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn drxe(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt enable."]
    #[inline(always)]
    pub const fn set_drxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit error corrected interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bece(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error corrected interrupt enable."]
    #[inline(always)]
    pub const fn set_bece(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit error uncorrected interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn beue(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error uncorrected interrupt enable."]
    #[inline(always)]
    pub const fn set_beue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error logging overflow interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eloe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error logging overflow interrupt enable."]
    #[inline(always)]
    pub const fn set_eloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error passive interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn epe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive interrupt enable."]
    #[inline(always)]
    pub const fn set_epe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning status interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ewe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status interrupt enable."]
    #[inline(always)]
    pub const fn set_ewe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn boe(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status interrupt enable."]
    #[inline(always)]
    pub const fn set_boe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wdie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt enable."]
    #[inline(always)]
    pub const fn set_wdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol error in arbitration phase interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn peae(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase interrupt enable."]
    #[inline(always)]
    pub const fn set_peae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol error in data phase interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pede(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase interrupt enable."]
    #[inline(always)]
    pub const fn set_pede(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to reserved address interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn arae(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address interrupt enable."]
    #[inline(always)]
    pub const fn set_arae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ie {
    #[inline(always)]
    fn default() -> Ie {
        Ie(0)
    }
}
impl core::fmt::Debug for Ie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ie")
            .field("rf0ne", &self.rf0ne())
            .field("rf0we", &self.rf0we())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1we", &self.rf1we())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tfee", &self.tfee())
            .field("tefne", &self.tefne())
            .field("tefwe", &self.tefwe())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("drxe", &self.drxe())
            .field("bece", &self.bece())
            .field("beue", &self.beue())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ie {{ rf0ne: {=bool:?}, rf0we: {=bool:?}, rf0fe: {=bool:?}, rf0le: {=bool:?}, rf1ne: {=bool:?}, rf1we: {=bool:?}, rf1fe: {=bool:?}, rf1le: {=bool:?}, hpme: {=bool:?}, tce: {=bool:?}, tcfe: {=bool:?}, tfee: {=bool:?}, tefne: {=bool:?}, tefwe: {=bool:?}, teffe: {=bool:?}, tefle: {=bool:?}, tswe: {=bool:?}, mrafe: {=bool:?}, tooe: {=bool:?}, drxe: {=bool:?}, bece: {=bool:?}, beue: {=bool:?}, eloe: {=bool:?}, epe: {=bool:?}, ewe: {=bool:?}, boe: {=bool:?}, wdie: {=bool:?}, peae: {=bool:?}, pede: {=bool:?}, arae: {=bool:?} }}",
            self.rf0ne(),
            self.rf0we(),
            self.rf0fe(),
            self.rf0le(),
            self.rf1ne(),
            self.rf1we(),
            self.rf1fe(),
            self.rf1le(),
            self.hpme(),
            self.tce(),
            self.tcfe(),
            self.tfee(),
            self.tefne(),
            self.tefwe(),
            self.teffe(),
            self.tefle(),
            self.tswe(),
            self.mrafe(),
            self.tooe(),
            self.drxe(),
            self.bece(),
            self.beue(),
            self.eloe(),
            self.epe(),
            self.ewe(),
            self.boe(),
            self.wdie(),
            self.peae(),
            self.pede(),
            self.arae()
        )
    }
}
#[doc = "Interrupt Line Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ile(pub u32);
impl Ile {
    #[doc = "Enable interrupt line 0."]
    #[must_use]
    #[inline(always)]
    pub const fn eint0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable interrupt line 0."]
    #[inline(always)]
    pub const fn set_eint0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable interrupt line 1."]
    #[must_use]
    #[inline(always)]
    pub const fn eint1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable interrupt line 1."]
    #[inline(always)]
    pub const fn set_eint1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Ile {
    #[inline(always)]
    fn default() -> Ile {
        Ile(0)
    }
}
impl core::fmt::Debug for Ile {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ile")
            .field("eint0", &self.eint0())
            .field("eint1", &self.eint1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ile {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ile {{ eint0: {=bool:?}, eint1: {=bool:?} }}",
            self.eint0(),
            self.eint1()
        )
    }
}
#[doc = "Interrupt Line Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ils(pub u32);
impl Ils {
    #[doc = "Rx FIFO 0 new message interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0nl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 new message interrupt line."]
    #[inline(always)]
    pub const fn set_rf0nl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0wl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt line."]
    #[inline(always)]
    pub const fn set_rf0wl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 full interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0fl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full interrupt line."]
    #[inline(always)]
    pub const fn set_rf0fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 message lost interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0ll(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost interrupt line."]
    #[inline(always)]
    pub const fn set_rf0ll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 new message interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1nl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 new message interrupt line."]
    #[inline(always)]
    pub const fn set_rf1nl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1wl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt line."]
    #[inline(always)]
    pub const fn set_rf1wl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 full interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1fl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full interrupt line."]
    #[inline(always)]
    pub const fn set_rf1fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 message lost interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1ll(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost interrupt line."]
    #[inline(always)]
    pub const fn set_rf1ll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High priority message interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn hpml(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High priority message interrupt line."]
    #[inline(always)]
    pub const fn set_hpml(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission completed interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tcl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission completed interrupt line."]
    #[inline(always)]
    pub const fn set_tcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission cancellation finished interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tcfl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission cancellation finished interrupt line."]
    #[inline(always)]
    pub const fn set_tcfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO empty interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tfel(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO empty interrupt line."]
    #[inline(always)]
    pub const fn set_tfel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO new entry interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tefnl(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO new entry interrupt line."]
    #[inline(always)]
    pub const fn set_tefnl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx event FIFO watermark reached interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tefwl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO watermark reached interrupt line."]
    #[inline(always)]
    pub const fn set_tefwl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx event FIFO full interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn teffl(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO full interrupt line."]
    #[inline(always)]
    pub const fn set_teffl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx event FIFO element lost interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tefll(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost interrupt line."]
    #[inline(always)]
    pub const fn set_tefll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp wraparound interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tswl(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp wraparound interrupt line."]
    #[inline(always)]
    pub const fn set_tswl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM access failure interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn mrafl(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM access failure interrupt line."]
    #[inline(always)]
    pub const fn set_mrafl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout occurred interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn tool(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout occurred interrupt line."]
    #[inline(always)]
    pub const fn set_tool(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn drxl(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt line."]
    #[inline(always)]
    pub const fn set_drxl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit error corrected interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn becl(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error corrected interrupt line."]
    #[inline(always)]
    pub const fn set_becl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit error uncorrected interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn beul(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error uncorrected interrupt line."]
    #[inline(always)]
    pub const fn set_beul(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error logging overflow interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn elol(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error logging overflow interrupt line."]
    #[inline(always)]
    pub const fn set_elol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error passive interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn epl(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive interrupt line."]
    #[inline(always)]
    pub const fn set_epl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning status interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn ewl(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status interrupt line."]
    #[inline(always)]
    pub const fn set_ewl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn bol(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status interrupt line."]
    #[inline(always)]
    pub const fn set_bol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn wdil(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt line."]
    #[inline(always)]
    pub const fn set_wdil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol error in arbitration phase interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn peal(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase interrupt line."]
    #[inline(always)]
    pub const fn set_peal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol error in data phase interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn pedl(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase interrupt line."]
    #[inline(always)]
    pub const fn set_pedl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to reserved address interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn aral(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address interrupt line."]
    #[inline(always)]
    pub const fn set_aral(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ils {
    #[inline(always)]
    fn default() -> Ils {
        Ils(0)
    }
}
impl core::fmt::Debug for Ils {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ils")
            .field("rf0nl", &self.rf0nl())
            .field("rf0wl", &self.rf0wl())
            .field("rf0fl", &self.rf0fl())
            .field("rf0ll", &self.rf0ll())
            .field("rf1nl", &self.rf1nl())
            .field("rf1wl", &self.rf1wl())
            .field("rf1fl", &self.rf1fl())
            .field("rf1ll", &self.rf1ll())
            .field("hpml", &self.hpml())
            .field("tcl", &self.tcl())
            .field("tcfl", &self.tcfl())
            .field("tfel", &self.tfel())
            .field("tefnl", &self.tefnl())
            .field("tefwl", &self.tefwl())
            .field("teffl", &self.teffl())
            .field("tefll", &self.tefll())
            .field("tswl", &self.tswl())
            .field("mrafl", &self.mrafl())
            .field("tool", &self.tool())
            .field("drxl", &self.drxl())
            .field("becl", &self.becl())
            .field("beul", &self.beul())
            .field("elol", &self.elol())
            .field("epl", &self.epl())
            .field("ewl", &self.ewl())
            .field("bol", &self.bol())
            .field("wdil", &self.wdil())
            .field("peal", &self.peal())
            .field("pedl", &self.pedl())
            .field("aral", &self.aral())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ils {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ils {{ rf0nl: {=bool:?}, rf0wl: {=bool:?}, rf0fl: {=bool:?}, rf0ll: {=bool:?}, rf1nl: {=bool:?}, rf1wl: {=bool:?}, rf1fl: {=bool:?}, rf1ll: {=bool:?}, hpml: {=bool:?}, tcl: {=bool:?}, tcfl: {=bool:?}, tfel: {=bool:?}, tefnl: {=bool:?}, tefwl: {=bool:?}, teffl: {=bool:?}, tefll: {=bool:?}, tswl: {=bool:?}, mrafl: {=bool:?}, tool: {=bool:?}, drxl: {=bool:?}, becl: {=bool:?}, beul: {=bool:?}, elol: {=bool:?}, epl: {=bool:?}, ewl: {=bool:?}, bol: {=bool:?}, wdil: {=bool:?}, peal: {=bool:?}, pedl: {=bool:?}, aral: {=bool:?} }}",
            self.rf0nl(),
            self.rf0wl(),
            self.rf0fl(),
            self.rf0ll(),
            self.rf1nl(),
            self.rf1wl(),
            self.rf1fl(),
            self.rf1ll(),
            self.hpml(),
            self.tcl(),
            self.tcfl(),
            self.tfel(),
            self.tefnl(),
            self.tefwl(),
            self.teffl(),
            self.tefll(),
            self.tswl(),
            self.mrafl(),
            self.tool(),
            self.drxl(),
            self.becl(),
            self.beul(),
            self.elol(),
            self.epl(),
            self.ewl(),
            self.bol(),
            self.wdil(),
            self.peal(),
            self.pedl(),
            self.aral()
        )
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "Rx FIFO 0 new message."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0n(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 new message."]
    #[inline(always)]
    pub const fn set_rf0n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 watermark reached."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0w(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub const fn set_rf0w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 full."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full."]
    #[inline(always)]
    pub const fn set_rf0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0l(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[inline(always)]
    pub const fn set_rf0l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 new message."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1n(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 new message."]
    #[inline(always)]
    pub const fn set_rf1n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 watermark reached."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1w(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub const fn set_rf1w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 full."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1f(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full."]
    #[inline(always)]
    pub const fn set_rf1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1l(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[inline(always)]
    pub const fn set_rf1l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High priority message."]
    #[must_use]
    #[inline(always)]
    pub const fn hpm(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High priority message."]
    #[inline(always)]
    pub const fn set_hpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission completed."]
    #[must_use]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission completed."]
    #[inline(always)]
    pub const fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission cancellation finished."]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission cancellation finished."]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO empty."]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO empty."]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO new entry."]
    #[must_use]
    #[inline(always)]
    pub const fn tefn(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO new entry."]
    #[inline(always)]
    pub const fn set_tefn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx event FIFO watermark reached."]
    #[must_use]
    #[inline(always)]
    pub const fn tefw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO watermark reached."]
    #[inline(always)]
    pub const fn set_tefw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx event FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn teff(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO full."]
    #[inline(always)]
    pub const fn set_teff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx event FIFO element lost."]
    #[must_use]
    #[inline(always)]
    pub const fn tefl(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost."]
    #[inline(always)]
    pub const fn set_tefl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp wraparound."]
    #[must_use]
    #[inline(always)]
    pub const fn tsw(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp wraparound."]
    #[inline(always)]
    pub const fn set_tsw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM access failure."]
    #[must_use]
    #[inline(always)]
    pub const fn mraf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM access failure."]
    #[inline(always)]
    pub const fn set_mraf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn too(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout occurred."]
    #[inline(always)]
    pub const fn set_too(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored in dedicated Rx buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn drx(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub const fn set_drx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit error corrected."]
    #[must_use]
    #[inline(always)]
    pub const fn bec(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error corrected."]
    #[inline(always)]
    pub const fn set_bec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit error uncorrected."]
    #[must_use]
    #[inline(always)]
    pub const fn beu(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error uncorrected."]
    #[inline(always)]
    pub const fn set_beu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error logging overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn elo(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error logging overflow."]
    #[inline(always)]
    pub const fn set_elo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn ep(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive."]
    #[inline(always)]
    pub const fn set_ep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning status."]
    #[must_use]
    #[inline(always)]
    pub const fn ew(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status."]
    #[inline(always)]
    pub const fn set_ew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bo(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status."]
    #[inline(always)]
    pub const fn set_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdi(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt."]
    #[inline(always)]
    pub const fn set_wdi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol error in arbitration phase."]
    #[must_use]
    #[inline(always)]
    pub const fn pea(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase."]
    #[inline(always)]
    pub const fn set_pea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol error in data phase."]
    #[must_use]
    #[inline(always)]
    pub const fn ped(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase."]
    #[inline(always)]
    pub const fn set_ped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to reserved address."]
    #[must_use]
    #[inline(always)]
    pub const fn ara(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address."]
    #[inline(always)]
    pub const fn set_ara(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
impl core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ir")
            .field("rf0n", &self.rf0n())
            .field("rf0w", &self.rf0w())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1w", &self.rf1w())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("tefw", &self.tefw())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("drx", &self.drx())
            .field("bec", &self.bec())
            .field("beu", &self.beu())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ir {{ rf0n: {=bool:?}, rf0w: {=bool:?}, rf0f: {=bool:?}, rf0l: {=bool:?}, rf1n: {=bool:?}, rf1w: {=bool:?}, rf1f: {=bool:?}, rf1l: {=bool:?}, hpm: {=bool:?}, tc: {=bool:?}, tcf: {=bool:?}, tfe: {=bool:?}, tefn: {=bool:?}, tefw: {=bool:?}, teff: {=bool:?}, tefl: {=bool:?}, tsw: {=bool:?}, mraf: {=bool:?}, too: {=bool:?}, drx: {=bool:?}, bec: {=bool:?}, beu: {=bool:?}, elo: {=bool:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, wdi: {=bool:?}, pea: {=bool:?}, ped: {=bool:?}, ara: {=bool:?} }}",
            self.rf0n(),
            self.rf0w(),
            self.rf0f(),
            self.rf0l(),
            self.rf1n(),
            self.rf1w(),
            self.rf1f(),
            self.rf1l(),
            self.hpm(),
            self.tc(),
            self.tcf(),
            self.tfe(),
            self.tefn(),
            self.tefw(),
            self.teff(),
            self.tefl(),
            self.tsw(),
            self.mraf(),
            self.too(),
            self.drx(),
            self.bec(),
            self.beu(),
            self.elo(),
            self.ep(),
            self.ew(),
            self.bo(),
            self.wdi(),
            self.pea(),
            self.ped(),
            self.ara()
        )
    }
}
#[doc = "CAN Message RAM Base Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrba(pub u32);
impl Mrba {
    #[doc = "Base address for the message RAM in the chip memory map."]
    #[must_use]
    #[inline(always)]
    pub const fn ba(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base address for the message RAM in the chip memory map."]
    #[inline(always)]
    pub const fn set_ba(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mrba {
    #[inline(always)]
    fn default() -> Mrba {
        Mrba(0)
    }
}
impl core::fmt::Debug for Mrba {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrba").field("ba", &self.ba()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrba {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mrba {{ ba: {=u16:?} }}", self.ba())
    }
}
#[doc = "Nominal Bit Timing and Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nbtp(pub u32);
impl Nbtp {
    #[doc = "Nominal time segment after sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn ntseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal time segment after sample point."]
    #[inline(always)]
    pub const fn set_ntseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Nominal time segment before sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn ntseg1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal time segment before sample point."]
    #[inline(always)]
    pub const fn set_ntseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Nominal bit rate prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn nbrp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Nominal bit rate prescaler."]
    #[inline(always)]
    pub const fn set_nbrp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Nominal (re)synchronization jump width."]
    #[must_use]
    #[inline(always)]
    pub const fn nsjw(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal (re)synchronization jump width."]
    #[inline(always)]
    pub const fn set_nsjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Nbtp {
    #[inline(always)]
    fn default() -> Nbtp {
        Nbtp(0)
    }
}
impl core::fmt::Debug for Nbtp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nbtp")
            .field("ntseg2", &self.ntseg2())
            .field("ntseg1", &self.ntseg1())
            .field("nbrp", &self.nbrp())
            .field("nsjw", &self.nsjw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nbtp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nbtp {{ ntseg2: {=u8:?}, ntseg1: {=u8:?}, nbrp: {=u16:?}, nsjw: {=u8:?} }}",
            self.ntseg2(),
            self.ntseg1(),
            self.nbrp(),
            self.nsjw()
        )
    }
}
#[doc = "New Data 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat1(pub u32);
impl Ndat1 {
    #[doc = "New Data."]
    #[must_use]
    #[inline(always)]
    pub const fn nd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "New Data."]
    #[inline(always)]
    pub const fn set_nd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ndat1 {
    #[inline(always)]
    fn default() -> Ndat1 {
        Ndat1(0)
    }
}
impl core::fmt::Debug for Ndat1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ndat1").field("nd", &self.nd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ndat1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ndat1 {{ nd: {=u32:?} }}", self.nd())
    }
}
#[doc = "New Data 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat2(pub u32);
impl Ndat2 {
    #[doc = "New Data."]
    #[must_use]
    #[inline(always)]
    pub const fn nd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "New Data."]
    #[inline(always)]
    pub const fn set_nd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ndat2 {
    #[inline(always)]
    fn default() -> Ndat2 {
        Ndat2(0)
    }
}
impl core::fmt::Debug for Ndat2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ndat2").field("nd", &self.nd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ndat2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ndat2 {{ nd: {=u32:?} }}", self.nd())
    }
}
#[doc = "Protocol Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "Last error code."]
    #[must_use]
    #[inline(always)]
    pub const fn lec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Last error code."]
    #[inline(always)]
    pub const fn set_lec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Activity."]
    #[must_use]
    #[inline(always)]
    pub const fn act(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Activity."]
    #[inline(always)]
    pub const fn set_act(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Error Passive."]
    #[must_use]
    #[inline(always)]
    pub const fn ep(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive."]
    #[inline(always)]
    pub const fn set_ep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Warning status."]
    #[must_use]
    #[inline(always)]
    pub const fn ew(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status."]
    #[inline(always)]
    pub const fn set_ew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus Off Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bo(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Status."]
    #[inline(always)]
    pub const fn set_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Data phase last error code."]
    #[must_use]
    #[inline(always)]
    pub const fn dlec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Data phase last error code."]
    #[inline(always)]
    pub const fn set_dlec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "ESI flag of the last received CAN FD message."]
    #[must_use]
    #[inline(always)]
    pub const fn resi(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ESI flag of the last received CAN FD message."]
    #[inline(always)]
    pub const fn set_resi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BRS flag of last received CAN FD message."]
    #[must_use]
    #[inline(always)]
    pub const fn rbrs(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BRS flag of last received CAN FD message."]
    #[inline(always)]
    pub const fn set_rbrs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Received a CAN FD message."]
    #[must_use]
    #[inline(always)]
    pub const fn rfdf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Received a CAN FD message."]
    #[inline(always)]
    pub const fn set_rfdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol exception event."]
    #[must_use]
    #[inline(always)]
    pub const fn pxe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol exception event."]
    #[inline(always)]
    pub const fn set_pxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmitter delay compensation value."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation value."]
    #[inline(always)]
    pub const fn set_tdcv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr")
            .field("lec", &self.lec())
            .field("act", &self.act())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("dlec", &self.dlec())
            .field("resi", &self.resi())
            .field("rbrs", &self.rbrs())
            .field("rfdf", &self.rfdf())
            .field("pxe", &self.pxe())
            .field("tdcv", &self.tdcv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psr {{ lec: {=u8:?}, act: {=u8:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, dlec: {=u8:?}, resi: {=bool:?}, rbrs: {=bool:?}, rfdf: {=bool:?}, pxe: {=bool:?}, tdcv: {=u8:?} }}",
            self.lec(),
            self.act(),
            self.ep(),
            self.ew(),
            self.bo(),
            self.dlec(),
            self.resi(),
            self.rbrs(),
            self.rfdf(),
            self.pxe(),
            self.tdcv()
        )
    }
}
#[doc = "Rx Buffer Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxbc(pub u32);
impl Rxbc {
    #[doc = "Rx buffer start address."]
    #[must_use]
    #[inline(always)]
    pub const fn rbsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx buffer start address."]
    #[inline(always)]
    pub const fn set_rbsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
}
impl Default for Rxbc {
    #[inline(always)]
    fn default() -> Rxbc {
        Rxbc(0)
    }
}
impl core::fmt::Debug for Rxbc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxbc").field("rbsa", &self.rbsa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxbc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxbc {{ rbsa: {=u16:?} }}", self.rbsa())
    }
}
#[doc = "Rx Buffer and FIFO Element Size Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxesc(pub u32);
impl Rxesc {
    #[doc = "Rx FIFO 0 data field size."]
    #[must_use]
    #[inline(always)]
    pub const fn f0ds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 0 data field size."]
    #[inline(always)]
    pub const fn set_f0ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Rx FIFO 1 data field size."]
    #[must_use]
    #[inline(always)]
    pub const fn f1ds(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 1 data field size."]
    #[inline(always)]
    pub const fn set_f1ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn rbds(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_rbds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Rxesc {
    #[inline(always)]
    fn default() -> Rxesc {
        Rxesc(0)
    }
}
impl core::fmt::Debug for Rxesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxesc")
            .field("f0ds", &self.f0ds())
            .field("f1ds", &self.f1ds())
            .field("rbds", &self.rbds())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxesc {{ f0ds: {=u8:?}, f1ds: {=u8:?}, rbds: {=u8:?} }}",
            self.f0ds(),
            self.f1ds(),
            self.rbds()
        )
    }
}
#[doc = "Rx FIFO 0 Acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf0a(pub u32);
impl Rxf0a {
    #[doc = "Rx FIFO 0 acknowledge index."]
    #[must_use]
    #[inline(always)]
    pub const fn f0ai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 acknowledge index."]
    #[inline(always)]
    pub const fn set_f0ai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Rxf0a {
    #[inline(always)]
    fn default() -> Rxf0a {
        Rxf0a(0)
    }
}
impl core::fmt::Debug for Rxf0a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxf0a").field("f0ai", &self.f0ai()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxf0a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxf0a {{ f0ai: {=u8:?} }}", self.f0ai())
    }
}
#[doc = "Rx FIFO 0 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf0c(pub u32);
impl Rxf0c {
    #[doc = "Rx FIFO 0 start address."]
    #[must_use]
    #[inline(always)]
    pub const fn f0sa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 0 start address."]
    #[inline(always)]
    pub const fn set_f0sa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Rx FIFO 0 size."]
    #[must_use]
    #[inline(always)]
    pub const fn f0s(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 size."]
    #[inline(always)]
    pub const fn set_f0s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 0 watermark 0 = Watermark interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn f0wm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub const fn set_f0wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 0 operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn f0om(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 operation mode."]
    #[inline(always)]
    pub const fn set_f0om(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxf0c {
    #[inline(always)]
    fn default() -> Rxf0c {
        Rxf0c(0)
    }
}
impl core::fmt::Debug for Rxf0c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxf0c")
            .field("f0sa", &self.f0sa())
            .field("f0s", &self.f0s())
            .field("f0wm", &self.f0wm())
            .field("f0om", &self.f0om())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxf0c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxf0c {{ f0sa: {=u16:?}, f0s: {=u8:?}, f0wm: {=u8:?}, f0om: {=bool:?} }}",
            self.f0sa(),
            self.f0s(),
            self.f0wm(),
            self.f0om()
        )
    }
}
#[doc = "Rx FIFO 0 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf0s(pub u32);
impl Rxf0s {
    #[doc = "Rx FIFO 0 fill level."]
    #[must_use]
    #[inline(always)]
    pub const fn f0fl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 fill level."]
    #[inline(always)]
    pub const fn set_f0fl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 0 get index."]
    #[must_use]
    #[inline(always)]
    pub const fn f0gi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 get index."]
    #[inline(always)]
    pub const fn set_f0gi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 0 put index."]
    #[must_use]
    #[inline(always)]
    pub const fn f0pi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 put index."]
    #[inline(always)]
    pub const fn set_f0pi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 0 full."]
    #[must_use]
    #[inline(always)]
    pub const fn f0f(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full."]
    #[inline(always)]
    pub const fn set_f0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn rf0l(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[inline(always)]
    pub const fn set_rf0l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Rxf0s {
    #[inline(always)]
    fn default() -> Rxf0s {
        Rxf0s(0)
    }
}
impl core::fmt::Debug for Rxf0s {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxf0s")
            .field("f0fl", &self.f0fl())
            .field("f0gi", &self.f0gi())
            .field("f0pi", &self.f0pi())
            .field("f0f", &self.f0f())
            .field("rf0l", &self.rf0l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxf0s {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxf0s {{ f0fl: {=u8:?}, f0gi: {=u8:?}, f0pi: {=u8:?}, f0f: {=bool:?}, rf0l: {=bool:?} }}",
            self.f0fl(),
            self.f0gi(),
            self.f0pi(),
            self.f0f(),
            self.rf0l()
        )
    }
}
#[doc = "Rx FIFO 1 Acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf1a(pub u32);
impl Rxf1a {
    #[doc = "Rx FIFO 1 acknowledge index."]
    #[must_use]
    #[inline(always)]
    pub const fn f1ai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 acknowledge index."]
    #[inline(always)]
    pub const fn set_f1ai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Rxf1a {
    #[inline(always)]
    fn default() -> Rxf1a {
        Rxf1a(0)
    }
}
impl core::fmt::Debug for Rxf1a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxf1a").field("f1ai", &self.f1ai()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxf1a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxf1a {{ f1ai: {=u8:?} }}", self.f1ai())
    }
}
#[doc = "Rx FIFO 1 Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf1c(pub u32);
impl Rxf1c {
    #[doc = "Rx FIFO 1 start address."]
    #[must_use]
    #[inline(always)]
    pub const fn f1sa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 1 start address."]
    #[inline(always)]
    pub const fn set_f1sa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Rx FIFO 1 size 0 = No Rx FIFO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn f1s(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 size 0 = No Rx FIFO 1."]
    #[inline(always)]
    pub const fn set_f1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 1 watermark 0 = Watermark interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn f1wm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub const fn set_f1wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 1 operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn f1om(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 1 operation mode."]
    #[inline(always)]
    pub const fn set_f1om(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rxf1c {
    #[inline(always)]
    fn default() -> Rxf1c {
        Rxf1c(0)
    }
}
impl core::fmt::Debug for Rxf1c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxf1c")
            .field("f1sa", &self.f1sa())
            .field("f1s", &self.f1s())
            .field("f1wm", &self.f1wm())
            .field("f1om", &self.f1om())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxf1c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxf1c {{ f1sa: {=u16:?}, f1s: {=u8:?}, f1wm: {=u8:?}, f1om: {=bool:?} }}",
            self.f1sa(),
            self.f1s(),
            self.f1wm(),
            self.f1om()
        )
    }
}
#[doc = "Rx FIFO 1 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxf1s(pub u32);
impl Rxf1s {
    #[doc = "Rx FIFO 1 fill level."]
    #[must_use]
    #[inline(always)]
    pub const fn f1fl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 fill level."]
    #[inline(always)]
    pub const fn set_f1fl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 1 get index."]
    #[must_use]
    #[inline(always)]
    pub const fn f1gi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 get index."]
    #[inline(always)]
    pub const fn set_f1gi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 1 put index."]
    #[must_use]
    #[inline(always)]
    pub const fn f1pi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 put index."]
    #[inline(always)]
    pub const fn set_f1pi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 1 full."]
    #[must_use]
    #[inline(always)]
    pub const fn f1f(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full."]
    #[inline(always)]
    pub const fn set_f1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn rf1l(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[inline(always)]
    pub const fn set_rf1l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Rxf1s {
    #[inline(always)]
    fn default() -> Rxf1s {
        Rxf1s(0)
    }
}
impl core::fmt::Debug for Rxf1s {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxf1s")
            .field("f1fl", &self.f1fl())
            .field("f1gi", &self.f1gi())
            .field("f1pi", &self.f1pi())
            .field("f1f", &self.f1f())
            .field("rf1l", &self.rf1l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxf1s {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxf1s {{ f1fl: {=u8:?}, f1gi: {=u8:?}, f1pi: {=u8:?}, f1f: {=bool:?}, rf1l: {=bool:?} }}",
            self.f1fl(),
            self.f1gi(),
            self.f1pi(),
            self.f1f(),
            self.rf1l()
        )
    }
}
#[doc = "Standard ID Filter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidfc(pub u32);
impl Sidfc {
    #[doc = "Filter list standard start address."]
    #[must_use]
    #[inline(always)]
    pub const fn flssa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter list standard start address."]
    #[inline(always)]
    pub const fn set_flssa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List size standard 0 = No standard message ID filter."]
    #[must_use]
    #[inline(always)]
    pub const fn lss(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List size standard 0 = No standard message ID filter."]
    #[inline(always)]
    pub const fn set_lss(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Sidfc {
    #[inline(always)]
    fn default() -> Sidfc {
        Sidfc(0)
    }
}
impl core::fmt::Debug for Sidfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sidfc")
            .field("flssa", &self.flssa())
            .field("lss", &self.lss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sidfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sidfc {{ flssa: {=u16:?}, lss: {=u8:?} }}",
            self.flssa(),
            self.lss()
        )
    }
}
#[doc = "Transmitter Delay Compensator Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdcr(pub u32);
impl Tdcr {
    #[doc = "Transmitter delay compensation filter window length."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation filter window length."]
    #[inline(always)]
    pub const fn set_tdcf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transmitter delay compensation offset."]
    #[must_use]
    #[inline(always)]
    pub const fn tdco(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation offset."]
    #[inline(always)]
    pub const fn set_tdco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for Tdcr {
    #[inline(always)]
    fn default() -> Tdcr {
        Tdcr(0)
    }
}
impl core::fmt::Debug for Tdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdcr")
            .field("tdcf", &self.tdcf())
            .field("tdco", &self.tdco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tdcr {{ tdcf: {=u8:?}, tdco: {=u8:?} }}",
            self.tdcf(),
            self.tdco()
        )
    }
}
#[doc = "Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test(pub u32);
impl Test {
    #[doc = "Loop back mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lbck(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Loop back mode."]
    #[inline(always)]
    pub const fn set_lbck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control of transmit pin."]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Control of transmit pin."]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Monitors the actual value of the CAN_RXD."]
    #[must_use]
    #[inline(always)]
    pub const fn rx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the actual value of the CAN_RXD."]
    #[inline(always)]
    pub const fn set_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Test {
    #[inline(always)]
    fn default() -> Test {
        Test(0)
    }
}
impl core::fmt::Debug for Test {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Test")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Test {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Test {{ lbck: {=bool:?}, tx: {=u8:?}, rx: {=bool:?} }}",
            self.lbck(),
            self.tx(),
            self.rx()
        )
    }
}
#[doc = "Timeout Counter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tocc(pub u32);
impl Tocc {
    #[doc = "Enable timeout counter."]
    #[must_use]
    #[inline(always)]
    pub const fn etoc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable timeout counter."]
    #[inline(always)]
    pub const fn set_etoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timeout select."]
    #[must_use]
    #[inline(always)]
    pub const fn tos(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Timeout select."]
    #[inline(always)]
    pub const fn set_tos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Timeout period."]
    #[must_use]
    #[inline(always)]
    pub const fn top(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout period."]
    #[inline(always)]
    pub const fn set_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tocc {
    #[inline(always)]
    fn default() -> Tocc {
        Tocc(0)
    }
}
impl core::fmt::Debug for Tocc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tocc")
            .field("etoc", &self.etoc())
            .field("tos", &self.tos())
            .field("top", &self.top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tocc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tocc {{ etoc: {=bool:?}, tos: {=u8:?}, top: {=u16:?} }}",
            self.etoc(),
            self.tos(),
            self.top()
        )
    }
}
#[doc = "Timeout Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tocv(pub u32);
impl Tocv {
    #[doc = "Timeout counter."]
    #[must_use]
    #[inline(always)]
    pub const fn toc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout counter."]
    #[inline(always)]
    pub const fn set_toc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tocv {
    #[inline(always)]
    fn default() -> Tocv {
        Tocv(0)
    }
}
impl core::fmt::Debug for Tocv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tocv").field("toc", &self.toc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tocv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tocv {{ toc: {=u16:?} }}", self.toc())
    }
}
#[doc = "Timestamp Counter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscc(pub u32);
impl Tscc {
    #[doc = "Timestamp select."]
    #[must_use]
    #[inline(always)]
    pub const fn tss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Timestamp select."]
    #[inline(always)]
    pub const fn set_tss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times."]
    #[must_use]
    #[inline(always)]
    pub const fn tcp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times."]
    #[inline(always)]
    pub const fn set_tcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Tscc {
    #[inline(always)]
    fn default() -> Tscc {
        Tscc(0)
    }
}
impl core::fmt::Debug for Tscc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tscc")
            .field("tss", &self.tss())
            .field("tcp", &self.tcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tscc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tscc {{ tss: {=u8:?}, tcp: {=u8:?} }}",
            self.tss(),
            self.tcp()
        )
    }
}
#[doc = "Timestamp Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscv(pub u32);
impl Tscv {
    #[doc = "Timestamp counter."]
    #[must_use]
    #[inline(always)]
    pub const fn tsc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timestamp counter."]
    #[inline(always)]
    pub const fn set_tsc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tscv {
    #[inline(always)]
    fn default() -> Tscv {
        Tscv(0)
    }
}
impl core::fmt::Debug for Tscv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tscv").field("tsc", &self.tsc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tscv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tscv {{ tsc: {=u16:?} }}", self.tsc())
    }
}
#[doc = "Tx Buffer Add Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbar(pub u32);
impl Txbar {
    #[doc = "Add request."]
    #[must_use]
    #[inline(always)]
    pub const fn ar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Add request."]
    #[inline(always)]
    pub const fn set_ar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbar {
    #[inline(always)]
    fn default() -> Txbar {
        Txbar(0)
    }
}
impl core::fmt::Debug for Txbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbar").field("ar", &self.ar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbar {{ ar: {=u32:?} }}", self.ar())
    }
}
#[doc = "Tx Buffer Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbc(pub u32);
impl Txbc {
    #[doc = "Tx buffers start address."]
    #[must_use]
    #[inline(always)]
    pub const fn tbsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Tx buffers start address."]
    #[inline(always)]
    pub const fn set_tbsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[must_use]
    #[inline(always)]
    pub const fn ndtb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[inline(always)]
    pub const fn set_ndtb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn tfqs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[inline(always)]
    pub const fn set_tfqs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Tx FIFO/queue mode."]
    #[must_use]
    #[inline(always)]
    pub const fn tfqm(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/queue mode."]
    #[inline(always)]
    pub const fn set_tfqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Txbc {
    #[inline(always)]
    fn default() -> Txbc {
        Txbc(0)
    }
}
impl core::fmt::Debug for Txbc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbc")
            .field("tbsa", &self.tbsa())
            .field("ndtb", &self.ndtb())
            .field("tfqs", &self.tfqs())
            .field("tfqm", &self.tfqm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txbc {{ tbsa: {=u16:?}, ndtb: {=u8:?}, tfqs: {=u8:?}, tfqm: {=bool:?} }}",
            self.tbsa(),
            self.ndtb(),
            self.tfqs(),
            self.tfqm()
        )
    }
}
#[doc = "Tx Buffer Cancellation Finished"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbcf(pub u32);
impl Txbcf {
    #[doc = "Cancellation finished."]
    #[must_use]
    #[inline(always)]
    pub const fn to(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation finished."]
    #[inline(always)]
    pub const fn set_to(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbcf {
    #[inline(always)]
    fn default() -> Txbcf {
        Txbcf(0)
    }
}
impl core::fmt::Debug for Txbcf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbcf").field("to", &self.to()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbcf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbcf {{ to: {=u32:?} }}", self.to())
    }
}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbcie(pub u32);
impl Txbcie {
    #[doc = "Cancellation finished interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cfie(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation finished interrupt enable."]
    #[inline(always)]
    pub const fn set_cfie(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbcie {
    #[inline(always)]
    fn default() -> Txbcie {
        Txbcie(0)
    }
}
impl core::fmt::Debug for Txbcie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbcie")
            .field("cfie", &self.cfie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbcie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbcie {{ cfie: {=u32:?} }}", self.cfie())
    }
}
#[doc = "Tx Buffer Cancellation Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbcr(pub u32);
impl Txbcr {
    #[doc = "Cancellation request."]
    #[must_use]
    #[inline(always)]
    pub const fn cr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation request."]
    #[inline(always)]
    pub const fn set_cr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbcr {
    #[inline(always)]
    fn default() -> Txbcr {
        Txbcr(0)
    }
}
impl core::fmt::Debug for Txbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbcr").field("cr", &self.cr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbcr {{ cr: {=u32:?} }}", self.cr())
    }
}
#[doc = "Tx Buffer Request Pending"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbrp(pub u32);
impl Txbrp {
    #[doc = "Transmission request pending."]
    #[must_use]
    #[inline(always)]
    pub const fn trp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission request pending."]
    #[inline(always)]
    pub const fn set_trp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbrp {
    #[inline(always)]
    fn default() -> Txbrp {
        Txbrp(0)
    }
}
impl core::fmt::Debug for Txbrp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbrp").field("trp", &self.trp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbrp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbrp {{ trp: {=u32:?} }}", self.trp())
    }
}
#[doc = "Tx Buffer Transmission Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbtie(pub u32);
impl Txbtie {
    #[doc = "Transmission interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission interrupt enable."]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbtie {
    #[inline(always)]
    fn default() -> Txbtie {
        Txbtie(0)
    }
}
impl core::fmt::Debug for Txbtie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbtie").field("tie", &self.tie()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbtie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbtie {{ tie: {=u32:?} }}", self.tie())
    }
}
#[doc = "Tx Buffer Transmission Occurred"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txbto(pub u32);
impl Txbto {
    #[doc = "Transmission occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn to(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission occurred."]
    #[inline(always)]
    pub const fn set_to(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txbto {
    #[inline(always)]
    fn default() -> Txbto {
        Txbto(0)
    }
}
impl core::fmt::Debug for Txbto {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txbto").field("to", &self.to()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txbto {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txbto {{ to: {=u32:?} }}", self.to())
    }
}
#[doc = "Tx Event FIFO Acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txefa(pub u32);
impl Txefa {
    #[doc = "Event FIFO acknowledge index."]
    #[must_use]
    #[inline(always)]
    pub const fn efai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO acknowledge index."]
    #[inline(always)]
    pub const fn set_efai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Txefa {
    #[inline(always)]
    fn default() -> Txefa {
        Txefa(0)
    }
}
impl core::fmt::Debug for Txefa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txefa").field("efai", &self.efai()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txefa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txefa {{ efai: {=u8:?} }}", self.efai())
    }
}
#[doc = "Tx Event FIFO Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txefc(pub u32);
impl Txefc {
    #[doc = "Event FIFO start address."]
    #[must_use]
    #[inline(always)]
    pub const fn efsa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Event FIFO start address."]
    #[inline(always)]
    pub const fn set_efsa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Event FIFO size 0 = Tx event FIFO disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn efs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO size 0 = Tx event FIFO disabled."]
    #[inline(always)]
    pub const fn set_efs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Event FIFO watermark 0 = Watermark interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn efwm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub const fn set_efwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Txefc {
    #[inline(always)]
    fn default() -> Txefc {
        Txefc(0)
    }
}
impl core::fmt::Debug for Txefc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txefc")
            .field("efsa", &self.efsa())
            .field("efs", &self.efs())
            .field("efwm", &self.efwm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txefc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txefc {{ efsa: {=u16:?}, efs: {=u8:?}, efwm: {=u8:?} }}",
            self.efsa(),
            self.efs(),
            self.efwm()
        )
    }
}
#[doc = "Tx Event FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txefs(pub u32);
impl Txefs {
    #[doc = "Event FIFO fill level."]
    #[must_use]
    #[inline(always)]
    pub const fn effl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO fill level."]
    #[inline(always)]
    pub const fn set_effl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Event FIFO get index."]
    #[must_use]
    #[inline(always)]
    pub const fn efgi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO get index."]
    #[inline(always)]
    pub const fn set_efgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Event FIFO put index."]
    #[must_use]
    #[inline(always)]
    pub const fn efpi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO put index."]
    #[inline(always)]
    pub const fn set_efpi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Event FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn eff(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event FIFO full."]
    #[inline(always)]
    pub const fn set_eff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tx event FIFO element lost."]
    #[must_use]
    #[inline(always)]
    pub const fn tefl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost."]
    #[inline(always)]
    pub const fn set_tefl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Txefs {
    #[inline(always)]
    fn default() -> Txefs {
        Txefs(0)
    }
}
impl core::fmt::Debug for Txefs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txefs")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txefs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txefs {{ effl: {=u8:?}, efgi: {=u8:?}, efpi: {=u8:?}, eff: {=bool:?}, tefl: {=bool:?} }}",
            self.effl(),
            self.efgi(),
            self.efpi(),
            self.eff(),
            self.tefl()
        )
    }
}
#[doc = "Tx Buffer Element Size Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txesc(pub u32);
impl Txesc {
    #[doc = "Tx buffer data field size."]
    #[must_use]
    #[inline(always)]
    pub const fn tbds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tx buffer data field size."]
    #[inline(always)]
    pub const fn set_tbds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Txesc {
    #[inline(always)]
    fn default() -> Txesc {
        Txesc(0)
    }
}
impl core::fmt::Debug for Txesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txesc").field("tbds", &self.tbds()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txesc {{ tbds: {=u8:?} }}", self.tbds())
    }
}
#[doc = "Tx FIFO/Queue Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txfqs(pub u32);
impl Txfqs {
    #[doc = "Tx FIFO get index."]
    #[must_use]
    #[inline(always)]
    pub const fn tfgi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO get index."]
    #[inline(always)]
    pub const fn set_tfgi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Tx FIFO/queue put index."]
    #[must_use]
    #[inline(always)]
    pub const fn tfqpi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO/queue put index."]
    #[inline(always)]
    pub const fn set_tfqpi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Tx FIFO/queue full."]
    #[must_use]
    #[inline(always)]
    pub const fn tfqf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/queue full."]
    #[inline(always)]
    pub const fn set_tfqf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Txfqs {
    #[inline(always)]
    fn default() -> Txfqs {
        Txfqs(0)
    }
}
impl core::fmt::Debug for Txfqs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txfqs")
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txfqs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txfqs {{ tfgi: {=u8:?}, tfqpi: {=u8:?}, tfqf: {=bool:?} }}",
            self.tfgi(),
            self.tfqpi(),
            self.tfqf()
        )
    }
}
#[doc = "Extended ID AND Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xidam(pub u32);
impl Xidam {
    #[doc = "Extended ID mask."]
    #[must_use]
    #[inline(always)]
    pub const fn eidm(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Extended ID mask."]
    #[inline(always)]
    pub const fn set_eidm(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for Xidam {
    #[inline(always)]
    fn default() -> Xidam {
        Xidam(0)
    }
}
impl core::fmt::Debug for Xidam {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xidam").field("eidm", &self.eidm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xidam {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xidam {{ eidm: {=u32:?} }}", self.eidm())
    }
}
#[doc = "Extended ID Filter Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xidfc(pub u32);
impl Xidfc {
    #[doc = "Filter list extended start address."]
    #[must_use]
    #[inline(always)]
    pub const fn flesa(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter list extended start address."]
    #[inline(always)]
    pub const fn set_flesa(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List size extended 0 = No extended message ID filter."]
    #[must_use]
    #[inline(always)]
    pub const fn lse(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List size extended 0 = No extended message ID filter."]
    #[inline(always)]
    pub const fn set_lse(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Xidfc {
    #[inline(always)]
    fn default() -> Xidfc {
        Xidfc(0)
    }
}
impl core::fmt::Debug for Xidfc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xidfc")
            .field("flesa", &self.flesa())
            .field("lse", &self.lse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xidfc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xidfc {{ flesa: {=u16:?}, lse: {=u8:?} }}",
            self.flesa(),
            self.lse()
        )
    }
}
