use display_as::{with_template, format_as, HTML, URL, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use rcu_clean::RcRcu;

use crate::data::{CourseID, Course,
                  RepresentationID, Representation,
                  ActivityGroup, ActivityView, ConceptID,
                  ConceptChoice, ChangeRelationship,
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
    #[serde(skip)]
    pub addremove: ChangeRelationship,
}
#[with_template("/concept/" slug::slugify(&self.name))]
impl DisplayAs<URL> for Concept {}
#[with_template("[%" "%]" "concept.html")]
impl DisplayAs<HTML> for Concept {}

impl Concept {
    pub fn remove(&self, from: impl DisplayAs<HTML>, relationship: &'static str)
                  -> Concept
    {
        Concept {
            addremove: ChangeRelationship::parent(from, "Remove", relationship)
                .child(self.id),
            .. self.clone()
        }
    }
}


/// This is a concept, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ConceptView {
    pub id: ConceptID,
    pub name: String,

    pub activities: Vec<RcRcu<ActivityView>>,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<RcRcu<ConceptView>>,
    pub prereq_groups: Vec<ActivityGroup>,
    pub needed_for_concepts: Vec<RcRcu<ConceptView>>,

    pub all_concepts: Vec<Concept>, // used to generate ConceptChoices

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<Representation>,
    pub courses: Vec<Course>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
#[with_template("[%" "%]" "concept-view.html")]
impl DisplayAs<HTML> for ConceptView {}
#[with_template("/concept/" slug::slugify(&self.name))]
impl DisplayAs<URL> for ConceptView {}

impl PartialEq for ConceptView {
    fn eq(&self, other: &ConceptView) -> bool {
        self.id == other.id
    }
}
impl Eq for ConceptView {}

impl ConceptView {
    pub fn possibly_needed_for_concepts(&self) -> ConceptChoice {
        let mut ch = ConceptChoice {
            id: format_as!(HTML, self.id),
            field: "needed for".to_string(),
            choices: Vec::new(),
        };
        for c in self.all_concepts.iter() {
            // Try to list only the concepts that we might plausibly want.
            if !self.prereq_concepts.iter().any(|pre| pre.id == c.id) &&
                !self.needed_for_concepts.iter().any(|pre| pre.id == c.id)
            {
                ch.choices.push(c.clone());
            }
        }
        ch
    }
    pub fn possibly_prereq_concepts(&self) -> ConceptChoice {
        ConceptChoice {
            id: format_as!(HTML, self.id),
            field: "prereq".to_string(),
            choices: self.possibly_needed_for_concepts().choices,
        }
    }
}
