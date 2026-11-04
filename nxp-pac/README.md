# nxp-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for NXP microcontrollers.

MCXA and MCXN MetaPAC devices are generated at build time from the curated YAML and chip
metadata packaged in `data/`, using Git-pinned
[chiptool](https://github.com/embassy-rs/chiptool/). Only the selected chip's required
peripherals are rendered into Cargo's `OUT_DIR`; generated Rust is not maintained
in the source tree. Curated YAML is authoritative and maintained directly.

Legacy PAC devices retain their checked-in generated sources, originally derived
from [NXP's mcux-soc-svd repo](https://github.com/nxp-mcuxpresso/mcux-soc-svd).

This crate will (hopefully) be used for [`embassy-nxp`](https://github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the NXP microcontrollers.

## Cloning

Select exactly one chip feature, for example `mcxa266`, `mcxa287`, or `mcxn235`.
The `pac` feature is enabled by default; `metadata`, `rt`, and `defmt` retain their
existing roles. Metadata-only host builds are supported with default features
disabled. Building a MetaPAC does not require a vendor SVD checkout, external
formatter, or generator executable.

MCXN MetaPAC chip features are `mcxn235`, `mcxn236`, `mcxn247`, `mcxn526`,
`mcxn527`, `mcxn536`, `mcxn537`, `mcxn546`, `mcxn547`, `mcxn946`, and
`mcxn947`. They each select one chip PAC; the legacy `mcxn947_cm33_core0` and
`mcxn947_cm33_core1` features remain separate legacy PAC selections. For MCXN947,
both CPU firmware images can use the single `mcxn947` PAC with the appropriate
target/runtime ABI. Core peripherals such as SysTick are supplied by the Cortex-M
runtime ecosystem, not modeled as device peripherals here. Do not infer the same
FPU or MPU features for both CPUs from a shared chip PAC.

These are raw register-access bindings, not a HAL or safety/security certification.
Factory trim, behavioral requirements, and errata handling remain the caller's
responsibility. NPU controls are opaque in the available public register
documentation; this does not imply a public NPU register model.

Only explicit SVD import and legacy source regeneration require a recursive clone.
Import is an onboarding tool, not an automatic update of curated models.

If you forgot this, you can use `git submodule update --checkout --init` to fetch the submodules.

## Metadata connectivity and reset fields

With `metadata` enabled, `Peripheral::direct_signals()` selects records from
`metadata::DIRECT_SIGNALS` for that peripheral on the selected chip.
`DirectSignalKind::Analog` and `DirectSignalKind::DirectInput` describe non-PORT-mux
connectivity, not pin initialization recipes. These routes have no digital ALT
number; existing `Peripheral.signals` and `SignalPin.alt` remain digital mux routes.
Pins may include dedicated analog/PHY pads as well as GPIO pads; every route
references a declared pin. Connectivity does not imply all-package bonding.
JSON represents the routes as optional `direct_signals` entries with `name` and
`pins: [{"pin": "P1_8", "kind": "analog"}]` (or `"direct_input"`).

`Gate.bit` remains the clock-enable field and `Gate.reset` names the reset
register. When `reset` is present, use `Gate::reset_bit()` for its reset field.
It resolves explicit JSON `gate.reset_bit` overrides through `metadata::RESET_GATES`
using `(enable, bit, reset)`, or falls back to `Gate.bit`. An omitted JSON `bit`
still defaults to the lowercased peripheral name. Empty or orphan reset overrides
and conflicting shared-key reset fields are rejected. These lookups perform no
hardware access. Existing public struct layouts and legacy initializers are
unchanged; legacy devices have empty extension tables and retain the fallback.
MetaPAC metadata builds emit the tables in a separate `metadata_ext.rs` artifact.

`Peripheral::named_gates()` selects `NamedGate` records from
`metadata::NAMED_GATES`. Each record has a peripheral owner, a unique control
name for that peripheral, and a `Gate` using the same reset-field lookup and
fallback rules as the existing primary gate. JSON uses optional `named_gates:
[{"name": "processor", "gate": {"enable": "ahbclkctrl3", "bit": "coolflux",
"reset": "presetctrl3", "reset_bit": "coolflux_rst"}}]`; for example, BSP32 has
separate `processor` and `apb` controls. These records are information, not an
initialization sequence or ordering requirement. They do not infer or replace
`Peripheral.gate`. Reset-key consistency is checked across primary and named
controls together. Legacy and current MCXA metadata have no named controls.

The documented pin labels `NVM_TM0` through `NVM_TM3` have no established owning
public peripheral association or purpose in the supplied sources. They are not
assigned invented FMU/test ownership; this is a qualified connectivity coverage
limit, separate from the represented analog/ADC routes.

Do not consume an inspection output directory from a failed generator invocation.

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by NXP under a BSD-3-Clause license.
