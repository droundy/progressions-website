use display_as::{with_template, format_as, HTML, URL, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use rcu_clean::RcRcu;

use crate::data::{Course,
                  RepresentationID, Child, Representation,
                  ActivityGroup, ActivityView, ConceptID,
                  ConceptChoice, ChangeRelationship,
                  PrereqCourse};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Concept {
    pub id: ConceptID,
    pub name: String,
    pub prereq_concepts: Vec<ConceptID>,
    pub representations: Vec<RepresentationID>,
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

    pub activities: Vec<RcRcu<ActivityView>>,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<ConceptID>,
    pub prereq_groups: Vec<ActivityGroup>,
    pub needed_for_concepts: Vec<ConceptID>,

    pub all_concepts: Vec<Concept>, // used to generate ConceptChoices

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<Child<Representation>>,
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
            if !self.prereq_concepts.contains(&c.id) &&
                !self.needed_for_concepts.contains(&c.id)
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
