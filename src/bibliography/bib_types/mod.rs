use crate::bibliography::bib_types::article::Article;
use crate::bibliography::bib_types::book::Book;
use chrono::{Date, Local};
use crate::bibliography::bib_types::booklet::Booklet;

pub mod article;
pub mod book;
pub mod booklet;

pub type LocalDate = Date<Local>;

/// A type of bibliography entry
#[derive(Clone, Debug)]
pub enum BibliographyType {
    Article(Article),
    Book(Book),
    Booklet(Booklet),
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