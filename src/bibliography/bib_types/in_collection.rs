use crate::bibliography::bib_types::LocalDate;

/// A source that is in a collection
#[derive(Clone, Debug)]
pub struct InCollection {
    author: String,
    title: String,
    publisher: String,
    date: LocalDate,
    editor: Option<String>,
    volume: Option<String>,
    series: Option<String>,
    chapter: Option<String>,
    pages: Option<String>,
    address: Option<String>,
    edition: Option<String>,
}

impl InCollection {
    /// Creates a new InCollection source with only the mandatory fields filled
    pub fn new(author: String, title: String, publisher: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            publisher,
            date,
            editor: None,
            volume: None,
            series: None,
            chapter: None,
            pages: None,
            address: None,
            edition: None,
        }
    }
}
