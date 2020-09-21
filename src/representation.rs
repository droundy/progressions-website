use display_as::{with_template, HTML, URL, UTF8, DisplayAs};
use serde::{Serialize, Deserialize};
use crate::data::{absolute_url, RepresentationID, ActivityGroup, Child, ConceptRepresentationView};
use crate::markdown::Markdown;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Icon {
    Fig(String),
    Html(String),
}
#[with_template("[%" "%]" "icon.html")]
impl DisplayAs<HTML> for Icon {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Representation {
    pub id: RepresentationID,
    pub name: String,
    #[serde(default)]
    pub description: Markdown,
    pub icon: Icon,
}
#[with_template( self.id )]
impl DisplayAs<URL> for Representation {}

#[derive(Debug, Clone)]
pub struct RepresentationView {
    pub id: RepresentationID,
    pub name: String,
    pub description: Markdown,
    pub icon: Icon,
    pub groups: Vec<ActivityGroup>,
    pub other_concepts: Vec<Child<ConceptRepresentationView>>,
}
#[with_template( self.id )]
impl DisplayAs<URL> for RepresentationView {}

#[with_template("[%" "%]" "representation-view.html")]
impl DisplayAs<HTML> for RepresentationView {}
