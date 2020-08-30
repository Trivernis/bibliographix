pub mod references;

#[cfg(test)]
mod tests {
    use crate::references::reference_manager::BibRefManager;
    use crate::references::bib_reference::BibRef;

    #[test]
    fn it_inserts_and_flattens() {
        let manager = BibRefManager::new();
        let root_anchor = manager.root_anchor();
        let mut root_anchor = root_anchor.lock().unwrap();
        root_anchor.insert(BibRef::new("test".to_string()));
        let child_anchor = root_anchor.create_anchor();
        child_anchor.lock().unwrap().insert(BibRef::new("test2".to_string()));
        child_anchor.lock().unwrap().insert(BibRef::new("test3".to_string()));
        root_anchor.flatten();

        assert_eq!(root_anchor.references().len(), 3)
    }
}
