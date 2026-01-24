#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aesdecrypt {
    #[doc = "Encrypt"]
    ENCRYPT = 0x0,
    #[doc = "Decrypt"]
    DECRYPT = 0x01,
}
impl Aesdecrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesdecrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesdecrypt {
    #[inline(always)]
    fn from(val: u8) -> Aesdecrypt {
        Aesdecrypt::from_bits(val)
    }
}
impl From<Aesdecrypt> for u8 {
    #[inline(always)]
    fn from(val: Aesdecrypt) -> u8 {
        Aesdecrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aesfault {
    #[doc = "No AES fault has occurred."]
    NO_FAULT = 0x0,
    #[doc = "An AES fault has occurred."]
    FAULT = 0x01,
}
impl Aesfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesfault {
    #[inline(always)]
    fn from(val: u8) -> Aesfault {
        Aesfault::from_bits(val)
    }
}
impl From<Aesfault> for u8 {
    #[inline(always)]
    fn from(val: Aesfault) -> u8 {
        Aesfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aesflush {
    #[doc = "Do not flush the AES engine registers."]
    NOFLUSH = 0x0,
    #[doc = "Flush the AES engine registers."]
    FLUSH = 0x01,
}
impl Aesflush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesflush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesflush {
    #[inline(always)]
    fn from(val: u8) -> Aesflush {
        Aesflush::from_bits(val)
    }
}
impl From<Aesflush> for u8 {
    #[inline(always)]
    fn from(val: Aesflush) -> u8 {
        Aesflush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aeskeysz {
    #[doc = "128 bit key"]
    BITS_128 = 0x0,
    #[doc = "192 bit key"]
    BITS_192 = 0x01,
    #[doc = "256 bit key"]
    BITS_256 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Aeskeysz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aeskeysz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aeskeysz {
    #[inline(always)]
    fn from(val: u8) -> Aeskeysz {
        Aeskeysz::from_bits(val)
    }
}
impl From<Aeskeysz> for u8 {
    #[inline(always)]
    fn from(val: Aeskeysz) -> u8 {
        Aeskeysz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aesmode {
    #[doc = "ECB - used as is"]
    ECB = 0x0,
    #[doc = "CBC mode (see details on IV/nonce)"]
    CBC = 0x01,
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    CTR = 0x02,
    _RESERVED_3 = 0x03,
}
impl Aesmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aesmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aesmode {
    #[inline(always)]
    fn from(val: u8) -> Aesmode {
        Aesmode::from_bits(val)
    }
}
impl From<Aesmode> for u8 {
    #[inline(always)]
    fn from(val: Aesmode) -> u8 {
        Aesmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aessecret {
    #[doc = "User key provided in normal way"]
    NORMAL_WAY = 0x0,
    #[doc = "Secret key provided in hidden way by HW"]
    HIDDEN_WAY = 0x01,
}
impl Aessecret {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aessecret {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aessecret {
    #[inline(always)]
    fn from(val: u8) -> Aessecret {
        Aessecret::from_bits(val)
    }
}
impl From<Aessecret> for u8 {
    #[inline(always)]
    fn from(val: Aessecret) -> u8 {
        Aessecret::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaI {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED = 0x0,
    #[doc = "DMA will push in the data."]
    PUSH = 0x01,
}
impl DmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaI {
    #[inline(always)]
    fn from(val: u8) -> DmaI {
        DmaI::from_bits(val)
    }
}
impl From<DmaI> for u8 {
    #[inline(always)]
    fn from(val: DmaI) -> u8 {
        DmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaO {
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    NOTUSED = 0x0,
    _RESERVED_1 = 0x01,
}
impl DmaO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaO {
    #[inline(always)]
    fn from(val: u8) -> DmaO {
        DmaO::from_bits(val)
    }
}
impl From<DmaO> for u8 {
    #[inline(always)]
    fn from(val: DmaO) -> u8 {
        DmaO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntensetDigest {
    #[doc = "Will not interrupt when Digest is ready"]
    NO_INTERRUPT = 0x0,
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    INTERRUPT = 0x01,
}
impl IntensetDigest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetDigest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetDigest {
    #[inline(always)]
    fn from(val: u8) -> IntensetDigest {
        IntensetDigest::from_bits(val)
    }
}
impl From<IntensetDigest> for u8 {
    #[inline(always)]
    fn from(val: IntensetDigest) -> u8 {
        IntensetDigest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntensetError {
    #[doc = "Will not interrupt on Error."]
    NO_INTERRUPT = 0x0,
    #[doc = "Will interrupt on Error (until cleared)."]
    INTERRUPT = 0x01,
}
impl IntensetError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetError {
    #[inline(always)]
    fn from(val: u8) -> IntensetError {
        IntensetError::from_bits(val)
    }
}
impl From<IntensetError> for u8 {
    #[inline(always)]
    fn from(val: IntensetError) -> u8 {
        IntensetError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntensetFault {
    #[doc = "No interrupt on an AES or PRNG fault"]
    NO_INTERRUPT = 0x0,
    #[doc = "Interrupt on an AES or PRNG fault"]
    INTERRUPT = 0x01,
}
impl IntensetFault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetFault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetFault {
    #[inline(always)]
    fn from(val: u8) -> IntensetFault {
        IntensetFault::from_bits(val)
    }
}
impl From<IntensetFault> for u8 {
    #[inline(always)]
    fn from(val: IntensetFault) -> u8 {
        IntensetFault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntensetWaiting {
    #[doc = "Will not interrupt when waiting."]
    NO_INTERRUPT = 0x0,
    #[doc = "Will interrupt when waiting"]
    INTERRUPT = 0x01,
}
impl IntensetWaiting {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetWaiting {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetWaiting {
    #[inline(always)]
    fn from(val: u8) -> IntensetWaiting {
        IntensetWaiting::from_bits(val)
    }
}
impl From<IntensetWaiting> for u8 {
    #[inline(always)]
    fn from(val: IntensetWaiting) -> u8 {
        IntensetWaiting::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED = 0x0,
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "SHA1 is enabled"]
    SHA1 = 0x01,
    #[doc = "SHA2-256 is enabled"]
    SHA2_256 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    AES = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Neediv {
    #[doc = "No IV/Nonce is needed, either because written already or because not needed."]
    NOT_NEED = 0x0,
    #[doc = "IV/Nonce is needed and INDATA/ALIAS will be accepted as IV/Nonce. Will also set WAITING."]
    NEED = 0x01,
}
impl Neediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Neediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Neediv {
    #[inline(always)]
    fn from(val: u8) -> Neediv {
        Neediv::from_bits(val)
    }
}
impl From<Neediv> for u8 {
    #[inline(always)]
    fn from(val: Neediv) -> u8 {
        Neediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Needkey {
    #[doc = "No Key is needed and writes will not be treated as Key"]
    NOT_NEED = 0x0,
    #[doc = "Key is needed and INDATA/ALIAS will be accepted as Key. Will also set WAITING."]
    NEED = 0x01,
}
impl Needkey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Needkey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Needkey {
    #[inline(always)]
    fn from(val: u8) -> Needkey {
        Needkey::from_bits(val)
    }
}
impl From<Needkey> for u8 {
    #[inline(always)]
    fn from(val: Needkey) -> u8 {
        Needkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NewHash {
    _RESERVED_0 = 0x0,
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    START = 0x01,
}
impl NewHash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NewHash {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NewHash {
    #[inline(always)]
    fn from(val: u8) -> NewHash {
        NewHash::from_bits(val)
    }
}
impl From<NewHash> for u8 {
    #[inline(always)]
    fn from(val: NewHash) -> u8 {
        NewHash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prngfault {
    #[doc = "No PRNG fault has occurred."]
    NO_FAULT = 0x0,
    #[doc = "A PRNG fault has occurred."]
    FAULT = 0x01,
}
impl Prngfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prngfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prngfault {
    #[inline(always)]
    fn from(val: u8) -> Prngfault {
        Prngfault::from_bits(val)
    }
}
impl From<Prngfault> for u8 {
    #[inline(always)]
    fn from(val: Prngfault) -> u8 {
        Prngfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reload {
    _RESERVED_0 = 0x0,
    #[doc = "Allow RELOAD registers to be used."]
    RELOAD = 0x01,
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
pub enum Seclock {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK = 0x0,
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Seclock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seclock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seclock {
    #[inline(always)]
    fn from(val: u8) -> Seclock {
        Seclock::from_bits(val)
    }
}
impl From<Seclock> for u8 {
    #[inline(always)]
    fn from(val: Seclock) -> u8 {
        Seclock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusDigest {
    #[doc = "No Digest is ready"]
    NOT_READY = 0x0,
    #[doc = "Digest is ready. Application may read it or may write more data"]
    READY = 0x01,
}
impl StatusDigest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusDigest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusDigest {
    #[inline(always)]
    fn from(val: u8) -> StatusDigest {
        StatusDigest::from_bits(val)
    }
}
impl From<StatusDigest> for u8 {
    #[inline(always)]
    fn from(val: StatusDigest) -> u8 {
        StatusDigest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusError {
    #[doc = "No error."]
    NO_ERROR = 0x0,
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    ERROR = 0x01,
}
impl StatusError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusError {
    #[inline(always)]
    fn from(val: u8) -> StatusError {
        StatusError::from_bits(val)
    }
}
impl From<StatusError> for u8 {
    #[inline(always)]
    fn from(val: StatusError) -> u8 {
        StatusError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusFault {
    #[doc = "No AES or PRNG fault has occurred."]
    NO_FAULT = 0x0,
    #[doc = "An AES or PRNG fault has occurred."]
    FAULT = 0x01,
}
impl StatusFault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusFault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusFault {
    #[inline(always)]
    fn from(val: u8) -> StatusFault {
        StatusFault::from_bits(val)
    }
}
impl From<StatusFault> for u8 {
    #[inline(always)]
    fn from(val: StatusFault) -> u8 {
        StatusFault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusWaiting {
    #[doc = "Not waiting for data - may be disabled or may be busy. Note that for cryptographic uses, this is not set if IsLast is set nor will it set until at least 1 word is read of the output."]
    NOT_WAITING = 0x0,
    #[doc = "Waiting for data to be written in (16 words)"]
    WAITING = 0x01,
}
impl StatusWaiting {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusWaiting {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusWaiting {
    #[inline(always)]
    fn from(val: u8) -> StatusWaiting {
        StatusWaiting::from_bits(val)
    }
}
impl From<StatusWaiting> for u8 {
    #[inline(always)]
    fn from(val: StatusWaiting) -> u8 {
        StatusWaiting::to_bits(val)
    }
}
