pub mod metadata;
pub mod model;
mod render;
mod source;
pub mod validate;

use anyhow::{Context, ensure};
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

pub struct Outputs {
    pub pac: bool,
    pub metadata: bool,
    pub rt: bool,
}

pub struct Generation {
    pub input_files: Vec<PathBuf>,
    pub output_files: Vec<PathBuf>,
}

pub fn asset_path(name: &str) -> anyhow::Result<&Path> {
    let path = Path::new(name);
    ensure!(
        !name.is_empty()
            && !name.contains('\\')
            && path.components().all(|c| matches!(c, Component::Normal(_))),
        "invalid asset path: {name}"
    );
    Ok(path)
}

pub fn generate(
    chip: &str,
    inputs: &Path,
    out_dir: &Path,
    outputs: Outputs,
) -> anyhow::Result<Generation> {
    let chip =
        super::chips::find(chip).with_context(|| format!("unsupported MetaPAC chip: {chip}"))?;
    let core = chip.core_name();
    let catalog = super::models::Catalog::load(inputs).context("loading model catalog")?;
    let (mut metadata, mut input_files) = metadata::load(inputs, chip)
        .with_context(|| format!("loading {} metadata", chip.feature))?;
    input_files.push(inputs.join("peripherals/models.json"));
    for peripheral in &mut metadata.peripherals {
        let Some(block) = peripheral.parse_block_path()? else {
            continue;
        };
        let entry = catalog.entries.get(&block.path).with_context(|| {
            format!(
                "{} {}: unknown model {}",
                chip.feature, peripheral.name, block.path
            )
        })?;
        let suffix = peripheral
            .peripheral_block
            .as_deref()
            .and_then(|block| block.split_once("::"));
        peripheral.driver_name = Some(match suffix {
            Some((_, ty)) => format!("{}::{ty}", entry.driver),
            None => entry.driver.clone(),
        });
    }
    let mut pac_metadata = metadata.clone();
    let mut models = BTreeMap::new();
    let mut selected = Vec::new();
    for peripheral in pac_metadata.peripherals {
        let Some(block) = peripheral.parse_block_path()? else {
            continue;
        };
        let entry = &catalog.entries[&block.path];
        // Explicit null source preserves an existing metadata-only mapping.
        // Missing files for implemented models are never silently ignored.
        let Some(source) = &entry.source else {
            continue;
        };
        let path = inputs.join("peripherals").join(asset_path(source)?);
        if !models.contains_key(&block.path) {
            let transforms = chip
                .transforms
                .iter()
                .filter(|t| t.model == block.path)
                .flat_map(|t| t.files.iter())
                .map(|name| Ok(inputs.join("transforms").join(asset_path(name)?)))
                .collect::<anyhow::Result<Vec<_>>>()?;
            let mut model = model::load(&inputs.join("peripherals"), &path, &core)
                .with_context(|| format!("{}: loading {}", chip.feature, block.path))?;
            model::prepare(&mut model, &transforms)
                .with_context(|| format!("{}: preparing {}", chip.feature, block.path))?;
            validate::validate(&model)
                .with_context(|| format!("{}: validating {}", chip.feature, block.path))?;
            ensure!(
                model.ir.blocks.contains_key(&block.rust_type_name),
                "{} {}: missing block {} in {}",
                chip.feature,
                peripheral.name,
                block.rust_type_name,
                block.path
            );
            input_files.extend(model.input_files.iter().cloned());
            input_files.extend(transforms);
            models.insert(block.path.clone(), model);
        }
        ensure!(
            models[&block.path]
                .ir
                .blocks
                .contains_key(&block.rust_type_name),
            "{} {}: missing block {}",
            chip.feature,
            peripheral.name,
            block.rust_type_name
        );
        selected.push(peripheral);
    }
    pac_metadata.peripherals = selected;
    // Do not publish a new entry point until every required model has passed.
    fs::create_dir_all(out_dir)?;
    let mut output_files = render::write_device(&pac_metadata, &models, out_dir, &outputs)?;
    if outputs.metadata {
        let path = out_dir.join("metadata.rs");
        fs::write(&path, metadata::render(&core, &metadata)?.to_string())?;
        output_files.push(path);
    }
    input_files.sort();
    input_files.dedup();
    Ok(Generation {
        input_files,
        output_files,
    })
}
