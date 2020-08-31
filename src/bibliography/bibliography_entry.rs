use std::sync::{Arc, Mutex};
use crate::bibliography::bib_types::BibliographyType;

/// A single bibliography entry
#[derive(Clone, Debug)]
pub struct BibliographyEntry {
    key: String,
    note: Option<String>,
    bib_type: BibliographyType,
}

pub type BibliographyEntryReference = Arc<Mutex<BibliographyEntry>>;


impl BibliographyEntry {
    /// Creates a new bibliography entry with the given key
    pub fn new(key: String) -> Self {
        Self {
            key,
            note: None,
            bib_type: BibliographyType::Misc,
        }
    }

    /// Returns the key of the bibliography entry
    pub fn key(&self) -> String {
        self.key.clone()
    }
}

