use crate::bibliography::keys::{
    K_ADDRESS, K_AUTHOR, K_DATE, K_EDITION, K_PUBLISHER, K_SERIES, K_TITLE, K_URL, K_VOLUME,
};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub date: LocalDate,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
    pub url: Option<String>,
}

impl Book {
    /// Creates a new book with the mandatory fields filled
    pub fn new(author: String, title: String, publisher: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            publisher,
            date,
            volume: None,
            series: None,
            address: None,
            edition: None,
            url: None,
        }
    }
}

impl FromHashMap for Book {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let author = map.get(K_AUTHOR).ok_or(missing_field!(K_AUTHOR))?;
        let title = map.get(K_TITLE).ok_or(missing_field!(K_TITLE))?;
        let publisher = map.get(K_PUBLISHER).ok_or(missing_field!(K_PUBLISHER))?;
        let date = parse_date(map.get(K_DATE).ok_or(missing_field!(K_DATE))?)?;
        let mut book = Book::new(author.clone(), title.clone(), publisher.clone(), date);

        book.volume = map.get(K_VOLUME).cloned();
        book.series = map.get(K_SERIES).cloned();
        book.address = map.get(K_ADDRESS).cloned();
        book.edition = map.get(K_EDITION).cloned();
        book.url = map.get(K_URL).cloned();

        Ok(Box::new(book))
    }
}
