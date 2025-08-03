//! IOMUXC metadata generation for IMXRT1xxx parts.

#[derive(Debug)]
pub struct IomuxcRegisters {
    pub name: String,
    pub pad_address: u32,
    pub mux_address: u32,
}
