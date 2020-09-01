use crate::utils::date::LocalDate;

/// A source that is not formally published
#[derive(Clone, Debug)]
pub struct Unpublished {
    pub author: String,
    pub title: String,
    pub date: Option<LocalDate>,
}

impl Unpublished {
    /// Creates a new source of type unpublished with only the mandatory fields filled
    pub fn new(author: String, title: String) -> Self {
        Self {
            author,
            title,
            date: None,
        }
    }
}
