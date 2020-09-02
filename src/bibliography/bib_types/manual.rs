use crate::bibliography::keys::{K_ADDRESS, K_AUTHOR, K_DATE, K_EDITION, K_ORGANIZATION, K_TITLE};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A manual entry source
#[derive(Clone, Debug)]
pub struct Manual {
    pub title: String,
    pub author: Option<String>,
    pub organization: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
    pub date: Option<LocalDate>,
}

impl Manual {
    /// Creates a new manual source with only the mandatory fields filled
    pub fn new(title: String) -> Self {
        Self {
            title,
            author: None,
            organization: None,
            address: None,
            edition: None,
            date: None,
        }
    }
}

impl FromHashMap for Manual {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Option<Box<Self>> {
        let title = map.get(K_TITLE)?;
        let mut manual = Manual::new(title.clone());

        manual.author = map.get(K_AUTHOR).cloned();
        manual.organization = map.get(K_ORGANIZATION).cloned();
        manual.address = map.get(K_ADDRESS).cloned();
        manual.edition = map.get(K_EDITION).cloned();
        manual.date = map.get(K_DATE).and_then(|d| parse_date(d));

        Some(Box::new(manual))
    }
}
