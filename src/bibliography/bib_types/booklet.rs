use crate::bibliography::bib_types::LocalDate;

/// A booklet source where only the title can be known
#[derive(Clone, Debug)]
pub struct Booklet {
    pub title: String,
    pub author: Option<String>,
    pub how_published: Option<String>,
    pub address: Option<String>,
    pub date: Option<LocalDate>,
}

impl Booklet {
    /// Creates a new booklet with only the mandatory values
    pub fn new(title: String) -> Self {
        Self {
            title,
            author: None,
            how_published: None,
            address: None,
            date: None,
        }
    }
}
