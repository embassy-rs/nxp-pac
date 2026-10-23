# Curated MetaPAC peripherals

These YAML files are the authoritative register and type definitions for the
existing MetaPAC devices. They are maintained directly and reviewed, including
manual corrections and intentional unions. See individual comments and the
MCXA156 correction notes before changing existing definitions.

## Build pipeline

Selecting a chip feature compiles its PAC from packaged metadata and the required
curated YAML, using the internal `build/engine` modules. Models are fresh for each
selected chip, local inheritance is expanded, configured curated-model transforms
are applied, and register/storage widths and references are validated before Rust
is written to `OUT_DIR`. No SVD, vendor update, external formatter, or pre-generation
step is involved.

Optional `reset_value` annotations customize generated fieldset defaults. Narrow
fieldset storage widths must be explicit: inheritance copies fields, not bit_size.

## Importing a new family

SVD extraction is a dedicated onboarding pipeline, ideally used once per family.
From the repository root, `cargo run -p generator -- extract <chip>` places raw
output under `data/metadata/peripherals/raw`. Review that output and incorporate
appropriate definitions deliberately; do not copy it automatically over curated
models. Extraction transforms in root `data/transforms` are not build transforms.

After onboarding, edit the curated YAML here and metadata in `../metadata`
directly. Compile the selected chip to verify it. For inspection only,
`cargo run -p generator -- generate <chip> --out-dir <scratch-directory>` calls
the same engine and retains generated files beneath a chip-named subdirectory.
Do not commit generated MetaPAC Rust.
