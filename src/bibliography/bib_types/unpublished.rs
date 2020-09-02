use crate::bibliography::keys::{K_AUTHOR, K_DATE, K_TITLE};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A source that is not formally published
#[derive(Clone, Debug)]
pub struct Unpublished {
    pub author: String,
    pub title: String,
    pub date: Option<LocalDate>,
}

impl Unpublished {
    /// Creates a new source of type unpublished with only the mandatory fields filled
    pub fn new(author: String, title: String) -> Self {
        Self {
            author,
            title,
            date: None,
        }
    }
}

impl FromHashMap for Unpublished {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let author = map.get(K_AUTHOR)?;
        let title = map.get(K_TITLE)?;
        let mut unpub = Unpublished::new(author.clone(), title.clone());

        unpub.date = map.get(K_DATE).and_then(|d| parse_date(d));

        Some(Box::new(unpub))
    }
}
