use display_as::{with_template, HTML, URL, DisplayAs};
use std::cell::RefCell;
use std::hash::Hash;
use serde_derive::{Deserialize, Serialize};
use internment::Intern;

use crate::data::{CourseID, RepresentationID, ActivityGroup, ActivityView, ConceptID,
                  PrereqCourse};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Concept {
    pub id: ConceptID,
    pub name: String,
    pub prereq_concepts: Vec<ConceptID>,
    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
#[with_template("/concept/" slug::slugify(&self.name))]
impl DisplayAs<URL> for Concept {}

/// This is a concept, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ConceptView {
    pub id: ConceptID,
    pub name: String,

    pub activities: RefCell<Vec<Intern<ActivityView>>>,

    pub prereq_courses: RefCell<Vec<PrereqCourse>>,
    pub prereq_concepts: RefCell<Vec<Intern<ConceptView>>>,
    pub prereq_groups: RefCell<Vec<ActivityGroup>>,
    pub needed_for_concepts: RefCell<Vec<Intern<ConceptView>>>,

    pub output_groups: RefCell<Vec<ActivityGroup>>,

    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,

    pub am_initialized: RefCell<bool>,
}
impl Hash for ConceptView {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
#[with_template("concept.html")]
impl DisplayAs<HTML> for ConceptView {}
#[with_template("/concept/" slug::slugify(&self.name))]
impl DisplayAs<URL> for ConceptView {}

impl PartialEq for ConceptView {
    fn eq(&self, other: &ConceptView) -> bool {
        self.id == other.id
    }
}
impl Eq for ConceptView {}


/// This is a concept, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ConceptEdit {
    pub id: ConceptID,
    pub name: String,

    pub activities: RefCell<Vec<Intern<ActivityView>>>,

    pub prereq_courses: RefCell<Vec<PrereqCourse>>,
    pub prereq_concepts: RefCell<Vec<Intern<ConceptView>>>,
    pub prereq_groups: RefCell<Vec<ActivityGroup>>,
    pub needed_for_concepts: RefCell<Vec<Intern<ConceptView>>>,

    pub output_groups: RefCell<Vec<ActivityGroup>>,

    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
impl ConceptEdit {
    pub fn new(c: ConceptView) -> Self {
        ConceptEdit {
            id: c.id,
            name: c.name,
            activities: c.activities,
            prereq_courses: c.prereq_courses,
            prereq_concepts: c.prereq_concepts,
            prereq_groups: c.prereq_groups,
            needed_for_concepts: c.needed_for_concepts,
            output_groups: c.output_groups,
            representations: c.representations,
            courses: c.courses,
            figure: c.figure,
            long_description: c.long_description,
            external_url: c.external_url,
            status: c.status,
            notes: c.notes,
        }
    }
}
impl Hash for ConceptEdit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
#[with_template("concept-edit.html")]
impl DisplayAs<HTML> for ConceptEdit {}
#[with_template("/concept/edit/" slug::slugify(&self.name))]
impl DisplayAs<URL> for ConceptEdit {}

impl PartialEq for ConceptEdit {
    fn eq(&self, other: &ConceptEdit) -> bool {
        self.id == other.id
    }
}
impl Eq for ConceptEdit {}
