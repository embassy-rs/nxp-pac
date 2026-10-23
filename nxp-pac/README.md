# nxp-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for NXP microcontrollers.

MCXA MetaPAC devices are generated at build time from the curated YAML and chip
metadata packaged in `data/`, using Git-pinned
[chiptool](https://github.com/embassy-rs/chiptool/). Only the selected chip's required
peripherals are rendered into Cargo's `OUT_DIR`; generated Rust is not maintained
in the source tree. Curated YAML is authoritative and maintained directly.

Legacy PAC devices retain their checked-in generated sources, originally derived
from [NXP's mcux-soc-svd repo](https://github.com/nxp-mcuxpresso/mcux-soc-svd).

This crate will (hopefully) be used for [`embassy-nxp`](https://github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the NXP microcontrollers.

## Cloning

Select exactly one existing chip feature, for example `mcxa266` or `mcxa287`.
The `pac` feature is enabled by default; `metadata`, `rt`, and `defmt` retain their
existing roles. Metadata-only host builds are supported with default features
disabled. Building a MetaPAC does not require a vendor SVD checkout, external
formatter, or generator executable.

Only explicit SVD import and legacy source regeneration require a recursive clone.
Import is an onboarding tool, not an automatic update of curated models.

If you forgot this, you can use `git submodule update --checkout --init` to fetch the submodules.

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by NXP under a BSD-3-Clause license.
