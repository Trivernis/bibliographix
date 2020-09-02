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
    fn it_assigns_bib_entries_to_references() {
        let manager = BibManager::new();
        let ref1_1 = BibRef::new("ref1".to_string());
        let anchor1 = ref1_1.anchor();
        manager.root_ref_anchor().lock().unwrap().insert(ref1_1);
        let ref1_2 = BibRef::new("ref1".to_string());
        let anchor2 = ref1_2.anchor();
        manager.root_ref_anchor().lock().unwrap().insert(ref1_2);
        let ref3 = BibRef::new("ref2".to_string());
        let anchor3 = ref3.anchor();
        manager.root_ref_anchor().lock().unwrap().insert(ref3);
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "ref1".to_string());
        map.insert("type".to_string(), "article".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("journal".to_string(), "test_journal".to_string());
        map.insert("date".to_string(), "01.09.2020".to_string());
        manager.entry_dictionary().lock().unwrap().insert_map(&map);
        manager.assign_entries_to_references();

        assert!(anchor1.lock().unwrap().entry.is_some());
        assert!(anchor2.lock().unwrap().entry.is_some());
        assert!(anchor3.lock().unwrap().entry.is_none());
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

    #[test]
    fn it_creates_books_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "book".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("publisher".to_string(), "test_publisher".to_string());
        map.insert("date".to_string(), "01.09.2020".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "book".to_string())
    }

    #[test]
    fn it_creates_booklet_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("type".to_string(), "booklet".to_string());
        map.insert("date".to_string(), "01.09.2020".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "booklet".to_string())
    }

    #[test]
    fn it_creates_in_book_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("type".to_string(), "in_book".to_string());
        map.insert("publisher".to_string(), "test_publisher".to_string());
        map.insert("position".to_string(), "page 2".to_string());
        map.insert("date".to_string(), "01.09.2020".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "in_book".to_string())
    }

    #[test]
    fn it_creates_in_collection_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("publisher".to_string(), "test_publisher".to_string());
        map.insert("type".to_string(), "in_collection".to_string());
        map.insert("date".to_string(), "01.09.2020".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "in_collection".to_string())
    }

    #[test]
    fn it_creates_manuals_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("title".to_string(), "test_title".to_string());
        map.insert("type".to_string(), "manual".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "manual".to_string())
    }

    #[test]
    fn it_creates_misc_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "misc".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "misc".to_string())
    }

    #[test]
    fn it_creates_repos_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "repository".to_string());
        map.insert("author".to_string(), "trivernis".to_string());
        map.insert("title".to_string(), "snekdown".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "repository".to_string())
    }

    #[test]
    fn it_creates_tech_reports_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "tech_report".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test tech".to_string());
        map.insert("institution".to_string(), "test".to_string());
        map.insert("note".to_string(), "This is a test".to_string());
        map.insert("date".to_string(), "02.09.2020".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "tech_report".to_string())
    }

    #[test]
    fn it_creates_thesis_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "thesis".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test".to_string());
        map.insert("school".to_string(), "test".to_string());
        map.insert("note".to_string(), "This is a test".to_string());
        map.insert("date".to_string(), "02.09.2020".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "thesis".to_string())
    }

    #[test]
    fn it_creates_unpublished_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "unpublished".to_string());
        map.insert("author".to_string(), "test".to_string());
        map.insert("title".to_string(), "test".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "unpublished".to_string())
    }

    #[test]
    fn it_creates_websites_from_hashmaps() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("key".to_string(), "test_entry".to_string());
        map.insert("type".to_string(), "website".to_string());
        map.insert("url".to_string(), "https://github.com".to_string());
        map.insert("note".to_string(), "This is a test".to_string());

        let entry = BibliographyEntry::from_hash_map(&map).unwrap();
        assert_eq!(entry.bib_type.name(), "website".to_string())
    }
}
