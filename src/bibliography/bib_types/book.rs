use crate::bibliography::bib_types::LocalDate;

#[derive(Clone, Debug)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub date: LocalDate,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
    pub month: Option<String>,
    pub url: Option<String>,
}

impl Book {
    /// Creates a new book with the mandatory fields filled
    pub fn new(author: String, title: String, publisher: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            publisher,
            date,
            volume: None,
            series: None,
            address: None,
            edition: None,
            month: None,
            url: None,
        }
    }
}
