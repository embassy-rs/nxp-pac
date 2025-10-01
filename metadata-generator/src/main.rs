mod cores_info;
mod part_number;
mod signal_configuration;

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    sync::LazyLock,
    time::Instant,
};

use anyhow::{Context, bail};
use quick_xml::{Reader, events::Event};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use roxmltree::{Document, Node};

use crate::{part_number::PartNumber, signal_configuration::SignalConfiguration};

// TODO: Download data for repository
const BASE_DIR: &str = "/home/i509vcb/Dev/configtoolexport/";

fn main() -> anyhow::Result<()> {
    let mut stopwatch = Stopwatch::new();
    let current = env::current_dir()?;
    let build_dir = current.join("build");

    // For debugging, you may want to enable this so that debug spans for XML parsing are atomically logged.
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();

    let sources = Path::new(BASE_DIR);
    let npidata = sources.join("npidata.mf");

    if !fs::exists(&npidata)? {
        bail!("npidata.mf is missing from sources. Did you export the data from the pin tool?");
    }

    stopwatch.section("get processor list");
    let processors_list = get_processor_list(&npidata)?;

    let processors_dir = sources.join("processors");
    let missing = processors_list
        .iter()
        .filter(|processor| {
            let path = processors_dir.join(processor);
            let exists = match fs::exists(&path) {
                Ok(exists) => exists,
                Err(_) => false,
            };

            !exists
        })
        .collect::<Vec<_>>();

    if !missing.is_empty() {
        bail!("Missing processors in sources: {:#?}", missing);
    }

    let mut part_numbers = Vec::new();

    stopwatch.section("get part numbers");

    for processor in processors_list.iter() {
        let processor_dir = processors_dir.join(processor);

        let ksdk = fs::exists(processor_dir.join("ksdk2_0"))?;
        let imx = fs::exists(processor_dir.join("i_mx_2_0"))?;

        if !ksdk && !imx {
            bail!("{processor} does not have ksdk2_0 or i_mx_2_0 directory");
        }

        let metadata_dir = if ksdk {
            processor_dir.join("ksdk2_0")
        } else if imx {
            processor_dir.join("i_mx_2_0")
        } else {
            unreachable!();
        };

        let processor_part_numbers = get_part_numbers(processor, &metadata_dir)
            .context(format!("Getting part numbers for {}", processor))?;

        part_numbers.extend(
            processor_part_numbers
                .into_iter()
                .map(|part_number| (metadata_dir.clone(), part_number)),
        );
    }

    stopwatch.section("read chips");

    let chips = part_numbers
        .par_iter()
        .map(|(metadata_dir, part_number)| {
            let part_number_name = part_number.file_name().unwrap().to_string_lossy();

            let chip = Chip::read(&metadata_dir, part_number)
                .context(format!("while reading {part_number_name}"))?;

            Ok((part_number_name.to_string(), chip))
        })
        .collect::<anyhow::Result<BTreeMap<String, Chip>>>()?;

    stopwatch.stop();

    // dbg!(chips.keys().collect::<Vec<_>>());
    // chips
    //     .iter()
    //     .filter(|(key, _)| {
    //         *key == "MIMXRT1011CAE4A.xml"
    //     })
    //     .for_each(|(_, _chip)| {});

    Ok(())
}

#[derive(Debug)]
struct Chip {
    signal_configuration: SignalConfiguration,
}

impl Chip {
    fn read(metadata_dir: &Path, part_number: &Path) -> anyhow::Result<Self> {
        println!("Read: {}", part_number.display());

        let part_number_info = PartNumber::read(&part_number)?;

        let pins_model = part_number_info
            .pins_model()
            .context(format!("{} is missing pins_model", part_number.display()))?;

        let mut pins_model_path = metadata_dir.join(pins_model);

        if !fs::exists(&pins_model_path)? {
            bail!(
                "pins_model for {} does not exist at {}",
                part_number_info.id,
                pins_model_path.display(),
            );
        }

        const ITERS: usize = 1;
        // Resolving the configuration may take multiple iterations
        let mut iter = 0;
        let signal_configuration = loop {
            if iter > ITERS {
                bail!(
                    "Signal configuration for {} was not resolved after {ITERS} iterations",
                    part_number.display()
                );
            }

            let signal_configuration = SignalConfiguration::read(&pins_model_path)?;
            // let signal_configuration = SignalConfiguration::read_quickxml(&pins_model_path)?;

            if let Some(reference) = signal_configuration.reference {
                pins_model_path = metadata_dir.join(reference);

                if !fs::exists(&pins_model_path)? {
                    bail!(
                        "pins_model reference for {} does not exist at {}",
                        part_number_info.id,
                        pins_model_path.display(),
                    );
                }

                iter += 1;
                continue;
            }

            break signal_configuration;
        };

        // If the signal configuration is a reference, we need to read the true configuration.
        assert!(signal_configuration.reference.is_none());

        // TODO: Check cores_info.xml to get the number of application cores available (and suffix if needed).

        Ok(Chip {
            signal_configuration,
        })
    }
}

