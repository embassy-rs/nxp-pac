//! The cfg names of the IP-feature catalog (`data/features/<ip>.yaml`), for every chip.
//!
//! Read without a YAML parser, so PAC-only builds get `DEP_NXP_PAC_ALL_CFGS` without the
//! `_metapac` build dependencies: the file stem is the IP presence cfg, and each key of the
//! top-level block mapping `cfgs:` is an option cfg. The generator tests the result against the
//! full catalog parser in `build/features.rs`.

use std::{collections::BTreeSet, fs, path::Path};

/// Every cfg name a chip can get: the IP presence cfgs and every option cfg of `dir`
/// (`data/features`).
pub fn all_cfg_names(dir: &Path) -> Result<BTreeSet<String>, String> {
    let mut names = BTreeSet::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|e| e != "yaml") {
            continue;
        }
        let ip = path.file_stem().unwrap().to_string_lossy().into_owned();
        let text =
            fs::read_to_string(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
        names.extend(cfg_names(&ip, &text).map_err(|e| format!("{}: {e}", path.display()))?);
    }
    Ok(names)
}

/// The IP presence cfg `ip` and the option cfgs of one catalog file.
pub fn cfg_names(ip: &str, text: &str) -> Result<Vec<String>, String> {
    let mut names = vec![ip.to_string()];
    let mut in_cfgs = false;
    let mut key_indent = None;
    for line in text.lines() {
        let line = line.trim_end();
        let content = line.trim_start();
        if content.is_empty() || content.starts_with('#') {
            continue;
        }
        if line.starts_with('\t') {
            return Err("tab indentation".into());
        }
        let indent = line.len() - content.len();
        if indent == 0 {
            in_cfgs = false;
            if let Some(rest) = content.strip_prefix("cfgs:") {
                match rest.trim() {
                    "" => in_cfgs = true,
                    "{}" => {}
                    _ => return Err("`cfgs:` must be a block mapping".into()),
                }
            }
            continue;
        }
        if !in_cfgs {
            continue;
        }
        let key_indent = *key_indent.get_or_insert(indent);
        if indent < key_indent {
            return Err(format!("unexpected indentation: `{line}`"));
        }
        if indent > key_indent {
            continue;
        }
        let key = content
            .split_once(':')
            .map(|(key, _)| key.trim())
            .filter(|key| !key.is_empty() && !key.starts_with('-'))
            .ok_or_else(|| format!("expected `<cfg>:` in `cfgs`: `{line}`"))?;
        if !key.starts_with(&format!("{ip}_has_")) {
            return Err(format!("cfg `{key}` must be named `{ip}_has_<x>`"));
        }
        names.push(key.to_string());
    }
    Ok(names)
}
