# IP features

Per-chip data that nxp-pac hands to its direct dependents through Cargo `links` metadata, so a driver can gate on what the selected chip has (`#[cfg(lpuart_has_ctrl_swap)]`) without hand-set cargo features.

Each `<ip>.yaml` here is the catalog of one IP (hand-written): every option cfg the IP exports and how its value is obtained.

```yaml
cfgs:
  lpspi_has_no_pcscfg:                 # <ip>_has_<x>, after the SDK's FSL_FEATURE_<IP>_HAS_<X> (same polarity)
    derive: { field: Cfgr1.pcscfg, absent: true }
  i3c_has_errata_051617: {}            # no `derive`: import-only, false unless the chip file lists it
```

- `derive`: `register: <Block>.<item>` or `field: <Fieldset>.<field>`, evaluated on the chip's own curated model of the IP (the rule the SDK features follow: `HAS_X` = the register or field exists); `absent: true` inverts it. Use it whenever the option is a register or field presence.
- Import-only options (behaviour, errata, anything the registers can't show) come from the chip's MCUXpresso SDK `<SoC>_features.h`, macro `FSL_FEATURE_<CFG in upper case>`.
- A cfg is set when the chip has the IP and the value is true. The IP's own presence cfg (`lpuart`) is set when the chip's PAC has the module.

## Export

`build.rs` prints, as `cargo::metadata` lines:

| Key | Content | Seen by dependents as |
|-----|---------|-----------------------|
| `all_cfgs` | every name in the catalog, for every chip | `DEP_NXP_PAC_ALL_CFGS` |
| `cfgs` | the cfgs the selected chip sets (MetaPAC chips only) | `DEP_NXP_PAC_CFGS` |

`DEP_NXP_PAC_CFGS` is unset for a chip that has no export.

A dependent's `build.rs` re-emits them:

```rust
for c in env::var("DEP_NXP_PAC_ALL_CFGS").unwrap_or_default().split(',').filter(|c| !c.is_empty()) {
    println!("cargo::rustc-check-cfg=cfg({c})");
}
for c in env::var("DEP_NXP_PAC_CFGS").unwrap_or_default().split(',').filter(|c| !c.is_empty()) {
    println!("cargo::rustc-cfg={c}");
}
```

Only direct dependents of nxp-pac (normal, not build-dependency) receive `DEP_NXP_PAC_*`.

`all_cfgs` is read from the catalog files without a YAML parser (`build/feature_names.rs`): keep `cfgs:` a block mapping.

## Import-only options

A chip file `chips/<CORE>.yaml` lists the import-only options the chip sets (`imported:`). An import-only option is false for every chip that does not list it; a chip without any has no file. For a chip whose SDK header has `FSL_FEATURE_I3C_HAS_ERRATA_051617 (1)` the file contains:

```yaml
imported:
- i3c_has_errata_051617
```