fn get_processor_list(npidata: &Path) -> anyhow::Result<Vec<String>> {
    let npidata = fs::read_to_string(npidata)?;

    let processor_lines = npidata
        .lines()
        .filter(|line| line.starts_with("processors="))
        .collect::<Vec<_>>();

    if processor_lines.len() != 1 {
        bail!("npidata did not have list of processors, or had too many processors lines");
    }

    Ok(processor_lines
        .first()
        // Only a single entry exists.
        .unwrap()
        .strip_prefix("processors=")
        // processors entry must exist because of prefix check.
        .unwrap()
        .split(',')
        .map(String::from)
        .collect::<Vec<_>>())
}

fn get_part_numbers(
    processor: &str,
    processor_metadata_dir: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    // When we enter a processor directory, we may need to find multiple part numbers.
    //
    // The best way to do this is to brute force check every xml file and check if the schema URI is
    // `part_number.xsd`.
    let part_numbers = fs::read_dir(&processor_metadata_dir)?
        .filter_map(|f| {
            let entry = match f {
                Ok(entry) => entry,
                Err(err) => return Some(Err(err.into())),
            };

            let path = entry.path();
            let Some(extension) = path.extension() else {
                return None;
            };

            if extension == "xml" {
                let uri = match get_xml_schema_uri(&path) {
                    Ok(uri) => uri,
                    Err(err) => return Some(Err(err)),
                };

                if !PartNumber::matches_uri(&uri) {
                    return None;
                }

                return Some(Ok(path));
            }

            None
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    if part_numbers.is_empty() {
        bail!("{processor} has no part numbers");
    }

    Ok(part_numbers)
}

// TODO: Replace with roxmltree
fn get_xml_schema_uri(path: &Path) -> anyhow::Result<String> {
    let content = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&content);
    reader.config_mut().expand_empty_elements = true;

    loop {
        match reader.read_event()? {
            Event::Start(ref e) => {
                for attr in e.attributes().with_checks(false) {
                    let attr = attr?;

                    if attr.key.as_ref() == b"xsi:schemaLocation" {
                        let value = attr.unescape_value()?.to_string();

                        let parts = value.split_whitespace().collect::<Vec<_>>();

                        if parts.len() != 2 {
                            bail!(
                                "schemaLocation in {} does not have namespace and URI ({} parts)",
                                path.display(),
                                parts.len(),
                            );
                        }

                        return Ok(parts[1].to_string());
                    }
                }
            }

            Event::Eof => break,
            _ => {}
        }
    }

    bail!("Found no XML schema in {}", path.display());
}

fn require_string_attribute(
    doc: &Document<'_>,
    node: &Node<'_, '_>,
    attribute: &str,
) -> anyhow::Result<String> {
    let Some(attribute) = string_attribute(node, attribute) else {
        let start = doc.text_pos_at(node.range().start);
        let end = doc.text_pos_at(node.range().end);
        bail!("Attribute {attribute} is not available for node at {start} to {end}");
    };

    Ok(attribute.to_string())
}

fn string_attribute(node: &Node<'_, '_>, attribute: &str) -> Option<String> {
    node.attribute(attribute).map(String::from)
}

/// `false` if this node is a new line with some whitespace only.
fn skip_newline(node: &Node<'_, '_>) -> bool {
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:\n *)+$").unwrap());

    // If the node has children, then it is not a true new line.
    if node.has_children() {
        return true;
    }

    // Text nodes should be skipped if they are a new line
    if node.is_text() {
        return false;
    }

    node.text().filter(|&s| PATTERN.is_match(s)).is_none()
}

struct Stopwatch {
    start: Instant,
    section_start: Option<Instant>,
}

impl Stopwatch {
    fn new() -> Self {
        eprintln!("Starting timer");
        let start = Instant::now();
        Self {
            start,
            section_start: None,
        }
    }

    fn section(&mut self, status: &str) {
        let now = Instant::now();
        self.print_done(now);
        eprintln!("  {status}");
        self.section_start = Some(now);
    }

    fn stop(self) {
        let now = Instant::now();
        self.print_done(now);
        let total_elapsed = now - self.start;
        eprintln!("Total time: {:.3} seconds", total_elapsed.as_secs_f32());
    }

    fn print_done(&self, now: Instant) {
        if let Some(section_start) = self.section_start {
            let elapsed = now - section_start;
            eprintln!("    done in {:.3} seconds", elapsed.as_secs_f32());
        }
    }
}
