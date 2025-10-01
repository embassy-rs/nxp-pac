use std::{fs, path::Path};

use anyhow::bail;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};

#[derive(Debug)]
pub struct PartNumber {
    /// The part number id.
    pub id: String,

    pub db_links: Vec<DbLink>,
}

impl PartNumber {
    pub fn matches_uri(uri: &str) -> bool {
        matches!(
            uri,
            "http://swtools.freescale.net/XSD/part_number/2.0/part_number.xsd"
                | "http://swtools.freescale.net/XSD/part_number/4.0/part_number.xsd"
        )
    }

    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut reader = Reader::from_str(&content);
        let config = reader.config_mut();
        config.expand_empty_elements = true;

        let mut id = None;
        let mut db_links = Vec::new();

        loop {
            match reader.read_event()? {
                Event::Start(ref e) => match e.name().as_ref() {
                    b"part:part_number" => {
                        for attr in e.attributes() {
                            let attr = attr?;

                            if attr.key.as_ref() == b"id" {
                                if id.is_some() {
                                    bail!("part_id is defined twice");
                                }

                                id = Some(attr.unescape_value()?.to_string());
                            }
                        }
                    }

                    b"db_link" => db_links.push(DbLink::read(e)?),

                    _ => {}
                },

                Event::Eof => break,

                _ => {}
            }
        }

        let Some(id) = id else {
            bail!("part_number definition had no id field");
        };

        Ok(Self { id, db_links })
    }

    /// Get link to the pins model.
    pub fn pins_model(&self) -> Option<String> {
        self.db_links
            .iter()
            .find(|link| link.r#type == "pins_model")
            .map(|link| link.link.to_string())
    }

    // TODO: clocks, cores
}

#[derive(Debug)]
pub struct DbLink {
    pub r#type: String,
    pub link: String,
    #[allow(unused)]
    pub format_version: String,
    #[allow(unused)]
    pub description: String,
}

impl DbLink {
    fn read(e: &BytesStart<'_>) -> anyhow::Result<Self> {
        let mut r#type = None;
        let mut link = None;
        let mut format_version = None;
        let mut description = None;

        for attr in e.attributes() {
            let attr = attr?;

            match attr.key.as_ref() {
                b"type" => {
                    if r#type.is_some() {
                        bail!("type is defined twice");
                    }

                    r#type = Some(attr.unescape_value()?.to_string());
                }

                b"link" => {
                    if link.is_some() {
                        bail!("link is defined twice");
                    }

                    link = Some(attr.unescape_value()?.to_string());
                }

                b"format_version" => {
                    if format_version.is_some() {
                        bail!("format_version is defined twice");
                    }

                    format_version = Some(attr.unescape_value()?.to_string());
                }

                b"description" => {
                    if description.is_some() {
                        bail!("description is defined twice");
                    }

                    description = Some(attr.unescape_value()?.to_string());
                }

                _ => {}
            }
        }

        let Some(r#type) = r#type else {
            bail!("DB link has no type");
        };

        let Some(link) = link else {
            bail!("DB link has no link");
        };

        let Some(format_version) = format_version else {
            bail!("DB link has no format_version");
        };

        let Some(description) = description else {
            bail!("DB link has no description");
        };

        Ok(DbLink {
            r#type,
            link,
            format_version,
            description,
        })
    }
}
