use std::collections::HashMap;

pub mod bib_types;
pub mod bibliography_dict;
pub mod bibliography_entry;

/// A trait that provides the from_has_map function that can be used
/// to create a bibliography source type from a hashmap
pub trait FromHashMap {
    fn from_hash_map(map: &HashMap<String, String>) -> Option<Box<Self>>;
}
