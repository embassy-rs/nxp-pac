# How to Use the Generator

## Preparing your environment

To be able to run the generator you need install locally:
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

For metapac chips the peripheral definitions are manually curated in [`data/metadata/peripherals`](/data/metadata/peripherals). Refer to the [README](/data/metadata/peripherals/README.md) in that folder on how to update these files.

MCXA chip metadata includes one shared family file. Chip fields replace shared fields; `only_in` filters peripherals by chip name. Nested includes are not supported.

For all chips you can (re-)generate the nxp-pac code by running (replace MCXA577 with your chip).
```
cargo run -p generator -- generate MCXA577
```
