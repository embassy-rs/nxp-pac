mod sigmap;

use std::{collections::BTreeMap, fs, path::Path, sync::LazyLock};

use anyhow::{Context, bail};
use regex::Regex;

use crate::{
    iomuxc::IomuxcRegisters,
    metadata::{Daisy, Peripheral, PeripheralPin},
};
use sigmap::SIGMAP;

#[derive(Debug)]
struct PinFunctionId {
    pin_name: String,
    signal: String,
    mux_register: u32,
    mux_mode: u32,
    input_register: u32,
    input_daisy: u32,
    config_register: u32,
}

fn get_functions(current_dir: &Path, chip: &str) -> anyhow::Result<Vec<PinFunctionId>> {
    let devices_rt = current_dir.join("data").join("mcux-devices-rt");

    // NXP groups by family and some parts share the header with other parts. The actual fsl_iomuxc
    // header base path needs to be looked up.
    //
    // (<chip_name>, (<component 1>, <component 2>))
    const IDS: &[(&str, (&str, &str))] = &[
        ("MIMXRT1011", ("RT1010", "MIMXRT1011")),
        ("MIMXRT1015", ("RT1015", "MIMXRT1015")),
        ("MIMXRT1021", ("RT1020", "MIMXRT1021")),
        ("MIMXRT1024", ("RT1020", "MIMXRT1024")),
        ("MIMXRT1041", ("RT1040", "MIMXRT1042")),
        ("MIMXRT1042", ("RT1040", "MIMXRT1042")),
        ("MIMXRT1043", ("RT1040", "MIMXRT1042")),
        ("MIMXRT1046", ("RT1040", "MIMXRT1042")),
        ("MIMXRT1051", ("RT1050", "MIMXRT1051")),
        ("MIMXRT1052", ("RT1050", "MIMXRT1052")),
        ("MIMXRT1061", ("RT1060", "MIMXRT1061")),
        ("MIMXRT1062", ("RT1060", "MIMXRT1062")),
        ("MIMXRT1064", ("RT1060", "MIMXRT1064")),
        ("MIMXRT1165", ("RT1160", "MIMXRT1166")),
        ("MIMXRT1166", ("RT1160", "MIMXRT1166")),
        ("MIMXRT1171", ("RT1170", "MIMXRT1176")),
        ("MIMXRT1171", ("RT1170", "MIMXRT1176")),
        ("MIMXRT1172", ("RT1170", "MIMXRT1176")),
        ("MIMXRT1173", ("RT1170", "MIMXRT1176")),
        ("MIMXRT1175", ("RT1170", "MIMXRT1176")),
        ("MIMXRT1176", ("RT1170", "MIMXRT1176")),
        ("MIMXRT1181", ("RT1180", "MIMXRT1189")),
        ("MIMXRT1182", ("RT1180", "MIMXRT1189")),
        ("MIMXRT1186", ("RT1180", "MIMXRT1189")),
        ("MIMXRT1187", ("RT1180", "MIMXRT1189")),
        ("MIMXRT1189", ("RT1180", "MIMXRT1189")),
    ];

    let (_, (component1, component2)) = IDS
        .iter()
        .find(|(chip_name, _)| chip_name == &chip)
        .context(format!("{chip} had no entry in IDs table"))?;

    let fsl_iomuxc = devices_rt
        .join(component1)
        .join(component2)
        .join("drivers")
        .join("fsl_iomuxc.h");

    if !fs::exists(&fsl_iomuxc)? {
        bail!(
            "{}, fsl_iomuxc.h at \"{}\" does not exist.",
            chip,
            fsl_iomuxc.display()
        );
    }

    let header = fs::read_to_string(&fsl_iomuxc)?;
    let pin_functions = PinFunctionId::parse(&header)?;

    Ok(pin_functions)
}

