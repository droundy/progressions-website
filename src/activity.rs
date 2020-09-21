use display_as::{with_template, HTML, URL, DisplayAs};
use serde::{Serialize, Deserialize};
use crate::data::{absolute_url, Course,
                  Child, Representation, RepresentationID, AnyChoice,
                  ActivityGroup,
                  ConceptRepresentationID, ConceptRepresentationView,
                  ConceptRepresentationChoice,
                  ActivityID,
                  PrereqCourse, ChangeRelationship};
use crate::markdown::Markdown;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Activity {
    pub id: ActivityID,
    pub name: String,
    pub prereq_concepts: Vec<ConceptRepresentationID>,
    pub new_concepts: Vec<ConceptRepresentationID>,
    pub representations: Vec<RepresentationID>,
    pub long_description: Markdown,
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
    pub prereq_groups: Vec<ActivityGroup>,

    pub new_concepts: Vec<Child<ConceptRepresentationView>>,

    pub choices: ConceptRepresentationChoice,
    pub representation_choice: AnyChoice,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<Child<Representation>>,
    pub courses: Vec<Course>,
    pub figure: Option<String>,
    pub long_description: Markdown,
    pub external_url: Option<String>,
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
    pub fn choices_for(&self, field: &str) -> ConceptRepresentationChoice {
        ConceptRepresentationChoice { field: field.to_string(), .. self.choices.clone() }
    }
    pub fn prereq_concepts(&self) -> Vec<ConceptRepresentationID> {
        self.prereq_groups.iter()
            .flat_map(|g| g.concepts.iter())
            .map(|crv| crv.id)
            .chain(self.prereq_courses.iter()
                   .flat_map(|cour| cour.concepts.iter()
                   .map(|crv| crv.id)))
            .collect()
    }
}
