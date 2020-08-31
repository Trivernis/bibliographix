use crate::bibliography::bib_types::article::Article;

pub mod article;

/// A type of bibliography entry
#[derive(Clone, Debug)]
pub enum BibliographyType {
    Article(Article),
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