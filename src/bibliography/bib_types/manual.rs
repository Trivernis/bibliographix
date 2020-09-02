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

        if let Some(author) = map.get(K_AUTHOR) {
            manual.author = Some(author.clone());
        }
        if let Some(org) = map.get(K_ORGANIZATION) {
            manual.organization = Some(org.clone());
        }
        if let Some(address) = map.get(K_ADDRESS) {
            manual.address = Some(address.clone());
        }
        if let Some(edition) = map.get(K_EDITION).and_then(|s| Some(s.clone())) {
            Some(edition.clone());
        }
        if let Some(date) = map.get(K_DATE) {
            manual.date = parse_date(date);
        }

        Some(Box::new(manual))
    }
}
