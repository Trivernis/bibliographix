use crate::bibliography::keys::{K_ACCESSED_AT, K_AUTHOR, K_CMS, K_LICENSE, K_TITLE, K_URL};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A repository source that represents any git repository or similar
/// structures
#[derive(Clone, Debug)]
pub struct Repository {
    pub author: String,
    pub title: String,
    pub url: Option<String>,
    pub license: Option<String>,
    pub cms: Option<String>,
    pub accessed_at: Option<LocalDate>,
}

impl Repository {
    /// Creates a new repository source with only the mandatory fields filled
    pub fn new(author: String, title: String) -> Self {
        Self {
            author,
            title,
            url: None,
            license: None,
            cms: None,
            accessed_at: None,
        }
    }
}

impl FromHashMap for Repository {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let author = map.get(K_AUTHOR).ok_or(missing_field!(K_AUTHOR))?;
        let title = map.get(K_TITLE).ok_or(missing_field!(K_TITLE))?;
        let mut repo = Repository::new(author.clone(), title.clone());

        repo.url = map.get(K_URL).cloned();
        repo.license = map.get(K_LICENSE).cloned();
        repo.cms = map.get(K_CMS).cloned();
        repo.accessed_at = map
            .get(K_ACCESSED_AT)
            .ok_or(missing_field!(K_ACCESSED_AT))
            .and_then(|d| parse_date(d))
            .ok();

        Ok(Box::new(repo))
    }
}
