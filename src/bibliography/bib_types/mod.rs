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
use chrono::{Date, Local};
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
    Unpublished(Unpublished),
    Misc(Misc),
    Website(Website),
    Repository(Repository),
}

impl FromHashMap for BibliographyType {
    fn from_hash_map(map: &HashMap<String, String>) -> Option<Box<Self>> {
        if map.contains_key("type") {
            match map.get("type").unwrap().to_lowercase() {
                _ => None,
            }
        } else {
            None
        }
    }
}
