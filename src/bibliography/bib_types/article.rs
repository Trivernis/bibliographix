use crate::bibliography::bib_types::LocalDate;

/// An article source
#[derive(Clone, Debug)]
pub struct Article {
    pub author: String,
    pub title: String,
    pub journal: String,
    pub date: LocalDate,
    pub volume: Option<String>,
    pub number: Option<String>,
    pub pages: Option<String>,
}

impl Article {
    /// Creates a new article with the mandatory fields filled
    pub fn new(author: String, title: String, journal: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            journal,
            date,
            volume: None,
            number: None,
            pages: None,
        }
    }
}