use chrono::{Local, Date};

/// An article source
#[derive(Clone, Debug)]
pub struct Article {
    pub author: String,
    pub title: String,
    pub journal: String,
    pub date: Date<Local>,
    pub volume: Option<String>,
    pub number: Option<String>,
    pub pages: Option<String>,
}

impl Article {
    /// Creates a new article with the mandatory fields filled
    pub fn new(author: String, title: String, journal: String, date: Date<Local>) -> Self {
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