use crate::bibliography::bibliography_entry::{BibliographyEntry, BibliographyEntryReference};
use crate::bibliography::keys::K_KEY;
use crate::bibliography::FromHashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A dictionary that contains all bibliography entries
#[derive(Clone, Debug)]
pub struct BibliographyDictionary {
    entries: HashMap<String, BibliographyEntryReference>,
}

impl BibliographyDictionary {
    /// Creates a new empty BibliographyDictionary
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Inserts a bibliography entry into the map
    pub fn insert(&mut self, entry: BibliographyEntry) {
        self.entries
            .insert(entry.key(), Arc::new(Mutex::new(entry)));
    }

    /// Inserts a bibliography entry represented as a HashMap
    pub fn insert_map(&mut self, map: &HashMap<String, String>) -> Result<(), String> {
        let key = map.get(K_KEY).ok_or(missing_field!(K_KEY))?;
        let entry = *BibliographyEntry::from_hash_map(map)?;
        self.entries
            .insert(key.clone(), Arc::new(Mutex::new(entry)));

        Ok(())
    }

    /// Returns the reference to the bibliography entry with the given key
    pub fn get(&self, key: &str) -> Option<BibliographyEntryReference> {
        self.entries.get(&key.to_string()).cloned()
    }
}
