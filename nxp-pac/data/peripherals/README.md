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

## Additional A2/A5 register coverage

A2 now has register models for AOI, CMP, DBGMAILBOX, EIM, ERM, FMC, GLIKEY,
OPAMP, SMARTDMA, and UTICK. A5 adds AOI, CMP, DBGMAILBOX, EIM, ERM, EWM, FMC,
GLIKEY, SMARTDMA, UTICK, and VREF. These additions fill
gaps in A2/A5; they do not restrict or replace existing A1/A3 models. A2 retains
its existing `FMC0`, `OPAMP0`, and `UTICK0` catalog and driver identities.

The definitions follow MCXAP144M240F61RM Rev. 2 (sections 7.6.1, 17.7, 28.6,
48.5.1, and 53.5.2.1), MCXAP144M240F61SRM Rev. 2 (section 10.5.1), and
MCXAP172M240F70RM final Rev. 1 (sections 13.6.1, 25.7, 35.6, 41.5.1, 58.6.1,
66.4, and 67.5.1).

EIM, ERM, SMARTDMA, and CMP follow F61RM sections 8.6, 9.5, 42.4, and 46.7,
and F70RM sections 15.3, 16.5, 49.4, and 56.7. The manuals take precedence
over conflicting XML fields and generic-IP descriptions. Important distinctions:

- AOI uses 16-bit accesses; EWM uses 8-bit accesses, including backing storage.
- GLIKEY's VERSION offset is `0xFC` on A2 and `0xFFC` on A5. Its write-enable
  fields contain FSM sequence values, not ordinary enable/disable switches.
- Only A2 has the FMC flash-cache programming bank. REMAP writes require command
  keys in the lower halfword; setting a field with read-modify-write is not enough.
- DBGMAILBOX models core-side access: REQUEST is read-only and ID reads zero,
  unlike the external debugger's AP identification value of `0x002A0000`.
- VREF UTRIM is factory-loaded. Its generated default includes only known reset
  bits, not a device's factory trim. Preserve VREFTRIM when changing TRIM2V1.
  The manual disagrees on TRIM2V1 encoding 1's nominal voltage (Table 671 says
  1.25 V; section 58.6.1.4 says 1.2 V), so no voltage-named enum is provided.
- EIM exposes only the channel documented by its register map. ERM has two
  status/count channels, but only channel 0 has address and syndrome registers.
- A2 CMP1 has a four-bit external-trigger selector omitted by the XML. It uses
  a separate `CMP1` model with the same `mcxa/CMP` driver identity. CMP0's trigger
  is fixed. A5 also reserves RRCR0 bit 1 despite the XML exposing it.
- Comparator input 5 is reserved on A2 but connected on A5. On A5, zero filter
  count with sampling enabled is illegal and drives COUT low; A2 bypasses its
  filter at zero count. Window events follow COUT on A2 and CMPO on A5.
- SMARTDMA control writes require WKEY `0xC0DE` and AHB synchronization. IE=3
  is reserved on A2, but selects RXEV on A5; do not assume ordinary enable bits.

New A2/A5 UDF extraction is deferred pending NXP clarification. The local manuals
do not describe its registers, and the A5 XML assigns overlapping, unaligned
offsets to 32-bit UDF registers. Existing A3 definitions are unchanged; XML
presence alone is not confirmation of customer-visible hardware support.

A2 additionally models TDET0, EQDC, SLCD, MBC, and PKC0; A5 models TDET,
FLEXIO, MBC, PKC, ITRC, GDET, SECCON, and USBDCD. These follow the same F61
and F70 manuals, including the F61 security reference manual. EQDC accesses
are 16-bit, and SLCD preserves its overlapping byte/word waveform views.
PKC exposes each family's documented map: the A5 map does not establish
MCDATA, VERSION, or MODULE_ID registers. Reserved or undocumented addresses
must not be probed to infer additional coverage.

Protection models describe access and reset restrictions; they do not implement
unlock sequences or enforce per-field permissions. Contradictory TDET reset
descriptions, lower-A2 TDET routing, A5 NSE semantics, unknown SECCON reset
values, and ambiguous ITRC IN39 remain explicit source limitations. IN39 is
not exposed. USBDCD follows the existing nine-part USB1 availability predicate;
MCXA536/537 applicability remains unresolved, not proven absent. Aliased
USBDCD timer defaults describe their respective modes, not independent storage.

A5 also models TSI, eSPI, USBHS/USBNC, USBPHY, ENET, and the 10BASE-T1S PHY
from F70RM chapters 42, 44, 51, 52, 54, and 59. Existing per-device metadata
predicates and instance names are preserved. USB capability and recovery
registers retain their 8/16-bit widths; the T1S PHY uses 16-bit accesses.
USBHS has eight endpoint pairs (EP0 plus seven indexed control registers),
and eSPI has five ports. ENET has one queue/channel, with explicit nested
MTL and DMA banks rather than generic-IP maximum-sized arrays.

Writable USBPHY SET/CLR/TOG aliases use zero-default mask types, distinct
from primary register reset defaults. Closure-based writes therefore emit
only selected mask bits. Do not rely on alias readback or use read-modify-write:
the specific register map and generic SCT access/readback prose conflict.
Other explicit source conflicts include eSPI WIREOUT_GPIO access and incoming
SAF tag/length layout, TSI overrun polarity, PHY LINKUP wording, and USB trim
and buffer-capacity descriptions. Their conservative register views are not
claims that the contradictory vendor documentation has been resolved.

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
