pub struct Chip {
    pub feature: &'static str,
    pub metadata: &'static str,
    pub transforms: &'static [ModelTransforms],
}

impl Chip {
    /// Availability predicates identify the selected core, not its metadata file.
    pub fn core_name(&self) -> String {
        hardware_feature(self.feature).to_ascii_uppercase()
    }
}

pub struct ModelTransforms {
    pub model: &'static str,
    pub files: &'static [&'static str],
}

// Canonical hardware descriptions; compatibility feature names resolve in find.
macro_rules! chips {
    ($($feature:literal => $metadata:literal),* $(,)?) => {
        pub const CHIPS: &[Chip] = &[$(Chip { feature: $feature, metadata: $metadata, transforms: &[] }),*];
    };
}

chips! {
    "mcxa156" => "MCXA156",
    "mcxa175" => "MCXA175",
    "mcxa176" => "MCXA176",
    "mcxa185" => "MCXA185",
    "mcxa186" => "MCXA186",
    "mcxa255" => "MCXA255",
    "mcxa256" => "MCXA256",
    "mcxa265" => "MCXA265",
    "mcxa266" => "MCXA266",
    "mcxa286" => "MCXA286",
    "mcxa287" => "MCXA287",
    "mcxa456" => "MCXA456",
    "mcxa457" => "MCXA457",
    "mcxa536" => "MCXA536",
    "mcxa537" => "MCXA537",
    "mcxa556" => "MCXA556",
    "mcxa557" => "MCXA557",
    "mcxa566" => "MCXA566",
    "mcxa567" => "MCXA567",
    "mcxa577" => "MCXA577",
}

// MCXA276 was an earlier name used on existing MCXA266 boards. Keep the Cargo
// feature, but share one hardware description, including future transforms.
fn hardware_feature(feature: &str) -> &str {
    if feature.eq_ignore_ascii_case("mcxa276") {
        "mcxa266"
    } else {
        feature
    }
}

pub fn find(feature: &str) -> Option<&'static Chip> {
    let feature = hardware_feature(feature);
    CHIPS
        .iter()
        .find(|chip| chip.feature.eq_ignore_ascii_case(feature))
}
