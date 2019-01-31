use display_as::{with_template, format_as, HTML, URL, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use crate::data::{Course, CourseID,
                  Child, Representation, RepresentationID,
                  ActivityGroup,
                  ConceptID, Concept, ConceptChoice,
                  ActivityID,
                  PrereqCourse, ChangeRelationship};

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

    pub new_concepts: Vec<Concept>,

    pub all_concepts: Vec<Concept>, // used to generate ConceptChoices

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
    pub fn possibly_taught_concepts(&self) -> ConceptChoice {
        let mut ch = ConceptChoice {
            id: format_as!(HTML, self.id),
            field: "taught".to_string(),
            choices: Vec::new(),
        };
        for c in self.all_concepts.iter() {
            // Try to list only the concepts that we might plausibly want.
            if !self.prereq_concepts.iter().any(|pre| pre.id == c.id) &&
                !self.new_concepts.iter().any(|pre| pre.id == c.id)
            {
                ch.choices.push(c.clone());
            }
        }
        ch
    }
    pub fn possibly_prereq_concepts(&self) -> ConceptChoice {
        let mut ch = ConceptChoice {
            id: format_as!(HTML, self.id),
            field: "prereq".to_string(),
            choices: Vec::new(),
        };
        for c in self.all_concepts.iter() {
            // Try to list only the concepts that we might plausibly want.
            if !self.prereq_concepts.iter().any(|pre| pre.id == c.id) &&
                !self.new_concepts.iter().any(|pre| pre.id == c.id)
            {
                ch.choices.push(c.clone());
            }
        }
        ch
    }
}
