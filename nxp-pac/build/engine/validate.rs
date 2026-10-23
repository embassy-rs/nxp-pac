use super::model::Model;
use anyhow::{Context, ensure};
use chiptool::ir::{Array, BitOffset, BlockItemInner};

pub fn validate_sort_safety(model: &Model) -> anyhow::Result<()> {
    for (fieldset_name, fieldset) in &model.ir.fieldsets {
        for field in &fieldset.fields {
            if let BitOffset::Cursed(ranges) = &field.bit_offset {
                ensure!(
                    !ranges.is_empty(),
                    "fieldset {fieldset_name}: empty bit range list for {}",
                    field.name
                );
                for range in ranges {
                    ensure!(
                        !range.is_empty(),
                        "fieldset {fieldset_name}: reversed bit range for {}",
                        field.name
                    );
                }
                ensure!(
                    ranges
                        .windows(2)
                        .all(|pair| pair[0].end() < pair[1].start()),
                    "fieldset {fieldset_name}: overlapping or unsorted bit ranges for {}",
                    field.name
                );
            }
        }
    }
    Ok(())
}

pub fn validate(model: &Model) -> anyhow::Result<()> {
    validate_sort_safety(model)?;
    for (name, fieldset) in &model.ir.fieldsets {
        ensure!(
            matches!(fieldset.bit_size, 8 | 16 | 32 | 64),
            "fieldset {name}: unsupported storage width {}",
            fieldset.bit_size
        );
        for field in &fieldset.fields {
            ensure!(
                field.bit_size > 0,
                "fieldset {name}: zero-width field {}",
                field.name
            );
            let (end, encoded_width) = match &field.bit_offset {
                BitOffset::Regular(offset) => (
                    u64::from(*offset) + u64::from(field.bit_size),
                    u64::from(field.bit_size),
                ),
                BitOffset::Cursed(ranges) => {
                    let encoded_width = ranges
                        .iter()
                        .map(|range| u64::from(*range.end()) - u64::from(*range.start()) + 1)
                        .sum::<u64>();
                    (
                        u64::from(*ranges.iter().map(|r| r.end()).max().unwrap()) + 1,
                        encoded_width,
                    )
                }
            };
            ensure!(
                encoded_width == u64::from(field.bit_size),
                "fieldset {name}: bit ranges for {} encode {encoded_width} bits, expected {}",
                field.name,
                field.bit_size
            );
            let array_offset = match &field.array {
                Some(Array::Regular(array)) => {
                    ensure!(
                        array.len > 0,
                        "fieldset {name}: empty field array {}",
                        field.name
                    );
                    u64::from(array.len - 1) * u64::from(array.stride)
                }
                Some(Array::Cursed(array)) => {
                    u64::from(*array.offsets.iter().max().context("empty field array")?)
                }
                None => 0,
            };
            ensure!(
                end + array_offset <= u64::from(fieldset.bit_size),
                "fieldset {name}: field {} exceeds storage width {}",
                field.name,
                fieldset.bit_size
            );
        }
    }
    for (block_name, block) in &model.ir.blocks {
        for item in &block.items {
            if let BlockItemInner::Register(register) = &item.inner {
                ensure!(
                    matches!(register.bit_size, 8 | 16 | 32 | 64),
                    "{block_name}.{}: invalid register width {}",
                    item.name,
                    register.bit_size
                );
                if let Some(name) = &register.fieldset {
                    let fieldset = model.ir.fieldsets.get(name).with_context(|| {
                        format!("{block_name}.{}: missing fieldset {name}", item.name)
                    })?;
                    ensure!(
                        register.bit_size == fieldset.bit_size,
                        "{block_name}.{}: register width {} differs from fieldset {name} storage width {}",
                        item.name,
                        register.bit_size,
                        fieldset.bit_size
                    );
                }
            }
        }
    }
    for (name, value) in &model.reset_values {
        let fieldset = model
            .ir
            .fieldsets
            .get(name)
            .with_context(|| format!("missing reset fieldset {name}"))?;
        ensure!(
            fieldset.bit_size == 64 || *value < (1u64 << fieldset.bit_size),
            "reset_value exceeds storage width: {name}"
        );
    }
    let errors = chiptool::validate::validate(
        &model.ir,
        chiptool::validate::Options {
            allow_register_overlap: true,
            allow_field_overlap: true,
            allow_enum_dup_value: true,
            allow_unused_enums: true,
            allow_unused_fieldsets: true,
        },
    );
    ensure!(errors.is_empty(), "{}", errors.join("; "));
    Ok(())
}
