#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusEdc {
    #[doc = "No Bus-EDC error/fault detected."]
    BUS_EDC_NOERR = 0x0,
    #[doc = "Bus-EDC error/fault detected."]
    BUS_EDC_ERR = 0x01,
}
impl BusEdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusEdc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusEdc {
    #[inline(always)]
    fn from(val: u8) -> BusEdc {
        BusEdc::from_bits(val)
    }
}
impl From<BusEdc> for u8 {
    #[inline(always)]
    fn from(val: BusEdc) -> u8 {
        BusEdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusEdcClr {
    #[doc = "No effect, ignored"]
    BUS_EDC_NOEFFECT = 0x0,
    #[doc = "Clears the CSER\\[BUS_EDC\\] bit."]
    BUS_EDC_CLEAR = 0x01,
}
impl BusEdcClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusEdcClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusEdcClr {
    #[inline(always)]
    fn from(val: u8) -> BusEdcClr {
        BusEdcClr::from_bits(val)
    }
}
impl From<BusEdcClr> for u8 {
    #[inline(always)]
    fn from(val: BusEdcClr) -> u8 {
        BusEdcClr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigOpt(u8);
impl ConfigOpt {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    pub const CONFIG_OPT_VAL: Self = Self(0x0);
}
impl ConfigOpt {
    pub const fn from_bits(val: u8) -> ConfigOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ConfigOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CONFIG_OPT_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ConfigOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CONFIG_OPT_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ConfigOpt {
    #[inline(always)]
    fn from(val: u8) -> ConfigOpt {
        ConfigOpt::from_bits(val)
    }
}
impl From<ConfigOpt> for u8 {
    #[inline(always)]
    fn from(val: ConfigOpt) -> u8 {
        ConfigOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EcoRev(u8);
impl EcoRev {
    #[doc = "TRNG_ECO_REV for TRNG."]
    pub const ECO_REV_VAL: Self = Self(0x0);
}
impl EcoRev {
    pub const fn from_bits(val: u8) -> EcoRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EcoRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ECO_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcoRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECO_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EcoRev {
    #[inline(always)]
    fn from(val: u8) -> EcoRev {
        EcoRev::from_bits(val)
    }
}
impl From<EcoRev> for u8 {
    #[inline(always)]
    fn from(val: EcoRev) -> u8 {
        EcoRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Era(u8);
impl Era {
    #[doc = "ERA of the TRNG."]
    pub const ERA_VAL: Self = Self(0x0c);
}
impl Era {
    pub const fn from_bits(val: u8) -> Era {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Era {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("ERA_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Era {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "ERA_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Era {
    #[inline(always)]
    fn from(val: u8) -> Era {
        Era::from_bits(val)
    }
}
impl From<Era> for u8 {
    #[inline(always)]
    fn from(val: Era) -> u8 {
        Era::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlEntVal {
    #[doc = "Clears the INT_STATUS\\[ENT_VAL\\] bit. Will automatically set after writing."]
    ENT_VAL_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[ENT_VAL\\] bit to be set, thereby enabling interrupt generation for the ENT_VAL condition."]
    ENT_VAL_ON = 0x01,
}
impl IntCtrlEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlEntVal {
        IntCtrlEntVal::from_bits(val)
    }
}
impl From<IntCtrlEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlEntVal) -> u8 {
        IntCtrlEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlFrqCtFail {
    #[doc = "Clears the INT_STATUS\\[FRQ_CT_FAIL\\] bit. Will automatically set after writing."]
    FRQ_CT_FAIL_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[FRQ_CT_FAIL\\] bit to be set, thereby enabling interrupt generation for the FRQ_CT_FAIL condition."]
    FRQ_CT_FAIL_ON = 0x01,
}
impl IntCtrlFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlFrqCtFail {
        IntCtrlFrqCtFail::from_bits(val)
    }
}
impl From<IntCtrlFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlFrqCtFail) -> u8 {
        IntCtrlFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlHwErr {
    #[doc = "Clears the INT_STATUS\\[HW_ERR\\] bit. Will automatically set after writing."]
    HW_ERR_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[HW_ERR\\] bit to be set, thereby enabling interrupt generation for the HW_ERR condition."]
    HW_ERR_ON = 0x01,
}
impl IntCtrlHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlHwErr {
        IntCtrlHwErr::from_bits(val)
    }
}
impl From<IntCtrlHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlHwErr) -> u8 {
        IntCtrlHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlIntgFlt {
    #[doc = "Clears the INT_STATUS\\[INTG_FLT\\] bit. Will automatically set after writing."]
    INTG_FLT_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[INTG_FLT\\] bit to be set, thereby enabling interrupt generation for the INTG_FLT condition."]
    INTG_FLT_ON = 0x01,
}
impl IntCtrlIntgFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlIntgFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlIntgFlt {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlIntgFlt {
        IntCtrlIntgFlt::from_bits(val)
    }
}
impl From<IntCtrlIntgFlt> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlIntgFlt) -> u8 {
        IntCtrlIntgFlt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskEntVal {
    #[doc = "ENT_VAL interrupt is disabled."]
    ENT_VAL_MASKED = 0x0,
    #[doc = "ENT_VAL interrupt is enabled."]
    ENT_VAL_ACTIVE = 0x01,
}
impl IntMaskEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntMaskEntVal {
        IntMaskEntVal::from_bits(val)
    }
}
impl From<IntMaskEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntMaskEntVal) -> u8 {
        IntMaskEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskFrqCtFail {
    #[doc = "FRQ_CT_FAIL interrupt is disabled."]
    FRQ_CT_FAIL_MASKED = 0x0,
    #[doc = "FRQ_CT_FAIL interrupt is enabled."]
    FRQ_CT_FAIL_ACTIVE = 0x01,
}
impl IntMaskFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntMaskFrqCtFail {
        IntMaskFrqCtFail::from_bits(val)
    }
}
impl From<IntMaskFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntMaskFrqCtFail) -> u8 {
        IntMaskFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskHwErr {
    #[doc = "HW_ERR interrupt is disabled."]
    HW_ERR_MASKED = 0x0,
    #[doc = "HW_ERR interrupt is enabled."]
    HW_ERR_ACTIVE = 0x01,
}
impl IntMaskHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntMaskHwErr {
        IntMaskHwErr::from_bits(val)
    }
}
impl From<IntMaskHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntMaskHwErr) -> u8 {
        IntMaskHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskIntgFlt {
    #[doc = "INTG_FLT interrupt is disabled."]
    INTG_FLT_MASKED = 0x0,
    #[doc = "INTG_FLT interrupt is enabled."]
    INTG_FLT_ACTIVE = 0x01,
}
impl IntMaskIntgFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskIntgFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskIntgFlt {
    #[inline(always)]
    fn from(val: u8) -> IntMaskIntgFlt {
        IntMaskIntgFlt::from_bits(val)
    }
}
impl From<IntMaskIntgFlt> for u8 {
    #[inline(always)]
    fn from(val: IntMaskIntgFlt) -> u8 {
        IntMaskIntgFlt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusEntVal {
    #[doc = "Busy generating entropy. Any value read from the Entropy registers is invalid."]
    ENT_VAL_INVALID = 0x0,
    #[doc = "Values read from the Entropy registers are valid."]
    ENT_VAL_VALID = 0x01,
}
impl IntStatusEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntStatusEntVal {
        IntStatusEntVal::from_bits(val)
    }
}
impl From<IntStatusEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntStatusEntVal) -> u8 {
        IntStatusEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusFrqCtFail {
    #[doc = "No hardware nor self test frequency errors."]
    FRQ_CT_FAIL_NO_ERR = 0x0,
    #[doc = "The frequency counter has detected a failure."]
    FRQ_CT_FAIL_ERR = 0x01,
}
impl IntStatusFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntStatusFrqCtFail {
        IntStatusFrqCtFail::from_bits(val)
    }
}
impl From<IntStatusFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntStatusFrqCtFail) -> u8 {
        IntStatusFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusHwErr {
    #[doc = "No error."]
    HW_ERR_NO = 0x0,
    #[doc = "Error detected."]
    HW_ERR_YES = 0x01,
}
impl IntStatusHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntStatusHwErr {
        IntStatusHwErr::from_bits(val)
    }
}
impl From<IntStatusHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntStatusHwErr) -> u8 {
        IntStatusHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusIntgFlt {
    #[doc = "No internal fault has been detected."]
    INTG_FLT_NO_ERR = 0x0,
    #[doc = "TRNG has detected internal fault."]
    INTG_FLT_ERR = 0x01,
}
impl IntStatusIntgFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusIntgFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusIntgFlt {
    #[inline(always)]
    fn from(val: u8) -> IntStatusIntgFlt {
        IntStatusIntgFlt::from_bits(val)
    }
}
impl From<IntStatusIntgFlt> for u8 {
    #[inline(always)]
    fn from(val: IntStatusIntgFlt) -> u8 {
        IntStatusIntgFlt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntgOpt(u8);
impl IntgOpt {
    #[doc = "INTG_OPT for TRNG."]
    pub const INTG_OPT_VAL: Self = Self(0x0a);
}
impl IntgOpt {
    pub const fn from_bits(val: u8) -> IntgOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for IntgOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0a => f.write_str("INTG_OPT_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntgOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0a => defmt::write!(f, "INTG_OPT_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for IntgOpt {
    #[inline(always)]
    fn from(val: u8) -> IntgOpt {
        IntgOpt::from_bits(val)
    }
}
impl From<IntgOpt> for u8 {
    #[inline(always)]
    fn from(val: IntgOpt) -> u8 {
        IntgOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IpId(u16);
impl IpId {
    #[doc = "ID for TRNG."]
    pub const IP_ID_VAL: Self = Self(0x30);
}
impl IpId {
    pub const fn from_bits(val: u16) -> IpId {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for IpId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x30 => f.write_str("IP_ID_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpId {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x30 => defmt::write!(f, "IP_ID_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for IpId {
    #[inline(always)]
    fn from(val: u16) -> IpId {
        IpId::from_bits(val)
    }
}
impl From<IpId> for u16 {
    #[inline(always)]
    fn from(val: IpId) -> u16 {
        IpId::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LocalEdc {
    #[doc = "No Local-EDC error/fault detected."]
    LOCAL_EDC_NOERR = 0x0,
    #[doc = "Local-EDC error/fault detected."]
    LOCAL_EDC_ERR = 0x01,
}
impl LocalEdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LocalEdc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LocalEdc {
    #[inline(always)]
    fn from(val: u8) -> LocalEdc {
        LocalEdc::from_bits(val)
    }
}
impl From<LocalEdc> for u8 {
    #[inline(always)]
    fn from(val: LocalEdc) -> u8 {
        LocalEdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LocalEdcClr {
    #[doc = "No effect, ignored"]
    LOCAL_EDC_NOEFFECT = 0x0,
    #[doc = "Clears the CSER\\[LOCAL_EDC\\] bit."]
    LOCAL_EDC_CLEAR = 0x01,
}
impl LocalEdcClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LocalEdcClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LocalEdcClr {
    #[inline(always)]
    fn from(val: u8) -> LocalEdcClr {
        LocalEdcClr::from_bits(val)
    }
}
impl From<LocalEdcClr> for u8 {
    #[inline(always)]
    fn from(val: LocalEdcClr) -> u8 {
        LocalEdcClr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MajRev(u8);
impl MajRev {
    #[doc = "Major revision number for TRNG."]
    pub const MAJ_REV_VAL: Self = Self(0x14);
}
impl MajRev {
    pub const fn from_bits(val: u8) -> MajRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MajRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x14 => f.write_str("MAJ_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MajRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x14 => defmt::write!(f, "MAJ_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MajRev {
    #[inline(always)]
    fn from(val: u8) -> MajRev {
        MajRev::from_bits(val)
    }
}
impl From<MajRev> for u8 {
    #[inline(always)]
    fn from(val: MajRev) -> u8 {
        MajRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinRev(u8);
impl MinRev {
    #[doc = "Minor revision number for TRNG."]
    pub const MIN_REV_VAL: Self = Self(0x0c);
}
impl MinRev {
    pub const fn from_bits(val: u8) -> MinRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MinRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("MIN_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MinRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "MIN_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MinRev {
    #[inline(always)]
    fn from(val: u8) -> MinRev {
        MinRev::from_bits(val)
    }
}
impl From<MinRev> for u8 {
    #[inline(always)]
    fn from(val: MinRev) -> u8 {
        MinRev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoPrgm {
    #[doc = "TRNG configuration registers can be modified."]
    NO_PRGM_OFF = 0x0,
    #[doc = "TRNG configuration registers cannot be modified."]
    NO_PRGM_ON = 0x01,
}
impl NoPrgm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoPrgm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoPrgm {
    #[inline(always)]
    fn from(val: u8) -> NoPrgm {
        NoPrgm::from_bits(val)
    }
}
impl From<NoPrgm> for u8 {
    #[inline(always)]
    fn from(val: NoPrgm) -> u8 {
        NoPrgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc2Div {
    #[doc = "Use ring oscillator 2 with no divide"]
    OSC2_DIV_DIV1 = 0x0,
    #[doc = "Use ring oscillator 2 divided-by-2"]
    OSC2_DIV_DIV2 = 0x01,
    #[doc = "Use ring oscillator 2 divided-by-4"]
    OSC2_DIV_DIV4 = 0x02,
    #[doc = "Use ring oscillator 2 divided-by-8"]
    OSC2_DIV_DIV8 = 0x03,
}
impl Osc2Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc2Div {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc2Div {
    #[inline(always)]
    fn from(val: u8) -> Osc2Div {
        Osc2Div::from_bits(val)
    }
}
impl From<Osc2Div> for u8 {
    #[inline(always)]
    fn from(val: Osc2Div) -> u8 {
        Osc2Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc2OutEn {
    #[doc = "Ring oscillator 2 output is gated to an output pad."]
    OSC2_OUT_EN_0 = 0x0,
    #[doc = "Allows external viewing of divided-by-2 ring oscillator 2 if MCTL\\[PRGM\\] = 1 mode is also selected, else ring oscillator 2 output is gated to an output pad."]
    OSC2_OUT_EN_1 = 0x01,
}
impl Osc2OutEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc2OutEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc2OutEn {
    #[inline(always)]
    fn from(val: u8) -> Osc2OutEn {
        Osc2OutEn::from_bits(val)
    }
}
impl From<Osc2OutEn> for u8 {
    #[inline(always)]
    fn from(val: Osc2OutEn) -> u8 {
        Osc2OutEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscDiv {
    #[doc = "use ring oscillator with no divide"]
    OSC_DIV_DIV1 = 0x0,
    #[doc = "use ring oscillator divided-by-2"]
    OSC_DIV_DIV2 = 0x01,
    #[doc = "use ring oscillator divided-by-4"]
    OSC_DIV_DIV4 = 0x02,
    #[doc = "use ring oscillator divided-by-8"]
    OSC_DIV_DIV8 = 0x03,
}
impl OscDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscDiv {
    #[inline(always)]
    fn from(val: u8) -> OscDiv {
        OscDiv::from_bits(val)
    }
}
impl From<OscDiv> for u8 {
    #[inline(always)]
    fn from(val: OscDiv) -> u8 {
        OscDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscFailsafeLmt {
    #[doc = "The limit N is 4096 (2^12) system clocks."]
    OSC_FAILSAFE_LMT_4K = 0x0,
    #[doc = "The limit N is 65536 (2^16) system clocks. (default)"]
    OSC_FAILSAFE_LMT_64K = 0x01,
    #[doc = "N is 2^20 system clocks."]
    OSC_FAILSAFE_LMT_1M = 0x02,
    #[doc = "N is 2^22 system clocks (full range of the counter being used)."]
    OSC_FAILSAFE_LMT_4M = 0x03,
}
impl OscFailsafeLmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscFailsafeLmt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscFailsafeLmt {
    #[inline(always)]
    fn from(val: u8) -> OscFailsafeLmt {
        OscFailsafeLmt::from_bits(val)
    }
}
impl From<OscFailsafeLmt> for u8 {
    #[inline(always)]
    fn from(val: OscFailsafeLmt) -> u8 {
        OscFailsafeLmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RedFsm {
    #[doc = "No redundant FSM error/fault"]
    RED_FSM_NOERR = 0x0,
    #[doc = "Redundant FSM error/fault detected."]
    RED_FSM_ERR = 0x01,
}
impl RedFsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RedFsm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RedFsm {
    #[inline(always)]
    fn from(val: u8) -> RedFsm {
        RedFsm::from_bits(val)
    }
}
impl From<RedFsm> for u8 {
    #[inline(always)]
    fn from(val: RedFsm) -> u8 {
        RedFsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RedFsmClr {
    #[doc = "No effect, ignored"]
    RED_FSM_NOEFFECT = 0x0,
    #[doc = "Clears the CSER\\[RED_FSM\\] bit."]
    RED_FSM_CLEAR = 0x01,
}
impl RedFsmClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RedFsmClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RedFsmClr {
    #[inline(always)]
    fn from(val: u8) -> RedFsmClr {
        RedFsmClr::from_bits(val)
    }
}
impl From<RedFsmClr> for u8 {
    #[inline(always)]
    fn from(val: RedFsmClr) -> u8 {
        RedFsmClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RedSigs {
    #[doc = "No redundant signal error/fault"]
    RED_SIGS_NOERR = 0x0,
    #[doc = "Redundant signal error/fault detected."]
    RED_SIGS_ERR = 0x01,
}
impl RedSigs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RedSigs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RedSigs {
    #[inline(always)]
    fn from(val: u8) -> RedSigs {
        RedSigs::from_bits(val)
    }
}
impl From<RedSigs> for u8 {
    #[inline(always)]
    fn from(val: RedSigs) -> u8 {
        RedSigs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RedSigsClr {
    #[doc = "No effect, ignored"]
    RED_SIGS_NOEFFECT = 0x0,
    #[doc = "Clears the CSER\\[RED_SIGS\\] bit."]
    RED_SIGS_CLEAR = 0x01,
}
impl RedSigsClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RedSigsClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RedSigsClr {
    #[inline(always)]
    fn from(val: u8) -> RedSigsClr {
        RedSigsClr::from_bits(val)
    }
}
impl From<RedSigsClr> for u8 {
    #[inline(always)]
    fn from(val: RedSigsClr) -> u8 {
        RedSigsClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngEntCtl {
    #[doc = "Single oscillator mode, using OSC1 (default)"]
    TRNG_ENT_CTL_SINGLE_OSC1 = 0x0,
    #[doc = "Dual oscillator mode"]
    TRNG_ENT_CTL_DUAL_OSCS = 0x01,
    #[doc = "Single oscillator mode, using OSC2"]
    TRNG_ENT_CTL_SINGLE_OSC2 = 0x02,
    #[doc = "Unused, (bit field cannot be written to this value)"]
    OSC2_DIV_DIV8 = 0x03,
}
impl TrngEntCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngEntCtl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngEntCtl {
    #[inline(always)]
    fn from(val: u8) -> TrngEntCtl {
        TrngEntCtl::from_bits(val)
    }
}
impl From<TrngEntCtl> for u8 {
    #[inline(always)]
    fn from(val: TrngEntCtl) -> u8 {
        TrngEntCtl::to_bits(val)
    }
}
