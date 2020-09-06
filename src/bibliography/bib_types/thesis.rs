use crate::bibliography::keys::{K_ADDRESS, K_AUTHOR, K_DATE, K_SCHOOL, K_TITLE};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A thesis source entry
#[derive(Clone, Debug)]
pub struct Thesis {
    pub author: String,
    pub title: String,
    pub school: String,
    pub date: LocalDate,
    pub address: Option<String>,
}

impl Thesis {
    /// Creates a new thesis with only the mandatory fields filled
    pub fn new(author: String, title: String, school: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            school,
            date,
            address: None,
        }
    }
}

impl FromHashMap for Thesis {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let author = map.get(K_AUTHOR).ok_or(missing_field!(K_AUTHOR))?;
        let title = map.get(K_TITLE).ok_or(missing_field!(K_TITLE))?;
        let school = map.get(K_SCHOOL).ok_or(missing_field!(K_SCHOOL))?;
        let date = map
            .get(K_DATE)
            .ok_or(missing_field!(K_DATE))
            .and_then(|d| parse_date(d))?;
        let mut thesis = Thesis::new(author.clone(), title.clone(), school.clone(), date.clone());

        thesis.address = map.get(K_ADDRESS).cloned();

        Ok(Box::new(thesis))
    }
}
