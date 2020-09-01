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
            Self::Article(_) => "article".to_string(),
            Self::Book(_) => "book".to_string(),
            Self::Booklet(_) => "booklet".to_string(),
            Self::InBook(_) => "in_book".to_string(),
            Self::InCollection(_) => "in_collection".to_string(),
            Self::Manual(_) => "manual".to_string(),
            Self::Thesis(_) => "thesis".to_string(),
            Self::TechReport(_) => "tech_report".to_string(),
            Self::Unpublished(_) => "unpublished".to_string(),
            Self::Misc(_) => "misc".to_string(),
            Self::Website(_) => "website".to_string(),
            Self::Repository(_) => "repository".to_string(),
        }
    }
}

impl FromHashMap for BibliographyType {
    fn from_hash_map(map: &HashMap<String, String>) -> Option<Box<Self>> {
        if map.contains_key("type") {
            match map.get("type").unwrap().as_str() {
                "article" => Some(Box::new(Self::Article(*Article::from_hash_map(map)?))),
                _ => None,
            }
        } else {
            None
        }
    }
}
