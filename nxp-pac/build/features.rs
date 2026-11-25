//! Per-chip IP features, exported to dependent crates through Cargo `links` metadata.
//!
//! The catalog `data/features/<ip>.yaml` lists every option cfg of an IP and how it is obtained:
//! a register/field presence predicate over the chip's own PAC IR, or, where the registers can't
//! tell, the chip's MCUXpresso SDK `FSL_FEATURE_<IP>_HAS_<X>` macro (import-only). See
//! `data/features/README.md`.
//!
//! Shared by `build.rs`, which evaluates the chip at build time, and the generator, which
//! checks the derived values against the SDK and writes the chip files.

// build.rs uses only the evaluation; the generator uses the rest.
#![allow(dead_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};
use chiptool::ir::IR;
use serde::{Deserialize, Serialize};

/// `data/features` below the crate's data directory.
pub fn data_dir(inputs: &Path) -> PathBuf {
    inputs.join("features")
}

/// A register or field presence predicate over one IP's IR.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Predicate {
    /// `<Block>.<register>`: the block has an item of that name.
    pub register: Option<String>,
    /// `<Fieldset>.<field>`: the fieldset has a field of that name.
    pub field: Option<String>,
    /// Invert: true when the register/field is absent (SDK `HAS_NO_*` macros).
    #[serde(default)]
    pub absent: bool,
}

impl Predicate {
    fn target(&self) -> anyhow::Result<(bool, &str, &str)> {
        let (is_field, s) = match (&self.register, &self.field) {
            (Some(r), None) => (false, r),
            (None, Some(f)) => (true, f),
            _ => bail!("a predicate needs exactly one of `register` or `field`"),
        };
        let (owner, item) = s
            .split_once('.')
            .with_context(|| format!("`{s}` must be `<Name>.<item>`"))?;
        Ok((is_field, owner, item))
    }

    /// Whether the predicate holds on `ir`. A missing block/fieldset means the item is absent.
    pub fn eval(&self, ir: &IR) -> anyhow::Result<bool> {
        let (is_field, owner, item) = self.target()?;
        let present = if is_field {
            let sets = register_fieldsets(ir, owner);
            !sets.is_empty()
                && sets
                    .iter()
                    .all(|fs| fs.fields.iter().any(|f| f.name == item))
        } else {
            ir.blocks
                .get(owner)
                .is_some_and(|b| b.items.iter().any(|i| i.name == item))
        };
        Ok(present != self.absent)
    }

    fn describe(&self) -> String {
        let (kind, s) = match (&self.register, &self.field) {
            (Some(r), _) => ("register", r.as_str()),
            (_, Some(f)) => ("field", f.as_str()),
            _ => ("?", ""),
        };
        format!("{}{kind} {s}", if self.absent { "no " } else { "" })
    }
}

/// The fieldsets a `<Fieldset>.<field>` name refers to: those of the block registers named after
/// it (`Water` -> register `water`), so chips whose register uses a differently named fieldset of
/// another width (a curated MetaPAC union) resolve to their own ones; else the fieldset of that
/// name. A field counts as present only if every one of them has it.
fn register_fieldsets<'a>(ir: &'a IR, name: &str) -> Vec<&'a chiptool::ir::FieldSet> {
    let reg = snake_case(name);
    let mut found: BTreeSet<&str> = BTreeSet::new();
    for block in ir.blocks.values() {
        for item in block.items.iter().filter(|i| i.name == reg) {
            if let chiptool::ir::BlockItemInner::Register(r) = &item.inner
                && let Some(fs) = &r.fieldset
            {
                found.insert(fs.as_str());
            }
        }
    }
    if found.is_empty() {
        return ir.fieldsets.get(name).into_iter().collect();
    }
    found
        .into_iter()
        .filter_map(|n| ir.fieldsets.get(n))
        .collect()
}

/// `MconfigExt` -> `mconfig_ext`.
fn snake_case(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// How one option cfg is obtained.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CfgRule {
    /// Presence predicate over the chip's PAC IR. Without it the cfg is import-only.
    pub derive: Option<Predicate>,
}

/// `data/features/<ip>.yaml`.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpCatalog {
    #[serde(default)]
    pub cfgs: BTreeMap<String, CfgRule>,
}

/// The whole catalog, by IP (module) name.
pub type Catalog = BTreeMap<String, IpCatalog>;

