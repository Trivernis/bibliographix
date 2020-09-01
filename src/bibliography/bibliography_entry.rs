use crate::bibliography::bib_types::misc::Misc;
use crate::bibliography::bib_types::BibliographyType;
use crate::bibliography::FromHashMap;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A single bibliography entry
#[derive(Clone, Debug)]
pub struct BibliographyEntry {
    key: String,
    pub note: Option<String>,
    pub bib_type: BibliographyType,
    pub raw_fields: HashMap<String, String>,
}

pub type BibliographyEntryReference = Arc<Mutex<BibliographyEntry>>;

impl BibliographyEntry {
    /// Creates a new bibliography entry with the given key
    pub fn new(key: String) -> Self {
        Self {
            key,
            note: None,
            bib_type: BibliographyType::Misc(Misc::new()),
            raw_fields: HashMap::new(),
        }
    }

    /// Returns the key of the bibliography entry
    pub fn key(&self) -> String {
        self.key.clone()
    }
}

impl FromHashMap for BibliographyEntry {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        if let Some(key) = map.get("key") {
            if let Some(bib_type) = BibliographyType::from_hash_map(map) {
                let mut entry = Self::new(key.clone());

                if let Some(note) = map.get("note") {
                    entry.note = Some(note.clone())
                }
                entry.bib_type = *bib_type;
                entry.raw_fields = map.clone();

                Some(Box::new(entry))
            } else {
                None
            }
        } else {
            None
        }
    }
}
