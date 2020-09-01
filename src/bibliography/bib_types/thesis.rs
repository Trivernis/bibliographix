use crate::bibliography::bib_types::LocalDate;

/// A thesis source entry
#[derive(Clone, Debug)]
pub struct Thesis {
    author: String,
    title: String,
    school: String,
    date: LocalDate,
    address: Option<String>,
}

impl Thesis {
    /// Creates a new thesis with only the mandatory fields filled
    pub fn new(author: String, title: String, school: String, date: LocalDate) -> Self {
        Self {
            author,
            title,
            school,
            date,
            address: None,
        }
    }
}
