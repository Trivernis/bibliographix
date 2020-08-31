use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bibliography::bibliography_entry::{BibliographyEntryReference, BibliographyEntry};

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
        self.entries.insert(entry.key(), Arc::new(Mutex::new(entry)));
    }

    /// Returns the reference to the bibliography entry with the given key
    pub fn get(&mut self, key: &str) -> Option<BibliographyEntryReference> {
        if let Some(entry) = self.entries.get(&key.to_string()) {
            Some(Arc::clone(entry))
        } else {
            None
        }
    }
}


