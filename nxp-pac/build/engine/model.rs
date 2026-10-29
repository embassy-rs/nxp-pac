use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail, ensure};
use serde::Deserialize;

pub struct Model {
    pub ir: chiptool::ir::IR,
    pub reset_values: BTreeMap<String, u64>,
    pub input_files: Vec<PathBuf>,
}

pub fn load(root: &Path, path: &Path, chip: &str) -> anyhow::Result<Model> {
    let source = super::source::load(root, path, chip)
        .with_context(|| format!("{chip}: composing {}", path.display()))?;
    let mut reset_values = BTreeMap::new();
    for (key, definition) in &source.definitions {
        if let Some(value) = definition.get("reset_value") {
            let name = key
                .as_str()
                .and_then(|key| key.strip_prefix("fieldset/"))
                .context("reset_value requires a fieldset")?;
            reset_values.insert(
                name.to_owned(),
                value.as_u64().context("invalid reset_value")?,
            );
        }
    }
    Ok(Model {
        ir: serde_yaml::from_value(serde_yaml::Value::Mapping(source.definitions))
            .with_context(|| format!("loading {}", path.display()))?,
        reset_values,
        input_files: source.files,
    })
}

pub fn prepare(model: &mut Model, paths: &[PathBuf]) -> anyhow::Result<()> {
    expand(model)?;
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Transforms {
        transforms: Vec<chiptool::transform::Transform>,
    }
    for path in paths {
        let data =
            fs::read(path).with_context(|| format!("reading transform {}", path.display()))?;
        let transforms: Transforms = serde_yaml::from_slice(&data)
            .with_context(|| format!("parsing transform {}", path.display()))?;
        for (index, transform) in transforms.transforms.iter().enumerate() {
            transform
                .run(&mut model.ir)
                .with_context(|| format!("transform {} in {}", index + 1, path.display()))?;
            expand(model)?;
        }
    }
    for name in model.reset_values.keys() {
        ensure!(
            model.ir.fieldsets.contains_key(name),
            "reset_value fieldset removed or renamed: {name}"
        );
    }
    chiptool::transform::sort::Sort {}.run(&mut model.ir)?;
    Ok(())
}

fn expand(model: &mut Model) -> anyhow::Result<()> {
    super::validate::validate_sort_safety(model)?;
    check_inheritance(
        model
            .ir
            .blocks
            .iter()
            .map(|(name, block)| (name.as_str(), block.extends.as_deref())),
    )?;
    check_inheritance(
        model
            .ir
            .fieldsets
            .iter()
            .map(|(name, fields)| (name.as_str(), fields.extends.as_deref())),
    )?;
    chiptool::transform::expand_extends::ExpandExtends {}.run(&mut model.ir)?;
    // Inheritance has been resolved. Keeping the links would reintroduce deleted
    // inherited fields when a later chip-specific transform runs.
    for block in model.ir.blocks.values_mut() {
        block.extends = None;
    }
    for fields in model.ir.fieldsets.values_mut() {
        fields.extends = None;
    }
    Ok(())
}

fn check_inheritance<'a>(
    items: impl Iterator<Item = (&'a str, Option<&'a str>)>,
) -> anyhow::Result<()> {
    let parents: BTreeMap<_, _> = items.collect();
    let mut done = BTreeSet::new();
    for name in parents.keys() {
        let mut visiting = BTreeSet::new();
        let mut current = *name;
        loop {
            if done.contains(current) {
                break;
            }
            ensure!(visiting.insert(current), "inheritance cycle at {current}");
            let Some(parent) = parents.get(current) else {
                bail!("missing inheritance parent {current}");
            };
            match parent {
                Some(parent) => current = parent,
                None => break,
            }
        }
        done.extend(visiting);
    }
    Ok(())
}
