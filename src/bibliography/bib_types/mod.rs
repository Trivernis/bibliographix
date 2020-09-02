use crate::bibliography::bib_types::article::Article;
use crate::bibliography::bib_types::book::Book;
use crate::bibliography::bib_types::booklet::Booklet;
use crate::bibliography::bib_types::in_book::InBook;
use crate::bibliography::bib_types::in_collection::InCollection;
use crate::bibliography::bib_types::manual::Manual;
use crate::bibliography::bib_types::misc::Misc;
use crate::bibliography::bib_types::repository::Repository;
use crate::bibliography::bib_types::tech_report::TechReport;
use crate::bibliography::bib_types::thesis::Thesis;
use crate::bibliography::bib_types::unpublished::Unpublished;
use crate::bibliography::bib_types::website::Website;
use crate::bibliography::keys::{
    K_TYPE, T_ARTICLE, T_BOOK, T_BOOKLET, T_IN_BOOK, T_IN_COLLECTION, T_MANUAL, T_MISC,
    T_REPOSITORY, T_TECH_REPORT, T_THESIS, T_UNPUBLISHED, T_WEBSITE,
};
use crate::bibliography::FromHashMap;
use std::collections::HashMap;

pub mod article;
pub mod book;
pub mod booklet;
pub mod in_book;
pub mod in_collection;
pub mod manual;
pub mod misc;
pub mod repository;
pub mod tech_report;
pub mod thesis;
pub mod unpublished;
pub mod website;

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
    Unpublished(Unpublished),
    Misc(Misc),
    Website(Website),
    Repository(Repository),
}

impl BibliographyType {
    /// Returns the name of the enums value as a string
    pub fn name(&self) -> String {
        match self {
            Self::Article(_) => T_ARTICLE.to_string(),
            Self::Book(_) => T_BOOK.to_string(),
            Self::Booklet(_) => T_BOOKLET.to_string(),
            Self::InBook(_) => T_IN_BOOK.to_string(),
            Self::InCollection(_) => T_IN_COLLECTION.to_string(),
            Self::Manual(_) => T_MANUAL.to_string(),
            Self::Thesis(_) => T_THESIS.to_string(),
            Self::TechReport(_) => T_TECH_REPORT.to_string(),
            Self::Unpublished(_) => T_UNPUBLISHED.to_string(),
            Self::Misc(_) => T_MISC.to_string(),
            Self::Website(_) => T_WEBSITE.to_string(),
            Self::Repository(_) => T_REPOSITORY.to_string(),
        }
    }
}

impl FromHashMap for BibliographyType {
    fn from_hash_map(map: &HashMap<String, String>) -> Option<Box<Self>> {
        match map.get(K_TYPE)?.as_str() {
            T_ARTICLE => Some(Box::new(Self::Article(*Article::from_hash_map(map)?))),
            T_BOOK => Some(Box::new(Self::Book(*Book::from_hash_map(map)?))),
            T_BOOKLET => Some(Box::new(Self::Booklet(*Booklet::from_hash_map(map)?))),
            T_IN_BOOK => Some(Box::new(Self::InBook(*InBook::from_hash_map(map)?))),
            T_IN_COLLECTION => Some(Box::new(Self::InCollection(*InCollection::from_hash_map(
                map,
            )?))),
            T_MANUAL => Some(Box::new(Self::Manual(*Manual::from_hash_map(map)?))),
            T_MISC => Some(Box::new(Self::Misc(*Misc::from_hash_map(map)?))),
            T_REPOSITORY => Some(Box::new(Self::Repository(*Repository::from_hash_map(map)?))),
            T_TECH_REPORT => Some(Box::new(Self::TechReport(*TechReport::from_hash_map(map)?))),
            T_THESIS => Some(Box::new(Self::Thesis(*Thesis::from_hash_map(map)?))),
            T_UNPUBLISHED => Some(Box::new(Self::Unpublished(*Unpublished::from_hash_map(
                map,
            )?))),
            T_WEBSITE => Some(Box::new(Self::Website(*Website::from_hash_map(map)?))),
            _ => None,
        }
    }
}