pub fn load_catalog(inputs: &Path) -> anyhow::Result<Catalog> {
    let dir = data_dir(inputs);
    let mut catalog = Catalog::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if path.extension().is_none_or(|e| e != "yaml") {
            continue;
        }
        let ip = path.file_stem().unwrap().to_string_lossy().into_owned();
        let text = fs::read_to_string(&path)?;
        let cat: Option<IpCatalog> =
            serde_yaml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        let cat = cat.unwrap_or_default();
        for cfg in cat.cfgs.keys() {
            if !cfg.starts_with(&format!("{ip}_has_")) {
                bail!(
                    "{}: cfg `{cfg}` must be named `{ip}_has_<x>`",
                    path.display()
                );
            }
        }
        catalog.insert(ip, cat);
    }
    Ok(catalog)
}

/// Every cfg name a chip can get: the IP presence cfgs and every option cfg. build.rs reads the
/// same names with `build/feature_names.rs`.
pub fn all_cfgs(catalog: &Catalog) -> BTreeSet<String> {
    catalog
        .iter()
        .flat_map(|(ip, cat)| std::iter::once(ip.clone()).chain(cat.cfgs.keys().cloned()))
        .collect()
}

/// The option cfgs without a `derive` predicate.
pub fn import_only(catalog: &Catalog) -> BTreeSet<String> {
    catalog
        .values()
        .flat_map(|cat| cat.cfgs.iter())
        .filter(|(_, rule)| rule.derive.is_none())
        .map(|(cfg, _)| cfg.clone())
        .collect()
}

/// The SDK macro of an option cfg: `FSL_FEATURE_` + the cfg upper-cased.
pub fn sdk_macro(cfg: &str) -> String {
    format!("FSL_FEATURE_{}", cfg.to_uppercase())
}

/// The evaluated features of one chip.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChipFeatures {
    pub cfgs: BTreeSet<String>,
}

/// Evaluates the catalog for one chip. `modules` holds the flat IR of every catalog IP the chip
/// has. Import-only cfgs come from `sdk` (the SDK value of each option cfg found in the chip's
/// features header) when given, else from `previous` (the import-only cfgs already exported).
/// Fails where a derived value and the SDK disagree.
pub fn evaluate(
    chip: &str,
    catalog: &Catalog,
    modules: &BTreeMap<String, IR>,
    sdk: Option<&BTreeMap<String, bool>>,
    previous: &BTreeSet<String>,
) -> anyhow::Result<ChipFeatures> {
    let mut out = ChipFeatures::default();
    let mut errors = Vec::new();

    for (ip, cat) in catalog {
        let Some(ir) = modules.get(ip) else {
            continue;
        };
        out.cfgs.insert(ip.clone());

        for (cfg, rule) in &cat.cfgs {
            let derived = rule.derive.as_ref().map(|p| p.eval(ir)).transpose()?;
            let imported = sdk.and_then(|s| s.get(cfg)).copied();
            let value = match (derived, imported) {
                (Some(d), Some(i)) if d != i => {
                    errors.push(format!(
                        "{chip}: {cfg}: PAC has {} ({d}), but {} = {}",
                        rule.derive.as_ref().unwrap().describe(),
                        sdk_macro(cfg),
                        i as u8
                    ));
                    d
                }
                (Some(d), _) => d,
                (None, Some(i)) => i,
                (None, None) if sdk.is_some() => false,
                (None, None) => previous.contains(cfg),
            };
            if value {
                out.cfgs.insert(cfg.clone());
            }
        }
    }

    if !errors.is_empty() {
        bail!(
            "IP features derived from the PAC disagree with the SDK features file:\n{}",
            errors.join("\n")
        );
    }
    Ok(out)
}

const TXT_HEADER: &str = "# Generated by the nxp-pac generator (see data/features/README.md); do not edit.\n\
                          # build.rs prints each line as `cargo::metadata=<line>`.\n";

/// The chip's `cargo::metadata` lines. The `cfgs=` line is printed even when empty: dependents
/// tell a chip with an export from one without by whether `DEP_NXP_PAC_CFGS` is set.
pub fn render_txt(f: &ChipFeatures) -> String {
    format!(
        "{TXT_HEADER}cfgs={}\n",
        f.cfgs.iter().cloned().collect::<Vec<_>>().join(",")
    )
}

/// `data/features/chips/<CORE>.yaml`: the import-only cfgs a chip sets. A chip without
/// import-only cfgs has no file.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChipFile {
    #[serde(default)]
    pub imported: BTreeSet<String>,
}

