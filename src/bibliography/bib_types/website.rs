use crate::bibliography::bib_types::LocalDate;

/// A website source that can only consists of an url
#[derive(Clone, Debug)]
pub struct Website {
    pub url: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub accessed_at: Option<LocalDate>,
    pub date: Option<String>,
}

impl Website {
    /// Creates a new website source
    pub fn new(url: String) -> Self {
        Self {
            url,
            title: None,
            author: None,
            accessed_at: None,
            date: None,
        }
    }
}
