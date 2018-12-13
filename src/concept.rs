use display_as::{with_template, HTML, URL, DisplayAs};
use internment::Intern;
use std::cell::RefCell;
use std::hash::Hash;
use crate::data::{CourseID, RepresentationID, ActivityGroup, ActivityView, ConceptID,
                  PrereqCourse};

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
