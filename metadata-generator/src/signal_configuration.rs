use std::{fs, path::Path};

use anyhow::{Context, bail};
use roxmltree::{Document, Node, ParsingOptions};

use crate::{require_string_attribute, skip_newline, string_attribute};

#[derive(Debug)]
pub struct SignalConfiguration {
    /// A reference to a different file.
    ///
    /// This may appear if multiple part numbers share the same pin configuration.
    pub reference: Option<String>,

    pub peripherals: Vec<Peripheral>,
    pub package_functions: Vec<PackageFunction>,
    // TODO: non_peripheral_pin_functions?
    pub pins: Vec<Pin>,
    // TODO: Power groups (nice for symbols)
}

impl SignalConfiguration {
    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let options = ParsingOptions::default();
        let doc = roxmltree::Document::parse_with_options(&content, options)?;

        let reference = Self::read_reference(&doc)?;

        if reference.is_some() {
            return Ok(Self {
                reference,
                peripherals: Vec::new(),
                package_functions: Vec::new(),
                pins: Vec::new(),
            });
        }

        let peripherals = Self::read_peripherals(&doc)?;
        let package_functions = Vec::new();
        let pins = Self::read_pins(&doc)?;

        if pins.is_empty() {
            bail!("Pins is empty?");
        }

        Ok(Self {
            reference,
            peripherals,
            package_functions,
            pins,
        })
    }

    fn read_reference(doc: &Document<'_>) -> anyhow::Result<Option<String>> {
        doc.descendants()
            .find(|e| e.tag_name().name() == "reference")
            .map(|e| {
                e.attribute("file")
                    .map(String::from)
                    .context("reference has no file attribute")
            })
            .transpose()
    }

    fn read_peripherals(doc: &Document<'_>) -> anyhow::Result<Vec<Peripheral>> {
        let peripherals_node = doc
            .descendants()
            .find(|n| n.tag_name().name() == "peripherals")
            .context("document has no peripherals node in root")?;
        let mut peripherals = Vec::new();

        for peripheral in peripherals_node.children().filter(skip_newline) {
            peripherals.push(Peripheral::read(doc, peripheral)?);
        }

        Ok(peripherals)
    }

    fn read_pins(doc: &Document<'_>) -> anyhow::Result<Vec<Pin>> {
        let pins_node = doc
            .descendants()
            .find(|n| n.tag_name().name() == "pins")
            .context("document has no pins node in root")?;
        let mut pins = Vec::new();

        for pin in pins_node.children().filter(skip_newline) {
            pins.push(Pin::read(doc, pin)?);
        }

        Ok(pins)
    }
}

#[derive(Debug)]
pub struct Peripheral {
    pub id: String,
    pub name: String,
    pub peripheral_type: String,
}

impl Peripheral {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "peripheral" {
            bail!(
                "Node being read is not a peripheral. Is: {}",
                node.tag_name().name()
            );
        }

        let id = require_string_attribute(doc, &node, "id")?;
        let name = require_string_attribute(doc, &node, "name")?;
        let peripheral_type = require_string_attribute(doc, &node, "name")?;

        Ok(Self {
            id,
            name,
            peripheral_type,
        })
    }
}

#[derive(Debug)]
pub struct PackageFunction {
    pub id: String,
    pub name: String,
    // ignore description.
}

#[derive(Debug)]
pub struct Pin {
    /// This is often meaningless, as it contains every pin function separated by slashes.
    pub name: String,
    /// The location of this pin on the package.
    ///
    /// This can be a number for QFN/QFP packages while BGA packages use a letter and number.
    pub coords: String,
    pub power_group: Option<String>,
    pub connections: Vec<Connections>,
}

impl Pin {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "pin" {
            bail!(
                "Pin being read is not a pin. Is: {}",
                node.tag_name().name()
            );
        }

        let name = require_string_attribute(doc, &node, "name")?;
        let coords = require_string_attribute(doc, &node, "coords")?;
        let power_group = string_attribute(&node, "power_group");

