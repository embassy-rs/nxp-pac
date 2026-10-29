# Curated MetaPAC peripherals

These YAML files are the authoritative register and type definitions for the
existing MetaPAC devices. They are maintained directly and reviewed, including
manual corrections and intentional unions. See the individual YAML comments
before changing those definitions.

## Source hierarchy and model identities

MCX-A sources live in `mcx/a/a1`, `mcx/a/a2`, `mcx/a/a3`, and `mcx/a/a5`.
Compatible definitions are shared from `mcx/a/common`; true subfamily-specific
layouts remain local.
Future families get their own directories when there are actual definitions to
add, not placeholder directories or implicit fallback to another family.

`models.json` declares three independent properties for each model:

- `id`: canonical identity, for example `mcx/a/a2/LPUART`.
- `source`: the entry YAML relative to this directory. A `null` source explicitly
  preserves an existing metadata-only mapping without a PAC implementation.
- `driver`: the base HAL driver identity. A2/A5 keep names such as `mcxa/LPUART`
  so existing HAL cfgs remain compatible; A1 uses its new `mcx/a/a1/...` identities.

Chip metadata selects the canonical `block` ID and optional `::type` suffix. That
suffix is also retained on the runtime driver name. `pac_only` still leaves the
runtime driver name empty. `META_PERIPHERALS` enumerates implemented driver
identities from the catalog, not physical YAML paths or include fragments.

None of these directories creates a Rust namespace: generated modules stay flat,
such as `nxp_pac::lpuart`, with existing peripheral instance constants unchanged.

## Includes and per-chip availability

An entry YAML can compose uniquely named IR definitions from relative includes:

```yaml
includes:
  - ../common/LPUART.yaml
```

Includes may be nested, but must stay within this packaged peripheral tree,
including after symlink resolution. Missing files, cycles, duplicate YAML keys,
and duplicate top-level definition IDs are errors. Definitions are never
deep-merged or silently overridden, even if a duplicate would later be excluded.

Prefer a curated union with `only_in` for additive variant availability:

```yaml
fieldset/Example:
  bit_size: 16
  fields:
    - name: supported_on_a1
      only_in: '^MCXA156$'
      bit_offset: 0
      bit_size: 1
```

`only_in` is a regex matched against the canonical uppercase hardware core name.
The `mcxa276` feature selects the MCXA266 hardware model, so its predicates match
`MCXA266`; no separate sample-specific register model is maintained. The runtime
metadata name still identifies the requested compatibility feature as `MCXA276`.
The annotation is supported on block/fieldset/enum definitions and entries in
their items, fields, and variants. An absent annotation means unconditional
availability. Validate all annotations, including excluded definitions, before filtering. Disjoint
conditional entries can retain variant-specific documentation; after filtering,
the selected model must have valid references and unambiguous definitions.

The order is include composition, availability filtering, IR parsing/inheritance,
ordered transforms, final validation, and rendering. Do not use transforms merely
to remove optional fields that `only_in` can describe. Use separate definitions
or transforms for genuine incompatible layouts or meanings, with source evidence.

## GPIO capabilities

All three subfamilies include the same GPIO union. The documented split is basic
A1/A2 GPIO versus extended A5 GPIO, not A1 versus A2/A5:

| Selection | GPIO capabilities |
| --- | --- |
| MCXA156, MCXA175/176/185/186/255/256/265/266 | One ISFR bank; ICR[23:20] reserved; no documented protection-register bank |
| MCXA286/287/456/457/536/537/556/557/566/567/577 | Two ISFR banks, ICR.IRQS/LK, and LOCK/PCNS/ICNS/PCNP/ICNP |
| MCXA276 compatibility selection | Same basic GPIO and complete hardware model as MCXA266; retained for existing boards |

References: MCXAP100M96FS6RM Rev. 7, chapter 12; MCXAP144M240F61RM Rev. 2,
sections 12.7.1.1, 12.7.1.12 and 12.7.1.15; MCXAP144M240F61SRM Rev. 2,
section 2.5.9; MCXAP172M240F70RM final Rev. 1, sections 20.7.1.1, 20.7.1.17
and 20.7.1.20. The A2 security reference manual redirects GPIO coverage to the
main reference manual; it does not establish the A5 protection extensions.

The source migration intentionally corrects the old A2/A5 superset for the
documented A2 devices: unsupported protection/routing/lock accessors disappear,
and `isfr(1)` is rejected on single-bank devices. The flat module, instance names,
and indexed `isfr(n)` shape remain unchanged. MCXA276 is resolved to MCXA266 in the
chip registry, rather than handled as an extended-GPIO exception. PDR register
and fieldset storage remain explicitly 8-bit.

## WAKETIMER compatibility

All supported MCXA1/2/3/5 devices share `common/WAKETIMER.yaml`, with the flat
`waketimer` module, `Waketimer` type, and `WAKETIMER0` instance. Every selection
advertises runtime driver identity `mcxa/WAKETIMER::Waketimer`; the catalog-wide
`META_PERIPHERALS` lists its base identity `mcxa/WAKETIMER`. This replaces the
former family-specific identities, A2 metadata-only `mcxa/WAKETIMER0`, and A1
`pac_only` exceptions. MCXA276 uses the same implementation as MCXA266.

The timer is a one-shot down counter. Its status flag is write-one-to-clear, and
counter reads require two consecutive matching values because of clock domain
crossing. Stop the counter before reloading it. Source-clock setup remains the
caller's responsibility: `OSC_DIV_ENA` alone does not enable the oscillator's
output clock. The timer chapter's rounded "1 kHz" wording must not be interpreted
as exactly one millisecond per tick:
the clock chapters specify a nominal 16.384 kHz source and the timer divides it
by 16. This PAC exposes raw counts, not a time conversion or a HAL driver.

## Build pipeline

Selecting a chip feature compiles its PAC from packaged metadata and the required
curated YAML, using the internal `build/engine` modules. Models are fresh for each
selected chip, local inheritance is expanded, configured curated-model transforms
are applied, and register/storage widths and references are validated before Rust
is written to `OUT_DIR`. Every included input is tracked for Cargo rebuilds.
No SVD, vendor update, external formatter, or pre-generation
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
