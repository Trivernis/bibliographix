use crate::bibliography::keys::{K_ACCESSED_AT, K_AUTHOR, K_DATE, K_TITLE, K_URL};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A website source that can only consists of an url
#[derive(Clone, Debug)]
pub struct Website {
    pub url: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub accessed_at: Option<LocalDate>,
    pub date: Option<LocalDate>,
}

impl Website {
    /// Creates a new website source
    pub fn new(url: String) -> Self {
        Self {
            url,
            title: None,
            author: None,
            accessed_at: None,
            date: None,
        }
    }
}

impl FromHashMap for Website {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let url = map.get(K_URL)?;
        let mut website = Website::new(url.clone());

        website.title = map.get(K_TITLE).cloned();
        website.author = map.get(K_AUTHOR).cloned();
        website.accessed_at = map.get(K_ACCESSED_AT).and_then(|d| parse_date(d));
        website.date = map.get(K_DATE).and_then(|d| parse_date(d));

        Some(Box::new(website))
    }
}
