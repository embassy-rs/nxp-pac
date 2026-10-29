use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::*;

use crate::{CHIPS, ChipDescription};

/// Generate legacy PAC sources and validate build-time MetaPAC generation.
#[derive(Parser)]
pub struct Generate {
    /// Select a single chip, or all chips if not provided.
    pub chip: Option<String>,
    /// Retain MetaPAC inspection output beneath this directory, grouped by chip.
    #[clap(long)]
    pub out_dir: Option<PathBuf>,
}

pub fn generate(args: Generate) -> anyhow::Result<()> {
    let chips = if let Some(chip_name) = args.chip {
        vec![crate::commands::select_chip(&chip_name)?]
    } else {
        CHIPS.iter().collect()
    };
    let current = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("repository root")?;
    let outputs: Vec<anyhow::Result<()>> = chips
        .par_iter()
        .map(|feature| generate_chip(current, feature, args.out_dir.as_deref()))
        .collect();
    let mut failed = false;
    for output in outputs {
        if let Err(error) = output {
            failed = true;
            error!("Error for output {error:#}");
        }
    }
    if failed {
        bail!("Failed to generate selected chips")
    }
    info!("Generate command completed successfully");
    Ok(())
}

fn generate_chip(
    current: &Path,
    feature: &ChipDescription,
    output: Option<&Path>,
) -> anyhow::Result<()> {
    for core in feature.cores {
        if feature.metapac {
            let temporary;
            let output = match output {
                Some(output) => output.join(core.to_lowercase()),
                None => {
                    temporary = temp_dir::TempDir::new()?;
                    temporary.path().to_owned()
                }
            };
            let inputs = current.join("nxp-pac/data");
            let generated = crate::metapac::generate(
                core,
                &inputs,
                &output,
                crate::metapac::Outputs {
                    pac: true,
                    metadata: true,
                    rt: true,
                },
            )
            .with_context(|| format!("Assembling MetaPAC for {core}"))?;
            crate::inventory::write(&inputs, &output)?;
            info!(
                "Validated {core}: {} inputs, {} generated artifacts",
                generated.input_files.len(),
                generated.output_files.len()
            );
        } else {
            let svd = current
                .join("data/mcux-soc-svd")
                .join(feature.chip)
                .join(core)
                .with_extension("xml");
            let chips_dir = current.join("nxp-pac/src/chips");
            let transforms_dir = current.join("data/transforms");
            crate::pac::generate_core(&svd, &chips_dir, &transforms_dir, core)
                .context("Generating PAC")?;
            if let Some(metadata) = feature.metadata {
                crate::metadata::generate(
                    &chips_dir,
                    &current
                        .join("data/metadata")
                        .join(metadata)
                        .with_extension("json"),
                    core,
                )
                .context("Generating metadata")?;
            }
        }
    }
    Ok(())
}