        let connections = node
            .children()
            .filter(skip_newline)
            .filter(|child| child.tag_name().name() == "connections")
            .map(|child| Connections::read(doc, child))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self {
            name,
            coords,
            power_group,
            connections,
        })
    }
}

#[derive(Debug)]
pub struct Connections {
    /// [`None`] sometimes.
    ///
    /// Examples where it is [`None`]: KL32L2A31VLH1A
    pub name_part: Option<String>,

    /// [`None`] if this is a power pin.
    pub package_function: Option<String>,

    pub connection: Vec<Connection>,
}

impl Connections {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "connections" {
            bail!(
                "Connections being read is not a connections. Is: {}",
                node.tag_name().name()
            );
        }

        let name_part = string_attribute(&node, "name_part");
        let package_function = string_attribute(&node, "package_function");
        let connection = node
            .children()
            .filter(skip_newline)
            .map(|child| Connection::read(doc, child))
            .collect::<anyhow::Result<Vec<_>>>()?;

        if connection.len() > 1 {
            println!("More than one connection?");
        }

        Ok(Self {
            name_part,
            package_function,
            connection,
        })
    }
}

#[derive(Debug)]
pub struct Connection {
    /// This may be none for specific pins.
    ///
    /// Example: PTE31, PORTE_PCR31 mux in K32L2A31VLH1A
    pub peripheral_signal_ref: Option<PeripheralSignalRef>,
    pub configuration: Option<Configuration>,
}

impl Connection {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "connection" {
            bail!(
                "Connection being read is not a connection. Is: {}",
                node.tag_name().name()
            );
        }

        let peripheral_signal_ref = node
            .children()
            .find(|child| child.tag_name().name() == "peripheral_signal_ref")
            .map(|child| PeripheralSignalRef::read(doc, child))
            .transpose()?;

        let configuration = node
            .children()
            .find(|child| child.tag_name().name() == "configuration")
            .map(|child| Configuration::read(doc, child))
            .transpose()?;

        Ok(Self {
            peripheral_signal_ref,
            configuration,
        })
    }
}

#[derive(Debug)]
pub struct PeripheralSignalRef {
    pub signal: String,

    pub peripheral: String,

    pub channel: Option<String>,

    pub features: Vec<String>,
}

impl PeripheralSignalRef {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "peripheral_signal_ref" {
            bail!(
                "PeripheralSignalRef being read is not a peripheral_signal_ref. Is: {}",
                node.tag_name().name()
            );
        }

        let signal = require_string_attribute(doc, &node, "signal")?;
        let peripheral = require_string_attribute(doc, &node, "peripheral")?;
        let channel = string_attribute(&node, "channel");
        let features = Vec::new();

        Ok(Self {
            signal,
            peripheral,
            channel,
            features,
        })
    }
}

#[derive(Debug)]
pub struct Configuration {
    pub assignments: Vec<RegisterAssignment>,
}

impl Configuration {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "configuration" {
            bail!(
                "Configuration being read is not a configuration. Is: {}",
                node.tag_name().name()
            );
        }

        let assignments = node
            .children()
            .filter(skip_newline)
            .filter(|node| node.tag_name().name() == "assign")
            .map(|child| RegisterAssignment::read(doc, child))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self {
            assignments,
        })
    }
}

#[derive(Debug)]
pub struct RegisterAssignment {
    pub register: String,
    /// [`None`] for some pins in MC56F80623
    pub bit_field: Option<String>,
    pub bit_field_mask: String,
    pub bit_field_value: String,
}

impl RegisterAssignment {
    fn read(doc: &Document<'_>, node: Node<'_, '_>) -> anyhow::Result<Self> {
        if node.tag_name().name() != "assign" {
            bail!(
                "RegisterAssignment being read is not a assign. Is: {}",
                node.tag_name().name()
            );
        }

        let register = require_string_attribute(doc, &node, "register")?;
        let bit_field = string_attribute(&node, "bit_field");
        let bit_field_mask = require_string_attribute(doc, &node, "bit_field_mask")?;
        let bit_field_value = require_string_attribute(doc, &node, "bit_field_value")?;

        Ok(Self {
            register,
            bit_field,
            bit_field_mask,
            bit_field_value,
        })
    }
}
