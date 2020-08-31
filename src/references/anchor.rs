use crate::references::bib_reference::BibRef;
use std::sync::{Arc, Mutex};

/// A bib list anchor that can be used to concurrently insert entries into a list
#[derive(Clone, Debug)]
pub struct BibListAnchor {
    entries: Vec<BibListEntry>,
}

/// Enum that represents a single entry of a bib list
#[derive(Clone, Debug)]
enum BibListEntry {
    Ref(BibRef),
    Anchor(Arc<Mutex<BibListAnchor>>),
}

impl BibListAnchor {
    /// Creates a new empty BibListAnchor.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Inserts a reference at the current position
    pub fn insert(&mut self, bib_ref: BibRef) {
        self.entries.push(BibListEntry::Ref(bib_ref))
    }

    /// Creates a new anchor at the current position and inserts it into the entry vec.
    pub fn create_anchor(&mut self) -> Arc<Mutex<BibListAnchor>> {
        let anchor = Arc::new(Mutex::new(BibListAnchor::new()));
        self.entries.push(BibListEntry::Anchor(Arc::clone(&anchor)));

        anchor
    }

    /// Flattens the inner entry structure by inserting the elements of child anchors for every
    /// anchor in the vector
    pub fn flatten(&mut self) {
        let mut new_entries = Vec::with_capacity(self.entries.len());
        self.entries.iter_mut().for_each(|e| match e {
            BibListEntry::Anchor(a) => {
                let mut anchor = a.lock().unwrap();
                anchor.flatten();
                new_entries.append(&mut anchor.entries);
            }
            BibListEntry::Ref(bib_ref) => new_entries.push(BibListEntry::Ref(bib_ref.clone())),
        });

        self.entries = new_entries;
    }

    /// Returns all references that are contained in the entry list
    pub fn references(&self) -> Vec<BibRef> {
        self.entries
            .iter()
            .filter_map(|e| {
                if let BibListEntry::Ref(r) = e {
                    Some(r.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
