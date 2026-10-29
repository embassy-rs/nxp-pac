#[path = "../../nxp-pac/build/chips.rs"]
mod chips;
pub mod commands;
#[path = "../../nxp-pac/build/inventory.rs"]
mod inventory;
pub mod metadata;
#[path = "../../nxp-pac/build/engine/mod.rs"]
pub mod metapac;
#[path = "../../nxp-pac/build/models.rs"]
mod models;
pub mod pac;
pub mod util;

pub struct ChipDescription {
    /// The name of the chip to generate.
    chip: &'static str,

    /// Metadata file for this chip.
    metadata: Option<&'static str>,

    /// Cores to generate. If the chip has a single core, then this is the same as the
    /// [`name`](Feature::name) of the chip.
    cores: &'static [&'static str],

    /// When true, the source of the chip won't be the SVD anymore, but the manually curated peripherals
    metapac: bool,
}

/// Parts (and cores) to generate.
#[rustfmt::skip]
pub const CHIPS: &[ChipDescription] = &[
    ChipDescription { chip: "MIMXRT1011", metadata: Some("MIMXRT1011"), cores: &["MIMXRT1011"], metapac: false },
    ChipDescription { chip: "MIMXRT1062", metadata: Some("MIMXRT106x"), cores: &["MIMXRT1062"], metapac: false },
    ChipDescription { chip: "MIMXRT1064", metadata: Some("MIMXRT106x"), cores: &["MIMXRT1064"], metapac: false },
    ChipDescription { chip: "MIMXRT685S", metadata: None, cores: &["MIMXRT685S_cm33"], metapac: false },
    ChipDescription { chip: "LPC55S16", metadata: Some("LPC55S16"), cores: &["LPC55S16"], metapac: false },
    ChipDescription { chip: "LPC55S69", metadata: Some("LPC55S6x"), cores: &["LPC55S69_cm33_core0", "LPC55S69_cm33_core1"], metapac: false },
    ChipDescription { chip: "MCXN947", metadata: None, cores: &["MCXN947_cm33_core0", "MCXN947_cm33_core1"], metapac: false },
    ChipDescription { chip: "MCXA156", metadata: Some("MCXA156"), cores: &["MCXA156"], metapac: true },
    ChipDescription { chip: "MCXA175", metadata: Some("MCXA175"), cores: &["MCXA175"], metapac: true },
    ChipDescription { chip: "MCXA176", metadata: Some("MCXA176"), cores: &["MCXA176"], metapac: true },
    ChipDescription { chip: "MCXA185", metadata: Some("MCXA185"), cores: &["MCXA185"], metapac: true },
    ChipDescription { chip: "MCXA186", metadata: Some("MCXA186"), cores: &["MCXA186"], metapac: true },
    ChipDescription { chip: "MCXA255", metadata: Some("MCXA255"), cores: &["MCXA255"], metapac: true },
    ChipDescription { chip: "MCXA256", metadata: Some("MCXA256"), cores: &["MCXA256"], metapac: true },
    ChipDescription { chip: "MCXA265", metadata: Some("MCXA265"), cores: &["MCXA265"], metapac: true },
    ChipDescription { chip: "MCXA266", metadata: Some("MCXA266"), cores: &["MCXA266"], metapac: true },
    ChipDescription { chip: "MCXA276", metadata: Some("MCXA266"), cores: &["MCXA276"], metapac: true },
    ChipDescription { chip: "MCXA286", metadata: Some("MCXA286"), cores: &["MCXA286"], metapac: true },
    ChipDescription { chip: "MCXA287", metadata: Some("MCXA287"), cores: &["MCXA287"], metapac: true },
    ChipDescription { chip: "MCXA456", metadata: Some("MCXA456"), cores: &["MCXA456"], metapac: true },
    ChipDescription { chip: "MCXA457", metadata: Some("MCXA457"), cores: &["MCXA457"], metapac: true },
    ChipDescription { chip: "MCXA536", metadata: Some("MCXA536"), cores: &["MCXA536"], metapac: true },
    ChipDescription { chip: "MCXA537", metadata: Some("MCXA537"), cores: &["MCXA537"], metapac: true },
    ChipDescription { chip: "MCXA556", metadata: Some("MCXA556"), cores: &["MCXA556"], metapac: true },
    ChipDescription { chip: "MCXA557", metadata: Some("MCXA557"), cores: &["MCXA557"], metapac: true },
    ChipDescription { chip: "MCXA566", metadata: Some("MCXA566"), cores: &["MCXA566"], metapac: true },
    ChipDescription { chip: "MCXA567", metadata: Some("MCXA567"), cores: &["MCXA567"], metapac: true },
    ChipDescription { chip: "MCXA577", metadata: Some("MCXA577"), cores: &["MCXA577"], metapac: true },
];
