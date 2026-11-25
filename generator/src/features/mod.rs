//! Per-chip IP features, exported to dependent crates through Cargo `links` metadata.
//!
//! The catalog, its evaluation and the rendering live in `nxp-pac/build/features.rs`, shared with
//! `nxp-pac/build.rs`. This module adds what only `generate` does: with `--sdk-features`, read the
//! chip's SDK `<SoC>_features.h`, fail where a derived option disagrees with it, and write the
//! chip file `nxp-pac/data/features/chips/<CORE>.yaml` with the import-only options the chip
//! sets.
//!
//! The list of every cfg name (`all_cfgs`) is computed by build.rs from the catalog.

pub mod sdk;

#[path = "../../../nxp-pac/build/features.rs"]
mod shared;

#[cfg(test)]
#[path = "../../../nxp-pac/build/feature_names.rs"]
mod feature_names;

pub use shared::*;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};

/// Feature options of `generate`.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// The chip's SDK `<SoC>_features.h`.
    pub sdk_features: Option<PathBuf>,
}

/// `nxp-pac/data`.
pub fn inputs_dir(root: &Path) -> PathBuf {
    root.join("nxp-pac").join("data")
}

/// A macro value read from an SDK features header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    /// Per-instance values (`*`: every instance not listed).
    PerInstance(BTreeMap<String, i64>),
}

impl Value {
    /// The value if it is the same for every instance.
    fn uniform(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            Value::PerInstance(m) => {
                let mut values = m.values();
                let first = *values.next()?;
                values.all(|v| *v == first).then_some(first)
            }
        }
    }
}

/// The catalog's macros found in an SDK features header.
pub fn read_sdk_features(
    path: &Path,
    catalog: &Catalog,
) -> anyhow::Result<BTreeMap<String, Value>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let prefixes: Vec<String> = catalog
        .keys()
        .map(|ip| format!("FSL_FEATURE_{}_", ip.to_uppercase()))
        .collect();
    sdk::parse_features(&text, &prefixes)
}

/// The SDK value of every option cfg whose macro the header defines, by cfg name. Fails on a
/// value that isn't 0/1 or differs between instances.
pub fn sdk_options(
    chip: &str,
    catalog: &Catalog,
    values: &BTreeMap<String, Value>,
) -> anyhow::Result<BTreeMap<String, bool>> {
    let mut out = BTreeMap::new();
    for cfg in catalog.values().flat_map(|cat| cat.cfgs.keys()) {
        let name = sdk_macro(cfg);
        let Some(v) = values.get(&name) else {
            continue;
        };
        let value = match v.uniform() {
            Some(0) => false,
            Some(1) => true,
            _ => bail!("{chip}: {name} is {v:?}, not a 0/1 value the same for every instance"),
        };
        out.insert(cfg.clone(), value);
    }
    Ok(out)
}

/// Writes a core's chip file with the import-only cfgs of `f`; a core without any has no file.
fn write_chip_file(
    inputs: &Path,
    core: &str,
    catalog: &Catalog,
    f: &ChipFeatures,
) -> anyhow::Result<()> {
    let chip_file = ChipFile {
        imported: f
            .cfgs
            .intersection(&import_only(catalog))
            .cloned()
            .collect(),
    };
    let path = ChipFile::path(inputs, core);
    if chip_file.imported.is_empty() {
        if path.exists() {
            fs::remove_file(&path)?;
        }
        return Ok(());
    }
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, chip_file.render()?)?;
    Ok(())
}

