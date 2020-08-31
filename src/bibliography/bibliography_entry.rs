use crate::bibliography::bib_types::BibliographyType;
use std::sync::{Arc, Mutex};

/// A single bibliography entry
#[derive(Clone, Debug)]
pub struct BibliographyEntry {
    key: String,
    pub note: Option<String>,
    pub bib_type: BibliographyType,
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
