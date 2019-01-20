use display_as::{with_template, HTML, URL, UTF8, DisplayAs};
use serde_derive::{Deserialize, Serialize};
use crate::data::{RepresentationID, ActivityGroup, Child, Concept};
use crate::markdown::Markdown;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Representation {
    pub id: RepresentationID,
    pub name: String,
    #[serde(default)]
    pub description: Markdown,
    pub icon: String,
}
#[with_template("/representation/" slug::slugify(&self.name))]
impl DisplayAs<URL> for Representation {}

#[derive(Debug, Clone)]
pub struct RepresentationView {
    pub id: RepresentationID,
    pub name: String,
    pub description: Markdown,
    pub icon: String,
    pub groups: Vec<ActivityGroup>,
    pub other_concepts: Vec<Child<Concept>>,
}
#[with_template("/representation/" slug::slugify(&self.name))]
impl DisplayAs<URL> for RepresentationView {}

#[with_template("[%" "%]" "representation-view.html")]
impl DisplayAs<HTML> for RepresentationView {}

