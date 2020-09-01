use crate::utils::date::LocalDate;

/// A source that does not fit any of the other types
#[derive(Clone, Debug)]
pub struct Misc {
    pub author: Option<String>,
    pub title: Option<String>,
    pub how_published: Option<String>,
    pub date: Option<LocalDate>,
}

impl Misc {
    /// Creates an empty Misc
    pub fn new() -> Self {
        Self {
            author: None,
            title: None,
            how_published: None,
            date: None,
        }
    }
}
