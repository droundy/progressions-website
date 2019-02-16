use display_as::{with_template, format_as, HTML, URL, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::data::{Course,
                  RepresentationID, Child, Representation, ConceptRepresentationID,
                  Activity, ActivityChoice, ActivityGroup, ConceptID,
                  ConceptChoice, AnyChoice, ChangeRelationship,
                  PrereqCourse};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Concept {
    pub id: ConceptID,
    pub name: String,
    pub prereq_concepts: Vec<ConceptID>,
    pub representations: BTreeMap<RepresentationID,ConceptRepresentation>, // fixme change to ConceptRepresentation, possible BTreeMap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figure: Option<String>,
    pub long_description: String,
}
impl Concept {
    pub fn add_representation(&mut self, id: RepresentationID) {
        self.representations.insert(id, ConceptRepresentation {
            name: "".to_string(),
            long_description: "".to_string(),
            figure: None,
        });
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConceptRepresentation {
    pub name: String,
    pub long_description: String,
    pub figure: Option<String>,
}

#[with_template( self.id )]
impl DisplayAs<URL> for Concept {}

/// This is a concept, but with all the relationships filled in.
#[derive(Debug, Clone)]
pub struct ConceptView {
    pub id: ConceptID,
    pub name: String,

    pub activities: Vec<Activity>,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: Vec<ConceptID>,
    pub prereq_groups: Vec<ActivityGroup>,
    pub needed_for_concepts: Vec<ConceptID>,

    pub all_concepts: Vec<Concept>, // used to generate ConceptChoices
    pub all_activities: Vec<Activity>, // used to generate ActivityChoices
    pub representation_choice: AnyChoice,

    pub output_groups: Vec<ActivityGroup>,

    pub representations: Vec<ActivityGroup>,
    pub courses: Vec<Course>,
    pub figure: Option<String>,
    pub long_description: String,
}
#[with_template("[%" "%]" "concept-view.html")]
impl DisplayAs<HTML> for ConceptView {}
#[with_template( self.id )]
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
    pub fn possible_activities(&self) -> ActivityChoice {
        let mut ch = ActivityChoice {
            id: format_as!(HTML, self.id),
            field: "taught by".to_string(),
            choices: Vec::new(),
        };
        for a in self.all_activities.iter() {
            // Try to list only the concepts that we might plausibly want.
            if !a.new_concepts.contains(&self.id.into()) {
                ch.choices.push(a.clone());
            }
        }
        ch
    }
}

#[derive(Debug, Clone)]
pub struct ConceptRepresentationView {
    pub id: ConceptRepresentationID,
    pub activities: Vec<Activity>,
    pub representation: Option<Representation>,
    pub summary_name: String,
    pub name: String,
    pub long_description: String,
    pub figure: Option<String>,
}
#[with_template("[%" "%]" "concept-representation-view.html")]
impl DisplayAs<HTML> for Child<ConceptRepresentationView> {}
#[with_template( self.id )]
impl DisplayAs<URL> for ConceptRepresentationView {}