impl ChipFile {
    pub fn path(inputs: &Path, core: &str) -> PathBuf {
        data_dir(inputs)
            .join("chips")
            .join(core.to_uppercase())
            .with_extension("yaml")
    }

    pub fn load(path: &Path) -> anyhow::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let text = fs::read_to_string(path)?;
        let file: Option<Self> =
            serde_yaml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        Ok(Some(file.unwrap_or_default()))
    }

    pub fn render(&self) -> anyhow::Result<String> {
        Ok(format!(
            "# Generated by the nxp-pac generator (see ../README.md); do not edit.\n{}",
            serde_yaml::to_string(self)?
        ))
    }
}

/// The catalog IPs of a chip, loaded from the curated models.
pub struct MetapacModules {
    /// Flat IR per module name.
    pub modules: BTreeMap<String, IR>,
    pub input_files: Vec<PathBuf>,
}

/// The flat IR of every catalog IP a chip has, keyed by module name. Peripherals are selected
/// and their models prepared the same way `engine::generate` does.
pub fn metapac_modules(
    inputs: &Path,
    chip: &crate::chips::Chip,
    catalog: &Catalog,
) -> anyhow::Result<MetapacModules> {
    let core = chip.core_name();
    let models = crate::models::Catalog::load(inputs).context("loading model catalog")?;
    let (metadata, mut input_files) = crate::engine::metadata::load(inputs, chip)
        .with_context(|| format!("loading {} metadata", chip.feature))?;
    input_files.push(inputs.join("peripherals/models.json"));

    let mut paths: BTreeMap<String, String> = BTreeMap::new();
    for peripheral in &metadata.peripherals {
        let Some(block) = peripheral.parse_block_path()? else {
            continue;
        };
        if !catalog.contains_key(&block.rust_mod_name) {
            continue;
        }
        let entry = models.entries.get(&block.path).with_context(|| {
            format!(
                "{} {}: unknown model {}",
                chip.feature, peripheral.name, block.path
            )
        })?;
        if entry.source.is_none() {
            continue;
        }
        match paths.get(&block.rust_mod_name) {
            Some(path) if *path != block.path => bail!(
                "module `{}` maps to both {path} and {}",
                block.rust_mod_name,
                block.path
            ),
            _ => {
                paths.insert(block.rust_mod_name.clone(), block.path.clone());
            }
        }
    }

    let mut modules = BTreeMap::new();
    for (module, path) in paths {
        let source = models.entries[&path].source.as_deref().unwrap();
        let file = inputs
            .join("peripherals")
            .join(crate::engine::asset_path(source)?);
        let transforms = chip
            .transforms
            .iter()
            .filter(|t| t.model == path)
            .flat_map(|t| t.files.iter())
            .map(|name| {
                Ok(inputs
                    .join("transforms")
                    .join(crate::engine::asset_path(name)?))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let mut model = crate::engine::model::load(&inputs.join("peripherals"), &file, &core)
            .with_context(|| format!("{}: loading {path}", chip.feature))?;
        crate::engine::model::prepare(&mut model, &transforms)
            .with_context(|| format!("{}: preparing {path}", chip.feature))?;
        input_files.extend(model.input_files.iter().cloned());
        input_files.extend(transforms);
        modules.insert(module, model.ir);
    }
    Ok(MetapacModules {
        modules,
        input_files,
    })
}

/// The export of a chip.
pub struct Export {
    pub features: ChipFeatures,
    pub input_files: Vec<PathBuf>,
}

/// Evaluates a chip. Import-only cfgs come from its chip file, if it has one.
pub fn metapac_export(inputs: &Path, chip: &crate::chips::Chip) -> anyhow::Result<Export> {
    let core = chip.core_name();
    let chip_path = ChipFile::path(inputs, &core);
    let chip_file = ChipFile::load(&chip_path)?.unwrap_or_default();
    let catalog = load_catalog(inputs)?;
    let unknown: Vec<_> = chip_file
        .imported
        .difference(&import_only(&catalog))
        .cloned()
        .collect();
    if !unknown.is_empty() {
        bail!(
            "{}: not import-only cfgs of the catalog: {}",
            chip_path.display(),
            unknown.join(", ")
        );
    }
    let m = metapac_modules(inputs, chip, &catalog)?;
    let features = evaluate(&core, &catalog, &m.modules, None, &chip_file.imported)?;
    let mut input_files = m.input_files;
    if chip_path.exists() {
        input_files.push(chip_path);
    }
    input_files.push(data_dir(inputs));
    Ok(Export {
        features,
        input_files,
    })
}
