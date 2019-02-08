use display_as::{with_template, HTML, URL, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use crate::data::{Course,
                  Child, Representation, RepresentationID, AnyChoice,
                  ActivityGroup,
                  Concept, ConceptRepresentationID, ConceptRepresentationView,
                  ConceptRepresentationChoice,
                  ActivityID,
                  PrereqCourse, ChangeRelationship};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Activity {
    pub id: ActivityID,
    pub name: String,
    pub prereq_concepts: Vec<ConceptRepresentationID>,
    pub new_concepts: Vec<ConceptRepresentationID>,
    pub representations: Vec<RepresentationID>,
    pub long_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
}
#[with_template( self.id )]
impl DisplayAs<URL> for Activity {}

/// This is a activity, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ActivityView {
    pub id: ActivityID,
    pub name: String,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<Concept>,
    pub prereq_groups: Vec<ActivityGroup>,

    pub new_concepts: Vec<Child<ConceptRepresentationView>>,

    pub choices: ConceptRepresentationChoice,
    pub representation_choice: AnyChoice,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<Child<Representation>>,
    pub courses: Vec<Course>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub addremove: ChangeRelationship,
}

#[with_template("[%" "%]" "activity.html")]
impl DisplayAs<HTML> for ActivityView {}
#[with_template( self.id )]
impl DisplayAs<URL> for ActivityView {}

impl PartialEq for ActivityView {
    fn eq(&self, other: &ActivityView) -> bool {
        self.id == other.id
    }
}
impl Eq for ActivityView {}

impl ActivityView {
    pub fn remove(&self, from: impl DisplayAs<HTML>, relationship: &'static str)
                  -> Self
    {
        ActivityView {
            addremove: ChangeRelationship::parent(from, "Remove", relationship)
                .child(self.id),
            .. self.clone()
        }
    }
    pub fn add(&self, from: impl DisplayAs<HTML>, relationship: &'static str)
                  -> Self
    {
        ActivityView {
            addremove: ChangeRelationship::parent(from, "Add", relationship)
                .child(self.id),
            .. self.clone()
        }
    }
    pub fn choices_for(&self, field: &str) -> ConceptRepresentationChoice {
        ConceptRepresentationChoice { field: field.to_string(), .. self.choices.clone() }
    }
}
