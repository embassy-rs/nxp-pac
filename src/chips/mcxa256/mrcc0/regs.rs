#[doc = "ADCx clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdcClkdiv(pub u32);
impl MrccAdcClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccAdcClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccAdcClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccAdcClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccAdcClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccAdcClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccAdcClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccAdcClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccAdcClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccAdcClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccAdcClkdiv {
    #[inline(always)]
    fn default() -> MrccAdcClkdiv {
        MrccAdcClkdiv(0)
    }
}
impl core::fmt::Debug for MrccAdcClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdcClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdcClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccAdcClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADCx clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdcClksel(pub u32);
impl MrccAdcClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccAdcClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccAdcClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccAdcClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccAdcClksel {
    #[inline(always)]
    fn default() -> MrccAdcClksel {
        MrccAdcClksel(0)
    }
}
impl core::fmt::Debug for MrccAdcClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdcClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdcClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccAdcClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CLKOUT clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClkdiv(pub u32);
impl MrccClkoutClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccClkoutClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccClkoutClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccClkoutClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccClkoutClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccClkoutClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccClkoutClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccClkoutClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccClkoutClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccClkoutClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccClkoutClkdiv {
    #[inline(always)]
    fn default() -> MrccClkoutClkdiv {
        MrccClkoutClkdiv(0)
    }
}
impl core::fmt::Debug for MrccClkoutClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccClkoutClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccClkoutClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccClkoutClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CLKOUT clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClksel(pub u32);
impl MrccClkoutClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccClkoutClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccClkoutClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccClkoutClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccClkoutClksel {
    #[inline(always)]
    fn default() -> MrccClkoutClksel {
        MrccClkoutClksel(0)
    }
}
impl core::fmt::Debug for MrccClkoutClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccClkoutClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccClkoutClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccClkoutClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP0_FUNC clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0FuncClkdiv(pub u32);
impl MrccCmp0FuncClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmpFuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmpFuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmpFuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmpFuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmpFuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmpFuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmpFuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmpFuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmpFuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp0FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp0FuncClkdiv {
        MrccCmp0FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp0FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp0FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0RrClkdiv(pub u32);
impl MrccCmp0RrClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmpRrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmpRrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmpRrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmpRrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmpRrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmpRrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmpRrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmpRrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmpRrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp0RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp0RrClkdiv {
        MrccCmp0RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp0RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp0RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0RrClksel(pub u32);
impl MrccCmp0RrClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCmpRrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCmpRrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCmpRrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp0RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp0RrClksel {
        MrccCmp0RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp0RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp0RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP1_FUNC clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1FuncClkdiv(pub u32);
impl MrccCmp1FuncClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmpFuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmpFuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmpFuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmpFuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmpFuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmpFuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmpFuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmpFuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmpFuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp1FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp1FuncClkdiv {
        MrccCmp1FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp1FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp1FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1_RR clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1RrClkdiv(pub u32);
impl MrccCmp1RrClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmpRrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmpRrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmpRrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmpRrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmpRrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmpRrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmpRrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmpRrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmpRrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp1RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp1RrClkdiv {
        MrccCmp1RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp1RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp1RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1_RR clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1RrClksel(pub u32);
impl MrccCmp1RrClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCmpRrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCmpRrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCmpRrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp1RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp1RrClksel {
        MrccCmp1RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp1RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp1RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP2_FUNC clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp2FuncClkdiv(pub u32);
impl MrccCmp2FuncClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmpFuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmpFuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmpFuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmpFuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmpFuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmpFuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmpFuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmpFuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmpFuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp2FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp2FuncClkdiv {
        MrccCmp2FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp2FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp2FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp2FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp2FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP2_RR clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp2RrClkdiv(pub u32);
impl MrccCmp2RrClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCmpRrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCmpRrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCmpRrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCmpRrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCmpRrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCmpRrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCmpRrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCmpRrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCmpRrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp2RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp2RrClkdiv {
        MrccCmp2RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp2RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp2RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp2RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp2RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP2_RR clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp2RrClksel(pub u32);
impl MrccCmp2RrClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCmpRrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCmpRrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCmpRrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp2RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp2RrClksel {
        MrccCmp2RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp2RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp2RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp2RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp2RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer0Clkdiv(pub u32);
impl MrccCtimer0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimerClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimerClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimerClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimerClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimerClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimerClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimerClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimerClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimerClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer0Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer0Clkdiv {
        MrccCtimer0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer0Clksel(pub u32);
impl MrccCtimer0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer0Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer0Clksel {
        MrccCtimer0Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER1 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer1Clkdiv(pub u32);
impl MrccCtimer1Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimerClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimerClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimerClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimerClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimerClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimerClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimerClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimerClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimerClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer1Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer1Clkdiv {
        MrccCtimer1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER1 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer1Clksel(pub u32);
impl MrccCtimer1Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer1Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer1Clksel {
        MrccCtimer1Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER2 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer2Clkdiv(pub u32);
impl MrccCtimer2Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimerClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimerClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimerClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimerClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimerClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimerClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimerClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimerClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimerClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer2Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer2Clkdiv {
        MrccCtimer2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER2 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer2Clksel(pub u32);
impl MrccCtimer2Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer2Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer2Clksel {
        MrccCtimer2Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER3 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer3Clkdiv(pub u32);
impl MrccCtimer3Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimerClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimerClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimerClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimerClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimerClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimerClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimerClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimerClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimerClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer3Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer3Clkdiv {
        MrccCtimer3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER3 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer3Clksel(pub u32);
impl MrccCtimer3Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer3Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer3Clksel {
        MrccCtimer3Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER4 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer4Clkdiv(pub u32);
impl MrccCtimer4Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccCtimerClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccCtimerClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccCtimerClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccCtimerClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccCtimerClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccCtimerClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccCtimerClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccCtimerClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccCtimerClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer4Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer4Clkdiv {
        MrccCtimer4Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer4Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer4Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer4Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer4Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER4 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer4Clksel(pub u32);
impl MrccCtimer4Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccCtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccCtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccCtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer4Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer4Clksel {
        MrccCtimer4Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer4Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer4Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer4Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer4Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DAC0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDac0Clkdiv(pub u32);
impl MrccDac0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccDac0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccDac0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccDac0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccDac0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccDac0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccDac0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccDac0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccDac0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccDac0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccDac0Clkdiv {
    #[inline(always)]
    fn default() -> MrccDac0Clkdiv {
        MrccDac0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccDac0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDac0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDac0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccDac0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDac0Clksel(pub u32);
impl MrccDac0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccDac0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccDac0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccDac0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccDac0Clksel {
    #[inline(always)]
    fn default() -> MrccDac0Clksel {
        MrccDac0Clksel(0)
    }
}
impl core::fmt::Debug for MrccDac0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDac0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDac0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccDac0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DBG_TRACE clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClkdiv(pub u32);
impl MrccDbgTraceClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccDbgTraceClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccDbgTraceClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccDbgTraceClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccDbgTraceClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccDbgTraceClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccDbgTraceClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccDbgTraceClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccDbgTraceClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccDbgTraceClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccDbgTraceClkdiv {
    #[inline(always)]
    fn default() -> MrccDbgTraceClkdiv {
        MrccDbgTraceClkdiv(0)
    }
}
impl core::fmt::Debug for MrccDbgTraceClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDbgTraceClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDbgTraceClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccDbgTraceClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DBG_TRACE clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClksel(pub u32);
impl MrccDbgTraceClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccDbgTraceClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccDbgTraceClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccDbgTraceClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccDbgTraceClksel {
    #[inline(always)]
    fn default() -> MrccDbgTraceClksel {
        MrccDbgTraceClksel(0)
    }
}
impl core::fmt::Debug for MrccDbgTraceClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDbgTraceClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDbgTraceClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccDbgTraceClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXCAN0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan0Clkdiv(pub u32);
impl MrccFlexcan0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccFlexcanClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccFlexcanClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccFlexcanClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccFlexcanClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccFlexcanClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccFlexcanClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccFlexcanClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccFlexcanClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccFlexcanClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexcan0Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexcan0Clkdiv {
        MrccFlexcan0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexcan0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexcan0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan0Clksel(pub u32);
impl MrccFlexcan0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccFlexcanClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccFlexcanClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccFlexcanClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexcan0Clksel {
    #[inline(always)]
    fn default() -> MrccFlexcan0Clksel {
        MrccFlexcan0Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexcan0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexcan0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXCAN1 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan1Clkdiv(pub u32);
impl MrccFlexcan1Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccFlexcanClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccFlexcanClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccFlexcanClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccFlexcanClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccFlexcanClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccFlexcanClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccFlexcanClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccFlexcanClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccFlexcanClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexcan1Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexcan1Clkdiv {
        MrccFlexcan1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexcan1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexcan1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN1 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan1Clksel(pub u32);
impl MrccFlexcan1Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccFlexcanClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccFlexcanClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccFlexcanClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexcan1Clksel {
    #[inline(always)]
    fn default() -> MrccFlexcan1Clksel {
        MrccFlexcan1Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexcan1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexcan1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXIO0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexio0Clkdiv(pub u32);
impl MrccFlexio0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccFlexio0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccFlexio0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccFlexio0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccFlexio0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccFlexio0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccFlexio0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccFlexio0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccFlexio0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccFlexio0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexio0Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexio0Clkdiv {
        MrccFlexio0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexio0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexio0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexio0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexio0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexio0Clksel(pub u32);
impl MrccFlexio0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccFlexio0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccFlexio0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccFlexio0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexio0Clksel {
    #[inline(always)]
    fn default() -> MrccFlexio0Clksel {
        MrccFlexio0Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexio0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexio0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexio0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexio0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "Control Automatic Clock Gating 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc0(pub u32);
impl MrccGlbAcc0 {
    #[doc = "INPUTMUX0"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0"]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SMARTDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0"]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0"]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0"]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0"]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FMC"]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FMC"]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AOI1"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1"]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0"]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0"]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbAcc0 {
    #[inline(always)]
    fn default() -> MrccGlbAcc0 {
        MrccGlbAcc0(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("fmc", &self.fmc())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.wwdt0(),
            self.smartdma0(),
            self.dma0(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.fmc(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc1(pub u32);
impl MrccGlbAcc1 {
    #[doc = "FLEXPWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1"]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0"]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3"]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0"]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0"]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SGI0"]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0"]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "UDF0"]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0"]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ADC2"]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2"]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3"]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3"]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MrccGlbAcc1 {
    #[inline(always)]
    fn default() -> MrccGlbAcc1 {
        MrccGlbAcc1(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc1")
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "Control Automatic Clock Gating 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc2(pub u32);
impl MrccGlbAcc2 {
    #[doc = "RAMA"]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA"]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMB"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB"]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMC"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC"]
    #[inline(always)]
    pub const fn set_ramc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ROMC"]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC"]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for MrccGlbAcc2 {
    #[inline(always)]
    fn default() -> MrccGlbAcc2 {
        MrccGlbAcc2(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc2")
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("ramc", &self.ramc())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc2 {{ rama: {=bool:?}, ramb: {=bool:?}, ramc: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?}, romc: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.ramc(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0(pub u32);
impl MrccGlbCc0 {
    #[doc = "INPUTMUX0"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0"]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SMARTDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0"]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0"]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0"]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0"]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FMC"]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FMC"]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AOI1"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1"]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0"]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0"]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbCc0 {
    #[inline(always)]
    fn default() -> MrccGlbCc0 {
        MrccGlbCc0(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("fmc", &self.fmc())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.wwdt0(),
            self.smartdma0(),
            self.dma0(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.fmc(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0()
        )
    }
}
#[doc = "AHB Clock Control Clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0Clr(pub u32);
impl MrccGlbCc0Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc0Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc0Clr {
        MrccGlbCc0Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc0Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0Set(pub u32);
impl MrccGlbCc0Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc0Set {
    #[inline(always)]
    fn default() -> MrccGlbCc0Set {
        MrccGlbCc0Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc0Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1(pub u32);
impl MrccGlbCc1 {
    #[doc = "FLEXPWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1"]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0"]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3"]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0"]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0"]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TDET0"]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TDET0"]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SGI0"]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0"]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "UDF0"]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0"]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ADC2"]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2"]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3"]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3"]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MrccGlbCc1 {
    #[inline(always)]
    fn default() -> MrccGlbCc1 {
        MrccGlbCc1(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1")
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("tdet0", &self.tdet0())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, tdet0: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.tdet0(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "AHB Clock Control Clear 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1Clr(pub u32);
impl MrccGlbCc1Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc1Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc1Clr {
        MrccGlbCc1Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc1Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1Set(pub u32);
impl MrccGlbCc1Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc1Set {
    #[inline(always)]
    fn default() -> MrccGlbCc1Set {
        MrccGlbCc1Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc1Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc2(pub u32);
impl MrccGlbCc2 {
    #[doc = "RAMA"]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA"]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMB"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB"]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMC"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC"]
    #[inline(always)]
    pub const fn set_ramc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ROMC"]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC"]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for MrccGlbCc2 {
    #[inline(always)]
    fn default() -> MrccGlbCc2 {
        MrccGlbCc2(0)
    }
}
impl core::fmt::Debug for MrccGlbCc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc2")
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("ramc", &self.ramc())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc2 {{ rama: {=bool:?}, ramb: {=bool:?}, ramc: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?}, romc: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.ramc(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control Clear 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc2Clr(pub u32);
impl MrccGlbCc2Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc2Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc2Clr {
        MrccGlbCc2Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc2Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc2Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc2Set(pub u32);
impl MrccGlbCc2Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbCc2Set {
    #[inline(always)]
    fn default() -> MrccGlbCc2Set {
        MrccGlbCc2Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc2Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc2Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0(pub u32);
impl MrccGlbRst0 {
    #[doc = "INPUTMUX0"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0"]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SMARTDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0"]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0"]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0"]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0"]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1"]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0"]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0"]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0"]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbRst0 {
    #[inline(always)]
    fn default() -> MrccGlbRst0 {
        MrccGlbRst0(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.smartdma0(),
            self.dma0(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0Clr(pub u32);
impl MrccGlbRst0Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst0Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst0Clr {
        MrccGlbRst0Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst0Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0Set(pub u32);
impl MrccGlbRst0Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst0Set {
    #[inline(always)]
    fn default() -> MrccGlbRst0Set {
        MrccGlbRst0Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst0Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1(pub u32);
impl MrccGlbRst1 {
    #[doc = "FLEXPWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1"]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0"]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3"]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0"]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0"]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC2"]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2"]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3"]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3"]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MrccGlbRst1 {
    #[inline(always)]
    fn default() -> MrccGlbRst1 {
        MrccGlbRst1(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1")
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("pkc0", &self.pkc0())
            .field("trng0", &self.trng0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, pkc0: {=bool:?}, trng0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.pkc0(),
            self.trng0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1Clr(pub u32);
impl MrccGlbRst1Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst1Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst1Clr {
        MrccGlbRst1Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst1Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1Set(pub u32);
impl MrccGlbRst1Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst1Set {
    #[inline(always)]
    fn default() -> MrccGlbRst1Set {
        MrccGlbRst1Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst1Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst2(pub u32);
impl MrccGlbRst2 {
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for MrccGlbRst2 {
    #[inline(always)]
    fn default() -> MrccGlbRst2 {
        MrccGlbRst2(0)
    }
}
impl core::fmt::Debug for MrccGlbRst2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst2")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst2 {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?} }}",
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst2Clr(pub u32);
impl MrccGlbRst2Clr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst2Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst2Clr {
        MrccGlbRst2Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst2Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst2Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst2Set(pub u32);
impl MrccGlbRst2Set {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MrccGlbRst2Set {
    #[inline(always)]
    fn default() -> MrccGlbRst2Set {
        MrccGlbRst2Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst2Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst2Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "I3C0_FCLK clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccI3c0FclkClkdiv(pub u32);
impl MrccI3c0FclkClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccI3c0FclkClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccI3c0FclkClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccI3c0FclkClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccI3c0FclkClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccI3c0FclkClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccI3c0FclkClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccI3c0FclkClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccI3c0FclkClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccI3c0FclkClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccI3c0FclkClkdiv {
    #[inline(always)]
    fn default() -> MrccI3c0FclkClkdiv {
        MrccI3c0FclkClkdiv(0)
    }
}
impl core::fmt::Debug for MrccI3c0FclkClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccI3c0FclkClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccI3c0FclkClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccI3c0FclkClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0_FCLK clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccI3c0FclkClksel(pub u32);
impl MrccI3c0FclkClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccI3c0FclkClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccI3c0FclkClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccI3c0FclkClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccI3c0FclkClksel {
    #[inline(always)]
    fn default() -> MrccI3c0FclkClksel {
        MrccI3c0FclkClksel(0)
    }
}
impl core::fmt::Debug for MrccI3c0FclkClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccI3c0FclkClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccI3c0FclkClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccI3c0FclkClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c0Clkdiv(pub u32);
impl MrccLpi2c0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2cClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2cClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2cClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2cClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2cClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2cClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2cClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2cClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2cClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c0Clkdiv {
        MrccLpi2c0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c0Clksel(pub u32);
impl MrccLpi2c0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2cClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2cClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2cClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c0Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c0Clksel {
        MrccLpi2c0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C1 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c1Clkdiv(pub u32);
impl MrccLpi2c1Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2cClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2cClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2cClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2cClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2cClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2cClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2cClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2cClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2cClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c1Clkdiv {
        MrccLpi2c1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C1 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c1Clksel(pub u32);
impl MrccLpi2c1Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2cClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2cClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2cClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c1Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c1Clksel {
        MrccLpi2c1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C2 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c2Clkdiv(pub u32);
impl MrccLpi2c2Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2cClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2cClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2cClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2cClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2cClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2cClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2cClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2cClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2cClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c2Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c2Clkdiv {
        MrccLpi2c2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C2 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c2Clksel(pub u32);
impl MrccLpi2c2Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2cClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2cClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2cClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c2Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c2Clksel {
        MrccLpi2c2Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C3 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c3Clkdiv(pub u32);
impl MrccLpi2c3Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpi2cClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpi2cClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpi2cClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpi2cClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpi2cClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpi2cClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpi2cClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpi2cClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpi2cClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c3Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c3Clkdiv {
        MrccLpi2c3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C3 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c3Clksel(pub u32);
impl MrccLpi2c3Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpi2cClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpi2cClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpi2cClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c3Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c3Clksel {
        MrccLpi2c3Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi0Clkdiv(pub u32);
impl MrccLpspi0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpspiClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpspiClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpspiClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpspiClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpspiClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpspiClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpspiClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpspiClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpspiClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpspi0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpspi0Clkdiv {
        MrccLpspi0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpspi0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpspi0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi0Clksel(pub u32);
impl MrccLpspi0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpspiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpspiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpspiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpspi0Clksel {
    #[inline(always)]
    fn default() -> MrccLpspi0Clksel {
        MrccLpspi0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpspi0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpspi0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI1 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi1Clkdiv(pub u32);
impl MrccLpspi1Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpspiClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpspiClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpspiClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpspiClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpspiClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpspiClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpspiClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpspiClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpspiClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpspi1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpspi1Clkdiv {
        MrccLpspi1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpspi1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpspi1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI1 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi1Clksel(pub u32);
impl MrccLpspi1Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpspiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpspiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpspiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpspi1Clksel {
    #[inline(always)]
    fn default() -> MrccLpspi1Clksel {
        MrccLpspi1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpspi1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpspi1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPTMR0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLptmr0Clkdiv(pub u32);
impl MrccLptmr0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLptmr0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLptmr0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLptmr0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLptmr0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLptmr0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLptmr0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLptmr0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLptmr0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLptmr0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLptmr0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLptmr0Clkdiv {
        MrccLptmr0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLptmr0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLptmr0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLptmr0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLptmr0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPTMR0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLptmr0Clksel(pub u32);
impl MrccLptmr0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLptmr0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLptmr0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLptmr0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLptmr0Clksel {
    #[inline(always)]
    fn default() -> MrccLptmr0Clksel {
        MrccLptmr0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLptmr0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLptmr0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLptmr0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLptmr0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart0Clkdiv(pub u32);
impl MrccLpuart0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuartClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuartClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuartClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuartClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuartClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuartClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuartClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuartClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuartClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart0Clkdiv {
        MrccLpuart0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart0Clksel(pub u32);
impl MrccLpuart0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart0Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart0Clksel {
        MrccLpuart0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART1 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart1Clkdiv(pub u32);
impl MrccLpuart1Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuartClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuartClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuartClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuartClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuartClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuartClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuartClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuartClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuartClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart1Clkdiv {
        MrccLpuart1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART1 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart1Clksel(pub u32);
impl MrccLpuart1Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart1Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart1Clksel {
        MrccLpuart1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART2 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart2Clkdiv(pub u32);
impl MrccLpuart2Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuartClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuartClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuartClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuartClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuartClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuartClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuartClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuartClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuartClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart2Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart2Clkdiv {
        MrccLpuart2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART2 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart2Clksel(pub u32);
impl MrccLpuart2Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart2Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart2Clksel {
        MrccLpuart2Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART3 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart3Clkdiv(pub u32);
impl MrccLpuart3Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuartClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuartClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuartClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuartClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuartClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuartClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuartClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuartClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuartClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart3Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart3Clkdiv {
        MrccLpuart3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART3 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart3Clksel(pub u32);
impl MrccLpuart3Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart3Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart3Clksel {
        MrccLpuart3Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART4 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart4Clkdiv(pub u32);
impl MrccLpuart4Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuartClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuartClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuartClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuartClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuartClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuartClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuartClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuartClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuartClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart4Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart4Clkdiv {
        MrccLpuart4Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart4Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart4Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart4Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart4Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART4 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart4Clksel(pub u32);
impl MrccLpuart4Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart4Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart4Clksel {
        MrccLpuart4Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart4Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart4Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart4Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart4Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART5 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart5Clkdiv(pub u32);
impl MrccLpuart5Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccLpuartClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccLpuartClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccLpuartClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccLpuartClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccLpuartClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccLpuartClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccLpuartClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccLpuartClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccLpuartClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart5Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart5Clkdiv {
        MrccLpuart5Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart5Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart5Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart5Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart5Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART5 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart5Clksel(pub u32);
impl MrccLpuart5Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccLpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MrccLpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccLpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart5Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart5Clksel {
        MrccLpuart5Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart5Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart5Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart5Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart5Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "OSTIMER0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccOstimer0Clksel(pub u32);
impl MrccOstimer0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccOstimer0ClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccOstimer0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccOstimer0ClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccOstimer0Clksel {
    #[inline(always)]
    fn default() -> MrccOstimer0Clksel {
        MrccOstimer0Clksel(0)
    }
}
impl core::fmt::Debug for MrccOstimer0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccOstimer0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccOstimer0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccOstimer0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "SYSTICK clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClkdiv(pub u32);
impl MrccSystickClkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccSystickClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccSystickClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccSystickClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccSystickClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccSystickClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccSystickClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccSystickClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccSystickClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccSystickClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccSystickClkdiv {
    #[inline(always)]
    fn default() -> MrccSystickClkdiv {
        MrccSystickClkdiv(0)
    }
}
impl core::fmt::Debug for MrccSystickClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccSystickClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccSystickClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccSystickClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SYSTICK clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClksel(pub u32);
impl MrccSystickClksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccSystickClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccSystickClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccSystickClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccSystickClksel {
    #[inline(always)]
    fn default() -> MrccSystickClksel {
        MrccSystickClksel(0)
    }
}
impl core::fmt::Debug for MrccSystickClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccSystickClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccSystickClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccSystickClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "USB0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccUsb0Clkdiv(pub u32);
impl MrccUsb0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccUsb0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccUsb0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccUsb0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccUsb0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccUsb0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccUsb0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccUsb0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccUsb0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccUsb0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccUsb0Clkdiv {
    #[inline(always)]
    fn default() -> MrccUsb0Clkdiv {
        MrccUsb0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccUsb0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccUsb0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccUsb0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccUsb0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "USB0 clock selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccUsb0Clksel(pub u32);
impl MrccUsb0Clksel {
    #[doc = "Functional Clock Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::MrccUsb0ClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MrccUsb0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::MrccUsb0ClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccUsb0Clksel {
    #[inline(always)]
    fn default() -> MrccUsb0Clksel {
        MrccUsb0Clksel(0)
    }
}
impl core::fmt::Debug for MrccUsb0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccUsb0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccUsb0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccUsb0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "WWDT0 clock divider control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccWwdt0Clkdiv(pub u32);
impl MrccWwdt0Clkdiv {
    #[doc = "Functional Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MrccWwdt0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MrccWwdt0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MrccWwdt0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MrccWwdt0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MrccWwdt0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MrccWwdt0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MrccWwdt0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MrccWwdt0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MrccWwdt0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccWwdt0Clkdiv {
    #[inline(always)]
    fn default() -> MrccWwdt0Clkdiv {
        MrccWwdt0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccWwdt0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccWwdt0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccWwdt0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccWwdt0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
