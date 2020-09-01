pub mod bib_manager;
pub mod bibliography;
pub mod references;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::bib_manager::BibManager;
    use crate::bibliography::bibliography_entry::BibliographyEntry;
    use crate::bibliography::FromHashMap;
    use crate::references::bib_reference::BibRef;
    use std::collections::HashMap;

    #[test]
    fn it_inserts_and_flattens() {
        let manager = BibManager::new();
        let root_anchor = manager.root_ref_anchor();
        let mut root_anchor = root_anchor.lock().unwrap();
        root_anchor.insert(BibRef::new("test".to_string()));
        let child_anchor = root_anchor.create_anchor();
        child_anchor
            .lock()
            .unwrap()
            .insert(BibRef::new("test2".to_string()));
        child_anchor
            .lock()
            .unwrap()
            .insert(BibRef::new("test3".to_string()));
        root_anchor.flatten();

        assert_eq!(root_anchor.references().len(), 3)
    }

    #[test]
    fn it_creates_articles_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "article".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("journal".to_string(), "test_journal".to_string());
        map.insert("date".to_string(), "01.09.2020".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "article".to_string())
    }
}
