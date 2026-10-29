use std::{env, path::PathBuf};

#[cfg(feature = "_metapac")]
#[path = "build/chips.rs"]
mod chips;
#[cfg(feature = "_metapac")]
#[path = "build/engine/mod.rs"]
mod engine;
#[path = "build/inventory.rs"]
mod inventory;
#[path = "build/models.rs"]
mod models;

fn main() {
    let mut chips = env::vars().map(|(name, _)| name).filter(|name| {
        name.starts_with("CARGO_FEATURE_MIMXRT")
            || name.starts_with("CARGO_FEATURE_MCX")
            || name.starts_with("CARGO_FEATURE_LPC55")
    });
    let chip = chips
        .next()
        .expect("No mimxrt/mcx/lpc Cargo feature enabled");
    assert!(
        chips.next().is_none(),
        "Multiple mimxrt/mcx/lpc Cargo features enabled"
    );
    let chip = chip
        .strip_prefix("CARGO_FEATURE_")
        .unwrap()
        .to_ascii_lowercase();
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let inputs = manifest.join("data");
    println!("cargo:rustc-check-cfg=cfg(nxp_metapac)");
    println!("cargo:rerun-if-changed=build.rs");
    // Directory tracking also notices newly available mappings in PAC-only builds.
    // Enumerating names does not parse or render unselected peripheral models.
    for directory in inventory::write(&inputs, &out).expect("generating peripheral inventory") {
        println!("cargo:rerun-if-changed={}", directory.display());
    }
    #[cfg(feature = "_metapac")]
    if chips::find(&chip).is_some() {
        println!("cargo:rustc-cfg=nxp_metapac");
        let generated = engine::generate(
            &chip,
            &inputs,
            &out,
            engine::Outputs {
                pac: cfg!(feature = "pac"),
                metadata: cfg!(feature = "metadata"),
                rt: cfg!(feature = "rt"),
            },
        )
        .unwrap_or_else(|error| panic!("generating {chip}: {error:#}"));
        for path in generated.input_files {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        for path in generated.output_files {
            assert!(path.starts_with(&out), "generation escaped OUT_DIR");
        }
        if cfg!(feature = "rt") {
            println!("cargo:rustc-link-search={}", out.display());
        }
        return;
    }
    if cfg!(feature = "rt") {
        println!(
            "cargo:rustc-link-search={}",
            manifest.join("src/chips").join(chip).display()
        );
    }
}
