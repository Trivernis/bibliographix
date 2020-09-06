use std::collections::HashMap;

macro_rules! missing_field {
    ($e:expr) => {
        format!("Missing field '{}'", $e)
    };
}

pub mod bib_types;
pub mod bibliography_dict;
pub mod bibliography_entry;
pub mod keys;

/// A trait that provides the from_has_map function that can be used
/// to create a bibliography source type from a hashmap
pub trait FromHashMap {
    fn from_hash_map(map: &HashMap<String, String>) -> Result<Box<Self>, String>;
}
