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

    /// Assigns the corresponding bib entry to each bib reference
    pub fn assign_entries_to_references(&self) {
        let entry_dict = self.entry_dictionary.lock().unwrap();
        let mut root_anchor = self.root_ref_anchor.lock().unwrap();
        root_anchor.flatten();
        let entries = root_anchor.references();
        entries.iter().for_each(|e| {
            if let Some(bib) = entry_dict.get(e.key()) {
                e.anchor().lock().unwrap().entry = Some(bib)
            }
        })
    }
}
