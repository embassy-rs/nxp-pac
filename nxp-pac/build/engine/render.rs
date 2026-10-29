use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, ensure};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

use super::{Outputs, metadata::Metadata, model::Model};

fn ident(name: &str) -> anyhow::Result<Ident> {
    syn::parse_str(name).with_context(|| format!("invalid Rust identifier: {name}"))
}

pub fn peripheral(model: &Model) -> anyhow::Result<TokenStream> {
    let options = chiptool::generate::Options::new()
        .with_common_module(chiptool::generate::CommonModule::External(quote!(
            crate::pac::common
        )))
        .with_skip_no_std(true);
    let tokens = chiptool::generate::render(&model.ir, &options)?;
    if model.reset_values.is_empty() {
        return Ok(tokens);
    }
    let mut remaining = model.reset_values.clone();
    let mut file: syn::File = syn::parse2(tokens)?;
    for item in &mut file.items {
        let syn::Item::Impl(implementation) = item else {
            continue;
        };
        if !implementation
            .trait_
            .as_ref()
            .is_some_and(|(_, path, _)| path.is_ident("Default"))
        {
            continue;
        }
        let syn::Type::Path(self_type) = &*implementation.self_ty else {
            continue;
        };
        let Some(name) = self_type.path.get_ident() else {
            continue;
        };
        let Some(value) = remaining.remove(&name.to_string()) else {
            continue;
        };
        let value = Literal::u64_unsuffixed(value);
        let method = implementation
            .items
            .iter_mut()
            .find_map(|item| match item {
                syn::ImplItem::Fn(method) if method.sig.ident == "default" => Some(method),
                _ => None,
            })
            .context("Default implementation has no default method")?;
        method.block = syn::parse_quote!({ #name(#value) });
    }
    ensure!(
        remaining.is_empty(),
        "reset_value has no generated Default implementation: {remaining:?}"
    );
    Ok(quote!(#file))
}

pub fn write_device(
    metadata: &Metadata,
    models: &BTreeMap<String, Model>,
    out: &Path,
    outputs: &Outputs,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut modules = BTreeMap::new();
    let mut instances = Vec::new();
    for peripheral in &metadata.peripherals {
        let block = peripheral
            .parse_block_path()?
            .context("selected peripheral has no block")?;
        if let Some(previous) = modules.insert(block.rust_mod_name.clone(), block.path.clone()) {
            ensure!(
                previous == block.path,
                "conflicting module {}: {previous} versus {}",
                block.rust_mod_name,
                block.path
            );
        }
        let Some(address) = &peripheral.peripheral_address else {
            continue;
        };
        let address: TokenStream = address
            .parse::<TokenStream>()
            .map_err(|error| anyhow::anyhow!("invalid address for {}: {error}", peripheral.name))?;
        let name = ident(&peripheral.name)?;
        let module = ident(&block.rust_mod_name)?;
        let ty = ident(&block.rust_type_name)?;
        instances.push(quote!(pub const #name: #module::#ty = unsafe { #module::#ty::from_ptr(#address as _) };));
    }
    let interrupts = metadata
        .interrupts
        .iter()
        .map(|(name, number)| {
            ensure!(*number <= u16::MAX as u32, "interrupt out of range: {name}");
            let value = Literal::u32_unsuffixed(*number);
            let doc = format!("{number} - {name}");
            let name = ident(name)?;
            Ok(quote!(#[doc = #doc] #name = #value))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    if outputs.rt {
        let device_x = metadata
            .interrupts
            .keys()
            .map(|name| format!("PROVIDE({name} = DefaultHandler);\n"))
            .collect::<String>();
        write(out, "device.x", device_x, &mut files)?;
    }
    if !outputs.pac {
        return Ok(files);
    }
    for (name, model) in models {
        let path = format!("peripherals/{name}.rs");
        write(out, &path, peripheral(model)?.to_string(), &mut files)?;
    }
    write(
        out,
        "common.rs",
        chiptool::generate::COMMON_MODULE,
        &mut files,
    )?;
    if outputs.rt {
        let functions = metadata
            .interrupts
            .keys()
            .map(|name| {
                let name = ident(name)?;
                Ok(quote!(fn #name();))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let count = metadata
            .interrupts
            .values()
            .copied()
            .max()
            .unwrap_or_default()
            + 1;
        let vectors = (0..count)
            .map(
                |number| match metadata.interrupts.iter().find(|(_, n)| **n == number) {
                    Some((name, _)) => {
                        let name = ident(name)?;
                        Ok(quote!(Vector { _handler: #name }))
                    }
                    None => Ok(quote!(Vector { _reserved: 0 })),
                },
            )
            .collect::<anyhow::Result<Vec<_>>>()?;
        let count = Literal::u32_unsuffixed(count);
        write(
            out,
            "_vectors.rs",
            quote! {
                unsafe extern "C" { #(#functions)* }
                pub union Vector { _handler: unsafe extern "C" fn(), _reserved: u32 }
                #[unsafe(link_section = ".vector_table.interrupts")]
                #[unsafe(no_mangle)]
                pub static __INTERRUPTS: [Vector; #count] = [#(#vectors),*];
            }
            .to_string(),
            &mut files,
        )?;
    }
    let modules = modules
        .iter()
        .map(|(module, path)| {
            let module = ident(module)?;
            let path = format!("peripherals/{path}.rs");
            Ok(quote!(#[path = #path] pub mod #module;))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let bits = Literal::u32_unsuffixed(metadata.nvic_prio_bits);
    write(
        out,
        "pac.rs",
        quote! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            pub enum Interrupt { #(#interrupts),* }
            unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
                #[inline(always)]
                fn number(self) -> u16 { self as u16 }
            }
            #[cfg(feature = "rt")]
            mod _vectors;
            #[doc = " Number available in the NVIC for configuring priority"]
            #[cfg(feature = "rt")]
            pub const NVIC_PRIO_BITS: u8 = #bits;
            #[cfg(feature = "rt")]
            pub use Interrupt as interrupt;
            #[cfg(feature = "rt")]
            pub use cortex_m_rt::interrupt;
            #(#instances)*
            #(#modules)*
            pub mod common;
        }
        .to_string(),
        &mut files,
    )?;
    Ok(files)
}

fn write(
    out: &Path,
    relative: &str,
    contents: impl AsRef<[u8]>,
    files: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    let path = out.join(super::asset_path(relative)?);
    fs::create_dir_all(path.parent().context("output has no parent")?)?;
    fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))?;
    files.push(path);
    Ok(())
}
