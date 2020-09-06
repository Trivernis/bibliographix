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
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let mut misc = Misc::new();

        misc.author = map.get(K_AUTHOR).cloned();
        misc.title = map.get(K_TITLE).cloned();
        misc.url = map.get(K_URL).cloned();
        misc.how_published = map.get(K_HOW_PUBLISHED).cloned();
        misc.date = map
            .get(K_DATE)
            .ok_or(missing_field!(K_DATE))
            .and_then(|d| parse_date(d))
            .ok();

        Ok(Box::new(misc))
    }
}
