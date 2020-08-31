use crate::bibliography::bib_types::LocalDate;

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
