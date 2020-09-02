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
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let author = map.get(K_AUTHOR)?;
        let title = map.get(K_TITLE)?;
        let publisher = map.get(K_PUBLISHER)?;
        let date = parse_date(map.get(K_DATE)?)?;
        let mut book = Book::new(author.clone(), title.clone(), publisher.clone(), date);

        if let Some(volume) = map.get(K_VOLUME) {
            book.volume = Some(volume.clone());
        }
        if let Some(series) = map.get(K_SERIES) {
            book.series = Some(series.clone());
        }
        if let Some(address) = map.get(K_ADDRESS) {
            book.address = Some(address.clone());
        }
        if let Some(edition) = map.get(K_EDITION) {
            book.edition = Some(edition.clone());
        }
        if let Some(url) = map.get(K_URL) {
            book.url = Some(url.clone());
        }

        Some(Box::new(book))
    }
}
