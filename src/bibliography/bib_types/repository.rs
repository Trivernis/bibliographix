use crate::utils::date::LocalDate;

/// A repository source that represents any git repository or similar
/// structures
#[derive(Clone, Debug)]
pub struct Repository {
    pub author: String,
    pub title: String,
    pub url: Option<String>,
    pub license: Option<String>,
    pub cms: Option<String>,
    pub accessed_at: Option<LocalDate>,
}

impl Repository {
    /// Creates a new repository source with only the mandatory fields filled
    pub fn new(author: String, title: String) -> Self {
        Self {
            author,
            title,
            url: None,
            license: None,
            cms: None,
            accessed_at: None,
        }
    }
}
