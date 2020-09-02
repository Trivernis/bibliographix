use crate::bibliography::keys::{K_ADDRESS, K_AUTHOR, K_DATE, K_HOW_PUBLISHED, K_TITLE};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A booklet source where only the title can be known
#[derive(Clone, Debug)]
pub struct Booklet {
    pub title: String,
    pub author: Option<String>,
    pub how_published: Option<String>,
    pub address: Option<String>,
    pub date: Option<LocalDate>,
}

impl Booklet {
    /// Creates a new booklet with only the mandatory values
    pub fn new(title: String) -> Self {
        Self {
            title,
            author: None,
            how_published: None,
            address: None,
            date: None,
        }
    }
}

impl FromHashMap for Booklet {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let title = map.get(K_TITLE)?;
        let mut booklet = Booklet::new(title.clone());

        booklet.author = map.get(K_AUTHOR).cloned();
        booklet.how_published = map.get(K_HOW_PUBLISHED).cloned();
        booklet.address = map.get(K_ADDRESS).cloned();
        booklet.date = map.get(K_DATE).and_then(|d| parse_date(d));

        Some(Box::new(booklet))
    }
}
