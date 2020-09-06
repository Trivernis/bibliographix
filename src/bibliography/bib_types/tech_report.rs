use crate::bibliography::keys::{K_ADDRESS, K_AUTHOR, K_DATE, K_INSTITUTION, K_NUMBER, K_TITLE};
use crate::bibliography::FromHashMap;
use crate::utils::date::{parse_date, LocalDate};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

/// A tech report for example a white paper
#[derive(Clone, Debug)]
pub struct TechReport {
    pub author: String,
    pub title: String,
    pub institution: String,
    pub date: LocalDate,
    pub number: Option<String>,
    pub address: Option<String>,
}

impl TechReport {
    /// Creates a new tech report with only the mandatory fields filled
    pub fn new(author: String, title: String, institution: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            institution,
            date,
            number: None,
            address: None,
        }
    }
}

impl FromHashMap for TechReport {
    fn from_hash_map(map: &HashMap<String, String, RandomState>) -> Result<Box<Self>, String> {
        let author = map.get(K_AUTHOR).ok_or(missing_field!(K_AUTHOR))?;
        let title = map.get(K_TITLE).ok_or(missing_field!(K_TITLE))?;
        let institution = map
            .get(K_INSTITUTION)
            .ok_or(missing_field!(K_INSTITUTION))?;
        let date = map
            .get(K_DATE)
            .ok_or(missing_field!(K_DATE))
            .and_then(|d| parse_date(d))?;
        let mut tech_report =
            TechReport::new(author.clone(), title.clone(), institution.clone(), date);

        tech_report.number = map.get(K_NUMBER).cloned();
        tech_report.address = map.get(K_ADDRESS).cloned();

        Ok(Box::new(tech_report))
    }
}
