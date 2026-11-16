#[doc = "Clear DONE Status Bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdne(pub u8);
impl Cdne {
    #[doc = "Clear DONE field."]
    #[must_use]
    #[inline(always)]
    pub const fn cdne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear DONE field."]
    #[inline(always)]
    pub const fn set_cdne(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clears All DONE fields."]
    #[must_use]
    #[inline(always)]
    pub const fn cadn(&self) -> super::vals::Cadn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cadn::from_bits(val as u8)
    }
    #[doc = "Clears All DONE fields."]
    #[inline(always)]
    pub const fn set_cadn(&mut self, val: super::vals::Cadn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CdneNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CdneNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CdneNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cdne {
    #[inline(always)]
    fn default() -> Cdne {
        Cdne(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Cdne {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdne")
            .field("cdne", &self.cdne())
            .field("cadn", &self.cadn())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdne {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdne {{ cdne: {=u8:?}, cadn: {:?}, nop: {:?} }}",
            self.cdne(),
            self.cadn(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Error Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceei(pub u8);
impl Ceei {
    #[doc = "Clear Enable Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ceei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Enable Error Interrupt."]
    #[inline(always)]
    pub const fn set_ceei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Enable Error Interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn caee(&self) -> super::vals::Caee {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caee::from_bits(val as u8)
    }
    #[doc = "Clear All Enable Error Interrupts."]
    #[inline(always)]
    pub const fn set_caee(&mut self, val: super::vals::Caee) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CeeiNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CeeiNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CeeiNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ceei {
    #[inline(always)]
    fn default() -> Ceei {
        Ceei(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Ceei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ceei")
            .field("ceei", &self.ceei())
            .field("caee", &self.caee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ceei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ceei {{ ceei: {=u8:?}, caee: {:?}, nop: {:?} }}",
            self.ceei(),
            self.caee(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerq(pub u8);
impl Cerq {
    #[doc = "Clear Enable Request."]
    #[must_use]
    #[inline(always)]
    pub const fn cerq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Enable Request."]
    #[inline(always)]
    pub const fn set_cerq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Enable Requests."]
    #[must_use]
    #[inline(always)]
    pub const fn caer(&self) -> super::vals::Caer {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caer::from_bits(val as u8)
    }
    #[doc = "Clear All Enable Requests."]
    #[inline(always)]
    pub const fn set_caer(&mut self, val: super::vals::Caer) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CerqNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CerqNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CerqNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerq {
    #[inline(always)]
    fn default() -> Cerq {
        Cerq(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Cerq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerq")
            .field("cerq", &self.cerq())
            .field("caer", &self.caer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerq {{ cerq: {=u8:?}, caer: {:?}, nop: {:?} }}",
            self.cerq(),
            self.caer(),
            self.nop()
        )
    }
}
#[doc = "Clear Error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerr(pub u8);
impl Cerr {
    #[doc = "Clear Error Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn cerr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Error Indicator."]
    #[inline(always)]
    pub const fn set_cerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Error Indicators."]
    #[must_use]
    #[inline(always)]
    pub const fn caei(&self) -> super::vals::Caei {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caei::from_bits(val as u8)
    }
    #[doc = "Clear All Error Indicators."]
    #[inline(always)]
    pub const fn set_caei(&mut self, val: super::vals::Caei) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CerrNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CerrNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CerrNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerr {
    #[inline(always)]
    fn default() -> Cerr {
        Cerr(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Cerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerr")
            .field("cerr", &self.cerr())
            .field("caei", &self.caei())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerr {{ cerr: {=u8:?}, caei: {:?}, nop: {:?} }}",
            self.cerr(),
            self.caei(),
            self.nop()
        )
    }
}
#[doc = "Clear Interrupt Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cint(pub u8);
impl Cint {
    #[doc = "Clear Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn cint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Clear Interrupt Request."]
    #[inline(always)]
    pub const fn set_cint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Clear All Interrupt Requests."]
    #[must_use]
    #[inline(always)]
    pub const fn cair(&self) -> super::vals::Cair {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cair::from_bits(val as u8)
    }
    #[doc = "Clear All Interrupt Requests."]
    #[inline(always)]
    pub const fn set_cair(&mut self, val: super::vals::Cair) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CintNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CintNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CintNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cint {
    #[inline(always)]
    fn default() -> Cint {
        Cint(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Cint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cint")
            .field("cint", &self.cint())
            .field("cair", &self.cair())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cint {{ cint: {=u8:?}, cair: {:?}, nop: {:?} }}",
            self.cint(),
            self.cair(),
            self.nop()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Enable Debug."]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug."]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Round Robin Group Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn erga(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Group Arbitration."]
    #[inline(always)]
    pub const fn set_erga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Halt On Error."]
    #[must_use]
    #[inline(always)]
    pub const fn hoe(&self) -> super::vals::Hoe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hoe::from_bits(val as u8)
    }
    #[doc = "Halt On Error."]
    #[inline(always)]
    pub const fn set_hoe(&mut self, val: super::vals::Hoe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt eDMA Operations."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Halt eDMA Operations."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Continuous Link Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn clm(&self) -> super::vals::Clm {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Clm::from_bits(val as u8)
    }
    #[doc = "Continuous Link Mode."]
    #[inline(always)]
    pub const fn set_clm(&mut self, val: super::vals::Clm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Minor Loop Mapping."]
    #[must_use]
    #[inline(always)]
    pub const fn emlm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Minor Loop Mapping."]
    #[inline(always)]
    pub const fn set_emlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Channel Group 0 Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grp0pri(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Group 0 Priority."]
    #[inline(always)]
    pub const fn set_grp0pri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel Group 1 Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grp1pri(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Group 1 Priority."]
    #[inline(always)]
    pub const fn set_grp1pri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error Cancel Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::CrEcx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrEcx::from_bits(val as u8)
    }
    #[doc = "Error Cancel Transfer."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::CrEcx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Cancel Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> super::vals::Cx {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer."]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: super::vals::Cx) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "eDMA Active Status."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "eDMA Active Status."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("erga", &self.erga())
            .field("hoe", &self.hoe())
            .field("halt", &self.halt())
            .field("clm", &self.clm())
            .field("emlm", &self.emlm())
            .field("grp0pri", &self.grp0pri())
            .field("grp1pri", &self.grp1pri())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ edbg: {=bool:?}, erca: {=bool:?}, erga: {=bool:?}, hoe: {:?}, halt: {:?}, clm: {:?}, emlm: {=bool:?}, grp0pri: {=bool:?}, grp1pri: {=bool:?}, ecx: {:?}, cx: {:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.erga(),
            self.hoe(),
            self.halt(),
            self.clm(),
            self.emlm(),
            self.grp0pri(),
            self.grp1pri(),
            self.ecx(),
            self.cx(),
            self.active()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri0(pub u8);
impl Dchpri0 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri0Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri0Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri0Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri0 {
    #[inline(always)]
    fn default() -> Dchpri0 {
        Dchpri0(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri0")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri0 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri1(pub u8);
impl Dchpri1 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri1Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri1Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri1Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri1 {
    #[inline(always)]
    fn default() -> Dchpri1 {
        Dchpri1(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri1")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri1 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri10(pub u8);
impl Dchpri10 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri10Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri10Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri10Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri10 {
    #[inline(always)]
    fn default() -> Dchpri10 {
        Dchpri10(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri10")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri10 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri11(pub u8);
impl Dchpri11 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri11Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri11Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri11Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri11 {
    #[inline(always)]
    fn default() -> Dchpri11 {
        Dchpri11(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri11")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri11 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri12(pub u8);
impl Dchpri12 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri12Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri12Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri12Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri12 {
    #[inline(always)]
    fn default() -> Dchpri12 {
        Dchpri12(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri12")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri12 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri13(pub u8);
impl Dchpri13 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri13Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri13Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri13Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri13 {
    #[inline(always)]
    fn default() -> Dchpri13 {
        Dchpri13(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri13")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri13 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri14(pub u8);
impl Dchpri14 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri14Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri14Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri14Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri14 {
    #[inline(always)]
    fn default() -> Dchpri14 {
        Dchpri14(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri14")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri14 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri15(pub u8);
impl Dchpri15 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri15Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri15Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri15Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri15 {
    #[inline(always)]
    fn default() -> Dchpri15 {
        Dchpri15(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri15")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri15 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri16(pub u8);
impl Dchpri16 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri16Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri16Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri16Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri16 {
    #[inline(always)]
    fn default() -> Dchpri16 {
        Dchpri16(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri16")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri16 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri17(pub u8);
impl Dchpri17 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri17Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri17Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri17Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri17 {
    #[inline(always)]
    fn default() -> Dchpri17 {
        Dchpri17(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri17")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri17 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri18(pub u8);
impl Dchpri18 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri18Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri18Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri18Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri18 {
    #[inline(always)]
    fn default() -> Dchpri18 {
        Dchpri18(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri18")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri18 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri19(pub u8);
impl Dchpri19 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri19Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri19Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri19Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri19 {
    #[inline(always)]
    fn default() -> Dchpri19 {
        Dchpri19(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri19")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri19 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri2(pub u8);
impl Dchpri2 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri2Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri2Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri2Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri2 {
    #[inline(always)]
    fn default() -> Dchpri2 {
        Dchpri2(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri2")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri2 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri20(pub u8);
impl Dchpri20 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri20Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri20Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri20Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri20 {
    #[inline(always)]
    fn default() -> Dchpri20 {
        Dchpri20(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri20")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri20 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri21(pub u8);
impl Dchpri21 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri21Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri21Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri21Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri21 {
    #[inline(always)]
    fn default() -> Dchpri21 {
        Dchpri21(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri21")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri21 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri22(pub u8);
impl Dchpri22 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri22Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri22Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri22Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri22 {
    #[inline(always)]
    fn default() -> Dchpri22 {
        Dchpri22(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri22")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri22 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri23(pub u8);
impl Dchpri23 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri23Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri23Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri23Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri23 {
    #[inline(always)]
    fn default() -> Dchpri23 {
        Dchpri23(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri23")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri23 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri24(pub u8);
impl Dchpri24 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri24Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri24Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri24Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri24 {
    #[inline(always)]
    fn default() -> Dchpri24 {
        Dchpri24(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri24")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri24 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri25(pub u8);
impl Dchpri25 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri25Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri25Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri25Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri25 {
    #[inline(always)]
    fn default() -> Dchpri25 {
        Dchpri25(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri25")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri25 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri26(pub u8);
impl Dchpri26 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri26Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri26Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri26Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri26 {
    #[inline(always)]
    fn default() -> Dchpri26 {
        Dchpri26(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri26")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri26 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri27(pub u8);
impl Dchpri27 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri27Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri27Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri27Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri27 {
    #[inline(always)]
    fn default() -> Dchpri27 {
        Dchpri27(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri27")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri27 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri28(pub u8);
impl Dchpri28 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri28Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri28Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri28Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri28 {
    #[inline(always)]
    fn default() -> Dchpri28 {
        Dchpri28(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri28")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri28 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri29(pub u8);
impl Dchpri29 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri29Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri29Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri29Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri29 {
    #[inline(always)]
    fn default() -> Dchpri29 {
        Dchpri29(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri29")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri29 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri3(pub u8);
impl Dchpri3 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri3Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri3Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri3Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri3 {
    #[inline(always)]
    fn default() -> Dchpri3 {
        Dchpri3(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri3")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri3 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri30(pub u8);
impl Dchpri30 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri30Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri30Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri30Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri30 {
    #[inline(always)]
    fn default() -> Dchpri30 {
        Dchpri30(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri30")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri30 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri31(pub u8);
impl Dchpri31 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri31Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri31Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri31Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri31 {
    #[inline(always)]
    fn default() -> Dchpri31 {
        Dchpri31(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri31")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri31 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri4(pub u8);
impl Dchpri4 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri4Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri4Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri4Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri4 {
    #[inline(always)]
    fn default() -> Dchpri4 {
        Dchpri4(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri4")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri4 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri5(pub u8);
impl Dchpri5 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri5Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri5Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri5Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri5 {
    #[inline(always)]
    fn default() -> Dchpri5 {
        Dchpri5(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri5")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri5 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri6(pub u8);
impl Dchpri6 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri6Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri6Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri6Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri6 {
    #[inline(always)]
    fn default() -> Dchpri6 {
        Dchpri6(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri6")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri6 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri7(pub u8);
impl Dchpri7 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri7Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri7Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri7Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri7 {
    #[inline(always)]
    fn default() -> Dchpri7 {
        Dchpri7(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri7")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri7 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri8(pub u8);
impl Dchpri8 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri8Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri8Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri8Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri8 {
    #[inline(always)]
    fn default() -> Dchpri8 {
        Dchpri8(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri8")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri8 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri9(pub u8);
impl Dchpri9 {
    #[doc = "Channel n Arbitration Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority."]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Channel n Current Group Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn grppri(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel n Current Group Priority."]
    #[inline(always)]
    pub const fn set_grppri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri9Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri9Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri9Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri9 {
    #[inline(always)]
    fn default() -> Dchpri9 {
        Dchpri9(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Dchpri9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri9")
            .field("chpri", &self.chpri())
            .field("grppri", &self.grppri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri9 {{ chpri: {=u8:?}, grppri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.grppri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Enable Asynchronous Request in Stop."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ears(pub u32);
impl Ears {
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub const fn set_edreq_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub const fn set_edreq_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub const fn set_edreq_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub const fn set_edreq_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    #[inline(always)]
    pub const fn set_edreq_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    #[inline(always)]
    pub const fn set_edreq_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    #[inline(always)]
    pub const fn set_edreq_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    #[inline(always)]
    pub const fn set_edreq_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    #[inline(always)]
    pub const fn set_edreq_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    #[inline(always)]
    pub const fn set_edreq_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    #[inline(always)]
    pub const fn set_edreq_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    #[inline(always)]
    pub const fn set_edreq_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    #[inline(always)]
    pub const fn set_edreq_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    #[inline(always)]
    pub const fn set_edreq_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    #[inline(always)]
    pub const fn set_edreq_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    #[inline(always)]
    pub const fn set_edreq_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 16."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 16."]
    #[inline(always)]
    pub const fn set_edreq_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 17."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 17."]
    #[inline(always)]
    pub const fn set_edreq_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 18."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 18."]
    #[inline(always)]
    pub const fn set_edreq_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 19."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 19."]
    #[inline(always)]
    pub const fn set_edreq_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 20."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 20."]
    #[inline(always)]
    pub const fn set_edreq_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 21."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 21."]
    #[inline(always)]
    pub const fn set_edreq_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 22."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 22."]
    #[inline(always)]
    pub const fn set_edreq_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 23."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 23."]
    #[inline(always)]
    pub const fn set_edreq_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 24."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 24."]
    #[inline(always)]
    pub const fn set_edreq_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 25."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 25."]
    #[inline(always)]
    pub const fn set_edreq_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 26."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 26."]
    #[inline(always)]
    pub const fn set_edreq_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 27."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 27."]
    #[inline(always)]
    pub const fn set_edreq_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 28."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 28."]
    #[inline(always)]
    pub const fn set_edreq_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 29."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 29."]
    #[inline(always)]
    pub const fn set_edreq_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 30."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 30."]
    #[inline(always)]
    pub const fn set_edreq_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 31."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 31."]
    #[inline(always)]
    pub const fn set_edreq_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ears {
    #[inline(always)]
    fn default() -> Ears {
        Ears(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Ears {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ears")
            .field("edreq_0", &self.edreq_0())
            .field("edreq_1", &self.edreq_1())
            .field("edreq_2", &self.edreq_2())
            .field("edreq_3", &self.edreq_3())
            .field("edreq_4", &self.edreq_4())
            .field("edreq_5", &self.edreq_5())
            .field("edreq_6", &self.edreq_6())
            .field("edreq_7", &self.edreq_7())
            .field("edreq_8", &self.edreq_8())
            .field("edreq_9", &self.edreq_9())
            .field("edreq_10", &self.edreq_10())
            .field("edreq_11", &self.edreq_11())
            .field("edreq_12", &self.edreq_12())
            .field("edreq_13", &self.edreq_13())
            .field("edreq_14", &self.edreq_14())
            .field("edreq_15", &self.edreq_15())
            .field("edreq_16", &self.edreq_16())
            .field("edreq_17", &self.edreq_17())
            .field("edreq_18", &self.edreq_18())
            .field("edreq_19", &self.edreq_19())
            .field("edreq_20", &self.edreq_20())
            .field("edreq_21", &self.edreq_21())
            .field("edreq_22", &self.edreq_22())
            .field("edreq_23", &self.edreq_23())
            .field("edreq_24", &self.edreq_24())
            .field("edreq_25", &self.edreq_25())
            .field("edreq_26", &self.edreq_26())
            .field("edreq_27", &self.edreq_27())
            .field("edreq_28", &self.edreq_28())
            .field("edreq_29", &self.edreq_29())
            .field("edreq_30", &self.edreq_30())
            .field("edreq_31", &self.edreq_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ears {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ears {{ edreq_0: {=bool:?}, edreq_1: {=bool:?}, edreq_2: {=bool:?}, edreq_3: {=bool:?}, edreq_4: {=bool:?}, edreq_5: {=bool:?}, edreq_6: {=bool:?}, edreq_7: {=bool:?}, edreq_8: {=bool:?}, edreq_9: {=bool:?}, edreq_10: {=bool:?}, edreq_11: {=bool:?}, edreq_12: {=bool:?}, edreq_13: {=bool:?}, edreq_14: {=bool:?}, edreq_15: {=bool:?}, edreq_16: {=bool:?}, edreq_17: {=bool:?}, edreq_18: {=bool:?}, edreq_19: {=bool:?}, edreq_20: {=bool:?}, edreq_21: {=bool:?}, edreq_22: {=bool:?}, edreq_23: {=bool:?}, edreq_24: {=bool:?}, edreq_25: {=bool:?}, edreq_26: {=bool:?}, edreq_27: {=bool:?}, edreq_28: {=bool:?}, edreq_29: {=bool:?}, edreq_30: {=bool:?}, edreq_31: {=bool:?} }}",
            self.edreq_0(),
            self.edreq_1(),
            self.edreq_2(),
            self.edreq_3(),
            self.edreq_4(),
            self.edreq_5(),
            self.edreq_6(),
            self.edreq_7(),
            self.edreq_8(),
            self.edreq_9(),
            self.edreq_10(),
            self.edreq_11(),
            self.edreq_12(),
            self.edreq_13(),
            self.edreq_14(),
            self.edreq_15(),
            self.edreq_16(),
            self.edreq_17(),
            self.edreq_18(),
            self.edreq_19(),
            self.edreq_20(),
            self.edreq_21(),
            self.edreq_22(),
            self.edreq_23(),
            self.edreq_24(),
            self.edreq_25(),
            self.edreq_26(),
            self.edreq_27(),
            self.edreq_28(),
            self.edreq_29(),
            self.edreq_30(),
            self.edreq_31()
        )
    }
}
#[doc = "Enable Error Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eei(pub u32);
impl Eei {
    #[doc = "Enable Error Interrupt 0."]
    #[must_use]
    #[inline(always)]
    pub const fn eei0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 0."]
    #[inline(always)]
    pub const fn set_eei0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Error Interrupt 1."]
    #[must_use]
    #[inline(always)]
    pub const fn eei1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 1."]
    #[inline(always)]
    pub const fn set_eei1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Error Interrupt 2."]
    #[must_use]
    #[inline(always)]
    pub const fn eei2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 2."]
    #[inline(always)]
    pub const fn set_eei2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Error Interrupt 3."]
    #[must_use]
    #[inline(always)]
    pub const fn eei3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 3."]
    #[inline(always)]
    pub const fn set_eei3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Error Interrupt 4."]
    #[must_use]
    #[inline(always)]
    pub const fn eei4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 4."]
    #[inline(always)]
    pub const fn set_eei4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable Error Interrupt 5."]
    #[must_use]
    #[inline(always)]
    pub const fn eei5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 5."]
    #[inline(always)]
    pub const fn set_eei5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Error Interrupt 6."]
    #[must_use]
    #[inline(always)]
    pub const fn eei6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 6."]
    #[inline(always)]
    pub const fn set_eei6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Error Interrupt 7."]
    #[must_use]
    #[inline(always)]
    pub const fn eei7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 7."]
    #[inline(always)]
    pub const fn set_eei7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable Error Interrupt 8."]
    #[must_use]
    #[inline(always)]
    pub const fn eei8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 8."]
    #[inline(always)]
    pub const fn set_eei8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable Error Interrupt 9."]
    #[must_use]
    #[inline(always)]
    pub const fn eei9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 9."]
    #[inline(always)]
    pub const fn set_eei9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Error Interrupt 10."]
    #[must_use]
    #[inline(always)]
    pub const fn eei10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 10."]
    #[inline(always)]
    pub const fn set_eei10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Error Interrupt 11."]
    #[must_use]
    #[inline(always)]
    pub const fn eei11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 11."]
    #[inline(always)]
    pub const fn set_eei11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Error Interrupt 12."]
    #[must_use]
    #[inline(always)]
    pub const fn eei12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 12."]
    #[inline(always)]
    pub const fn set_eei12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable Error Interrupt 13."]
    #[must_use]
    #[inline(always)]
    pub const fn eei13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 13."]
    #[inline(always)]
    pub const fn set_eei13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable Error Interrupt 14."]
    #[must_use]
    #[inline(always)]
    pub const fn eei14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 14."]
    #[inline(always)]
    pub const fn set_eei14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Error Interrupt 15."]
    #[must_use]
    #[inline(always)]
    pub const fn eei15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 15."]
    #[inline(always)]
    pub const fn set_eei15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable Error Interrupt 16."]
    #[must_use]
    #[inline(always)]
    pub const fn eei16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 16."]
    #[inline(always)]
    pub const fn set_eei16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable Error Interrupt 17."]
    #[must_use]
    #[inline(always)]
    pub const fn eei17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 17."]
    #[inline(always)]
    pub const fn set_eei17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable Error Interrupt 18."]
    #[must_use]
    #[inline(always)]
    pub const fn eei18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 18."]
    #[inline(always)]
    pub const fn set_eei18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable Error Interrupt 19."]
    #[must_use]
    #[inline(always)]
    pub const fn eei19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 19."]
    #[inline(always)]
    pub const fn set_eei19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable Error Interrupt 20."]
    #[must_use]
    #[inline(always)]
    pub const fn eei20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 20."]
    #[inline(always)]
    pub const fn set_eei20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable Error Interrupt 21."]
    #[must_use]
    #[inline(always)]
    pub const fn eei21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 21."]
    #[inline(always)]
    pub const fn set_eei21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable Error Interrupt 22."]
    #[must_use]
    #[inline(always)]
    pub const fn eei22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 22."]
    #[inline(always)]
    pub const fn set_eei22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable Error Interrupt 23."]
    #[must_use]
    #[inline(always)]
    pub const fn eei23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 23."]
    #[inline(always)]
    pub const fn set_eei23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable Error Interrupt 24."]
    #[must_use]
    #[inline(always)]
    pub const fn eei24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 24."]
    #[inline(always)]
    pub const fn set_eei24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable Error Interrupt 25."]
    #[must_use]
    #[inline(always)]
    pub const fn eei25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 25."]
    #[inline(always)]
    pub const fn set_eei25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable Error Interrupt 26."]
    #[must_use]
    #[inline(always)]
    pub const fn eei26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 26."]
    #[inline(always)]
    pub const fn set_eei26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable Error Interrupt 27."]
    #[must_use]
    #[inline(always)]
    pub const fn eei27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 27."]
    #[inline(always)]
    pub const fn set_eei27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable Error Interrupt 28."]
    #[must_use]
    #[inline(always)]
    pub const fn eei28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 28."]
    #[inline(always)]
    pub const fn set_eei28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable Error Interrupt 29."]
    #[must_use]
    #[inline(always)]
    pub const fn eei29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 29."]
    #[inline(always)]
    pub const fn set_eei29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable Error Interrupt 30."]
    #[must_use]
    #[inline(always)]
    pub const fn eei30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 30."]
    #[inline(always)]
    pub const fn set_eei30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable Error Interrupt 31."]
    #[must_use]
    #[inline(always)]
    pub const fn eei31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 31."]
    #[inline(always)]
    pub const fn set_eei31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eei {
    #[inline(always)]
    fn default() -> Eei {
        Eei(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Eei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eei")
            .field("eei0", &self.eei0())
            .field("eei1", &self.eei1())
            .field("eei2", &self.eei2())
            .field("eei3", &self.eei3())
            .field("eei4", &self.eei4())
            .field("eei5", &self.eei5())
            .field("eei6", &self.eei6())
            .field("eei7", &self.eei7())
            .field("eei8", &self.eei8())
            .field("eei9", &self.eei9())
            .field("eei10", &self.eei10())
            .field("eei11", &self.eei11())
            .field("eei12", &self.eei12())
            .field("eei13", &self.eei13())
            .field("eei14", &self.eei14())
            .field("eei15", &self.eei15())
            .field("eei16", &self.eei16())
            .field("eei17", &self.eei17())
            .field("eei18", &self.eei18())
            .field("eei19", &self.eei19())
            .field("eei20", &self.eei20())
            .field("eei21", &self.eei21())
            .field("eei22", &self.eei22())
            .field("eei23", &self.eei23())
            .field("eei24", &self.eei24())
            .field("eei25", &self.eei25())
            .field("eei26", &self.eei26())
            .field("eei27", &self.eei27())
            .field("eei28", &self.eei28())
            .field("eei29", &self.eei29())
            .field("eei30", &self.eei30())
            .field("eei31", &self.eei31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eei {{ eei0: {=bool:?}, eei1: {=bool:?}, eei2: {=bool:?}, eei3: {=bool:?}, eei4: {=bool:?}, eei5: {=bool:?}, eei6: {=bool:?}, eei7: {=bool:?}, eei8: {=bool:?}, eei9: {=bool:?}, eei10: {=bool:?}, eei11: {=bool:?}, eei12: {=bool:?}, eei13: {=bool:?}, eei14: {=bool:?}, eei15: {=bool:?}, eei16: {=bool:?}, eei17: {=bool:?}, eei18: {=bool:?}, eei19: {=bool:?}, eei20: {=bool:?}, eei21: {=bool:?}, eei22: {=bool:?}, eei23: {=bool:?}, eei24: {=bool:?}, eei25: {=bool:?}, eei26: {=bool:?}, eei27: {=bool:?}, eei28: {=bool:?}, eei29: {=bool:?}, eei30: {=bool:?}, eei31: {=bool:?} }}",
            self.eei0(),
            self.eei1(),
            self.eei2(),
            self.eei3(),
            self.eei4(),
            self.eei5(),
            self.eei6(),
            self.eei7(),
            self.eei8(),
            self.eei9(),
            self.eei10(),
            self.eei11(),
            self.eei12(),
            self.eei13(),
            self.eei14(),
            self.eei15(),
            self.eei16(),
            self.eei17(),
            self.eei18(),
            self.eei19(),
            self.eei20(),
            self.eei21(),
            self.eei22(),
            self.eei23(),
            self.eei24(),
            self.eei25(),
            self.eei26(),
            self.eei27(),
            self.eei28(),
            self.eei29(),
            self.eei30(),
            self.eei31()
        )
    }
}
#[doc = "Enable Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erq(pub u32);
impl Erq {
    #[doc = "Enable DMA Request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn erq0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 0."]
    #[inline(always)]
    pub const fn set_erq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable DMA Request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn erq1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 1."]
    #[inline(always)]
    pub const fn set_erq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable DMA Request 2."]
    #[must_use]
    #[inline(always)]
    pub const fn erq2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 2."]
    #[inline(always)]
    pub const fn set_erq2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable DMA Request 3."]
    #[must_use]
    #[inline(always)]
    pub const fn erq3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 3."]
    #[inline(always)]
    pub const fn set_erq3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable DMA Request 4."]
    #[must_use]
    #[inline(always)]
    pub const fn erq4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 4."]
    #[inline(always)]
    pub const fn set_erq4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable DMA Request 5."]
    #[must_use]
    #[inline(always)]
    pub const fn erq5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 5."]
    #[inline(always)]
    pub const fn set_erq5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable DMA Request 6."]
    #[must_use]
    #[inline(always)]
    pub const fn erq6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 6."]
    #[inline(always)]
    pub const fn set_erq6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable DMA Request 7."]
    #[must_use]
    #[inline(always)]
    pub const fn erq7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 7."]
    #[inline(always)]
    pub const fn set_erq7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable DMA Request 8."]
    #[must_use]
    #[inline(always)]
    pub const fn erq8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 8."]
    #[inline(always)]
    pub const fn set_erq8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable DMA Request 9."]
    #[must_use]
    #[inline(always)]
    pub const fn erq9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 9."]
    #[inline(always)]
    pub const fn set_erq9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable DMA Request 10."]
    #[must_use]
    #[inline(always)]
    pub const fn erq10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 10."]
    #[inline(always)]
    pub const fn set_erq10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable DMA Request 11."]
    #[must_use]
    #[inline(always)]
    pub const fn erq11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 11."]
    #[inline(always)]
    pub const fn set_erq11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable DMA Request 12."]
    #[must_use]
    #[inline(always)]
    pub const fn erq12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 12."]
    #[inline(always)]
    pub const fn set_erq12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable DMA Request 13."]
    #[must_use]
    #[inline(always)]
    pub const fn erq13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 13."]
    #[inline(always)]
    pub const fn set_erq13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable DMA Request 14."]
    #[must_use]
    #[inline(always)]
    pub const fn erq14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 14."]
    #[inline(always)]
    pub const fn set_erq14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable DMA Request 15."]
    #[must_use]
    #[inline(always)]
    pub const fn erq15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 15."]
    #[inline(always)]
    pub const fn set_erq15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable DMA Request 16."]
    #[must_use]
    #[inline(always)]
    pub const fn erq16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 16."]
    #[inline(always)]
    pub const fn set_erq16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable DMA Request 17."]
    #[must_use]
    #[inline(always)]
    pub const fn erq17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 17."]
    #[inline(always)]
    pub const fn set_erq17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable DMA Request 18."]
    #[must_use]
    #[inline(always)]
    pub const fn erq18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 18."]
    #[inline(always)]
    pub const fn set_erq18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable DMA Request 19."]
    #[must_use]
    #[inline(always)]
    pub const fn erq19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 19."]
    #[inline(always)]
    pub const fn set_erq19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable DMA Request 20."]
    #[must_use]
    #[inline(always)]
    pub const fn erq20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 20."]
    #[inline(always)]
    pub const fn set_erq20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable DMA Request 21."]
    #[must_use]
    #[inline(always)]
    pub const fn erq21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 21."]
    #[inline(always)]
    pub const fn set_erq21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable DMA Request 22."]
    #[must_use]
    #[inline(always)]
    pub const fn erq22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 22."]
    #[inline(always)]
    pub const fn set_erq22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable DMA Request 23."]
    #[must_use]
    #[inline(always)]
    pub const fn erq23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 23."]
    #[inline(always)]
    pub const fn set_erq23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable DMA Request 24."]
    #[must_use]
    #[inline(always)]
    pub const fn erq24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 24."]
    #[inline(always)]
    pub const fn set_erq24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable DMA Request 25."]
    #[must_use]
    #[inline(always)]
    pub const fn erq25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 25."]
    #[inline(always)]
    pub const fn set_erq25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable DMA Request 26."]
    #[must_use]
    #[inline(always)]
    pub const fn erq26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 26."]
    #[inline(always)]
    pub const fn set_erq26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable DMA Request 27."]
    #[must_use]
    #[inline(always)]
    pub const fn erq27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 27."]
    #[inline(always)]
    pub const fn set_erq27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable DMA Request 28."]
    #[must_use]
    #[inline(always)]
    pub const fn erq28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 28."]
    #[inline(always)]
    pub const fn set_erq28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable DMA Request 29."]
    #[must_use]
    #[inline(always)]
    pub const fn erq29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 29."]
    #[inline(always)]
    pub const fn set_erq29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable DMA Request 30."]
    #[must_use]
    #[inline(always)]
    pub const fn erq30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 30."]
    #[inline(always)]
    pub const fn set_erq30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable DMA Request 31."]
    #[must_use]
    #[inline(always)]
    pub const fn erq31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 31."]
    #[inline(always)]
    pub const fn set_erq31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erq {
    #[inline(always)]
    fn default() -> Erq {
        Erq(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Erq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erq")
            .field("erq0", &self.erq0())
            .field("erq1", &self.erq1())
            .field("erq2", &self.erq2())
            .field("erq3", &self.erq3())
            .field("erq4", &self.erq4())
            .field("erq5", &self.erq5())
            .field("erq6", &self.erq6())
            .field("erq7", &self.erq7())
            .field("erq8", &self.erq8())
            .field("erq9", &self.erq9())
            .field("erq10", &self.erq10())
            .field("erq11", &self.erq11())
            .field("erq12", &self.erq12())
            .field("erq13", &self.erq13())
            .field("erq14", &self.erq14())
            .field("erq15", &self.erq15())
            .field("erq16", &self.erq16())
            .field("erq17", &self.erq17())
            .field("erq18", &self.erq18())
            .field("erq19", &self.erq19())
            .field("erq20", &self.erq20())
            .field("erq21", &self.erq21())
            .field("erq22", &self.erq22())
            .field("erq23", &self.erq23())
            .field("erq24", &self.erq24())
            .field("erq25", &self.erq25())
            .field("erq26", &self.erq26())
            .field("erq27", &self.erq27())
            .field("erq28", &self.erq28())
            .field("erq29", &self.erq29())
            .field("erq30", &self.erq30())
            .field("erq31", &self.erq31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erq {{ erq0: {=bool:?}, erq1: {=bool:?}, erq2: {=bool:?}, erq3: {=bool:?}, erq4: {=bool:?}, erq5: {=bool:?}, erq6: {=bool:?}, erq7: {=bool:?}, erq8: {=bool:?}, erq9: {=bool:?}, erq10: {=bool:?}, erq11: {=bool:?}, erq12: {=bool:?}, erq13: {=bool:?}, erq14: {=bool:?}, erq15: {=bool:?}, erq16: {=bool:?}, erq17: {=bool:?}, erq18: {=bool:?}, erq19: {=bool:?}, erq20: {=bool:?}, erq21: {=bool:?}, erq22: {=bool:?}, erq23: {=bool:?}, erq24: {=bool:?}, erq25: {=bool:?}, erq26: {=bool:?}, erq27: {=bool:?}, erq28: {=bool:?}, erq29: {=bool:?}, erq30: {=bool:?}, erq31: {=bool:?} }}",
            self.erq0(),
            self.erq1(),
            self.erq2(),
            self.erq3(),
            self.erq4(),
            self.erq5(),
            self.erq6(),
            self.erq7(),
            self.erq8(),
            self.erq9(),
            self.erq10(),
            self.erq11(),
            self.erq12(),
            self.erq13(),
            self.erq14(),
            self.erq15(),
            self.erq16(),
            self.erq17(),
            self.erq18(),
            self.erq19(),
            self.erq20(),
            self.erq21(),
            self.erq22(),
            self.erq23(),
            self.erq24(),
            self.erq25(),
            self.erq26(),
            self.erq27(),
            self.erq28(),
            self.erq29(),
            self.erq30(),
            self.erq31()
        )
    }
}
#[doc = "Error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Err(pub u32);
impl Err {
    #[doc = "Error In Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 0."]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Error In Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 1."]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error In Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 2."]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Error In Channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 3."]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Error In Channel 4."]
    #[must_use]
    #[inline(always)]
    pub const fn err4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 4."]
    #[inline(always)]
    pub const fn set_err4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Error In Channel 5."]
    #[must_use]
    #[inline(always)]
    pub const fn err5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 5."]
    #[inline(always)]
    pub const fn set_err5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Error In Channel 6."]
    #[must_use]
    #[inline(always)]
    pub const fn err6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 6."]
    #[inline(always)]
    pub const fn set_err6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Error In Channel 7."]
    #[must_use]
    #[inline(always)]
    pub const fn err7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 7."]
    #[inline(always)]
    pub const fn set_err7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error In Channel 8."]
    #[must_use]
    #[inline(always)]
    pub const fn err8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 8."]
    #[inline(always)]
    pub const fn set_err8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Error In Channel 9."]
    #[must_use]
    #[inline(always)]
    pub const fn err9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 9."]
    #[inline(always)]
    pub const fn set_err9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error In Channel 10."]
    #[must_use]
    #[inline(always)]
    pub const fn err10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 10."]
    #[inline(always)]
    pub const fn set_err10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error In Channel 11."]
    #[must_use]
    #[inline(always)]
    pub const fn err11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 11."]
    #[inline(always)]
    pub const fn set_err11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Error In Channel 12."]
    #[must_use]
    #[inline(always)]
    pub const fn err12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 12."]
    #[inline(always)]
    pub const fn set_err12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Error In Channel 13."]
    #[must_use]
    #[inline(always)]
    pub const fn err13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 13."]
    #[inline(always)]
    pub const fn set_err13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error In Channel 14."]
    #[must_use]
    #[inline(always)]
    pub const fn err14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 14."]
    #[inline(always)]
    pub const fn set_err14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error In Channel 15."]
    #[must_use]
    #[inline(always)]
    pub const fn err15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 15."]
    #[inline(always)]
    pub const fn set_err15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Error In Channel 16."]
    #[must_use]
    #[inline(always)]
    pub const fn err16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 16."]
    #[inline(always)]
    pub const fn set_err16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error In Channel 17."]
    #[must_use]
    #[inline(always)]
    pub const fn err17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 17."]
    #[inline(always)]
    pub const fn set_err17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error In Channel 18."]
    #[must_use]
    #[inline(always)]
    pub const fn err18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 18."]
    #[inline(always)]
    pub const fn set_err18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error In Channel 19."]
    #[must_use]
    #[inline(always)]
    pub const fn err19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 19."]
    #[inline(always)]
    pub const fn set_err19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Error In Channel 20."]
    #[must_use]
    #[inline(always)]
    pub const fn err20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 20."]
    #[inline(always)]
    pub const fn set_err20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Error In Channel 21."]
    #[must_use]
    #[inline(always)]
    pub const fn err21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 21."]
    #[inline(always)]
    pub const fn set_err21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error In Channel 22."]
    #[must_use]
    #[inline(always)]
    pub const fn err22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 22."]
    #[inline(always)]
    pub const fn set_err22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error In Channel 23."]
    #[must_use]
    #[inline(always)]
    pub const fn err23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 23."]
    #[inline(always)]
    pub const fn set_err23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Error In Channel 24."]
    #[must_use]
    #[inline(always)]
    pub const fn err24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 24."]
    #[inline(always)]
    pub const fn set_err24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Error In Channel 25."]
    #[must_use]
    #[inline(always)]
    pub const fn err25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 25."]
    #[inline(always)]
    pub const fn set_err25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Error In Channel 26."]
    #[must_use]
    #[inline(always)]
    pub const fn err26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 26."]
    #[inline(always)]
    pub const fn set_err26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Error In Channel 27."]
    #[must_use]
    #[inline(always)]
    pub const fn err27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 27."]
    #[inline(always)]
    pub const fn set_err27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Error In Channel 28."]
    #[must_use]
    #[inline(always)]
    pub const fn err28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 28."]
    #[inline(always)]
    pub const fn set_err28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Error In Channel 29."]
    #[must_use]
    #[inline(always)]
    pub const fn err29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 29."]
    #[inline(always)]
    pub const fn set_err29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Error In Channel 30."]
    #[must_use]
    #[inline(always)]
    pub const fn err30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 30."]
    #[inline(always)]
    pub const fn set_err30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Error In Channel 31."]
    #[must_use]
    #[inline(always)]
    pub const fn err31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 31."]
    #[inline(always)]
    pub const fn set_err31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Err {
    #[inline(always)]
    fn default() -> Err {
        Err(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Err")
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("err4", &self.err4())
            .field("err5", &self.err5())
            .field("err6", &self.err6())
            .field("err7", &self.err7())
            .field("err8", &self.err8())
            .field("err9", &self.err9())
            .field("err10", &self.err10())
            .field("err11", &self.err11())
            .field("err12", &self.err12())
            .field("err13", &self.err13())
            .field("err14", &self.err14())
            .field("err15", &self.err15())
            .field("err16", &self.err16())
            .field("err17", &self.err17())
            .field("err18", &self.err18())
            .field("err19", &self.err19())
            .field("err20", &self.err20())
            .field("err21", &self.err21())
            .field("err22", &self.err22())
            .field("err23", &self.err23())
            .field("err24", &self.err24())
            .field("err25", &self.err25())
            .field("err26", &self.err26())
            .field("err27", &self.err27())
            .field("err28", &self.err28())
            .field("err29", &self.err29())
            .field("err30", &self.err30())
            .field("err31", &self.err31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Err {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Err {{ err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, err4: {=bool:?}, err5: {=bool:?}, err6: {=bool:?}, err7: {=bool:?}, err8: {=bool:?}, err9: {=bool:?}, err10: {=bool:?}, err11: {=bool:?}, err12: {=bool:?}, err13: {=bool:?}, err14: {=bool:?}, err15: {=bool:?}, err16: {=bool:?}, err17: {=bool:?}, err18: {=bool:?}, err19: {=bool:?}, err20: {=bool:?}, err21: {=bool:?}, err22: {=bool:?}, err23: {=bool:?}, err24: {=bool:?}, err25: {=bool:?}, err26: {=bool:?}, err27: {=bool:?}, err28: {=bool:?}, err29: {=bool:?}, err30: {=bool:?}, err31: {=bool:?} }}",
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.err4(),
            self.err5(),
            self.err6(),
            self.err7(),
            self.err8(),
            self.err9(),
            self.err10(),
            self.err11(),
            self.err12(),
            self.err13(),
            self.err14(),
            self.err15(),
            self.err16(),
            self.err17(),
            self.err18(),
            self.err19(),
            self.err20(),
            self.err21(),
            self.err22(),
            self.err23(),
            self.err24(),
            self.err25(),
            self.err26(),
            self.err27(),
            self.err28(),
            self.err29(),
            self.err30(),
            self.err31()
        )
    }
}
#[doc = "Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Es(pub u32);
impl Es {
    #[doc = "Destination Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Bus Error."]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Bus Error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Offset Error."]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Error."]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source Offset Error."]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Error."]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Channel Priority Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cpe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Priority Error."]
    #[inline(always)]
    pub const fn set_cpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Group Priority Error."]
    #[must_use]
    #[inline(always)]
    pub const fn gpe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Group Priority Error."]
    #[inline(always)]
    pub const fn set_gpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transfer Canceled."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::EsEcx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::EsEcx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Logical OR of all ERR status fields."]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Logical OR of all ERR status fields."]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Es {
    #[inline(always)]
    fn default() -> Es {
        Es(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Es {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Es")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("errchn", &self.errchn())
            .field("cpe", &self.cpe())
            .field("gpe", &self.gpe())
            .field("ecx", &self.ecx())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Es {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Es {{ dbe: {=bool:?}, sbe: {=bool:?}, sge: {=bool:?}, nce: {=bool:?}, doe: {=bool:?}, dae: {=bool:?}, soe: {=bool:?}, sae: {=bool:?}, errchn: {=u8:?}, cpe: {=bool:?}, gpe: {=bool:?}, ecx: {:?}, vld: {=bool:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.errchn(),
            self.cpe(),
            self.gpe(),
            self.ecx(),
            self.vld()
        )
    }
}
#[doc = "Hardware Request Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrs(pub u32);
impl Hrs {
    #[doc = "Hardware Request Status Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 0."]
    #[inline(always)]
    pub const fn set_hrs0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware Request Status Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 1."]
    #[inline(always)]
    pub const fn set_hrs1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Hardware Request Status Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 2."]
    #[inline(always)]
    pub const fn set_hrs2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Hardware Request Status Channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 3."]
    #[inline(always)]
    pub const fn set_hrs3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Hardware Request Status Channel 4."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 4."]
    #[inline(always)]
    pub const fn set_hrs4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Hardware Request Status Channel 5."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 5."]
    #[inline(always)]
    pub const fn set_hrs5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Hardware Request Status Channel 6."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 6."]
    #[inline(always)]
    pub const fn set_hrs6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Request Status Channel 7."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 7."]
    #[inline(always)]
    pub const fn set_hrs7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Request Status Channel 8."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 8."]
    #[inline(always)]
    pub const fn set_hrs8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Request Status Channel 9."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 9."]
    #[inline(always)]
    pub const fn set_hrs9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Hardware Request Status Channel 10."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 10."]
    #[inline(always)]
    pub const fn set_hrs10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Hardware Request Status Channel 11."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 11."]
    #[inline(always)]
    pub const fn set_hrs11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware Request Status Channel 12."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 12."]
    #[inline(always)]
    pub const fn set_hrs12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Hardware Request Status Channel 13."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 13."]
    #[inline(always)]
    pub const fn set_hrs13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Hardware Request Status Channel 14."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 14."]
    #[inline(always)]
    pub const fn set_hrs14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Hardware Request Status Channel 15."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 15."]
    #[inline(always)]
    pub const fn set_hrs15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Hardware Request Status Channel 16."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 16."]
    #[inline(always)]
    pub const fn set_hrs16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Hardware Request Status Channel 17."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 17."]
    #[inline(always)]
    pub const fn set_hrs17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Hardware Request Status Channel 18."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 18."]
    #[inline(always)]
    pub const fn set_hrs18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Hardware Request Status Channel 19."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 19."]
    #[inline(always)]
    pub const fn set_hrs19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Hardware Request Status Channel 20."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 20."]
    #[inline(always)]
    pub const fn set_hrs20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hardware Request Status Channel 21."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 21."]
    #[inline(always)]
    pub const fn set_hrs21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Hardware Request Status Channel 22."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 22."]
    #[inline(always)]
    pub const fn set_hrs22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Hardware Request Status Channel 23."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 23."]
    #[inline(always)]
    pub const fn set_hrs23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Hardware Request Status Channel 24."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 24."]
    #[inline(always)]
    pub const fn set_hrs24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Hardware Request Status Channel 25."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 25."]
    #[inline(always)]
    pub const fn set_hrs25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Hardware Request Status Channel 26."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 26."]
    #[inline(always)]
    pub const fn set_hrs26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware Request Status Channel 27."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 27."]
    #[inline(always)]
    pub const fn set_hrs27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Hardware Request Status Channel 28."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 28."]
    #[inline(always)]
    pub const fn set_hrs28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Hardware Request Status Channel 29."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 29."]
    #[inline(always)]
    pub const fn set_hrs29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Hardware Request Status Channel 30."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 30."]
    #[inline(always)]
    pub const fn set_hrs30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Hardware Request Status Channel 31."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 31."]
    #[inline(always)]
    pub const fn set_hrs31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hrs {
    #[inline(always)]
    fn default() -> Hrs {
        Hrs(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Hrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrs")
            .field("hrs0", &self.hrs0())
            .field("hrs1", &self.hrs1())
            .field("hrs2", &self.hrs2())
            .field("hrs3", &self.hrs3())
            .field("hrs4", &self.hrs4())
            .field("hrs5", &self.hrs5())
            .field("hrs6", &self.hrs6())
            .field("hrs7", &self.hrs7())
            .field("hrs8", &self.hrs8())
            .field("hrs9", &self.hrs9())
            .field("hrs10", &self.hrs10())
            .field("hrs11", &self.hrs11())
            .field("hrs12", &self.hrs12())
            .field("hrs13", &self.hrs13())
            .field("hrs14", &self.hrs14())
            .field("hrs15", &self.hrs15())
            .field("hrs16", &self.hrs16())
            .field("hrs17", &self.hrs17())
            .field("hrs18", &self.hrs18())
            .field("hrs19", &self.hrs19())
            .field("hrs20", &self.hrs20())
            .field("hrs21", &self.hrs21())
            .field("hrs22", &self.hrs22())
            .field("hrs23", &self.hrs23())
            .field("hrs24", &self.hrs24())
            .field("hrs25", &self.hrs25())
            .field("hrs26", &self.hrs26())
            .field("hrs27", &self.hrs27())
            .field("hrs28", &self.hrs28())
            .field("hrs29", &self.hrs29())
            .field("hrs30", &self.hrs30())
            .field("hrs31", &self.hrs31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hrs {{ hrs0: {=bool:?}, hrs1: {=bool:?}, hrs2: {=bool:?}, hrs3: {=bool:?}, hrs4: {=bool:?}, hrs5: {=bool:?}, hrs6: {=bool:?}, hrs7: {=bool:?}, hrs8: {=bool:?}, hrs9: {=bool:?}, hrs10: {=bool:?}, hrs11: {=bool:?}, hrs12: {=bool:?}, hrs13: {=bool:?}, hrs14: {=bool:?}, hrs15: {=bool:?}, hrs16: {=bool:?}, hrs17: {=bool:?}, hrs18: {=bool:?}, hrs19: {=bool:?}, hrs20: {=bool:?}, hrs21: {=bool:?}, hrs22: {=bool:?}, hrs23: {=bool:?}, hrs24: {=bool:?}, hrs25: {=bool:?}, hrs26: {=bool:?}, hrs27: {=bool:?}, hrs28: {=bool:?}, hrs29: {=bool:?}, hrs30: {=bool:?}, hrs31: {=bool:?} }}",
            self.hrs0(),
            self.hrs1(),
            self.hrs2(),
            self.hrs3(),
            self.hrs4(),
            self.hrs5(),
            self.hrs6(),
            self.hrs7(),
            self.hrs8(),
            self.hrs9(),
            self.hrs10(),
            self.hrs11(),
            self.hrs12(),
            self.hrs13(),
            self.hrs14(),
            self.hrs15(),
            self.hrs16(),
            self.hrs17(),
            self.hrs18(),
            self.hrs19(),
            self.hrs20(),
            self.hrs21(),
            self.hrs22(),
            self.hrs23(),
            self.hrs24(),
            self.hrs25(),
            self.hrs26(),
            self.hrs27(),
            self.hrs28(),
            self.hrs29(),
            self.hrs30(),
            self.hrs31()
        )
    }
}
#[doc = "Interrupt Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Interrupt Request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 0."]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 1."]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Request 2."]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 2."]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Request 3."]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 3."]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Request 4."]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 4."]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Request 5."]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 5."]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Request 6."]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> super::vals::Int6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Int6::from_bits(val as u8)
    }
    #[doc = "Interrupt Request 6."]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: super::vals::Int6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Request 7."]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 7."]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Request 8."]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 8."]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Request 9."]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 9."]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Request 10."]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 10."]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Request 11."]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 11."]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Request 12."]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 12."]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Request 13."]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 13."]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Request 14."]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 14."]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Request 15."]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 15."]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Request 16."]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 16."]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Request 17."]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 17."]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Request 18."]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 18."]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Request 19."]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 19."]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Request 20."]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 20."]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Request 21."]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 21."]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Request 22."]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 22."]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Request 23."]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 23."]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Request 24."]
    #[must_use]
    #[inline(always)]
    pub const fn int24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 24."]
    #[inline(always)]
    pub const fn set_int24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Request 25."]
    #[must_use]
    #[inline(always)]
    pub const fn int25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 25."]
    #[inline(always)]
    pub const fn set_int25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Request 26."]
    #[must_use]
    #[inline(always)]
    pub const fn int26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 26."]
    #[inline(always)]
    pub const fn set_int26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Request 27."]
    #[must_use]
    #[inline(always)]
    pub const fn int27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 27."]
    #[inline(always)]
    pub const fn set_int27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Request 28."]
    #[must_use]
    #[inline(always)]
    pub const fn int28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 28."]
    #[inline(always)]
    pub const fn set_int28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Request 29."]
    #[must_use]
    #[inline(always)]
    pub const fn int29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 29."]
    #[inline(always)]
    pub const fn set_int29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Request 30."]
    #[must_use]
    #[inline(always)]
    pub const fn int30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 30."]
    #[inline(always)]
    pub const fn set_int30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Request 31."]
    #[must_use]
    #[inline(always)]
    pub const fn int31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 31."]
    #[inline(always)]
    pub const fn set_int31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .field("int24", &self.int24())
            .field("int25", &self.int25())
            .field("int26", &self.int26())
            .field("int27", &self.int27())
            .field("int28", &self.int28())
            .field("int29", &self.int29())
            .field("int30", &self.int30())
            .field("int31", &self.int31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Int {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?}, int24: {=bool:?}, int25: {=bool:?}, int26: {=bool:?}, int27: {=bool:?}, int28: {=bool:?}, int29: {=bool:?}, int30: {=bool:?}, int31: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23(),
            self.int24(),
            self.int25(),
            self.int26(),
            self.int27(),
            self.int28(),
            self.int29(),
            self.int30(),
            self.int31()
        )
    }
}
#[doc = "Set Enable Error Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seei(pub u8);
impl Seei {
    #[doc = "Set Enable Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn seei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set Enable Error Interrupt."]
    #[inline(always)]
    pub const fn set_seei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All Enable Error Interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn saee(&self) -> super::vals::Saee {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Saee::from_bits(val as u8)
    }
    #[doc = "Set All Enable Error Interrupts."]
    #[inline(always)]
    pub const fn set_saee(&mut self, val: super::vals::Saee) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SeeiNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SeeiNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SeeiNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Seei {
    #[inline(always)]
    fn default() -> Seei {
        Seei(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Seei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seei")
            .field("seei", &self.seei())
            .field("saee", &self.saee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seei {{ seei: {=u8:?}, saee: {:?}, nop: {:?} }}",
            self.seei(),
            self.saee(),
            self.nop()
        )
    }
}
#[doc = "Set Enable Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serq(pub u8);
impl Serq {
    #[doc = "Set Enable Request."]
    #[must_use]
    #[inline(always)]
    pub const fn serq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set Enable Request."]
    #[inline(always)]
    pub const fn set_serq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All Enable Requests."]
    #[must_use]
    #[inline(always)]
    pub const fn saer(&self) -> super::vals::Saer {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Saer::from_bits(val as u8)
    }
    #[doc = "Set All Enable Requests."]
    #[inline(always)]
    pub const fn set_saer(&mut self, val: super::vals::Saer) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SerqNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SerqNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SerqNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Serq {
    #[inline(always)]
    fn default() -> Serq {
        Serq(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Serq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serq")
            .field("serq", &self.serq())
            .field("saer", &self.saer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Serq {{ serq: {=u8:?}, saer: {:?}, nop: {:?} }}",
            self.serq(),
            self.saer(),
            self.nop()
        )
    }
}
#[doc = "Set START Bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssrt(pub u8);
impl Ssrt {
    #[doc = "Set START field."]
    #[must_use]
    #[inline(always)]
    pub const fn ssrt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set START field."]
    #[inline(always)]
    pub const fn set_ssrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
    }
    #[doc = "Set All START fields (activates all channels)."]
    #[must_use]
    #[inline(always)]
    pub const fn sast(&self) -> super::vals::Sast {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sast::from_bits(val as u8)
    }
    #[doc = "Set All START fields (activates all channels)."]
    #[inline(always)]
    pub const fn set_sast(&mut self, val: super::vals::Sast) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SsrtNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SsrtNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable."]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SsrtNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ssrt {
    #[inline(always)]
    fn default() -> Ssrt {
        Ssrt(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Ssrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssrt")
            .field("ssrt", &self.ssrt())
            .field("sast", &self.sast())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrt {{ ssrt: {=u8:?}, sast: {:?}, nop: {:?} }}",
            self.ssrt(),
            self.sast(),
            self.nop()
        )
    }
}
#[doc = "TCD Transfer Attributes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
    #[doc = "Destination data transfer size."]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size."]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo."]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo."]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size."]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Ssize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Ssize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size."]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Ssize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo."]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> super::vals::Smod {
        let val = (self.0 >> 11usize) & 0x1f;
        super::vals::Smod::from_bits(val as u8)
    }
    #[doc = "Source Address Modulo."]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: super::vals::Smod) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val.to_bits() as u16) & 0x1f) << 11usize);
    }
}
impl Default for TcdAttr {
    #[inline(always)]
    fn default() -> TcdAttr {
        TcdAttr(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdAttr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdAttr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdAttr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
    #[doc = "Starting Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count."]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkno {
    #[inline(always)]
    fn default() -> TcdBiterElinkno {
        TcdBiterElinkno(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdBiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc = "Starting major iteration count."]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count."]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Link Channel Number."]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkyes {
    #[inline(always)]
    fn default() -> TcdBiterElinkyes {
        TcdBiterElinkyes(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdBiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
    #[doc = "Current Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count."]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkno {
    #[inline(always)]
    fn default() -> TcdCiterElinkno {
        TcdCiterElinkno(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdCiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
    #[doc = "Current Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count."]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number."]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkyes {
    #[inline(always)]
    fn default() -> TcdCiterElinkyes {
        TcdCiterElinkyes(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdCiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
    #[doc = "Channel Start."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request."]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request."]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing."]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Esg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Esg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing."]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Esg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete."]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete."]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number."]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "Bandwidth Control."]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Bwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control."]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Bwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for TcdCsr {
    #[inline(always)]
    fn default() -> TcdCsr {
        TcdCsr(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCsr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCsr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDaddr(pub u32);
impl TcdDaddr {
    #[doc = "Destination Address."]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address."]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdDaddr {
    #[inline(always)]
    fn default() -> TcdDaddr {
        TcdDaddr(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdDaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDaddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDaddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDlastsga(pub u32);
impl TcdDlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)."]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)."]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdDlastsga {
    #[inline(always)]
    fn default() -> TcdDlastsga {
        TcdDlastsga(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdDlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDoff(pub u16);
impl TcdDoff {
    #[doc = "Destination Address Signed Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset."]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TcdDoff {
    #[inline(always)]
    fn default() -> TcdDoff {
        TcdDoff(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdDoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDoff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDoff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMlno(pub u32);
impl TcdNbytesMlno {
    #[doc = "Minor Byte Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count."]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdNbytesMlno {
    #[inline(always)]
    fn default() -> TcdNbytesMlno {
        TcdNbytesMlno(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdNbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdNbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
    #[doc = "Minor Byte Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count."]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffno {
    #[inline(always)]
    fn default() -> TcdNbytesMloffno {
        TcdNbytesMloffno(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdNbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
    #[doc = "Minor Byte Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count."]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffyes {
    #[inline(always)]
    fn default() -> TcdNbytesMloffyes {
        TcdNbytesMloffyes(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdNbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSaddr(pub u32);
impl TcdSaddr {
    #[doc = "Source Address."]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address."]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdSaddr {
    #[inline(always)]
    fn default() -> TcdSaddr {
        TcdSaddr(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdSaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSaddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSaddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSlast(pub u32);
impl TcdSlast {
    #[doc = "Last Source Address Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment."]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdSlast {
    #[inline(always)]
    fn default() -> TcdSlast {
        TcdSlast(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdSlast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSlast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSlast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSlast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSoff(pub u16);
impl TcdSoff {
    #[doc = "Source address signed offset."]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset."]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TcdSoff {
    #[inline(always)]
    fn default() -> TcdSoff {
        TcdSoff(0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for TcdSoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSoff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSoff {{ soff: {=u16:?} }}", self.soff())
    }
}
