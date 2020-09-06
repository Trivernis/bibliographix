use crate::bibliography::keys::{
    K_ADDRESS, K_AUTHOR, K_DATE, K_EDITION, K_POSITION, K_PUBLISHER, K_SERIES, K_TITLE, K_VOLUME,
};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// Source that is part of a book
#[derive(Clone, Debug)]
pub struct InBook {
    pub author: String,
    pub title: String,
    pub position: String,
    pub publisher: String,
    pub date: LocalDate,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
}

impl InBook {
    /// Creates a new InBook source with only the mandatory values filled
    pub fn new(
        author: String,
        title: String,
        position: String,
        publisher: String,
        date: LocalDate,
    ) -> Self {
        Self {
            author,
            title,
            position,
            publisher,
            date,
            volume: None,
            series: None,
            address: None,
            edition: None,
        }
    }
}

impl FromHashMap for InBook {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let author = map.get(K_AUTHOR).ok_or(missing_field!(K_AUTHOR))?;
        let title = map.get(K_TITLE).ok_or(missing_field!(K_TITLE))?;
        let position = map.get(K_POSITION).ok_or(missing_field!(K_POSITION))?;
        let publisher = map.get(K_PUBLISHER).ok_or(missing_field!(K_PUBLISHER))?;
        let date = parse_date(map.get(K_DATE).ok_or(missing_field!(K_DATE))?)?;
        let mut in_book = InBook::new(
            author.clone(),
            title.clone(),
            position.clone(),
            publisher.clone(),
            date,
        );

        in_book.volume = map.get(K_VOLUME).cloned();
        in_book.series = map.get(K_SERIES).cloned();
        in_book.address = map.get(K_ADDRESS).cloned();
        in_book.edition = map.get(K_EDITION).cloned();

        Ok(Box::new(in_book))
    }
}
