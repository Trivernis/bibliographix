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
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub date: LocalDate,
    pub editor: Option<String>,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub position: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
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
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let author = map.get(K_AUTHOR).ok_or(missing_field!(K_AUTHOR))?;
        let title = map.get(K_TITLE).ok_or(missing_field!(K_TITLE))?;
        let publisher = map.get(K_PUBLISHER).ok_or(missing_field!(K_PUBLISHER))?;
        let date = parse_date(map.get(K_DATE).ok_or(missing_field!(K_DATE))?)?;
        let mut in_col = InCollection::new(author.clone(), title.clone(), publisher.clone(), date);

        in_col.editor = map.get(K_EDITOR).cloned();
        in_col.volume = map.get(K_VOLUME).cloned();
        in_col.series = map.get(K_SERIES).cloned();
        in_col.position = map.get(K_POSITION).cloned();
        in_col.address = map.get(K_ADDRESS).cloned();
        in_col.edition = map.get(K_EDITION).cloned();

        Ok(Box::new(in_col))
    }
}
