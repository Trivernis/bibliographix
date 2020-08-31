use crate::bibliography::bibliography_dict::BibliographyDictionary;
use crate::references::anchor::BibListAnchor;
use std::sync::{Arc, Mutex};

/// The root manager for that should be used for further reference operations that
/// go beyond insertion.
#[derive(Clone, Debug)]
pub struct BibManager {
    root_ref_anchor: Arc<Mutex<BibListAnchor>>,
    entry_dictionary: Arc<Mutex<BibliographyDictionary>>,
}

impl BibManager {
    /// Creates a new BibRefManager with an empty root anchor
    pub fn new() -> Self {
        Self {
            root_ref_anchor: Arc::new(Mutex::new(BibListAnchor::new())),
            entry_dictionary: Arc::new(Mutex::new(BibliographyDictionary::new())),
        }
    }

    /// Returns the BibRefManagers root anchor that.
    pub fn root_ref_anchor(&self) -> Arc<Mutex<BibListAnchor>> {
        Arc::clone(&self.root_ref_anchor)
    }

    /// Returns the reference to the entry dictionary
    pub fn entry_dictionary(&self) -> Arc<Mutex<BibliographyDictionary>> {
        Arc::clone(&self.entry_dictionary)
    }
}
