use crate::utils::date::LocalDate;

/// Source that is part of a book
#[derive(Clone, Debug)]
pub struct InBook {
    pub author: String,
    pub title: String,
    pub position: String,
    pub publisher: String,
    pub date: LocalDate,
    pub volume: Option<String>,
    pub series: Option<String>,
    pub address: Option<String>,
    pub edition: Option<String>,
}

impl InBook {
    /// Creates a new InBook source with only the mandatory values filled
    pub fn new(
        author: String,
        title: String,
        position: String,
        publisher: String,
        date: LocalDate,
    ) -> Self {
        Self {
            author,
            title,
            position,
            publisher,
            date,
            volume: None,
            series: None,
            address: None,
            edition: None,
        }
    }
}
