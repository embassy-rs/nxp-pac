#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/metadata.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/metadata.rs")]
#[cfg_attr(
    feature = "lpc55s69_core0",
    path = "./chips/lpc55s69_cm33_core0/metadata.rs"
)]
#[cfg_attr(
    feature = "lpc55s69_core1",
    path = "./chips/lpc55s69_cm33_core1/metadata.rs"
)]
mod _generated;

pub use _generated::*;
