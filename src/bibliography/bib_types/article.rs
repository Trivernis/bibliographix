use crate::bibliography::keys::{
    K_AUTHOR, K_DATE, K_JOURNAL, K_NUMBER, K_PAGES, K_TITLE, K_URL, K_VOLUME,
};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// An article source
#[derive(Clone, Debug)]
pub struct Article {
    pub author: String,
    pub title: String,
    pub journal: String,
    pub date: LocalDate,
    pub volume: Option<String>,
    pub number: Option<String>,
    pub pages: Option<String>,
    pub url: Option<String>,
}

impl Article {
    /// Creates a new article with the mandatory fields filled
    pub fn new(author: String, title: String, journal: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            journal,
            date,
            volume: None,
            number: None,
            pages: None,
            url: None,
        }
    }
}

impl FromHashMap for Article {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let author = map.get(K_AUTHOR)?;
        let title = map.get(K_TITLE)?;
        let journal = map.get(K_JOURNAL)?;
        let date = parse_date(map.get(K_DATE)?)?;
        let mut article = Self::new(author.clone(), title.clone(), journal.clone(), date);

        article.volume = map.get(K_VOLUME).cloned();
        article.number = map.get(K_NUMBER).cloned();
        article.number = map.get(K_NUMBER).cloned();
        article.pages = map.get(K_PAGES).cloned();
        article.url = map.get(K_URL).cloned();

        Some(Box::new(article))
    }
}
