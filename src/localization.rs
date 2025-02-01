use crate::vio::Identifier;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Localization {
    language: String,
    item_names: HashMap<String, String>,
    block_names: HashMap<String, String>,
    customs: HashMap<String, String>,
}

impl Localization {
    /// New localization for a certain language (id like en_US).
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            item_names: HashMap::new(),
            block_names: HashMap::new(),
            customs: HashMap::new(),
        }
    }

    pub fn add_block_name(&mut self, identifier: Identifier, name: impl Into<String>) {
        self.block_names.insert(identifier.render(), name.into());
    }

    pub fn add_item_name(&mut self, identifier: Identifier, name: impl Into<String>) {
        self.item_names.insert(identifier.render(), name.into());
    }

    pub fn add_custom(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.customs.insert(key.into(), value.into());
    }

    fn serialize(&self) -> String {
        let mut serialized = String::new();

        for (id, value) in self.item_names.clone() {
            serialized.push_str(&format!("item.{}.name={}", id, value))
        }

        for (id, value) in self.block_names.clone() {
            serialized.push_str(&format!("tile.{}.name={}", id, value))
        }

        for (id, value) in self.customs.clone() {
            serialized.push_str(&format!("{}={}", id, value))
        }

        serialized
    }

    pub fn build(&self, folder: String) {
        let path = PathBuf::from(format!("{folder}/{}.lang", self.language));

        fs::write(path, self.serialize()).unwrap();
    }
}
