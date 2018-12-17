use display_as::{with_template, HTML, URL, DisplayAs};
use std::hash::Hash;
use crate::data::{CourseID, RepresentationID, ActivityGroup, ConceptView, ActivityID,
                  PrereqCourse};
use std::rc::Rc;
use std::cell::RefCell;

/// This is a activity, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ActivityView {
    pub id: ActivityID,
    pub name: String,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<Rc<RefCell<ConceptView>>>,
    pub prereq_groups: Vec<ActivityGroup>,

    pub new_concepts: Vec<Rc<RefCell<ConceptView>>>,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
impl Hash for ActivityView {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
#[with_template("activity.html")]
impl DisplayAs<HTML> for ActivityView {}
#[with_template("/activity/" slug::slugify(&self.name))]
impl DisplayAs<URL> for ActivityView {}

impl PartialEq for ActivityView {
    fn eq(&self, other: &ActivityView) -> bool {
        self.id == other.id
    }
}
impl Eq for ActivityView {}

impl ActivityView {
    pub fn slugme(&self) -> String {
        slug::slugify(&self.name)
    }
}
