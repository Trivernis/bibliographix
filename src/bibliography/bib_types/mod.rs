use crate::bibliography::bib_types::article::Article;
use crate::bibliography::bib_types::book::Book;
use crate::bibliography::bib_types::booklet::Booklet;
use crate::bibliography::bib_types::in_book::InBook;
use crate::bibliography::bib_types::in_collection::InCollection;
use crate::bibliography::bib_types::manual::Manual;
use crate::bibliography::bib_types::tech_report::TechReport;
use crate::bibliography::bib_types::thesis::Thesis;
use chrono::{Date, Local};

pub mod article;
pub mod book;
pub mod booklet;
pub mod in_book;
pub mod in_collection;
pub mod manual;
pub mod tech_report;
pub mod thesis;

pub type LocalDate = Date<Local>;

/// A type of bibliography entry
#[derive(Clone, Debug)]
pub enum BibliographyType {
    Article(Article),
    Book(Book),
    Booklet(Booklet),
    InBook(InBook),
    InCollection(InCollection),
    Manual(Manual),
    Thesis(Thesis),
    TechReport(TechReport),
    Unpublished,
    Misc,
    Url,
    Repository,
}