/// `generate` hook for a MetaPAC chip given `--sdk-features`: checks the derived options against
/// the SDK header and writes the chip file.
pub fn generate_metapac(root: &Path, core: &str, opts: &Options) -> anyhow::Result<()> {
    let Some(sdk_path) = opts.sdk_features.as_deref() else {
        return Ok(());
    };
    let inputs = inputs_dir(root);
    let chip = crate::chips::find(&core.to_lowercase())
        .with_context(|| format!("unsupported MetaPAC chip: {core}"))?;
    let catalog = load_catalog(&inputs)?;
    let m = metapac_modules(&inputs, chip, &catalog)?;
    let sdk = sdk_options(core, &catalog, &read_sdk_features(sdk_path, &catalog)?)?;
    let f = evaluate(core, &catalog, &m.modules, Some(&sdk), &BTreeSet::new())?;
    write_chip_file(&inputs, &chip.core_name(), &catalog, &f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chiptool::ir::IR;

    const IR_YAML: &str = "\
block/Lpuart:
  items:
  - name: ctrl
    byte_offset: 24
    fieldset: Ctrl
  - name: water
    byte_offset: 44
    fieldset: Water
fieldset/Ctrl:
  fields:
  - name: swap
    bit_offset: 0
    bit_size: 1
fieldset/Water:
  fields:
  - name: rxwater
    bit_offset: 16
    bit_size: 2
";

    const LPUART_CATALOG: &str = "\
# comment
cfgs:
  lpuart_has_ctrl_swap:
    derive: { field: Ctrl.swap }
  # comment
  lpuart_has_no_water:
    derive: { register: Lpuart.water, absent: true }
  lpuart_has_errata_1: {}
";

    fn catalog() -> Catalog {
        let cat: IpCatalog = serde_yaml::from_str(LPUART_CATALOG).unwrap();
        Catalog::from([
            ("lpuart".to_string(), cat),
            ("lpspi".to_string(), IpCatalog::default()),
        ])
    }

    fn modules(yaml: &str) -> BTreeMap<String, IR> {
        BTreeMap::from([("lpuart".to_string(), serde_yaml::from_str(yaml).unwrap())])
    }

    fn sdk(pairs: &[(&str, Value)]) -> BTreeMap<String, bool> {
        let values = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        sdk_options("X", &catalog(), &values).unwrap()
    }

    #[test]
    fn derives_imports_and_renders() {
        let s = sdk(&[
            ("FSL_FEATURE_LPUART_HAS_CTRL_SWAP", Value::Int(1)),
            (
                "FSL_FEATURE_LPUART_HAS_ERRATA_1",
                Value::PerInstance(BTreeMap::from([
                    ("LPUART0".into(), 1),
                    ("LPUART1".into(), 1),
                ])),
            ),
        ]);
        let f = evaluate(
            "X",
            &catalog(),
            &modules(IR_YAML),
            Some(&s),
            &BTreeSet::new(),
        )
        .unwrap();
        assert_eq!(
            f.cfgs.iter().map(String::as_str).collect::<Vec<_>>(),
            ["lpuart", "lpuart_has_ctrl_swap", "lpuart_has_errata_1"]
        );
        assert_eq!(
            render_txt(&f).lines().skip(2).collect::<Vec<_>>(),
            ["cfgs=lpuart,lpuart_has_ctrl_swap,lpuart_has_errata_1"]
        );
        assert_eq!(
            all_cfgs(&catalog()).into_iter().collect::<Vec<_>>(),
            [
                "lpspi",
                "lpuart",
                "lpuart_has_ctrl_swap",
                "lpuart_has_errata_1",
                "lpuart_has_no_water"
            ]
        );
        assert_eq!(
            import_only(&catalog()),
            BTreeSet::from(["lpuart_has_errata_1".to_string()])
        );
    }

    #[test]
    fn import_only_cfgs_kept_without_sdk_file() {
        let previous = BTreeSet::from(["lpuart_has_errata_1".to_string(), "stale".to_string()]);
        let f = evaluate("X", &catalog(), &modules(IR_YAML), None, &previous).unwrap();
        assert!(f.cfgs.contains("lpuart_has_errata_1"));
        assert!(f.cfgs.contains("lpuart_has_ctrl_swap"));
        assert!(!f.cfgs.contains("stale"));
        // With an SDK file the previous value no longer counts.
        let f = evaluate(
            "X",
            &catalog(),
            &modules(IR_YAML),
            Some(&sdk(&[])),
            &previous,
        )
        .unwrap();
        assert!(!f.cfgs.contains("lpuart_has_errata_1"));
    }

    #[test]
    fn absent_predicate_and_missing_owner() {
        let yaml = IR_YAML.replace(
            "  - name: water\n    byte_offset: 44\n    fieldset: Water\n",
            "",
        );
        let f = evaluate("X", &catalog(), &modules(&yaml), None, &BTreeSet::new()).unwrap();
        assert!(f.cfgs.contains("lpuart_has_no_water"));
        let p = Predicate {
            field: Some("Nope.x".into()),
            ..Default::default()
        };
        assert!(!p.eval(&serde_yaml::from_str(IR_YAML).unwrap()).unwrap());
    }

    #[test]
    fn derived_vs_sdk_mismatch_fails() {
        let s = sdk(&[("FSL_FEATURE_LPUART_HAS_CTRL_SWAP", Value::Int(0))]);
        let e = evaluate(
            "X",
            &catalog(),
            &modules(IR_YAML),
            Some(&s),
            &BTreeSet::new(),
        )
        .unwrap_err();
        assert!(format!("{e:#}").contains(
            "lpuart_has_ctrl_swap: PAC has field Ctrl.swap (true), but FSL_FEATURE_LPUART_HAS_CTRL_SWAP = 0"
        ));
    }

    #[test]
    fn non_uniform_or_non_boolean_sdk_value_fails() {
        let values = BTreeMap::from([(
            "FSL_FEATURE_LPUART_HAS_ERRATA_1".to_string(),
            Value::PerInstance(BTreeMap::from([
                ("LPUART0".into(), 1),
                ("LPUART1".into(), 0),
            ])),
        )]);
        assert!(sdk_options("X", &catalog(), &values).is_err());
        let values =
            BTreeMap::from([("FSL_FEATURE_LPUART_HAS_ERRATA_1".to_string(), Value::Int(2))]);
        assert!(sdk_options("X", &catalog(), &values).is_err());
        // Macros of the IP that are not option cfgs are ignored.
        let values = BTreeMap::from([("FSL_FEATURE_LPUART_FIFO_SIZEn".to_string(), Value::Int(8))]);
        assert!(sdk_options("X", &catalog(), &values).unwrap().is_empty());
    }

    #[test]
    fn absent_ip_has_no_cfgs() {
        let f = evaluate("X", &catalog(), &BTreeMap::new(), None, &BTreeSet::new()).unwrap();
        assert!(f.cfgs.is_empty());
    }

    #[test]
    fn cfg_names_scan_matches_parser() {
        let mut names = feature_names::cfg_names("lpuart", LPUART_CATALOG).unwrap();
        names.extend(feature_names::cfg_names("lpspi", "# no options\n").unwrap());
        names.sort();
        assert_eq!(names, all_cfgs(&catalog()).into_iter().collect::<Vec<_>>());
        assert_eq!(feature_names::cfg_names("x", "cfgs: {}\n").unwrap(), ["x"]);
        for bad in [
            "cfgs:\n  lpuart_has_a: {}\n",
            "cfgs: { x_has_a: {} }\n",
            "cfgs:\n  - x_has_a\n",
            "cfgs:\n    x_has_a: {}\n  x_has_b: {}\n",
        ] {
            assert!(feature_names::cfg_names("x", bad).is_err(), "{bad}");
        }
    }

    /// build.rs reads the catalog's cfg names without a YAML parser: they must match the parser.
    #[test]
    fn cfg_names_scan_matches_catalog() {
        let inputs = inputs_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap());
        let scanned = feature_names::all_cfg_names(&data_dir(&inputs)).unwrap();
        assert_eq!(scanned, all_cfgs(&load_catalog(&inputs).unwrap()));
    }

    /// Dependents tell "the chip has an export" from "no export" by whether `DEP_NXP_PAC_CFGS`
    /// is set, so a chip without any catalog IP still prints the `cfgs=` line.
    #[test]
    fn empty_export_prints_cfgs_line() {
        let txt = render_txt(&ChipFeatures::default());
        assert!(txt.lines().any(|l| l == "cfgs="), "{txt}");
    }

    #[test]
    fn chip_file_round_trip() {
        let file = ChipFile {
            imported: BTreeSet::from(["i3c_has_errata_051617".to_string()]),
        };
        let back: ChipFile = serde_yaml::from_str(&file.render().unwrap()).unwrap();
        assert_eq!(back.imported, file.imported);
        assert!(serde_yaml::from_str::<ChipFile>("other: 1").is_err());
    }
}
