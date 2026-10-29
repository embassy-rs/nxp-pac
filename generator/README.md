# How to Use the Generator

## Preparing your environment

Legacy PAC regeneration requires:
- [form](https://crates.io/crates/form) - Used to format the generated code. If you try to install this tool via your package manager (apt, zypper, etc.) you may install a different tool. Please install it from cargo, as shown below.

```bash
cargo install form
```

## Running the Generator

You must switch to the root of the repository, which is the parent directory of
the generator crate directory.

```
cargo run -p generator -- -h
```

```
Usage: generator <COMMAND>

Commands:
  generate  Generate the nxp-pac Rust code for a single chip or all the chips
  extract   Extract all metadata information from the SVD and apply any transforms
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Updating

The chips supported by this project are defined in [`lib.rs`](/generator/src/lib.rs).
The SVDs can be selected from the submodule in `data/mcux-soc-svd`.
These SVDs are transformed using definitions in [`data/transforms`](/data/transforms).
Refer to the [chiptool](https://github.com/embassy-rs/chiptool/) project to find out how these transforms work.

For MetaPAC chips, authoritative peripheral definitions live in
[`nxp-pac/data/peripherals`](/nxp-pac/data/peripherals), and chip metadata lives in
[`nxp-pac/data/metadata`](/nxp-pac/data/metadata). Maintain these files directly.
See the [curation guide](/nxp-pac/data/peripherals/README.md).

MCXA chip metadata includes one shared family file. Chip fields replace shared fields; `only_in` filters peripherals by chip name. Nested includes are not supported.

MetaPAC Rust is generated automatically when the PAC is compiled. The development
command below validates the same generation engine in scratch output; it does not
write generated files back into `nxp-pac/src`:
```
cargo run -p generator -- generate MCXA577
```

To retain inspection output, add `--out-dir <directory>`; files are grouped by
lowercase chip name below that directory. This path uses no external formatter.
The generation command resolves inputs independently of the current directory.

The no-argument `generate` command still regenerates **legacy** sources from the
vendor checkout and checks MetaPAC generation in scratch directories. It is not
a read-only check. Explicit `extract <chip>` remains a separate import operation
whose output goes to `data/metadata/peripherals/raw` by default.
