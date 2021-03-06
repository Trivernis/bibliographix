use crate::bibliography::bib_types::misc::Misc;
use crate::bibliography::bib_types::BibliographyType;
use crate::bibliography::keys::{K_KEY, K_NOTE};
use crate::bibliography::FromHashMap;
use parking_lot::Mutex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::Arc;

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
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let key = map.get(K_KEY).ok_or(missing_field!(K_KEY))?;
        let bib_type = BibliographyType::from_hash_map(map)?;

        let mut entry = Self::new(key.clone());

        entry.note = map.get(K_NOTE).cloned();
        entry.bib_type = *bib_type;
        entry.raw_fields = map.clone();

        Ok(Box::new(entry))
    }
}
