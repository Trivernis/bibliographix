use std::sync::{Arc, Mutex};
use crate::references::anchor::BibListAnchor;

/// The root manager for references that should be used for further reference operations that
/// go beyond insertion.
#[derive(Clone, Debug)]
pub struct BibRefManager {
    root_anchor: Arc<Mutex<BibListAnchor>>,
}

impl BibRefManager {
    /// Creates a new BibRefManager with an empty root anchor
    pub fn new() -> Self {
        Self {
            root_anchor: Arc::new(Mutex::new(BibListAnchor::new()))
        }
    }

    /// Returns the BibRefManagers root anchor that.
    pub fn root_anchor(&self) -> Arc<Mutex<BibListAnchor>> {
        Arc::clone(&self.root_anchor)
    }
}