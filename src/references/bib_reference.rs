#[derive(Clone, Debug)]
pub struct BibRef {
    key: String,
}

impl BibRef {
    /// Creates a new BibRef with a given key
    pub fn new(key: String) -> Self {
        Self {
            key
        }
    }
}