/// A type of bibliography entry
#[derive(Clone, Debug)]
pub enum BibliographyType {
    Article,
    Book,
    Booklet,
    InBook,
    InCollection,
    Manual,
    Thesis,
    TechReport,
    Unpublished,
    Misc,
    Url,
    Repository,
}