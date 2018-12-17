use display_as::{with_template, HTML, URL, DisplayAs};
use std::hash::Hash;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;

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

    pub activities: Vec<Rc<RefCell<ActivityView>>>,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<Rc<RefCell<ConceptView>>>,
    pub prereq_groups: Vec<ActivityGroup>,
    pub needed_for_concepts: Vec<Rc<RefCell<ConceptView>>>,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
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

    pub activities: Vec<Rc<RefCell<ActivityView>>>,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<Rc<RefCell<ConceptView>>>,
    pub prereq_groups: Vec<ActivityGroup>,
    pub needed_for_concepts: Vec<Rc<RefCell<ConceptView>>>,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
