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
        }
    }
}

impl FromHashMap for Article {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let author = map.get("author")?;
        let title = map.get("title")?;
        let journal = map.get("journal")?;
        let date = parse_date(map.get("date")?)?;
        let mut article = Self::new(author.clone(), title.clone(), journal.clone(), date);

        if let Some(volume) = map.get("volume") {
            article.volume = Some(volume.clone());
        }
        if let Some(number) = map.get("number") {
            article.number = Some(number.clone());
        }
        if let Some(pages) = map.get("pages") {
            article.pages = Some(pages.clone());
        }

        Some(Box::new(article))
    }
}
