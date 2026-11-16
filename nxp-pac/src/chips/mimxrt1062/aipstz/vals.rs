#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mprot0(u8);
impl Mprot0 {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
    pub const Mpl0: Self = Self(0x0);
    #[doc = "This master is not trusted for read accesses."]
    pub const Mtr0: Self = Self(0x0);
    #[doc = "This master is not trusted for write accesses."]
    pub const Mtw0: Self = Self(0x0);
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
    pub const Mpl1: Self = Self(0x01);
    #[doc = "This master is trusted for write accesses."]
    pub const Mtw1: Self = Self(0x02);
    #[doc = "This master is trusted for read accesses."]
    pub const Mtr1: Self = Self(0x04);
    #[doc = "Write accesses from this master are allowed to be buffered."]
    pub const Mbw1: Self = Self(0x08);
}
impl Mprot0 {
    pub const fn from_bits(val: u8) -> Mprot0 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Mprot0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Mpl0"),
            0x0 => f.write_str("Mtr0"),
            0x0 => f.write_str("Mtw0"),
            0x01 => f.write_str("Mpl1"),
            0x02 => f.write_str("Mtw1"),
            0x04 => f.write_str("Mtr1"),
            0x08 => f.write_str("Mbw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mprot0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Mpl0"),
            0x0 => defmt::write!(f, "Mtr0"),
            0x0 => defmt::write!(f, "Mtw0"),
            0x01 => defmt::write!(f, "Mpl1"),
            0x02 => defmt::write!(f, "Mtw1"),
            0x04 => defmt::write!(f, "Mtr1"),
            0x08 => defmt::write!(f, "Mbw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mprot0 {
    #[inline(always)]
    fn from(val: u8) -> Mprot0 {
        Mprot0::from_bits(val)
    }
}
impl From<Mprot0> for u8 {
    #[inline(always)]
    fn from(val: Mprot0) -> u8 {
        Mprot0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mprot1(u8);
impl Mprot1 {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
    pub const Mpl0: Self = Self(0x0);
    #[doc = "This master is not trusted for read accesses."]
    pub const Mtr0: Self = Self(0x0);
    #[doc = "This master is not trusted for write accesses."]
    pub const Mtw0: Self = Self(0x0);
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
    pub const Mpl1: Self = Self(0x01);
    #[doc = "This master is trusted for write accesses."]
    pub const Mtw1: Self = Self(0x02);
    #[doc = "This master is trusted for read accesses."]
    pub const Mtr1: Self = Self(0x04);
    #[doc = "Write accesses from this master are allowed to be buffered."]
    pub const Mbw1: Self = Self(0x08);
}
impl Mprot1 {
    pub const fn from_bits(val: u8) -> Mprot1 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Mprot1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Mpl0"),
            0x0 => f.write_str("Mtr0"),
            0x0 => f.write_str("Mtw0"),
            0x01 => f.write_str("Mpl1"),
            0x02 => f.write_str("Mtw1"),
            0x04 => f.write_str("Mtr1"),
            0x08 => f.write_str("Mbw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mprot1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Mpl0"),
            0x0 => defmt::write!(f, "Mtr0"),
            0x0 => defmt::write!(f, "Mtw0"),
            0x01 => defmt::write!(f, "Mpl1"),
            0x02 => defmt::write!(f, "Mtw1"),
            0x04 => defmt::write!(f, "Mtr1"),
            0x08 => defmt::write!(f, "Mbw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mprot1 {
    #[inline(always)]
    fn from(val: u8) -> Mprot1 {
        Mprot1::from_bits(val)
    }
}
impl From<Mprot1> for u8 {
    #[inline(always)]
    fn from(val: Mprot1) -> u8 {
        Mprot1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mprot2(u8);
impl Mprot2 {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
    pub const Mpl0: Self = Self(0x0);
    #[doc = "This master is not trusted for read accesses."]
    pub const Mtr0: Self = Self(0x0);
    #[doc = "This master is not trusted for write accesses."]
    pub const Mtw0: Self = Self(0x0);
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
    pub const Mpl1: Self = Self(0x01);
    #[doc = "This master is trusted for write accesses."]
    pub const Mtw1: Self = Self(0x02);
    #[doc = "This master is trusted for read accesses."]
    pub const Mtr1: Self = Self(0x04);
    #[doc = "Write accesses from this master are allowed to be buffered."]
    pub const Mbw1: Self = Self(0x08);
}
impl Mprot2 {
    pub const fn from_bits(val: u8) -> Mprot2 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Mprot2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Mpl0"),
            0x0 => f.write_str("Mtr0"),
            0x0 => f.write_str("Mtw0"),
            0x01 => f.write_str("Mpl1"),
            0x02 => f.write_str("Mtw1"),
            0x04 => f.write_str("Mtr1"),
            0x08 => f.write_str("Mbw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mprot2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Mpl0"),
            0x0 => defmt::write!(f, "Mtr0"),
            0x0 => defmt::write!(f, "Mtw0"),
            0x01 => defmt::write!(f, "Mpl1"),
            0x02 => defmt::write!(f, "Mtw1"),
            0x04 => defmt::write!(f, "Mtr1"),
            0x08 => defmt::write!(f, "Mbw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mprot2 {
    #[inline(always)]
    fn from(val: u8) -> Mprot2 {
        Mprot2::from_bits(val)
    }
}
impl From<Mprot2> for u8 {
    #[inline(always)]
    fn from(val: Mprot2) -> u8 {
        Mprot2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mprot3(u8);
impl Mprot3 {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
    pub const Mpl0: Self = Self(0x0);
    #[doc = "This master is not trusted for read accesses."]
    pub const Mtr0: Self = Self(0x0);
    #[doc = "This master is not trusted for write accesses."]
    pub const Mtw0: Self = Self(0x0);
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
    pub const Mpl1: Self = Self(0x01);
    #[doc = "This master is trusted for write accesses."]
    pub const Mtw1: Self = Self(0x02);
    #[doc = "This master is trusted for read accesses."]
    pub const Mtr1: Self = Self(0x04);
    #[doc = "Write accesses from this master are allowed to be buffered."]
    pub const Mbw1: Self = Self(0x08);
}
impl Mprot3 {
    pub const fn from_bits(val: u8) -> Mprot3 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Mprot3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Mpl0"),
            0x0 => f.write_str("Mtr0"),
            0x0 => f.write_str("Mtw0"),
            0x01 => f.write_str("Mpl1"),
            0x02 => f.write_str("Mtw1"),
            0x04 => f.write_str("Mtr1"),
            0x08 => f.write_str("Mbw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mprot3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Mpl0"),
            0x0 => defmt::write!(f, "Mtr0"),
            0x0 => defmt::write!(f, "Mtw0"),
            0x01 => defmt::write!(f, "Mpl1"),
            0x02 => defmt::write!(f, "Mtw1"),
            0x04 => defmt::write!(f, "Mtr1"),
            0x08 => defmt::write!(f, "Mbw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mprot3 {
    #[inline(always)]
    fn from(val: u8) -> Mprot3 {
        Mprot3::from_bits(val)
    }
}
impl From<Mprot3> for u8 {
    #[inline(always)]
    fn from(val: Mprot3) -> u8 {
        Mprot3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac0(u8);
impl Opac0 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac0 {
    pub const fn from_bits(val: u8) -> Opac0 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac0 {
    #[inline(always)]
    fn from(val: u8) -> Opac0 {
        Opac0::from_bits(val)
    }
}
impl From<Opac0> for u8 {
    #[inline(always)]
    fn from(val: Opac0) -> u8 {
        Opac0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac1(u8);
impl Opac1 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac1 {
    pub const fn from_bits(val: u8) -> Opac1 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac1 {
    #[inline(always)]
    fn from(val: u8) -> Opac1 {
        Opac1::from_bits(val)
    }
}
impl From<Opac1> for u8 {
    #[inline(always)]
    fn from(val: Opac1) -> u8 {
        Opac1::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac10(u8);
impl Opac10 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac10 {
    pub const fn from_bits(val: u8) -> Opac10 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac10 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac10 {
    #[inline(always)]
    fn from(val: u8) -> Opac10 {
        Opac10::from_bits(val)
    }
}
impl From<Opac10> for u8 {
    #[inline(always)]
    fn from(val: Opac10) -> u8 {
        Opac10::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac11(u8);
impl Opac11 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac11 {
    pub const fn from_bits(val: u8) -> Opac11 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac11 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac11 {
    #[inline(always)]
    fn from(val: u8) -> Opac11 {
        Opac11::from_bits(val)
    }
}
impl From<Opac11> for u8 {
    #[inline(always)]
    fn from(val: Opac11) -> u8 {
        Opac11::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac12(u8);
impl Opac12 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac12 {
    pub const fn from_bits(val: u8) -> Opac12 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac12 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac12 {
    #[inline(always)]
    fn from(val: u8) -> Opac12 {
        Opac12::from_bits(val)
    }
}
impl From<Opac12> for u8 {
    #[inline(always)]
    fn from(val: Opac12) -> u8 {
        Opac12::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac13(u8);
impl Opac13 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac13 {
    pub const fn from_bits(val: u8) -> Opac13 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac13 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac13 {
    #[inline(always)]
    fn from(val: u8) -> Opac13 {
        Opac13::from_bits(val)
    }
}
impl From<Opac13> for u8 {
    #[inline(always)]
    fn from(val: Opac13) -> u8 {
        Opac13::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac14(u8);
impl Opac14 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac14 {
    pub const fn from_bits(val: u8) -> Opac14 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac14 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac14 {
    #[inline(always)]
    fn from(val: u8) -> Opac14 {
        Opac14::from_bits(val)
    }
}
impl From<Opac14> for u8 {
    #[inline(always)]
    fn from(val: Opac14) -> u8 {
        Opac14::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac15(u8);
impl Opac15 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac15 {
    pub const fn from_bits(val: u8) -> Opac15 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac15 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac15 {
    #[inline(always)]
    fn from(val: u8) -> Opac15 {
        Opac15::from_bits(val)
    }
}
impl From<Opac15> for u8 {
    #[inline(always)]
    fn from(val: Opac15) -> u8 {
        Opac15::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac16(u8);
impl Opac16 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac16 {
    pub const fn from_bits(val: u8) -> Opac16 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac16 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac16 {
    #[inline(always)]
    fn from(val: u8) -> Opac16 {
        Opac16::from_bits(val)
    }
}
impl From<Opac16> for u8 {
    #[inline(always)]
    fn from(val: Opac16) -> u8 {
        Opac16::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac17(u8);
impl Opac17 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac17 {
    pub const fn from_bits(val: u8) -> Opac17 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac17 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac17 {
    #[inline(always)]
    fn from(val: u8) -> Opac17 {
        Opac17::from_bits(val)
    }
}
impl From<Opac17> for u8 {
    #[inline(always)]
    fn from(val: Opac17) -> u8 {
        Opac17::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac18(u8);
impl Opac18 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac18 {
    pub const fn from_bits(val: u8) -> Opac18 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac18 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac18 {
    #[inline(always)]
    fn from(val: u8) -> Opac18 {
        Opac18::from_bits(val)
    }
}
impl From<Opac18> for u8 {
    #[inline(always)]
    fn from(val: Opac18) -> u8 {
        Opac18::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac19(u8);
impl Opac19 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac19 {
    pub const fn from_bits(val: u8) -> Opac19 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac19 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac19 {
    #[inline(always)]
    fn from(val: u8) -> Opac19 {
        Opac19::from_bits(val)
    }
}
impl From<Opac19> for u8 {
    #[inline(always)]
    fn from(val: Opac19) -> u8 {
        Opac19::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac2(u8);
impl Opac2 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac2 {
    pub const fn from_bits(val: u8) -> Opac2 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac2 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac2 {
    #[inline(always)]
    fn from(val: u8) -> Opac2 {
        Opac2::from_bits(val)
    }
}
impl From<Opac2> for u8 {
    #[inline(always)]
    fn from(val: Opac2) -> u8 {
        Opac2::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac20(u8);
impl Opac20 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac20 {
    pub const fn from_bits(val: u8) -> Opac20 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac20 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac20 {
    #[inline(always)]
    fn from(val: u8) -> Opac20 {
        Opac20::from_bits(val)
    }
}
impl From<Opac20> for u8 {
    #[inline(always)]
    fn from(val: Opac20) -> u8 {
        Opac20::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac21(u8);
impl Opac21 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac21 {
    pub const fn from_bits(val: u8) -> Opac21 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac21 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac21 {
    #[inline(always)]
    fn from(val: u8) -> Opac21 {
        Opac21::from_bits(val)
    }
}
impl From<Opac21> for u8 {
    #[inline(always)]
    fn from(val: Opac21) -> u8 {
        Opac21::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac22(u8);
impl Opac22 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac22 {
    pub const fn from_bits(val: u8) -> Opac22 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac22 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac22 {
    #[inline(always)]
    fn from(val: u8) -> Opac22 {
        Opac22::from_bits(val)
    }
}
impl From<Opac22> for u8 {
    #[inline(always)]
    fn from(val: Opac22) -> u8 {
        Opac22::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac23(u8);
impl Opac23 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac23 {
    pub const fn from_bits(val: u8) -> Opac23 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac23 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac23 {
    #[inline(always)]
    fn from(val: u8) -> Opac23 {
        Opac23::from_bits(val)
    }
}
impl From<Opac23> for u8 {
    #[inline(always)]
    fn from(val: Opac23) -> u8 {
        Opac23::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac24(u8);
impl Opac24 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac24 {
    pub const fn from_bits(val: u8) -> Opac24 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac24 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac24 {
    #[inline(always)]
    fn from(val: u8) -> Opac24 {
        Opac24::from_bits(val)
    }
}
impl From<Opac24> for u8 {
    #[inline(always)]
    fn from(val: Opac24) -> u8 {
        Opac24::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac25(u8);
impl Opac25 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac25 {
    pub const fn from_bits(val: u8) -> Opac25 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac25 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac25 {
    #[inline(always)]
    fn from(val: u8) -> Opac25 {
        Opac25::from_bits(val)
    }
}
impl From<Opac25> for u8 {
    #[inline(always)]
    fn from(val: Opac25) -> u8 {
        Opac25::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac26(u8);
impl Opac26 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac26 {
    pub const fn from_bits(val: u8) -> Opac26 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac26 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac26 {
    #[inline(always)]
    fn from(val: u8) -> Opac26 {
        Opac26::from_bits(val)
    }
}
impl From<Opac26> for u8 {
    #[inline(always)]
    fn from(val: Opac26) -> u8 {
        Opac26::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac27(u8);
impl Opac27 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac27 {
    pub const fn from_bits(val: u8) -> Opac27 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac27 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac27 {
    #[inline(always)]
    fn from(val: u8) -> Opac27 {
        Opac27::from_bits(val)
    }
}
impl From<Opac27> for u8 {
    #[inline(always)]
    fn from(val: Opac27) -> u8 {
        Opac27::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac28(u8);
impl Opac28 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac28 {
    pub const fn from_bits(val: u8) -> Opac28 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac28 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac28 {
    #[inline(always)]
    fn from(val: u8) -> Opac28 {
        Opac28::from_bits(val)
    }
}
impl From<Opac28> for u8 {
    #[inline(always)]
    fn from(val: Opac28) -> u8 {
        Opac28::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac29(u8);
impl Opac29 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac29 {
    pub const fn from_bits(val: u8) -> Opac29 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac29 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac29 {
    #[inline(always)]
    fn from(val: u8) -> Opac29 {
        Opac29::from_bits(val)
    }
}
impl From<Opac29> for u8 {
    #[inline(always)]
    fn from(val: Opac29) -> u8 {
        Opac29::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac3(u8);
impl Opac3 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac3 {
    pub const fn from_bits(val: u8) -> Opac3 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac3 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac3 {
    #[inline(always)]
    fn from(val: u8) -> Opac3 {
        Opac3::from_bits(val)
    }
}
impl From<Opac3> for u8 {
    #[inline(always)]
    fn from(val: Opac3) -> u8 {
        Opac3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac30(u8);
impl Opac30 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac30 {
    pub const fn from_bits(val: u8) -> Opac30 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac30 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac30 {
    #[inline(always)]
    fn from(val: u8) -> Opac30 {
        Opac30::from_bits(val)
    }
}
impl From<Opac30> for u8 {
    #[inline(always)]
    fn from(val: Opac30) -> u8 {
        Opac30::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac31(u8);
impl Opac31 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac31 {
    pub const fn from_bits(val: u8) -> Opac31 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac31 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac31 {
    #[inline(always)]
    fn from(val: u8) -> Opac31 {
        Opac31::from_bits(val)
    }
}
impl From<Opac31> for u8 {
    #[inline(always)]
    fn from(val: Opac31) -> u8 {
        Opac31::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac32(u8);
impl Opac32 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac32 {
    pub const fn from_bits(val: u8) -> Opac32 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac32 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac32 {
    #[inline(always)]
    fn from(val: u8) -> Opac32 {
        Opac32::from_bits(val)
    }
}
impl From<Opac32> for u8 {
    #[inline(always)]
    fn from(val: Opac32) -> u8 {
        Opac32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac33(u8);
impl Opac33 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac33 {
    pub const fn from_bits(val: u8) -> Opac33 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac33 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac33 {
    #[inline(always)]
    fn from(val: u8) -> Opac33 {
        Opac33::from_bits(val)
    }
}
impl From<Opac33> for u8 {
    #[inline(always)]
    fn from(val: Opac33) -> u8 {
        Opac33::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac4(u8);
impl Opac4 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac4 {
    pub const fn from_bits(val: u8) -> Opac4 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac4 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac4 {
    #[inline(always)]
    fn from(val: u8) -> Opac4 {
        Opac4::from_bits(val)
    }
}
impl From<Opac4> for u8 {
    #[inline(always)]
    fn from(val: Opac4) -> u8 {
        Opac4::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac5(u8);
impl Opac5 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac5 {
    pub const fn from_bits(val: u8) -> Opac5 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac5 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac5 {
    #[inline(always)]
    fn from(val: u8) -> Opac5 {
        Opac5::from_bits(val)
    }
}
impl From<Opac5> for u8 {
    #[inline(always)]
    fn from(val: Opac5) -> u8 {
        Opac5::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac6(u8);
impl Opac6 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac6 {
    pub const fn from_bits(val: u8) -> Opac6 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac6 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac6 {
    #[inline(always)]
    fn from(val: u8) -> Opac6 {
        Opac6::from_bits(val)
    }
}
impl From<Opac6> for u8 {
    #[inline(always)]
    fn from(val: Opac6) -> u8 {
        Opac6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac7(u8);
impl Opac7 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac7 {
    pub const fn from_bits(val: u8) -> Opac7 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac7 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac7 {
    #[inline(always)]
    fn from(val: u8) -> Opac7 {
        Opac7::from_bits(val)
    }
}
impl From<Opac7> for u8 {
    #[inline(always)]
    fn from(val: Opac7) -> u8 {
        Opac7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac8(u8);
impl Opac8 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac8 {
    pub const fn from_bits(val: u8) -> Opac8 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac8 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac8 {
    #[inline(always)]
    fn from(val: u8) -> Opac8 {
        Opac8::from_bits(val)
    }
}
impl From<Opac8> for u8 {
    #[inline(always)]
    fn from(val: Opac8) -> u8 {
        Opac8::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Opac9(u8);
impl Opac9 {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    pub const Sp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are allowed."]
    pub const Tp0: Self = Self(0x0);
    #[doc = "This peripheral allows write accesses."]
    pub const Wp0: Self = Self(0x0);
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Tp1: Self = Self(0x01);
    #[doc = "This peripheral is write protected. If a write access is attempted, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Wp1: Self = Self(0x02);
    #[doc = "This peripheral requires supervisor privilege level for accesses. The master privilege level must indicate supervisor via the hprot\\[1\\] access attribute, and the MPROTx\\[MPL\\] control bit for the master must be set. If not, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    pub const Sp1: Self = Self(0x04);
    #[doc = "Write accesses to this peripheral are allowed to be buffered by the AIPSTZ."]
    pub const Bw1: Self = Self(0x08);
}
impl Opac9 {
    pub const fn from_bits(val: u8) -> Opac9 {
        Self(val & 0x0f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for Opac9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Sp0"),
            0x0 => f.write_str("Tp0"),
            0x0 => f.write_str("Wp0"),
            0x01 => f.write_str("Tp1"),
            0x02 => f.write_str("Wp1"),
            0x04 => f.write_str("Sp1"),
            0x08 => f.write_str("Bw1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opac9 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Sp0"),
            0x0 => defmt::write!(f, "Tp0"),
            0x0 => defmt::write!(f, "Wp0"),
            0x01 => defmt::write!(f, "Tp1"),
            0x02 => defmt::write!(f, "Wp1"),
            0x04 => defmt::write!(f, "Sp1"),
            0x08 => defmt::write!(f, "Bw1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Opac9 {
    #[inline(always)]
    fn from(val: u8) -> Opac9 {
        Opac9::from_bits(val)
    }
}
impl From<Opac9> for u8 {
    #[inline(always)]
    fn from(val: Opac9) -> u8 {
        Opac9::to_bits(val)
    }
}
