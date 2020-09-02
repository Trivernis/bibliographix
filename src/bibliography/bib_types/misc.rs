use crate::bibliography::keys::{K_AUTHOR, K_DATE, K_HOW_PUBLISHED, K_TITLE, K_URL};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A source that does not fit any of the other types
#[derive(Clone, Debug)]
pub struct Misc {
    pub author: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub how_published: Option<String>,
    pub date: Option<LocalDate>,
}

impl Misc {
    /// Creates an empty Misc
    pub fn new() -> Self {
        Self {
            author: None,
            title: None,
            url: None,
            how_published: None,
            date: None,
        }
    }
}

impl FromHashMap for Misc {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let mut misc = Misc::new();

        if let Some(author) = map.get(K_AUTHOR) {
            misc.author = Some(author.clone())
        }
        if let Some(title) = map.get(K_TITLE) {
            misc.title = Some(title.clone())
        }
        if let Some(url) = map.get(K_URL) {
            misc.url = Some(url.clone())
        }
        if let Some(how_pub) = map.get(K_HOW_PUBLISHED) {
            misc.how_published = Some(how_pub.clone());
        }
        if let Some(date) = map.get(K_DATE) {
            misc.date = parse_date(date);
        }

        Some(Box::new(misc))
    }
}
