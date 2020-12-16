use crate::bibliography::bibliography_entry::BibliographyEntryReference;
use parking_lot::Mutex;
use std::sync::Arc;

/// A reference to a bibliography entry
#[derive(Clone, Debug)]
pub struct BibRef {
    key: String,
    anchor: Arc<Mutex<BibRefAnchor>>,
}

/// An anchor of a BibRef that can be used in a DOM to remember the place of a BibRef
/// and to access the corresponding bibliography entry.
#[derive(Clone, Debug)]
pub struct BibRefAnchor {
    pub entry: Option<BibliographyEntryReference>,
}

impl BibRef {
    /// Creates a new BibRef with a given key
    pub fn new(key: String) -> Self {
        Self {
            key,
            anchor: Arc::new(Mutex::new(BibRefAnchor { entry: None })),
        }
    }

    /// Returns the anchor of the BibRef
    pub fn anchor(&self) -> Arc<Mutex<BibRefAnchor>> {
        Arc::clone(&self.anchor)
    }

    /// Returns the key of the BibRef
    pub fn key(&self) -> &String {
        &self.key
    }
}
