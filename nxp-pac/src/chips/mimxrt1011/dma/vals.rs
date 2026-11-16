#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "eDMA is idle."]
    Idle = 0x0,
    #[doc = "eDMA is executing a channel."]
    Active = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwc {
    #[doc = "No eDMA engine stalls."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    Stall4 = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    Stall8 = 0x03,
}
impl Bwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwc {
    #[inline(always)]
    fn from(val: u8) -> Bwc {
        Bwc::from_bits(val)
    }
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(val: Bwc) -> u8 {
        Bwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadn {
    #[doc = "Writes 0 to only the TCDn_CSR\\[DONE\\] field specified in the CDNE field."]
    ClearDone = 0x0,
    #[doc = "Writes 0 to all bits in TCDn_CSR\\[DONE\\]."]
    ClearAll = 0x01,
}
impl Cadn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadn {
    #[inline(always)]
    fn from(val: u8) -> Cadn {
        Cadn::from_bits(val)
    }
}
impl From<Cadn> for u8 {
    #[inline(always)]
    fn from(val: Cadn) -> u8 {
        Cadn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caee {
    #[doc = "Write 0 only to the EEI field specified in the CEEI field."]
    ClearEei = 0x0,
    #[doc = "Write 0 to all fields in EEI."]
    ClearAll = 0x01,
}
impl Caee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caee {
    #[inline(always)]
    fn from(val: u8) -> Caee {
        Caee::from_bits(val)
    }
}
impl From<Caee> for u8 {
    #[inline(always)]
    fn from(val: Caee) -> u8 {
        Caee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caei {
    #[doc = "Write 0 to only the ERR field specified in the CERR field."]
    ClearErr = 0x0,
    #[doc = "Write 0 to all fields in ERR."]
    ClearAll = 0x01,
}
impl Caei {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caei {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caei {
    #[inline(always)]
    fn from(val: u8) -> Caei {
        Caei::from_bits(val)
    }
}
impl From<Caei> for u8 {
    #[inline(always)]
    fn from(val: Caei) -> u8 {
        Caei::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caer {
    #[doc = "Write 0 to only the ERQ field specified in the CERQ field."]
    ClearErq = 0x0,
    #[doc = "Write 0 to all fields in ERQ."]
    ClearAll = 0x01,
}
impl Caer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caer {
    #[inline(always)]
    fn from(val: u8) -> Caer {
        Caer::from_bits(val)
    }
}
impl From<Caer> for u8 {
    #[inline(always)]
    fn from(val: Caer) -> u8 {
        Caer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cair {
    #[doc = "Clear only the INT field specified in the CINT field."]
    ClearInt = 0x0,
    #[doc = "Clear all bits in INT."]
    ClearAll = 0x01,
}
impl Cair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cair {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cair {
    #[inline(always)]
    fn from(val: u8) -> Cair {
        Cair::from_bits(val)
    }
}
impl From<Cair> for u8 {
    #[inline(always)]
    fn from(val: Cair) -> u8 {
        Cair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CdneNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NoOps = 0x01,
}
impl CdneNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CdneNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CdneNop {
    #[inline(always)]
    fn from(val: u8) -> CdneNop {
        CdneNop::from_bits(val)
    }
}
impl From<CdneNop> for u8 {
    #[inline(always)]
    fn from(val: CdneNop) -> u8 {
        CdneNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CeeiNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation, ignore the other fields in this register."]
    NoOps = 0x01,
}
impl CeeiNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CeeiNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CeeiNop {
    #[inline(always)]
    fn from(val: u8) -> CeeiNop {
        CeeiNop::from_bits(val)
    }
}
impl From<CeeiNop> for u8 {
    #[inline(always)]
    fn from(val: CeeiNop) -> u8 {
        CeeiNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CerqNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation, ignore the other fields in this register."]
    NoOps = 0x01,
}
impl CerqNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CerqNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CerqNop {
    #[inline(always)]
    fn from(val: u8) -> CerqNop {
        CerqNop::from_bits(val)
    }
}
impl From<CerqNop> for u8 {
    #[inline(always)]
    fn from(val: CerqNop) -> u8 {
        CerqNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CerrNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NoOps = 0x01,
}
impl CerrNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CerrNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CerrNop {
    #[inline(always)]
    fn from(val: u8) -> CerrNop {
        CerrNop::from_bits(val)
    }
}
impl From<CerrNop> for u8 {
    #[inline(always)]
    fn from(val: CerrNop) -> u8 {
        CerrNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CintNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NoOps = 0x01,
}
impl CintNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CintNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CintNop {
    #[inline(always)]
    fn from(val: u8) -> CintNop {
        CintNop::from_bits(val)
    }
}
impl From<CintNop> for u8 {
    #[inline(always)]
    fn from(val: CintNop) -> u8 {
        CintNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clm {
    #[doc = "Continuous link mode is off."]
    ClmOff = 0x0,
    #[doc = "Continuous link mode is on."]
    ClmOn = 0x01,
}
impl Clm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clm {
    #[inline(always)]
    fn from(val: u8) -> Clm {
        Clm::from_bits(val)
    }
}
impl From<Clm> for u8 {
    #[inline(always)]
    fn from(val: Clm) -> u8 {
        Clm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrEcx {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "Cancel the remaining data transfer."]
    Cancel = 0x01,
}
impl CrEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrEcx {
    #[inline(always)]
    fn from(val: u8) -> CrEcx {
        CrEcx::from_bits(val)
    }
}
impl From<CrEcx> for u8 {
    #[inline(always)]
    fn from(val: CrEcx) -> u8 {
        CrEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cx {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "Cancel the remaining data transfer."]
    Cancel = 0x01,
}
impl Cx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cx {
    #[inline(always)]
    fn from(val: u8) -> Cx {
        Cx::from_bits(val)
    }
}
impl From<Cx> for u8 {
    #[inline(always)]
    fn from(val: Cx) -> u8 {
        Cx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DchpriDpa {
    #[doc = "Channel n can suspend a lower priority channel."]
    Enabled = 0x0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
    Disabled = 0x01,
}
impl DchpriDpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DchpriDpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DchpriDpa {
    #[inline(always)]
    fn from(val: u8) -> DchpriDpa {
        DchpriDpa::from_bits(val)
    }
}
impl From<DchpriDpa> for u8 {
    #[inline(always)]
    fn from(val: DchpriDpa) -> u8 {
        DchpriDpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EsEcx {
    #[doc = "No canceled transfers."]
    NoCancels = 0x0,
    #[doc = "The most-recently recorded entry was a canceled transfer initiated by the error cancel transfer field."]
    Canceled = 0x01,
}
impl EsEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EsEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EsEcx {
    #[inline(always)]
    fn from(val: u8) -> EsEcx {
        EsEcx::from_bits(val)
    }
}
impl From<EsEcx> for u8 {
    #[inline(always)]
    fn from(val: EsEcx) -> u8 {
        EsEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esg {
    #[doc = "The current channel's TCD is normal format."]
    Normal = 0x0,
    #[doc = "The current channel's TCD specifies a scatter gather format."]
    Scatter = 0x01,
}
impl Esg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esg {
    #[inline(always)]
    fn from(val: u8) -> Esg {
        Esg::from_bits(val)
    }
}
impl From<Esg> for u8 {
    #[inline(always)]
    fn from(val: Esg) -> u8 {
        Esg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "eDMA operations halted."]
    HaltDma = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hoe {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "Error causes HALT field to be automatically set to 1."]
    HaltOnError = 0x01,
}
impl Hoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hoe {
    #[inline(always)]
    fn from(val: u8) -> Hoe {
        Hoe::from_bits(val)
    }
}
impl From<Hoe> for u8 {
    #[inline(always)]
    fn from(val: Hoe) -> u8 {
        Hoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saee {
    #[doc = "Write 1 only to the EEI field specified in the SEEI field."]
    SetEei = 0x0,
    #[doc = "Writes 1 to all fields in EEI."]
    SetAll = 0x01,
}
impl Saee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saee {
    #[inline(always)]
    fn from(val: u8) -> Saee {
        Saee::from_bits(val)
    }
}
impl From<Saee> for u8 {
    #[inline(always)]
    fn from(val: Saee) -> u8 {
        Saee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saer {
    #[doc = "Write 1 to only the ERQ field specified in the SERQ field."]
    SetErq = 0x0,
    #[doc = "Write 1 to all fields in ERQ."]
    SetAll = 0x01,
}
impl Saer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saer {
    #[inline(always)]
    fn from(val: u8) -> Saer {
        Saer::from_bits(val)
    }
}
impl From<Saer> for u8 {
    #[inline(always)]
    fn from(val: Saer) -> u8 {
        Saer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sast {
    #[doc = "Write 1 to only the TCDn_CSR\\[START\\] field specified in the SSRT field."]
    SetStart = 0x0,
    #[doc = "Write 1 to all bits in TCDn_CSR\\[START\\]."]
    SetAll = 0x01,
}
impl Sast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sast {
    #[inline(always)]
    fn from(val: u8) -> Sast {
        Sast::from_bits(val)
    }
}
impl From<Sast> for u8 {
    #[inline(always)]
    fn from(val: Sast) -> u8 {
        Sast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeeiNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation, ignore the other fields in this register."]
    NoOps = 0x01,
}
impl SeeiNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeeiNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeeiNop {
    #[inline(always)]
    fn from(val: u8) -> SeeiNop {
        SeeiNop::from_bits(val)
    }
}
impl From<SeeiNop> for u8 {
    #[inline(always)]
    fn from(val: SeeiNop) -> u8 {
        SeeiNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SerqNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation, ignore the other fields in this register."]
    NoOps = 0x01,
}
impl SerqNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SerqNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SerqNop {
    #[inline(always)]
    fn from(val: u8) -> SerqNop {
        SerqNop::from_bits(val)
    }
}
impl From<SerqNop> for u8 {
    #[inline(always)]
    fn from(val: SerqNop) -> u8 {
        SerqNop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod {
    #[doc = "Source address modulo feature is disabled."]
    Disabled = 0x0,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled1 = 0x01,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled2 = 0x02,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled3 = 0x03,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled4 = 0x04,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled5 = 0x05,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled6 = 0x06,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled7 = 0x07,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled8 = 0x08,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled9 = 0x09,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled26 = 0x1a,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled27 = 0x1b,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled28 = 0x1c,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled29 = 0x1d,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled30 = 0x1e,
    #[doc = "Value defines address range used to set up circular data queue."]
    Enabled31 = 0x1f,
}
impl Smod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssize {
    #[doc = "8-bit."]
    Eight = 0x0,
    #[doc = "16-bit."]
    SixteenBit = 0x01,
    #[doc = "32-bit."]
    ThirtytwoBit = 0x02,
    #[doc = "64-bit."]
    Sixtyfour = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "32-byte burst (4 beats of 64 bits)."]
    ThirtytwoByte = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ssize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssize {
    #[inline(always)]
    fn from(val: u8) -> Ssize {
        Ssize::from_bits(val)
    }
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(val: Ssize) -> u8 {
        Ssize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrtNop {
    #[doc = "Normal operation."]
    NormalOps = 0x0,
    #[doc = "No operation; all other fields in this register are ignored."]
    NoOps = 0x01,
}
impl SsrtNop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrtNop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrtNop {
    #[inline(always)]
    fn from(val: u8) -> SsrtNop {
        SsrtNop::from_bits(val)
    }
}
impl From<SsrtNop> for u8 {
    #[inline(always)]
    fn from(val: SsrtNop) -> u8 {
        SsrtNop::to_bits(val)
    }
}