impl PinFunctionId {
    // FIXME: Correct RT1011 using GPIOMUX instead of GPIOxx_IOyy
    pub fn parse(header: &str) -> anyhow::Result<Vec<Self>> {
        static IOMUXC_DEFINE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?m)#define\s+IOMUXC_(?P<ident>[^\s]+)\s+(?P<mux_register>[^,]+),\s*(?P<mux_mode>[^,]+),\s*(?P<input_register>[^,]+),\s*(?P<input_daisy>[^,]+),\s*(?P<config_register>[^,\s]+)\s*$")
                .unwrap()
        });

        static GPIO_AND_SIGNAL: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^(?P<pin_name>GPIO(?:_[A-Z0-9]+)*?_\d+?)_(?P<signal>.+)$").unwrap()
        });

        fn hex_or_zero(s: &str) -> anyhow::Result<u32> {
            let trimmed = s.trim_end_matches('U');

            let value = if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
                u32::from_str_radix(&trimmed[2..], 16)?
            } else {
                trimmed.parse::<u32>()?
            };

            Ok(value)
        }

        let mut functions = Vec::new();

        for capture in IOMUXC_DEFINE.captures_iter(header) {
            let ident: String = capture["ident"].into();
            let mux_register = hex_or_zero(&capture["mux_register"])?;
            let mux_mode = hex_or_zero(&capture["mux_mode"])?;
            let input_register = hex_or_zero(&capture["input_register"])?;
            let input_daisy = hex_or_zero(&capture["input_daisy"])?;
            let config_register = hex_or_zero(&capture["config_register"])?;

            // If all of these are zero then the pin is not usable as GPIO.
            //
            // This is case with TEST_MODE, POR_B and ONOFF.
            if mux_register == 0 && mux_mode == 0 && input_register == 0 && input_daisy == 0 {
                continue;
            }

            let (pin_name, signal): (String, String) = if ident.starts_with("GPIO_") {
                // In this form, it is:
                // <pin name>_<signal>
                //
                // But some chips have more complicated names such as GPIO_SPI_B1_06 so we need to match until
                // the signal name and not where the first number suffix exists.
                let captures = GPIO_AND_SIGNAL
                    .captures(&ident)
                    .context(format!("{ident} does not match GPIO and signal pattern"))?;

                let pin_name = &captures["pin_name"];
                let signal = &captures["signal"];

                (pin_name.into(), signal.into())
            } else if ident.starts_with("SNVS_") {
                if ident.starts_with("SNVS_WAKEUP") {
                    (
                        "WAKEUP".into(),
                        ident.strip_prefix("SNVS_WAKEUP_").unwrap().into(),
                    )
                } else if ident.starts_with("SNVS_PMIC_ON_REQ_") {
                    let mut signal = ident.strip_prefix("SNVS_PMIC_ON_REQ_").unwrap().into();

                    // But RT1011 has a terrible name for PMIC_ON_REQ function, so we need to correct that.
                    if signal == "SNVS_LP_PMIC_ON_REQ" {
                        signal = "PMIC_ON_REQ".into();
                    }

                    ("PMIC_ON_REQ".into(), signal)
                } else if ident.starts_with("SNVS_PMIC_STBY_REQ_") {
                    let mut signal = ident.strip_prefix("SNVS_PMIC_STBY_REQ_").unwrap().into();

                    // RT1062 has a terrible name, so correct it.
                    if signal == "CCM_PMIC_STBY_REQ" {
                        signal = "PMIC_STBY_REQ".into();
                    }

                    ("PMIC_STBY_REQ".into(), signal)
                } else {
                    bail!("{ident} is unhandled SNVS function match");
                }
            } else {
                bail!("{ident} is unhandled pin function match");
            };

            functions.push(PinFunctionId {
                pin_name,
                signal,
                mux_register,
                mux_mode,
                input_register,
                input_daisy,
                config_register,
            });
        }

        Ok(functions)
    }
}

pub fn get_metadata(
    current_dir: &Path,
    chip: &str,
) -> anyhow::Result<(
    BTreeMap<String, Peripheral>,
    BTreeMap<String, IomuxcRegisters>,
)> {
    let functions = get_functions(current_dir, chip)?;

    let mut peripherals = BTreeMap::new();
    let mut iomuxc = BTreeMap::new();

    // TODO: use mux register and config registers
    for PinFunctionId {
        pin_name,
        signal,
        mux_register,
        mux_mode,
        input_register,
        input_daisy,
        config_register,
    } in functions.iter()
    {
        // We need to split the peripheral from the signal.
        //
        // But sometimes that doesn't apply.
        let peripheral_name = get_peripheral(&signal);
        let signal = SIGMAP.get(&format!("{chip}:{signal}"));

        let peripheral = peripherals
            .entry(peripheral_name.into())
            .or_insert_with(|| Peripheral {
                name: peripheral_name.into(),
                pins: vec![],
            });

        let iomuxc_daisy = (*input_register != 0).then_some(Daisy {
            register: *input_register,
            value: *input_daisy as u8,
        });

        if let Some(signal) = signal {
            peripheral.pins.push(PeripheralPin {
                pin: pin_name.clone(),
                signal: signal.to_string(),
                function: *mux_mode as u8,
                iomuxc_daisy,
            });
        }

        iomuxc.insert(
            pin_name.clone(),
            IomuxcRegisters {
                name: pin_name.to_string(),
                pad_address: *config_register,
                mux_address: *mux_register,
            },
        );
    }

    Ok((peripherals, iomuxc))
}

fn get_peripheral(pin_function_id: &str) -> &str {
    // NMI floats in some chips without a peripheral prefix.
    let split_index = if pin_function_id == "NMI" {
        return "ARM";
    } else if pin_function_id.starts_with("REF_CLK") {
        return "XTALOSC";
    } else if pin_function_id == "PMIC_ON_REQ" {
        return "SNVS";
    } else if pin_function_id.starts_with("FLEXSPIA") || pin_function_id.starts_with("FLEXSPIB") {
        // If flexspi has no numerical suffix make it instance 1 (since larger chips have 2 instances).
        return "FLEXSPI1";
    } else if pin_function_id.starts_with("FLEXSPI") {
        // But 1011 does put an underscore between peripheral type and signal.
        return "FLEXSPI1";
    } else if pin_function_id.starts_with("ENET") && !pin_function_id.starts_with("ENET2") {
        // If enet has no numerical suffix make it instance 1 (since larger chips have 2 instances).
        return "ENET1";
    } else if pin_function_id.starts_with("GPIOMUX") {
        // RT1011 uses GPIOMUX for GPIO1, which is unique.
        return "GPIO1";
    } else if pin_function_id.starts_with("USB_OTG") {
        // USB_OTGx is the peripheral, not USB
        // There are no more than 2 USB OTG peripherals on imxrt parts.
        Some("USB_OTGx".len())
    } else {
        pin_function_id.find('_')
    };

    let peripheral_name = &pin_function_id[..split_index.unwrap_or(0)];
    assert!(
        !peripheral_name.is_empty(),
        "{pin_function_id} resulted in empty peripheral"
    );

    peripheral_name
}
