use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, bail, ensure};
use serde_yaml::{Mapping, Value};

pub struct Source {
    pub definitions: Mapping,
    pub files: Vec<PathBuf>,
}

pub fn load(root: &Path, entry: &Path, chip: &str) -> anyhow::Result<Source> {
    let root = root
        .canonicalize()
        .with_context(|| format!("resolving peripheral root {}", root.display()))?;
    let mut source = Source {
        definitions: Mapping::new(),
        files: Vec::new(),
    };
    let mut origins = BTreeMap::new();
    read(&root, entry, &mut Vec::new(), &mut origins, &mut source)?;
    let mut patterns = BTreeMap::new();
    // Validate the complete union, including excluded definitions. Otherwise a
    // typo can remain hidden until somebody selects a less frequently used chip.
    for (id, value) in &source.definitions {
        let id = id.as_str().context("definition ID must be a string")?;
        let entries = kind_entries(id);
        annotations(value, entries, entries.is_some(), id, &mut patterns)
            .with_context(|| format!("validating {id} in {}", origins[id].display()))?;
    }
    let mut selected = Mapping::new();
    for (id, mut value) in source.definitions {
        if !select(&mut value, chip, &patterns)? {
            continue;
        }
        if let Some(entries) = kind_entries(id.as_str().unwrap())
            && let Some(entries) = value.get_mut(entries)
        {
            let entries = entries
                .as_sequence_mut()
                .context("definition entries must be a sequence")?;
            let mut filtered = Vec::new();
            for mut entry in std::mem::take(entries) {
                if select(&mut entry, chip, &patterns)? {
                    filtered.push(entry);
                }
            }
            *entries = filtered;
        }
        selected.insert(id, value);
    }
    source.definitions = selected;
    source.files.sort();
    source.files.dedup();
    Ok(source)
}

fn read(
    root: &Path,
    file: &Path,
    active: &mut Vec<PathBuf>,
    origins: &mut BTreeMap<String, PathBuf>,
    output: &mut Source,
) -> anyhow::Result<()> {
    let file = file
        .canonicalize()
        .with_context(|| format!("resolving include {}", file.display()))?;
    ensure!(
        file.starts_with(root),
        "include is outside peripheral root: {}",
        file.display()
    );
    ensure!(
        !active.contains(&file),
        "include cycle: {} -> {}",
        active
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(" -> "),
        file.display()
    );
    let yaml: Value = serde_yaml::from_slice(&fs::read(&file)?)
        .with_context(|| format!("parsing {}", file.display()))?;
    reject_tags(&yaml).with_context(|| format!("validating {}", file.display()))?;
    let mut definitions = yaml
        .as_mapping()
        .cloned()
        .context("peripheral YAML must be a mapping")?;
    active.push(file.clone());
    if let Some(includes) = definitions.remove(Value::String("includes".into())) {
        for include in includes
            .as_sequence()
            .context("includes must be a sequence")?
        {
            let include = include
                .as_str()
                .context("include must be a relative path string")?;
            ensure!(
                !include.is_empty()
                    && !include.contains(['\\', ':'])
                    && Path::new(include).components().all(|part| matches!(
                        part,
                        Component::Normal(_) | Component::CurDir | Component::ParentDir
                    )),
                "include must be a nonempty relative path: {include}"
            );
            let path = file
                .parent()
                .context("include has no parent")?
                .join(include);
            read(root, &path, active, origins, output)
                .with_context(|| format!("include {include} in {}", file.display()))?;
        }
    }
    for (id, definition) in definitions {
        let name = id.as_str().context("definition ID must be a string")?;
        let (kind, suffix) = name
            .split_once('/')
            .context("definition ID must have kind/name form")?;
        ensure!(
            matches!(kind, "block" | "fieldset" | "enum" | "device") && !suffix.is_empty(),
            "invalid definition ID: {name}"
        );
        ensure!(
            !origins.contains_key(name),
            "duplicate definition {name} in {} (first in {})",
            file.display(),
            origins
                .get(name)
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        );
        ensure!(
            definition.is_mapping(),
            "definition {name} in {} must be a mapping",
            file.display()
        );
        origins.insert(name.to_owned(), file.clone());
        output.definitions.insert(id, definition);
    }
    active.pop();
    output.files.push(file);
    Ok(())
}

fn kind_entries(id: &str) -> Option<&'static str> {
    match id.split_once('/').map(|(kind, _)| kind) {
        Some("block") => Some("items"),
        Some("fieldset") => Some("fields"),
        Some("enum") => Some("variants"),
        _ => None,
    }
}

// Mapping accessors transparently unwrap tags. Reject them before using those
// accessors, including at document roots and inside otherwise excluded entries.
// Chiptool's tagged transform syntax is parsed separately, not through this loader.
fn reject_tags(value: &Value) -> anyhow::Result<()> {
    match value {
        Value::Tagged(tagged) => bail!(
            "tagged YAML {} is not supported in curated peripheral sources",
            tagged.tag
        ),
        Value::Mapping(mapping) => {
            for (key, value) in mapping {
                reject_tags(key)?;
                reject_tags(value)?;
            }
        }
        Value::Sequence(values) => {
            for value in values {
                reject_tags(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn annotations(
    value: &Value,
    entries: Option<&str>,
    allowed: bool,
    location: &str,
    patterns: &mut BTreeMap<String, regex::Regex>,
) -> anyhow::Result<()> {
    match value {
        Value::Mapping(mapping) => {
            if let Some(pattern) = mapping.get(Value::String("only_in".into())) {
                ensure!(allowed, "only_in is not supported at {location}");
                let pattern = pattern
                    .as_str()
                    .with_context(|| format!("only_in must be a regex string at {location}"))?;
                if !patterns.contains_key(pattern) {
                    let regex = regex::Regex::new(pattern)
                        .with_context(|| format!("invalid only_in at {location}: {pattern}"))?;
                    patterns.insert(pattern.into(), regex);
                }
            }
            for (key, child) in mapping {
                let key = key.as_str().context("definition keys must be strings")?;
                if key == "only_in" {
                    continue;
                }
                let location = format!("{location}.{key}");
                if entries == Some(key) {
                    for (index, entry) in child
                        .as_sequence()
                        .context("definition entries must be a sequence")?
                        .iter()
                        .enumerate()
                    {
                        ensure!(
                            entry.is_mapping(),
                            "entry must be a mapping at {location}[{index}]"
                        );
                        annotations(entry, None, true, &format!("{location}[{index}]"), patterns)?;
                    }
                } else {
                    annotations(child, None, false, &location, patterns)?;
                }
            }
        }
        Value::Sequence(values) => {
            for value in values {
                annotations(value, None, false, location, patterns)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn select(
    value: &mut Value,
    chip: &str,
    patterns: &BTreeMap<String, regex::Regex>,
) -> anyhow::Result<bool> {
    let mapping = value
        .as_mapping_mut()
        .context("definition or entry must be a mapping")?;
    match mapping.remove(Value::String("only_in".into())) {
        Some(pattern) => {
            let pattern = pattern.as_str().context("invalid only_in")?;
            Ok(patterns
                .get(pattern)
                .with_context(|| format!("only_in regex was not validated: {pattern}"))?
                .is_match(chip))
        }
        None => Ok(true),
    }
}
