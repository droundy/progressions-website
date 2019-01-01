use display_as::{with_template, HTML, URL, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use crate::data::{Course, CourseID,
                  Representation, RepresentationID,
                  ActivityGroup,
                  ConceptID, ConceptView, ActivityID,
                  PrereqCourse};
use rcu_clean::RcRcu;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Activity {
    pub id: ActivityID,
    pub name: String,
    pub prereq_concepts: Vec<ConceptID>,
    pub new_concepts: Vec<ConceptID>,
    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
#[with_template("/activity/" slug::slugify(&self.name))]
impl DisplayAs<URL> for Activity {}

/// This is a activity, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ActivityView {
    pub id: ActivityID,
    pub name: String,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<RcRcu<ConceptView>>,
    pub prereq_groups: Vec<ActivityGroup>,

    pub new_concepts: Vec<RcRcu<ConceptView>>,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<Representation>,
    pub courses: Vec<Course>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[with_template("[%" "%]" "activity.html")]
impl DisplayAs<HTML> for ActivityView {}
#[with_template("/activity/" slug::slugify(&self.name))]
impl DisplayAs<URL> for ActivityView {}

impl PartialEq for ActivityView {
    fn eq(&self, other: &ActivityView) -> bool {
        self.id == other.id
    }
}
impl Eq for ActivityView {}
