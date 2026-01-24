#[doc = "Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "PUF SRAM Controller activation"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller activation"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF SRAM Clock Gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ckgating(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub const fn set_ckgating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("enable", &self.enable())
            .field("ckgating", &self.ckgating())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ enable: {=bool:?}, ckgating: {=bool:?} }}",
            self.enable(),
            self.ckgating()
        )
    }
}
#[doc = "Interrupt Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntClrEnable(pub u32);
impl IntClrEnable {
    #[doc = "READY Interrupt Enable clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable clear"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable clear"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable clear"]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntClrEnable {
    #[inline(always)]
    fn default() -> IntClrEnable {
        IntClrEnable(0)
    }
}
impl core::fmt::Debug for IntClrEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntClrEnable")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntClrEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntClrEnable {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Status Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntClrStatus(pub u32);
impl IntClrStatus {
    #[doc = "READY Interrupt Status clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status clear"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status Clear"]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntClrStatus {
    #[inline(always)]
    fn default() -> IntClrStatus {
        IntClrStatus(0)
    }
}
impl core::fmt::Debug for IntClrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntClrStatus")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntClrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntClrStatus {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEnable(pub u32);
impl IntEnable {
    #[doc = "READY Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable"]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntEnable {
    #[inline(always)]
    fn default() -> IntEnable {
        IntEnable(0)
    }
}
impl core::fmt::Debug for IntEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEnable")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntEnable {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSetEnable(pub u32);
impl IntSetEnable {
    #[doc = "READY Interrupt Enable set"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable set"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable set"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable set"]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntSetEnable {
    #[inline(always)]
    fn default() -> IntSetEnable {
        IntSetEnable(0)
    }
}
impl core::fmt::Debug for IntSetEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSetEnable")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSetEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSetEnable {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Status set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSetStatus(pub u32);
impl IntSetStatus {
    #[doc = "READY Interrupt Status set"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status set"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status Set"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status Set"]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntSetStatus {
    #[inline(always)]
    fn default() -> IntSetStatus {
        IntSetStatus(0)
    }
}
impl core::fmt::Debug for IntSetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSetStatus")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSetStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSetStatus {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "READY Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status"]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "PUF SRAM Controller State"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller State"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("ready", &self.ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ ready: {=bool:?} }}", self.ready())
    }
}
