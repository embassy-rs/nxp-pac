#[doc = "Write address for issuing the ADD command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add(pub u32);
impl Add {
    #[doc = "Address of ADD command"]
    #[must_use]
    #[inline(always)]
    pub const fn ad(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD command"]
    #[inline(always)]
    pub const fn set_ad(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add {
    #[inline(always)]
    fn default() -> Add {
        Add(0)
    }
}
impl core::fmt::Debug for Add {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add").field("ad", &self.ad()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add {{ ad: {=u32:?} }}", self.ad())
    }
}
#[doc = "Write address for issuing the ADD1 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add1(pub u32);
impl Add1 {
    #[doc = "Address of ADD1 command."]
    #[must_use]
    #[inline(always)]
    pub const fn ad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD1 command."]
    #[inline(always)]
    pub const fn set_ad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add1 {
    #[inline(always)]
    fn default() -> Add1 {
        Add1(0)
    }
}
impl core::fmt::Debug for Add1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add1").field("ad1", &self.ad1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add1 {{ ad1: {=u32:?} }}", self.ad1())
    }
}
#[doc = "Write address for issuing the ADD16 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add16(pub u32);
impl Add16 {
    #[doc = "Address of ADD16"]
    #[must_use]
    #[inline(always)]
    pub const fn ad16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD16"]
    #[inline(always)]
    pub const fn set_ad16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add16 {
    #[inline(always)]
    fn default() -> Add16 {
        Add16(0)
    }
}
impl core::fmt::Debug for Add16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add16").field("ad16", &self.ad16()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add16 {{ ad16: {=u32:?} }}", self.ad16())
    }
}
#[doc = "Write address for issuing the ADD16 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Add256(pub u32);
impl Add256 {
    #[doc = "Address of ADD256 command"]
    #[must_use]
    #[inline(always)]
    pub const fn ad256(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of ADD256 command"]
    #[inline(always)]
    pub const fn set_ad256(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Add256 {
    #[inline(always)]
    fn default() -> Add256 {
        Add256(0)
    }
}
impl core::fmt::Debug for Add256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Add256")
            .field("ad256", &self.ad256())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Add256 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Add256 {{ ad256: {=u32:?} }}", self.ad256())
    }
}
#[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Lock control field"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Lock control field"]
    #[inline(always)]
    pub const fn set_lock_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "TIMEOUT control"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout_ctrl(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "TIMEOUT control"]
    #[inline(always)]
    pub const fn set_timeout_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "MISCOMPARE control field"]
    #[must_use]
    #[inline(always)]
    pub const fn miscompare_ctrl(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "MISCOMPARE control field"]
    #[inline(always)]
    pub const fn set_miscompare_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "SEQUENCE control field"]
    #[must_use]
    #[inline(always)]
    pub const fn sequence_ctrl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "SEQUENCE control field"]
    #[inline(always)]
    pub const fn set_sequence_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "CONTROL control field"]
    #[must_use]
    #[inline(always)]
    pub const fn control_ctrl(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "CONTROL control field"]
    #[inline(always)]
    pub const fn set_control_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "STATE control field"]
    #[must_use]
    #[inline(always)]
    pub const fn state_ctrl(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "STATE control field"]
    #[inline(always)]
    pub const fn set_state_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "ADDRESS control field"]
    #[must_use]
    #[inline(always)]
    pub const fn address_ctrl(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "ADDRESS control field"]
    #[inline(always)]
    pub const fn set_address_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "IRQ pause control field"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_pause(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "IRQ pause control field"]
    #[inline(always)]
    pub const fn set_irq_pause(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "DEBUG_HALT control field"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_halt_ctrl(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "DEBUG_HALT control field"]
    #[inline(always)]
    pub const fn set_debug_halt_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("lock_ctrl", &self.lock_ctrl())
            .field("timeout_ctrl", &self.timeout_ctrl())
            .field("miscompare_ctrl", &self.miscompare_ctrl())
            .field("sequence_ctrl", &self.sequence_ctrl())
            .field("control_ctrl", &self.control_ctrl())
            .field("state_ctrl", &self.state_ctrl())
            .field("address_ctrl", &self.address_ctrl())
            .field("irq_pause", &self.irq_pause())
            .field("debug_halt_ctrl", &self.debug_halt_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Control {{ lock_ctrl: {=u8:?}, timeout_ctrl: {=u8:?}, miscompare_ctrl: {=u8:?}, sequence_ctrl: {=u8:?}, control_ctrl: {=u8:?}, state_ctrl: {=u8:?}, address_ctrl: {=u8:?}, irq_pause: {=u8:?}, debug_halt_ctrl: {=u8:?} }}",
            self.lock_ctrl(),
            self.timeout_ctrl(),
            self.miscompare_ctrl(),
            self.sequence_ctrl(),
            self.control_ctrl(),
            self.state_ctrl(),
            self.address_ctrl(),
            self.irq_pause(),
            self.debug_halt_ctrl()
        )
    }
}
#[doc = "Hardware flags"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flags(pub u32);
impl Flags {
    #[doc = "Timeout flag"]
    #[must_use]
    #[inline(always)]
    pub const fn to_flag(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout flag"]
    #[inline(always)]
    pub const fn set_to_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Miscompare flag"]
    #[must_use]
    #[inline(always)]
    pub const fn miscom_flag(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Miscompare flag"]
    #[inline(always)]
    pub const fn set_miscom_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sequence flag"]
    #[must_use]
    #[inline(always)]
    pub const fn seq_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence flag"]
    #[inline(always)]
    pub const fn set_seq_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control (fault) flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control (fault) flag"]
    #[inline(always)]
    pub const fn set_cnt_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "State flag"]
    #[must_use]
    #[inline(always)]
    pub const fn state_flag(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "State flag"]
    #[inline(always)]
    pub const fn set_state_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Address flag"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_flag(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Address flag"]
    #[inline(always)]
    pub const fn set_addr_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Power-on reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn por_flag(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on reset flag"]
    #[inline(always)]
    pub const fn set_por_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Flags {
    #[inline(always)]
    fn default() -> Flags {
        Flags(0)
    }
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flags")
            .field("to_flag", &self.to_flag())
            .field("miscom_flag", &self.miscom_flag())
            .field("seq_flag", &self.seq_flag())
            .field("cnt_flag", &self.cnt_flag())
            .field("state_flag", &self.state_flag())
            .field("addr_flag", &self.addr_flag())
            .field("por_flag", &self.por_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flags {{ to_flag: {=bool:?}, miscom_flag: {=bool:?}, seq_flag: {=bool:?}, cnt_flag: {=bool:?}, state_flag: {=bool:?}, addr_flag: {=bool:?}, por_flag: {=bool:?} }}",
            self.to_flag(),
            self.miscom_flag(),
            self.seq_flag(),
            self.cnt_flag(),
            self.state_flag(),
            self.addr_flag(),
            self.por_flag()
        )
    }
}
#[doc = "The INSTRUCTION TIMER itself"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstructionTimer(pub u32);
impl InstructionTimer {
    #[doc = "INSTRUCTION TIMER 32-bit value"]
    #[must_use]
    #[inline(always)]
    pub const fn instim(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "INSTRUCTION TIMER 32-bit value"]
    #[inline(always)]
    pub const fn set_instim(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for InstructionTimer {
    #[inline(always)]
    fn default() -> InstructionTimer {
        InstructionTimer(0)
    }
}
impl core::fmt::Debug for InstructionTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InstructionTimer")
            .field("instim", &self.instim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InstructionTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "InstructionTimer {{ instim: {=u32:?} }}", self.instim())
    }
}
#[doc = "Persistent (Ad. Hoc., quasi-NV) data storage"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Persistent(pub u32);
impl Persistent {
    #[doc = "32 regs free for user SW to enjoy"]
    #[must_use]
    #[inline(always)]
    pub const fn persis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32 regs free for user SW to enjoy"]
    #[inline(always)]
    pub const fn set_persis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Persistent {
    #[inline(always)]
    fn default() -> Persistent {
        Persistent(0)
    }
}
impl core::fmt::Debug for Persistent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Persistent")
            .field("persis", &self.persis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Persistent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Persistent {{ persis: {=u32:?} }}", self.persis())
    }
}
#[doc = "Instruction timer reload"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reload(pub u32);
impl Reload {
    #[doc = "Inst. Timer reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn rload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Inst. Timer reload value"]
    #[inline(always)]
    pub const fn set_rload(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Reload {
    #[inline(always)]
    fn default() -> Reload {
        Reload(0)
    }
}
impl core::fmt::Debug for Reload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reload")
            .field("rload", &self.rload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reload {{ rload: {=u32:?} }}", self.rload())
    }
}
#[doc = "Write address for issuing the RESTART command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Restart(pub u32);
impl Restart {
    #[doc = "Write address for issuing the RESTART command."]
    #[must_use]
    #[inline(always)]
    pub const fn rstrt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write address for issuing the RESTART command."]
    #[inline(always)]
    pub const fn set_rstrt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Restart {
    #[inline(always)]
    fn default() -> Restart {
        Restart(0)
    }
}
impl core::fmt::Debug for Restart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Restart")
            .field("rstrt", &self.rstrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Restart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Restart {{ rstrt: {=u32:?} }}", self.rstrt())
    }
}
#[doc = "Also known as SEC_CNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecureCounter(pub u32);
impl SecureCounter {
    #[doc = "Secure Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn seccnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Secure Counter"]
    #[inline(always)]
    pub const fn set_seccnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecureCounter {
    #[inline(always)]
    fn default() -> SecureCounter {
        SecureCounter(0)
    }
}
impl core::fmt::Debug for SecureCounter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecureCounter")
            .field("seccnt", &self.seccnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecureCounter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SecureCounter {{ seccnt: {=u32:?} }}", self.seccnt())
    }
}
#[doc = "Write address for issuing the START command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "Address of start command access"]
    #[must_use]
    #[inline(always)]
    pub const fn strt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of start command access"]
    #[inline(always)]
    pub const fn set_strt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Start {
    #[inline(always)]
    fn default() -> Start {
        Start(0)
    }
}
impl core::fmt::Debug for Start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Start").field("strt", &self.strt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Start {{ strt: {=u32:?} }}", self.strt())
    }
}
#[doc = "Status register (1 of 2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Number of Timeout Faults"]
    #[must_use]
    #[inline(always)]
    pub const fn numtof(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Timeout Faults"]
    #[inline(always)]
    pub const fn set_numtof(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of Miscompare Faults"]
    #[must_use]
    #[inline(always)]
    pub const fn nummiscompf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Miscompare Faults"]
    #[inline(always)]
    pub const fn set_nummiscompf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of illegal sequence faults"]
    #[must_use]
    #[inline(always)]
    pub const fn numilseqf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of illegal sequence faults"]
    #[inline(always)]
    pub const fn set_numilseqf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Current State"]
    #[must_use]
    #[inline(always)]
    pub const fn curst(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Current State"]
    #[inline(always)]
    pub const fn set_curst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("numtof", &self.numtof())
            .field("nummiscompf", &self.nummiscompf())
            .field("numilseqf", &self.numilseqf())
            .field("curst", &self.curst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ numtof: {=u8:?}, nummiscompf: {=u8:?}, numilseqf: {=u8:?}, curst: {=u8:?} }}",
            self.numtof(),
            self.nummiscompf(),
            self.numilseqf(),
            self.curst()
        )
    }
}
#[doc = "STATUS register (2 of 2)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status2(pub u32);
impl Status2 {
    #[doc = "Number (of) control faults"]
    #[must_use]
    #[inline(always)]
    pub const fn numcntf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number (of) control faults"]
    #[inline(always)]
    pub const fn set_numcntf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number (of) state faults"]
    #[must_use]
    #[inline(always)]
    pub const fn numillstf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number (of) state faults"]
    #[inline(always)]
    pub const fn set_numillstf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of (illegal) address faults"]
    #[must_use]
    #[inline(always)]
    pub const fn numilla(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of (illegal) address faults"]
    #[inline(always)]
    pub const fn set_numilla(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Status2 {
    #[inline(always)]
    fn default() -> Status2 {
        Status2(0)
    }
}
impl core::fmt::Debug for Status2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status2")
            .field("numcntf", &self.numcntf())
            .field("numillstf", &self.numillstf())
            .field("numilla", &self.numilla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status2 {{ numcntf: {=u8:?}, numillstf: {=u8:?}, numilla: {=u8:?} }}",
            self.numcntf(),
            self.numillstf(),
            self.numilla()
        )
    }
}
#[doc = "Write address for issuing the STOP command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "Address of stop command access"]
    #[must_use]
    #[inline(always)]
    pub const fn stp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of stop command access"]
    #[inline(always)]
    pub const fn set_stp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Stop {
    #[inline(always)]
    fn default() -> Stop {
        Stop(0)
    }
}
impl core::fmt::Debug for Stop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stop").field("stp", &self.stp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stop {{ stp: {=u32:?} }}", self.stp())
    }
}
#[doc = "Write address for issuing the SUB command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub(pub u32);
impl Sub {
    #[doc = "Address of SUB command."]
    #[must_use]
    #[inline(always)]
    pub const fn s0b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of SUB command."]
    #[inline(always)]
    pub const fn set_s0b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub {
    #[inline(always)]
    fn default() -> Sub {
        Sub(0)
    }
}
impl core::fmt::Debug for Sub {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub").field("s0b", &self.s0b()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub {{ s0b: {=u32:?} }}", self.s0b())
    }
}
#[doc = "Write address for issuing the SUB1 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub1(pub u32);
impl Sub1 {
    #[doc = "Address of SUB1 command."]
    #[must_use]
    #[inline(always)]
    pub const fn s1b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of SUB1 command."]
    #[inline(always)]
    pub const fn set_s1b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub1 {
    #[inline(always)]
    fn default() -> Sub1 {
        Sub1(0)
    }
}
impl core::fmt::Debug for Sub1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub1").field("s1b", &self.s1b()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub1 {{ s1b: {=u32:?} }}", self.s1b())
    }
}
#[doc = "Write address for issuing the SUB16 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub16(pub u32);
impl Sub16 {
    #[doc = "Address of SUB16 command."]
    #[must_use]
    #[inline(always)]
    pub const fn sb16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of SUB16 command."]
    #[inline(always)]
    pub const fn set_sb16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub16 {
    #[inline(always)]
    fn default() -> Sub16 {
        Sub16(0)
    }
}
impl core::fmt::Debug for Sub16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub16").field("sb16", &self.sb16()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub16 {{ sb16: {=u32:?} }}", self.sb16())
    }
}
#[doc = "Write address for issuing the SUB256 command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sub256(pub u32);
impl Sub256 {
    #[doc = "Address of (you guessed it) SUB256 command."]
    #[must_use]
    #[inline(always)]
    pub const fn sb256(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address of (you guessed it) SUB256 command."]
    #[inline(always)]
    pub const fn set_sb256(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sub256 {
    #[inline(always)]
    fn default() -> Sub256 {
        Sub256(0)
    }
}
impl core::fmt::Debug for Sub256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sub256")
            .field("sb256", &self.sb256())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sub256 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sub256 {{ sb256: {=u32:?} }}", self.sb256())
    }
}
