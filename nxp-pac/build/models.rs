use std::{
    collections::BTreeMap,
    fmt, fs, io,
    path::{Component, Path},
};

use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, Visitor},
};

pub struct Entry {
    pub source: Option<String>,
    pub driver: String,
}

pub struct Catalog {
    pub entries: BTreeMap<String, Entry>,
}

struct Declaration {
    id: String,
    source: Option<String>,
    driver: String,
}

// Keep the catalog helper usable without serde's derive macro/renderer graph in
// legacy builds. A Value map would silently overwrite duplicate JSON properties.
impl<'de> Deserialize<'de> for Declaration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DeclarationVisitor;
        impl<'de> Visitor<'de> for DeclarationVisitor {
            type Value = Declaration;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a model declaration with id, source and driver")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let (mut id, mut source, mut driver) = (None, None, None);
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        "source" => {
                            if source.is_some() {
                                return Err(de::Error::duplicate_field("source"));
                            }
                            source = Some(map.next_value()?);
                        }
                        "driver" => {
                            if driver.is_some() {
                                return Err(de::Error::duplicate_field("driver"));
                            }
                            driver = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(
                                &key,
                                &["id", "source", "driver"],
                            ));
                        }
                    }
                }
                Ok(Declaration {
                    id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                    source: source.ok_or_else(|| de::Error::missing_field("source"))?,
                    driver: driver.ok_or_else(|| de::Error::missing_field("driver"))?,
                })
            }
        }
        deserializer.deserialize_map(DeclarationVisitor)
    }
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn relative(name: &str) -> bool {
    !name.is_empty()
        && !name.contains(['\\', ':'])
        && Path::new(name)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

impl Catalog {
    pub fn load(inputs: &Path) -> io::Result<Self> {
        let path = inputs.join("peripherals/models.json");
        let declarations: Vec<Declaration> = serde_json::from_slice(&fs::read(&path)?)
            .map_err(|error| invalid(format!("{}: {error}", path.display())))?;
        let mut entries = BTreeMap::new();
        for declaration in declarations {
            let Declaration { id, source, driver } = declaration;
            if !relative(&id) || !relative(&driver) {
                return Err(invalid(format!(
                    "invalid model/driver identity: {id} / {driver}"
                )));
            }
            if let Some(source) = &source
                && (!relative(source) || !source.ends_with(".yaml"))
            {
                return Err(invalid(format!(
                    "source for {id} must be a contained YAML path or null"
                )));
            }
            if entries
                .insert(id.clone(), Entry { source, driver })
                .is_some()
            {
                return Err(invalid(format!("duplicate model identity: {id}")));
            }
        }
        Ok(Self { entries })
    }
}
