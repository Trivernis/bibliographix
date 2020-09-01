use crate::bibliography::bib_types::LocalDate;

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
