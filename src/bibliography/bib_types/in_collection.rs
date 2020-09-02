use crate::bibliography::keys::{
    K_ADDRESS, K_AUTHOR, K_DATE, K_EDITION, K_EDITOR, K_POSITION, K_PUBLISHER, K_SERIES, K_TITLE,
    K_VOLUME,
};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A source that is in a collection
#[derive(Clone, Debug)]
pub struct InCollection {
    author: String,
    title: String,
    publisher: String,
    date: LocalDate,
    editor: Option<String>,
    volume: Option<String>,
    series: Option<String>,
    position: Option<String>,
    address: Option<String>,
    edition: Option<String>,
}

impl InCollection {
    /// Creates a new InCollection source with only the mandatory fields filled
    pub fn new(author: String, title: String, publisher: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            publisher,
            date,
            editor: None,
            volume: None,
            series: None,
            position: None,
            address: None,
            edition: None,
        }
    }
}

impl FromHashMap for InCollection {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let author = map.get(K_AUTHOR)?;
        let title = map.get(K_TITLE)?;
        let publisher = map.get(K_PUBLISHER)?;
        let date = parse_date(map.get(K_DATE)?)?;
        let mut in_col = InCollection::new(author.clone(), title.clone(), publisher.clone(), date);

        if let Some(editor) = map.get(K_EDITOR) {
            in_col.editor = Some(editor.clone());
        }
        if let Some(volume) = map.get(K_VOLUME) {
            in_col.volume = Some(volume.clone());
        }
        if let Some(series) = map.get(K_SERIES) {
            in_col.series = Some(series.clone());
        }
        if let Some(position) = map.get(K_POSITION) {
            in_col.position = Some(position.clone());
        }
        if let Some(address) = map.get(K_ADDRESS) {
            in_col.address = Some(address.clone());
        }
        if let Some(edition) = map.get(K_EDITION) {
            in_col.edition = Some(edition.clone());
        }

        Some(Box::new(in_col))
    }
}
