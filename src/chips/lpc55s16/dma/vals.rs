#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Activeerrint {
    #[doc = "Not pending. No error interrupts are pending."]
    NOT_PENDING = 0x0,
    #[doc = "Pending. At least one error interrupt is pending."]
    PENDING = 0x01,
}
impl Activeerrint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Activeerrint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Activeerrint {
    #[inline(always)]
    fn from(val: u8) -> Activeerrint {
        Activeerrint::from_bits(val)
    }
}
impl From<Activeerrint> for u8 {
    #[inline(always)]
    fn from(val: Activeerrint) -> u8 {
        Activeerrint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Activeint {
    #[doc = "Not pending. No enabled interrupts are pending."]
    NOT_PENDING = 0x0,
    #[doc = "Pending. At least one enabled interrupt is pending."]
    PENDING = 0x01,
}
impl Activeint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Activeint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Activeint {
    #[inline(always)]
    fn from(val: u8) -> Activeint {
        Activeint::from_bits(val)
    }
}
impl From<Activeint> for u8 {
    #[inline(always)]
    fn from(val: Activeint) -> u8 {
        Activeint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfgvalid {
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    NOT_VALID = 0x0,
    #[doc = "Valid. The current channel descriptor is considered valid."]
    VALID = 0x01,
}
impl Cfgvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgvalid {
    #[inline(always)]
    fn from(val: u8) -> Cfgvalid {
        Cfgvalid::from_bits(val)
    }
}
impl From<Cfgvalid> for u8 {
    #[inline(always)]
    fn from(val: Cfgvalid) -> u8 {
        Cfgvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrtrig {
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED = 0x0,
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    CLEARED = 0x01,
}
impl Clrtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrtrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrtrig {
    #[inline(always)]
    fn from(val: u8) -> Clrtrig {
        Clrtrig::from_bits(val)
    }
}
impl From<Clrtrig> for u8 {
    #[inline(always)]
    fn from(val: Clrtrig) -> u8 {
        Clrtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dstburstwrap {
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0x0,
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED = 0x01,
}
impl Dstburstwrap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dstburstwrap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dstburstwrap {
    #[inline(always)]
    fn from(val: u8) -> Dstburstwrap {
        Dstburstwrap::from_bits(val)
    }
}
impl From<Dstburstwrap> for u8 {
    #[inline(always)]
    fn from(val: Dstburstwrap) -> u8 {
        Dstburstwrap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dstinc {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl Dstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dstinc {
    #[inline(always)]
    fn from(val: u8) -> Dstinc {
        Dstinc::from_bits(val)
    }
}
impl From<Dstinc> for u8 {
    #[inline(always)]
    fn from(val: Dstinc) -> u8 {
        Dstinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "Disabled. The DMA controller is disabled. This clears any triggers that were asserted at the point when disabled, but does not prevent re-triggering when the DMA controller is re-enabled."]
    DISABLED = 0x0,
    #[doc = "Enabled. The DMA controller is enabled."]
    ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hwtrigen {
    #[doc = "Disabled. Hardware triggering is not used."]
    DISABLED = 0x0,
    #[doc = "Enabled. Use hardware triggering."]
    ENABLED = 0x01,
}
impl Hwtrigen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hwtrigen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hwtrigen {
    #[inline(always)]
    fn from(val: u8) -> Hwtrigen {
        Hwtrigen::from_bits(val)
    }
}
impl From<Hwtrigen> for u8 {
    #[inline(always)]
    fn from(val: Hwtrigen) -> u8 {
        Hwtrigen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Periphreqen {
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    DISABLED = 0x0,
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    ENABLED = 0x01,
}
impl Periphreqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Periphreqen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Periphreqen {
    #[inline(always)]
    fn from(val: u8) -> Periphreqen {
        Periphreqen::from_bits(val)
    }
}
impl From<Periphreqen> for u8 {
    #[inline(always)]
    fn from(val: Periphreqen) -> u8 {
        Periphreqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reload {
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    ENABLED = 0x01,
}
impl Reload {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reload {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reload {
    #[inline(always)]
    fn from(val: u8) -> Reload {
        Reload::from_bits(val)
    }
}
impl From<Reload> for u8 {
    #[inline(always)]
    fn from(val: Reload) -> u8 {
        Reload::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setinta {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    SET = 0x01,
}
impl Setinta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setinta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setinta {
    #[inline(always)]
    fn from(val: u8) -> Setinta {
        Setinta::from_bits(val)
    }
}
impl From<Setinta> for u8 {
    #[inline(always)]
    fn from(val: Setinta) -> u8 {
        Setinta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setintb {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    SET = 0x01,
}
impl Setintb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setintb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setintb {
    #[inline(always)]
    fn from(val: u8) -> Setintb {
        Setintb::from_bits(val)
    }
}
impl From<Setintb> for u8 {
    #[inline(always)]
    fn from(val: Setintb) -> u8 {
        Setintb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srcburstwrap {
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0x0,
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    ENABLED = 0x01,
}
impl Srcburstwrap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srcburstwrap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srcburstwrap {
    #[inline(always)]
    fn from(val: u8) -> Srcburstwrap {
        Srcburstwrap::from_bits(val)
    }
}
impl From<Srcburstwrap> for u8 {
    #[inline(always)]
    fn from(val: Srcburstwrap) -> u8 {
        Srcburstwrap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srcinc {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl Srcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srcinc {
    #[inline(always)]
    fn from(val: u8) -> Srcinc {
        Srcinc::from_bits(val)
    }
}
impl From<Srcinc> for u8 {
    #[inline(always)]
    fn from(val: Srcinc) -> u8 {
        Srcinc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swtrig {
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    NOT_SET = 0x0,
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    SET = 0x01,
}
impl Swtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swtrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swtrig {
    #[inline(always)]
    fn from(val: u8) -> Swtrig {
        Swtrig::from_bits(val)
    }
}
impl From<Swtrig> for u8 {
    #[inline(always)]
    fn from(val: Swtrig) -> u8 {
        Swtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig {
    #[doc = "Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    NOT_TRIGGERED = 0x0,
    #[doc = "Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    TRIGGERED = 0x01,
}
impl Trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig {
    #[inline(always)]
    fn from(val: u8) -> Trig {
        Trig::from_bits(val)
    }
}
impl From<Trig> for u8 {
    #[inline(always)]
    fn from(val: Trig) -> u8 {
        Trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigburst {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE = 0x0,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST = 0x01,
}
impl Trigburst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigburst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigburst {
    #[inline(always)]
    fn from(val: u8) -> Trigburst {
        Trigburst::from_bits(val)
    }
}
impl From<Trigburst> for u8 {
    #[inline(always)]
    fn from(val: Trigburst) -> u8 {
        Trigburst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigpol {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING = 0x0,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING = 0x01,
}
impl Trigpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigpol {
    #[inline(always)]
    fn from(val: u8) -> Trigpol {
        Trigpol::from_bits(val)
    }
}
impl From<Trigpol> for u8 {
    #[inline(always)]
    fn from(val: Trigpol) -> u8 {
        Trigpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigtype {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE = 0x0,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL = 0x01,
}
impl Trigtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigtype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigtype {
    #[inline(always)]
    fn from(val: u8) -> Trigtype {
        Trigtype::from_bits(val)
    }
}
impl From<Trigtype> for u8 {
    #[inline(always)]
    fn from(val: Trigtype) -> u8 {
        Trigtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Validpending {
    #[doc = "No effect. No effect on DMA operation."]
    NO_EFFECT = 0x0,
    #[doc = "Valid pending."]
    VALID_PENDING = 0x01,
}
impl Validpending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Validpending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Validpending {
    #[inline(always)]
    fn from(val: u8) -> Validpending {
        Validpending::from_bits(val)
    }
}
impl From<Validpending> for u8 {
    #[inline(always)]
    fn from(val: Validpending) -> u8 {
        Validpending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Width {
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8 = 0x0,
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16 = 0x01,
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
