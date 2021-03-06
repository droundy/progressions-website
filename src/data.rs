use crate::atomicfile::AtomicFile;
use serde::{Serialize, Deserialize};
use display_as::{with_template, format_as, HTML, UTF8, URL, DisplayAs};
use crate::markdown::Markdown;

lazy_static::lazy_static! {
    static ref BASE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
}
pub fn set_base_url(new_base: &str) {
    *BASE.lock().unwrap() = new_base.to_string();
}
fn base_url() -> String {
    BASE.lock().unwrap().clone()
}
#[derive(Debug, Clone)]
struct AbsoluteURL(String);
#[with_template(r#"""# base_url() as UTF8 "" self.0 as UTF8 r#"""#)]
impl DisplayAs<HTML> for AbsoluteURL {}

pub fn absolute_url(u: impl DisplayAs<URL>) -> impl DisplayAs<HTML> {
    let x = format_as!(URL, u);
    if x.starts_with("/") {
        AbsoluteURL(x)
    } else {
        AbsoluteURL(format!("/{}", x))
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub field: String,
    pub content: String,
    pub html: String,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConceptID(usize);
#[with_template("c" self.0)]
impl DisplayAs<HTML> for ConceptID {}
#[with_template("/concept/" self.0)]
impl DisplayAs<URL> for ConceptID {}
impl std::str::FromStr for ConceptID {
    type Err = std::num::ParseIntError;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(ConceptID(usize::from_str(x)?))
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConceptRepresentationID {
    concept: ConceptID,
    #[serde(skip_serializing_if = "Option::is_none")]
    representation: Option<RepresentationID>,
}
#[with_template( if let Some(rid) = self.representation {
    self.concept "-" rid
} else {
    self.concept
} )]
impl DisplayAs<HTML> for ConceptRepresentationID {}
#[with_template( self.concept )]
impl DisplayAs<URL> for ConceptRepresentationID {}
impl ConceptRepresentationID {
    fn str2manifestation(x: &str) -> Option<Self> {
        use std::str::FromStr;
        if x == "" {
            return None;
        }
        if x.split_at(1).0 == "c" {
            let x = x.split_at(1).1;
            let mut things = x.split("-");
            let first_substr = things.next()?;
            let concept = ConceptID::from_str(first_substr).ok()?;
            let (r,x) = things.next()?.split_at(1);
            if r != "r" {
                return None;
            }
            let representation = Some(RepresentationID::from_str(x).ok()?);
            Some(ConceptRepresentationID { concept, representation })
        } else {
            let mut things = x.split("/");
            let first_substr = things.next()?;
            let concept = ConceptID::from_str(first_substr).ok()?;
            let representation = Some(RepresentationID::from_str(things.next()?).ok()?);
            Some(ConceptRepresentationID { concept, representation })
        }
    }
}
impl std::str::FromStr for ConceptRepresentationID {
    type Err = std::num::ParseIntError;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        if let Some(id) = ConceptRepresentationID::str2manifestation(x) {
            return Ok(id);
        }
        Ok(ConceptRepresentationID {
            concept: ConceptID::from_str(x)?,
            representation: None,
        })
    }
}
#[test]
fn test_concept_representation_id() {
    use std::str::FromStr;

    let oneone: ConceptRepresentationID = (ConceptID(1), RepresentationID(2)).into();
    assert_eq!("c1-r2", &format_as!(HTML, oneone));
    assert_eq!(Ok(oneone), ConceptRepresentationID::from_str(&format_as!(HTML, oneone)));

    let id: ConceptRepresentationID = (ConceptID(27), RepresentationID(6)).into();
    assert_eq!("c27-r6", &format_as!(HTML, id));
    assert_eq!(Ok(id), ConceptRepresentationID::from_str(&format_as!(HTML, id)));

    assert!(AnyID::parse("c27-r6").is_ok());
}
impl From<ConceptID> for ConceptRepresentationID {
    fn from(concept: ConceptID) -> Self {
        ConceptRepresentationID { concept, representation: None }
    }
}
impl From<(ConceptID, RepresentationID)> for ConceptRepresentationID {
    fn from(ids: (ConceptID, RepresentationID)) -> Self {
        ConceptRepresentationID { concept: ids.0, representation: Some(ids.1) }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActivityID(usize);
#[with_template("a" self.0)]
impl DisplayAs<HTML> for ActivityID {}
#[with_template("/activity/" self.0)]
impl DisplayAs<URL> for ActivityID {}
impl std::str::FromStr for ActivityID {
    type Err = std::num::ParseIntError;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(ActivityID(usize::from_str(x)?))
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RepresentationID(usize);
#[with_template("/representation/" self.0)]
impl DisplayAs<URL> for RepresentationID {}
#[with_template("r" self.0)]
impl DisplayAs<HTML> for RepresentationID {}
impl std::str::FromStr for RepresentationID {
    type Err = std::num::ParseIntError;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(RepresentationID(usize::from_str(x)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CourseID(usize);
#[with_template("C" self.0)]
impl DisplayAs<HTML> for CourseID {}
#[with_template("/course/" self.0)]
impl DisplayAs<URL> for CourseID {}
impl std::str::FromStr for CourseID {
    type Err = std::num::ParseIntError;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(CourseID(usize::from_str(x)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Course {
    pub id: CourseID,
    pub number: String,
    pub name: String,
    pub activities: Vec<ActivityID>,
}
#[with_template( self.id )]
impl DisplayAs<URL> for Course {}
#[with_template(r#"<a href="# absolute_url(self.id) r#" class="course">"# self.name r#"</a>"#)]
impl DisplayAs<HTML> for Course {}

pub use crate::concept::{Concept, ConceptView,
                         ConceptRepresentation, ConceptRepresentationView};
pub use crate::activity::{Activity, ActivityView};
pub use crate::representation::{Icon, Representation, RepresentationView};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    concepts: Vec<Concept>,
    activities: Vec<Activity>,
    representations: Vec<Representation>,
    courses: Vec<Course>,
}

#[derive(Debug)]
pub enum AnyID {
    Concept(ConceptID),
    Course(CourseID),
    Activity(ActivityID),
    Representation(RepresentationID),
    ConceptRepresentation(ConceptRepresentationID),
}
impl AnyID {
    fn parse(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s.chars().next() {
            Some('c') => {
                if s.contains("r") {
                    use std::str::FromStr;
                    Ok(AnyID::ConceptRepresentation(
                        ConceptRepresentationID::from_str(s)?))
                } else {
                    Ok(AnyID::Concept(ConceptID(s[1..].parse()?)))
                }
            },
            Some('C') => Ok(AnyID::Course(CourseID(s[1..].parse()?))),
            Some('a') => Ok(AnyID::Activity(ActivityID(s[1..].parse()?))),
            Some('r') => Ok(AnyID::Representation(RepresentationID(s[1..].parse()?))),
            _ => panic!("Crazy kind: {}", s),
        }
    }
}
impl std::str::FromStr for AnyID {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AnyID::parse(s)
    }
}

trait ID: Copy+Clone {
    type Target;
    fn get(self, data: &Data) -> &Self::Target;
    fn get_mut(self, data: &mut Data) -> &mut Self::Target;
}
impl ID for ConceptID {
    type Target = Concept;
    fn get(self, data: &Data) -> &Self::Target {
        &data.concepts[self.0]
    }
    fn get_mut(self, data: &mut Data) -> &mut Self::Target {
        &mut data.concepts[self.0]
    }
}
impl ID for ActivityID {
    type Target = Activity;
    fn get(self, data: &Data) -> &Self::Target {
        &data.activities[self.0]
    }
    fn get_mut(self, data: &mut Data) -> &mut Self::Target {
        &mut data.activities[self.0]
    }
}
impl ID for RepresentationID {
    type Target = Representation;
    fn get(self, data: &Data) -> &Self::Target {
        &data.representations[self.0]
    }
    fn get_mut(self, data: &mut Data) -> &mut Self::Target {
        &mut data.representations[self.0]
    }
}
impl ID for CourseID {
    type Target = Course;
    fn get(self, data: &Data) -> &Self::Target {
        &data.courses[self.0]
    }
    fn get_mut(self, data: &mut Data) -> &mut Self::Target {
        &mut data.courses[self.0]
    }
}

impl Data {
    fn get<I: ID>(&self, id: I) -> &I::Target {
        id.get(self)
    }
    fn get_mut<I: ID>(&mut self, id: I) -> &mut I::Target {
        id.get_mut(self)
    }
    pub fn save(self) {
        let f = AtomicFile::create("progression.yaml").expect("error creating save file");
        serde_yaml::to_writer(&f, &self).expect("error writing yaml");
        // self.spawn_dump_mirror();
    }
    pub fn new() -> Self {
        if let Ok(f) = std::fs::File::open("progression.yaml") {
            if let Ok(s) = serde_yaml::from_reader::<_,Self>(&f) {
                return s;
            }
        }
        Data {
            concepts: Vec::new(),
            activities: Vec::new(),
            representations: Vec::new(),
            courses: Vec::new(),
        }
    }
    pub fn uploaded_figure(mut self, id: AnyID, filename: &str)
                           -> Result<(), Box<dyn std::error::Error>>
    {
        match id {
            AnyID::Representation(id) => {
                self.get_mut(id).icon = Icon::Fig(format!("figs/{}", filename));
            }
            AnyID::Concept(id) => {
                self.get_mut(id).figure = Some(filename.to_string());
            }
            AnyID::Activity(id) => {
                self.get_mut(id).figure = Some(filename.to_string());
            }
            AnyID::ConceptRepresentation(id) => {
                let rid = id.representation.unwrap();
                self.get_mut(id.concept).representations.entry(rid)
                    .and_modify(|e| e.figure = Some(filename.to_string()));
            }
            _ => {
                panic!("Unhandled id type on uploaded_figure: {:?}", id);
            }
        }
        self.save();
        Ok(())
    }
    pub fn change(mut self, c: Change) -> Result<(), Box<dyn std::error::Error>> {
        println!("change is {:?}", c);
        match AnyID::parse(&c.id)? {
            AnyID::Course(id) => {
                match &c.field as &str {
                    "activity" => {
                        let a = self.activity_by_name_or_create(&c.content);
                        self.add_to_course(id, a);
                    }
                    "up" => {
                        match AnyID::parse(&c.content)? {
                            AnyID::Activity(aid) => {
                                println!("move activity up");
                                let a = &mut self.get_mut(id).activities;
                                let which = a.iter().enumerate().filter(|(_,x)| **x == aid)
                                    .map(|(i,_)| i)
                                    .next().unwrap();
                                a.swap(which-1, which);
                            }
                            _ => panic!("Cannot handle \"up\" with other types"),
                        }
                    }
                    "down" => {
                        match AnyID::parse(&c.content)? {
                            AnyID::Activity(aid) => {
                                println!("move activity down");
                                let a = &mut self.get_mut(id).activities;
                                let which = a.iter().enumerate().filter(|(_,x)| **x == aid)
                                    .map(|(i,_)| i)
                                    .next().unwrap();
                                a.swap(which+1, which);
                            }
                            _ => panic!("Cannot handle \"down\" with other types"),
                        }
                    }
                    "Remove" => {
                        match c.html.as_ref() {
                            "teaches" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Activity(a_id) => {
                                        self.get_mut(id).activities.retain(|&x| x != a_id);
                                    }
                                    _ => panic!("taughtby must be an activity in remove"),
                                }
                            }
                            _ => panic!("Unknown relationship on remove from course: {}", c.html),
                        }
                    }
                    _ => {
                        panic!("Weird field for course: {}", c.field);
                    }
                }
            }
            AnyID::ConceptRepresentation(id) => {
                match &c.field as &str {
                    "name" => {
                        self.get_mut(id.concept).representations
                            .get_mut(&id.representation.unwrap()).unwrap()
                            .name = c.content.trim().to_string();
                    }
                    "long_description" => {
                        self.get_mut(id.concept).representations
                            .get_mut(&id.representation.unwrap()).unwrap()
                            .long_description = Markdown::from_html(&c.html);
                    }
                    _ => {
                        panic!("Weird field for ConceptRepresentation: {}", c.field);
                    }
                }
            }
            AnyID::Concept(id) => {
                match &c.field as &str {
                    "long_description" => {
                        self.get_mut(id).long_description = Markdown::from_html(&c.html);
                    }
                    "name" => {
                        self.get_mut(id).name = c.content.trim().to_string();
                    }
                    "needed for" => {
                        let needed_for_id = self.concept_by_name_or_create(&c.content);
                        self.get_mut(needed_for_id).prereq_concepts.push(id);
                    }
                    "prereq" => {
                        let prereq_id = self.concept_by_name_or_create(&c.content);
                        self.get_mut(id).prereq_concepts.push(prereq_id);
                    }
                    "taught by" => {
                        let activity_id = self.activity_by_name_or_create(&c.content);
                        self.get_mut(activity_id).new_concepts.push(id.into());
                    }
                    "representation" => {
                        let rid = self.representation_by_name_or_create(&c.content);
                        self.get_mut(id).add_representation(rid);
                    }
                    "Add" => {
                        match c.html.as_ref() {
                            "needed for" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(needed_for_id) => {
                                        self.get_mut(needed_for_id).prereq_concepts.push(id)
                                    }
                                    _ => panic!("Cannot yet handle needed for with other types"),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(prereq_id) => {
                                        self.get_mut(id).prereq_concepts.push(prereq_id)
                                    }
                                    _ => panic!("prereq must be a concept"),
                                }
                            }
                            _ => panic!("Unknown relationship: {}", c.html),
                        }
                    }
                    "Remove" => {
                        match c.html.as_ref() {
                            "needed for" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(needed_for_id) => {
                                        self.get_mut(needed_for_id).prereq_concepts.retain(|&x| x != id);
                                    }
                                    _ => panic!("Cannot yet remove needed for with other types"),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(prereq_id) => {
                                        self.get_mut(id).prereq_concepts.retain(|&x| x != prereq_id);
                                    }
                                    _ => panic!("prereq must be a concept in remove"),
                                }
                            }
                            "taught by" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Activity(a_id) => {
                                        self.get_mut(a_id).new_concepts.retain(|&x| x.concept != id);
                                    }
                                    _ => panic!("taughtby must be an activity in remove"),
                                }
                            }
                            "with" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::ConceptRepresentation(crid) => {
                                        if let Some(r) = crid.representation {
                                            self.get_mut(id).representations.remove(&r);
                                        }
                                    }
                                    _ => panic!("prereq must be a concept in remove"),
                                }
                            }
                            _ => panic!("Unknown relationship on remove from concept: {}", c.html),
                        }
                    }
                    _ => panic!("Unknown field of concept: {}", c.field),
                }
            }
            AnyID::Activity(id) => {
                match &c.field as &str {
                    "long_description" => {
                        self.get_mut(id).long_description = Markdown::from_html(&c.html);
                    }
                    "name" => {
                        self.get_mut(id).name = c.content.trim().to_string();
                    }
                    "prereq" => {
                        let prereq_id = if let Some(prereq_id) = self.concept_representation_to_id(&c.content) {
                            prereq_id
                        } else {
                            self.concept_by_name_or_create(&c.content).into()
                        };
                        println!("the prereq id is {:?}", prereq_id);
                        self.get_mut(id).prereq_concepts.push(prereq_id);
                    }
                    "taught" => {
                        let prereq_id = if let Some(prereq_id) = self.concept_representation_to_id(&c.content) {
                            prereq_id
                        } else {
                            self.concept_by_name_or_create(&c.content).into()
                        };
                        self.get_mut(id).new_concepts.push(prereq_id);
                    }
                    "representation" => {
                        let rid = self.representation_by_name_or_create(&c.content);
                        self.get_mut(id).representations.push(rid);
                    }
                    "external_url" => {
                        self.get_mut(id).external_url = Some(c.content.trim().to_string());
                    }
                    "Remove" => {
                        match c.html.as_ref() {
                            "new" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(c_id) => {
                                        self.get_mut(id).new_concepts
                                            .retain(|&x| x.concept != c_id);
                                    }
                                    AnyID::ConceptRepresentation(c_id) => {
                                        self.get_mut(id).new_concepts.retain(|&x| x != c_id);
                                    }
                                    _ => panic!("No new as other type {}", c.content),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(c_id) => {
                                        self.get_mut(id).prereq_concepts.retain(|&x| x != c_id.into());
                                    }
                                    AnyID::ConceptRepresentation(c_id) => {
                                        self.get_mut(id).prereq_concepts.retain(|&x| x != c_id);
                                    }
                                    _ => panic!("No prereq as other type"),
                                }
                            }
                            "uses" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Representation(rid) => {
                                        self.get_mut(id).representations
                                            .retain(|&x| x != rid);
                                    }
                                    _ => panic!("No prereq as other type"),
                                }
                            }
                            _ => panic!("Unknown relationship on remove from activity: {}", c.html),
                        }
                    }
                    "Add" => {
                        match c.html.as_ref() {
                            "new" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(c_id) => {
                                        self.get_mut(id).new_concepts.push(c_id.into());
                                    }
                                    AnyID::ConceptRepresentation(c_id) => {
                                        self.get_mut(id).new_concepts.push(c_id);
                                    }
                                    _ => panic!("No new as other type {}", c.content),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(c_id) => {
                                        self.get_mut(id).prereq_concepts.push(c_id.into());
                                    }
                                    AnyID::ConceptRepresentation(c_id) => {
                                        self.get_mut(id).prereq_concepts.push(c_id);
                                    }
                                    _ => panic!("No prereq as other type"),
                                }
                            }
                            "uses" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Representation(rid) => {
                                        self.get_mut(id).representations.push(rid);
                                    }
                                    _ => panic!("No prereq as other type"),
                                }
                            }
                            _ => panic!("Unknown relationship on add to activity: {}", c.html),
                        }
                    }
                    _ => panic!("Unknown field of activity: {}", c.field),
                }
            }
            AnyID::Representation(id) => {
                match &c.field as &str {
                    "icon" => {
                        self.get_mut(id).icon = Icon::Html(c.html.trim().to_string());
                    }
                    "name" => {
                        self.get_mut(id).name = c.content.trim().to_string();
                    }
                    "description" => {
                        self.get_mut(id).description = Markdown::from_html(&c.html);
                    }
                    "Add" => {
                        match c.html.as_ref() {
                            "used by" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(child_id) => {
                                        self.get_mut(child_id).add_representation(id);
                                    }
                                    _ => panic!("Weird used by type: {:?}", c.content),
                                }
                            }
                            _ => panic!("Unknown relationship: {}", c.html),
                        }
                    }
                    "Remove" => {
                        match c.html.as_ref() {
                            "exists" => {
                                if id.0 == self.representations.len() - 1 {
                                    self.representations.pop();
                                    while self.representations[self.representations.len()-1].name.len() == 0 {
                                        self.representations.pop();
                                    }
                                } else {
                                    self.get_mut(id).name = "".to_string();
                                    self.get_mut(id).icon = Icon::Html("".to_string());
                                    self.get_mut(id).description = "".into();
                                }
                            }
                            "used by" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(child_id) => {
                                        self.get_mut(child_id).representations.remove(&id);
                                    }
                                    AnyID::Activity(child_id) => {
                                        self.get_mut(child_id).representations
                                            .retain(|&r| r != id);
                                    }
                                    _ => panic!("Weird used by type to remove: {:?}", c.content),
                                }
                            }
                            _ => panic!("Unknown relationship: {}", c.html),
                        }
                    }
                    _ => panic!("Unknown field of representation: {}", c.field),
                }
            }
        }
        self.save();
        Ok(())
    }
    pub fn concept_by_name(&self, name: &str) -> Option<ConceptID> {
        let name = name.trim();
        self.concepts.iter()
            .filter(|c| &c.name == name || Some(c.id.0) == name.parse().ok())
            .map(|c| c.id)
            .next()
    }
    pub fn concept_by_name_or_create(&mut self, name: &str) -> ConceptID {
        let name = name.trim();
        if let Some(c) = self.concept_by_name(name) {
            return c;
        }
        let newid = ConceptID(self.concepts.len());
        self.concepts.push(Concept {
            id: newid,
            name: name.to_string(),
            prereq_concepts: Vec::new(),
            representations: std::collections::BTreeMap::new(),
            figure: None,
            long_description: "".into(),
        });
        newid
    }
    pub fn activity_by_name(&self, name: &str) -> Option<ActivityID> {
        self.activities.iter()
            .filter(|c| &c.name == name)
            .map(|c| c.id)
            .next()
    }
    pub fn activity_by_name_or_create(&mut self, name: &str) -> ActivityID {
        if let Some(c) = self.activity_by_name(name) {
            return c;
        }
        let newid = ActivityID(self.activities.len());
        self.activities.push(Activity {
            id: newid,
            name: name.to_string(),
            prereq_concepts: Vec::new(),
            new_concepts: Vec::new(),
            representations: Vec::new(),
            figure: None,
            long_description: "".into(),
            external_url: None,
        });
        newid
    }
    pub fn representation_by_name(&self, name: &str) -> Option<RepresentationID> {
        self.representations.iter()
            .filter(|c| &c.name == name)
            .map(|c| c.id)
            .next()
    }
    pub fn representation_by_name_or_create(&mut self, name: &str) -> RepresentationID {
        if let Some(c) = self.representation_by_name(name) {
            return c;
        }
        if let Some(newidnum) = self.representations.iter().enumerate()
            .filter(|(_,r)| r.name.len() == 0).map(|(i,_)| i).next()
        {
            let newid = RepresentationID(newidnum);
            self.representations[newidnum] = Representation {
                id: newid,
                name: name.to_string(),
                description: Default::default(),
                icon: Icon::Html(name.to_string()),
            };
            newid
        } else {
            let newid = RepresentationID(self.representations.len());
            self.representations.push(Representation {
                id: newid,
                name: name.to_string(),
                description: Default::default(),
                icon: Icon::Html(name.to_string()),
            });
            newid
        }
    }
    pub fn course_by_name(&self, name: &str) -> Option<CourseID> {
        let name = name.trim();
        self.courses.iter()
            .filter(|c| &c.number == name || &c.name == name)
            .map(|c| c.id)
            .next()
    }
    pub fn add_to_course(&mut self, course: CourseID, a: ActivityID) {
        if !self.get(course).activities.contains(&a) {
            self.get_mut(course).activities.push(a);
        }
    }
    pub fn lower_anchor(&mut self, course_name: &str) -> ActivityID {
        let course = self.course_by_name_or_create(course_name);
        let id = self.activity_by_name_or_create(&format!("lower anchor {}", course_name));
        self.add_to_course(course, id);
        id
    }
    pub fn course_by_name_or_create(&mut self, name: &str) -> CourseID {
        let name = name.trim();
        if let Some(c) = self.course_by_name(name) {
            return c;
        }
        let newid = CourseID(self.courses.len());
        let (number, name) = match name {
            "MTH 251" => (name, "Differential Calculus"),
            "MTH 254" => (name, "Multivariable Calculus"),
            "MTH 255" => (name, "Vector Calculus"),
            "PH 423" => (name, "Energy and Entropy"),
            "PH 422" => (name, "Static Fields"),
            _ => (name, name),
        };
        self.courses.push(Course {
            id: newid,
            number: number.to_string(),
            name: name.to_string(),
            activities: Vec::new(),
        });
        newid
    }
    pub fn set_concept(&mut self, c: Concept) {
        let id = c.id;
        *self.get_mut(id) = c;
    }
    pub fn set_activity(&mut self, c: Activity) {
        let id = c.id;
        *self.get_mut(id) = c;
    }
    pub fn get_activity(&mut self, id: ActivityID) -> &mut Activity {
        self.get_mut(id)
    }

    fn courses_for_concept(&self, id: ConceptID) -> Vec<CourseID> {
        self.courses.iter()
            .filter(|c| self.course_is_for_concept(id.into(), c.id))
            .map(|c| c.id)
            .collect()
    }
    fn course_is_for_concept(&self, nid: ConceptRepresentationID, rid: CourseID) -> bool {
        for a in self.get(rid).activities.iter() {
            if self.get(*a).new_concepts.contains(&nid) {
                return true;
            }
        }
        false
    }
    pub fn concept_map(&self, max_width: usize, num_iters: usize) -> ConceptMap
    {
        let mut edges = Vec::new();
        for c in self.concepts.iter() {
            edges.extend(c.prereq_concepts.iter().map(|&pre| (pre, c.id)));
        }
        let layers = layer_concepts(edges.clone(), max_width);

        let concepts: Vec<ConceptID> = layers.iter().flat_map(|x| x.iter().cloned()).collect();
        // FIXME I should first ensure there is no cycle in the
        // edges... to avoid an infinite loop.
        use std::collections::BTreeMap;
        let mut children_map = BTreeMap::new();
        let mut parents_map = BTreeMap::new();
        for c in concepts.iter().cloned() {
            children_map.insert(c, Vec::new());
            parents_map.insert(c, Vec::new());
        }
        for (parent, child) in edges.into_iter() {
            children_map.entry(parent).or_insert(Vec::new()).push(child);
            parents_map.entry(child).or_insert(Vec::new()).push(parent);
        }
        let mut rows: Vec<Vec<ConceptNode>> = Vec::new();
        let mut extras: Vec<ConceptNode> = Vec::new();
        let mut next_fakeid = self.concepts.len();
        if layers.len() == 0 {
            println!("Why are there no layers?!");
            return ConceptMap { rows };
        }
        for i in 0..layers.len()-1 {
            let mut this_layer = Vec::new();
            let my_extras: Vec<_> = extras.drain(..).collect();
            for mut node in layers[i].iter().cloned()
                .map(|c| ConceptNode::CheapConcept {
                    concept: c,
                    children: children_map[&c].iter().map(|&c| c.into()).collect(),
                })
                .chain(my_extras.into_iter()) // also add in the fake nodes for lines passing through...
            {
                for child in node.children().filter(|c| !layers[i+1].contains(&ConceptID(c.0)))
                {
                    let fakeid = NodeID({ next_fakeid += 1; next_fakeid });
                    node.replace_child(child, fakeid);
                    extras.push(ConceptNode::Fake { fakeid, child });
                }
                this_layer.push(node);
            }
            let lastlayer = Vec::new();
            let lastlayer = rows.last().clone().unwrap_or(&lastlayer);
            let parentmean = |child: &ConceptNode| {
                // return the mean position of parents of this child,
                // in the previous row.
                let mut num_parents = 0;
                let mut total_index = 0;
                for (i,p) in lastlayer.iter().enumerate() {
                    if p.children().any(|x| x == child.id()) {
                        num_parents += 1;
                        total_index += i;
                    }
                }
                if num_parents < 2 {
                    500
                } else {
                    total_index*1000/(num_parents-1)
                }
            };
            this_layer.sort_by_key(parentmean);
            rows.push(this_layer);
        }
        rows.push(layers[layers.len()-1].iter()
                  .map(|&c| ConceptNode::CheapConcept {
                      concept: c,
                      children: children_map[&c].iter().map(|&c| c.into()).collect(),
                  })
                  .collect());
        let orphans: Vec<_> = self.concepts.iter()
            .filter(|c| !children_map.contains_key(&c.id) && !parents_map.contains_key(&c.id))
            .map(|c| ConceptNode::CheapConcept {
                concept: c.id,
                children: Vec::new(),
            })
            .collect();
        let mut cmap = ConceptMap { rows }.optimize(num_iters);
        for orph in orphans.chunks(max_width) {
            cmap.rows.push(orph.to_vec());
        }
        cmap.make_expensive(self)
    }

    pub fn concept_representation_view(&self, parent: impl Copy+DisplayAs<HTML>,
                                       field: &'static str,
                                       id: ConceptRepresentationID)
                                       -> Child<ConceptRepresentationView> {
        let c = self.get(id.concept);
        if let Some(rid) = id.representation {
            let rr = self.get(id.concept).representations.get(&rid)
                .expect("trying to access invalid ConceptRepresentationID");
            Child::remove(parent, field, ConceptRepresentationView {
                id,
                representation: Some(self.get(rid).clone()),
                activities: self.activities.iter()
                    .filter(|a| a.new_concepts.contains(&id))
                    .cloned().collect(),
                name: rr.name.clone(),
                summary_name: self.name_it(id),
                long_description: rr.long_description.clone(),
                figure: rr.figure.clone(),
            })
        } else {
            Child::remove(parent, field, ConceptRepresentationView {
                id,
                representation: None,
                activities: self.activities.iter()
                    .filter(|a| a.new_concepts.contains(&id))
                    .cloned().collect(),
                name: c.name.clone(),
                summary_name: c.name.clone(),
                long_description: c.long_description.clone(),
                figure: c.figure.clone(),
            })
        }
    }

    pub fn name_it(&self, id: ConceptRepresentationID) -> String {
        let c = self.get(id.concept);
        if let Some(rid) = id.representation {
            let n = c.representations.get(&rid).unwrap().name.clone();
            if n.len() == 0 {
                format!("{} using {}", c.name, self.get(rid).name.to_lowercase())
            } else {
                n
            }
        } else {
            c.name.clone()
        }
    }
    pub fn concept_representation_to_id(&self, name: &str) -> Option<ConceptRepresentationID> {
        if let Some(c) = self.concepts.iter().filter(|c| c.name == name).next() {
            return Some(c.id.into());
        } else {
            for c in self.concepts.iter() {
                if c.name == name {
                    return Some(c.id.into());
                }
                for &rid in c.representations.keys() {
                    if self.name_it((c.id, rid).into()) == name {
                        return Some((c.id, rid).into());
                    }
                }
            }
            None
        }
    }
    fn untaught_concept_representations(&self) -> Vec<ConceptRepresentationID> {
        let mut ids: Vec<ConceptRepresentationID> = self.concepts.iter()
            .map(|c| c.id.into())
            .chain(self.concepts.iter()
                   .flat_map(|c| {
                       let cid = c.id;
                       c.representations.keys()
                           .map(move |&x| (cid, x).into())
                   }))
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }
    pub fn all_concept_representations(&self, id: impl Copy+DisplayAs<HTML>, field: &str)
                                       -> ConceptRepresentationChoice {
        let mut names: Vec<String> = self.concepts.iter().map(|c| c.name.clone())
            .chain(self.concepts.iter()
                   .flat_map(|c| {
                       let cid = c.id;
                       c.representations.keys()
                           .map(move |&x| self.name_it((cid, x).into()))
                   }))
            .collect();
        names.sort();
        names.dedup();
        ConceptRepresentationChoice {
            id: format_as!(HTML, id),
            field: field.to_string(),
            choices: names,
        }
    }
    fn representation_used(&self, id: RepresentationID) -> bool {
        if self.activities.iter().any(|a| a.representations.contains(&id)) {
            return true;
        }
        if self.concepts.iter().any(|c| c.representations.contains_key(&id)) {
            return true;
        }
        false
    }
    pub fn all_representations(&self) -> impl DisplayAs<HTML> {
        List(self.representations.iter()
             .filter(|r| r.name.len() > 0)
             .map(|r| {
                 if self.representation_used(r.id) {
                     Child::none(r.clone())
                 } else {
                     Child::remove(r.id, "exists", r.clone())
                 }
             }).collect())
    }

    pub fn concept_view(&self, id: ConceptID) -> ConceptView {
        let c = &self.get(id);
        let my_prereq_concepts: Vec<_> =
            self.concepts.iter().filter(|x| c.prereq_concepts.contains(&x.id)).cloned().collect();
        let mut view = ConceptView {
            id,
            name: c.name.clone(),

            all_concepts: self.concepts.clone(),
            all_activities: self.activities.clone(),
            representation_choice: AnyChoice {
                class: "representation".to_string(),
                id: format_as!(HTML, id),
                field: "representation".to_string(),
                choices: self.representations.iter().map(|c| c.name.clone()).collect(),
            },

            activities: Vec::new(),
            prereq_courses: Vec::new(),

            prereq_concepts: Vec::new(),
            prereq_groups: Vec::new(),
            needed_for_concepts: Vec::new(),

            output_groups: Vec::new(),

            representations: self.group_concepts(c.representations.keys()
                                                 .map(|&rid| (id,rid).into()).collect(),
                                                 id, "with"),
            courses: self.courses_for_concept(c.id).iter().map(|&cid| self.get(cid).clone()).collect(),
            figure: c.figure.clone(),
            long_description: c.long_description.clone(),
        };
        // We haven't generated this view yet, so we need to add the
        // related concepts.
        view.prereq_courses = self.courses.iter().cloned()
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: my_prereq_concepts.iter()
                    .filter(|c| self.course_is_for_concept(c.id.into(), course.id))
                    .map(|c| self.concept_representation_view(id, "prereq", c.id.into()))
                    .collect(),
            })
            .filter(|xx| xx.concepts.len() > 0 &&
                    !self.course_is_for_concept(c.id.into(), xx.course.id))
            .collect();
        let the_prereq_courses: Vec<_> = view.prereq_courses.iter().map(|x| x.course.id).collect();

        let output_concepts: Vec<_> = self.concepts.iter()
            .filter(|x| x.prereq_concepts.contains(&id))
            .map(|x| x.id.into())
            .collect();
        view.output_groups = self.group_concepts(output_concepts, id, "needed for");
        for a in self.activities.iter()
            .filter(|a| a.prereq_concepts.iter().any(|cc| cc.concept == id))
        {
            self.extend_groups_with_activity(&mut view.output_groups, a.clone(), id, "needed for");
        }
        view.activities =
            self.activities.iter()
            .filter(|a| a.new_concepts.contains(&id.into()))
            .cloned().collect();
        view.prereq_concepts =
            c.prereq_concepts.iter()
            .map(|x| self.get(*x))
            .filter(|x| !the_prereq_courses.iter()
                    .any(|&z| self.course_is_for_concept(x.id.into(), z)))
            .map(|x| x.id)
            .collect();
        view.prereq_groups = self.group_concepts(view.prereq_concepts.iter().cloned()
                                                 .map(|c| c.into()).collect(),
                                                 id, "prereq");
        view.needed_for_concepts =
            self.concepts.iter()
            .filter(|x| x.prereq_concepts.contains(&id))
            .map(|x| x.id)
            .collect();
        view
    }

    pub fn activity_view(&self, id: ActivityID) -> ActivityView {
        let a = &self.get(id);
        let prereq_concepts_in_this_course: Vec<_> =
            a.prereq_concepts.iter().cloned()
            .filter(|&cid| self.courses.iter()
                    .filter(|xx| xx.activities.contains(&id))
                    .any(|cc| self.course_is_for_concept(cid, cc.id)))
            .collect();
        let mut view = ActivityView {
            id,
            name: a.name.clone(),

            choices: self.all_concept_representations(id, "???"),
            representation_choice: AnyChoice {
                class: "representation".to_string(),
                id: format_as!(HTML, id),
                field: "representation".to_string(),
                choices: self.representations.iter().map(|c| c.name.clone()).collect(),
            },

            prereq_courses: self.courses.iter().cloned()
                .filter(|course| !course.activities.contains(&a.id))
                .map(|course| {
                    use std::collections::BTreeSet;
                    let crs: BTreeSet<ConceptRepresentationID> = course.activities.iter()
                        .flat_map(|&a| self.get(a).new_concepts.clone()).collect();
                    PrereqCourse {
                        course: course.clone(),
                        concepts: crs.iter()
                            .filter(|&cid| a.prereq_concepts.contains(&cid)
                                    //   && !prereq_concepts_in_this_course.contains(&cid)
                            )
                            .map(|&cid| self.concept_representation_view(id, "prereq", cid))
                            .collect(),
                    }
                })
                .filter(|xx| xx.concepts.len() > 0)
                .collect(),

            prereq_groups: self.group_concepts(prereq_concepts_in_this_course,
                                               id, "prereq"),
            new_concepts: a.new_concepts.iter()
                .map(|&cid| self.concept_representation_view(id, "new", cid))
                .collect(),

            output_groups: self.group_concepts(a.new_concepts.clone(), id, "depends on"),

            representations: a.representations.iter()
                .map(|&rid| Child::remove(id, "uses", self.get(rid).clone())).collect(),
            courses: self.courses.iter().filter(|c| c.activities.contains(&id))
                .cloned().collect(),
            figure: a.figure.clone(),
            long_description: a.long_description.clone(),
            external_url: a.external_url.clone(),
        };

        for a in self.activities.iter()
            .filter(|aa| a.new_concepts.iter().any(|cc| aa.prereq_concepts.contains(&cc)))
        {
            self.extend_groups_with_activity(&mut view.output_groups, a.clone(), id,
                                             "depends on");
        }

        view
    }

    pub fn representation_view(&self, id: RepresentationID) -> RepresentationView {
        let r = self.get(id).clone();
        let all_concepts_using_r: Vec<Child<ConceptRepresentationView>> = self.concepts.iter()
            .filter(|c| c.representations.contains_key(&id))
            .map(|c| self.concept_representation_view(id, "", (c.id, r.id).into()))
            .collect();
        let all_activities_using_r: Vec<_> = self.activities.iter()
            .filter(|c| c.representations.contains(&id))
            .cloned()
            .collect();
        let activity_concepts: Vec<_> = all_concepts_using_r.iter()
            .filter(|c| all_activities_using_r.iter()
                    .any(|a| a.new_concepts.iter().any(|&xx| xx == c.id)))
            .cloned()
            .collect();
        let other_concepts: Vec<_> = all_concepts_using_r.into_iter()
            .filter(|c| !activity_concepts.contains(&c))
            .collect();
        let mut groups: Vec<_> = self.group_concepts(activity_concepts.iter()
                                                     .map(|c| c.id.into()).collect(),
                                                     id, "used by");
        for a in self.activities.iter()
            .filter(|a| a.representations.contains(&id))
        {
            self.extend_groups_with_activity(&mut groups, a.clone(), id, "used by");
        }
        RepresentationView {
            id,
            name: r.name,
            description: r.description,
            icon: r.icon,
            other_concepts,
            groups,
        }
    }

    pub fn progression_view(&self) -> ProgressionView {
        let courses: Vec<_> = self.courses.iter().map(|c| self.course_sequence(c.id))
            .filter(|x| x.groups.len() > 1) // FIXME should handle prereqs better?
            .collect();
        let prereq_courses: Vec<_> = self.courses.iter()
            .take(courses[0].course.id.0).cloned()
            .map(|course| {
                use std::collections::BTreeSet;
                let crs: BTreeSet<ConceptRepresentationID> =
                    course.activities.iter().flat_map(|&a| self.get(a).new_concepts.clone()).collect();
                PrereqCourse {
                    course: course.clone(),
                    concepts:  crs.iter().map(|&cid| self.concept_representation_view(course.id, "", cid))
                        .collect(),
                }
            })
            .collect();
        ProgressionView {
            prereq_courses,
            courses,
        }
    }

    pub fn course_sequence(&self, id: CourseID) -> CourseSequence {
        let course = self.get(id).clone();
        let course_activities: Vec<_> = course.activities.iter()
            .map(|&a| self.activity_view(a)).collect();
        let mut groups = Vec::new();
        for a in course_activities.into_iter() {
            groups.push(ProgressionGroup {
                concepts: a.new_concepts.iter()
                    .map(|c| self.concept_representation_view(a.id, "new", c.id.into()))
                    .collect(),
                activity: Child::remove(id, "teaches", a),
            });
        }
        let new_activity = ActivityChoice {
            id: format_as!(HTML, id),
            field: "activity".to_string(),
            choices: self.activities.iter().filter(|a| !course.activities.contains(&a.id))
                .cloned().collect(),
        };
        CourseSequence { course, prereq_courses: Vec::new(), new_activity, groups }
    }

    pub fn course_view(&self, id: CourseID) -> CourseSequence {
        let mut cs = self.course_sequence(id);

        let my_activities: Vec<Activity> = self.get(id).activities.iter()
            .map(|&a| self.get(a).clone()).collect();

        use std::collections::BTreeSet;
        let my_prereq_concepts: BTreeSet<ConceptRepresentationID> =
            my_activities.iter().flat_map(|a| a.prereq_concepts.clone()).collect();
        let my_prereq_concepts: Vec<Child<ConceptRepresentationView>> =
            my_prereq_concepts.iter().map(|&cr| self.concept_representation_view(id, "prereq", cr)).collect();

        cs.prereq_courses = self.courses.iter()
            .filter(|course| course.id != id)
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: my_prereq_concepts.iter()
                    .filter(|c| self.course_is_for_concept(c.id, course.id))
                    .cloned().collect(),
            })
            .filter(|xx| xx.concepts.len() > 0)
            .collect();
        cs
    }

    fn group_concepts(&self, x: Vec<ConceptRepresentationID>,
                      parentid: impl Copy+DisplayAs<HTML>,
                      relationship: &'static str) -> Vec<ActivityGroup> {
        let mut out: Vec<ActivityGroup> = Vec::new();
        for cid in x.iter().cloned() {
            let act: Vec<_> = self.activities.iter()
                .filter(|a| a.new_concepts.contains(&cid))
                .map(|x| x.id).collect();
            let cc = self.concept_representation_view(parentid, relationship, cid);
            if let Some(ref mut group) = out.iter_mut()
                .filter(|x| act.iter().any(|&xx| Some(Child::remove(parentid, relationship, self.get(xx).clone())) == x.activity))
                .next()
            {
                group.concepts.push(cc);
                group.hint_concepts.retain(|x| x.id != cid);
            } else {
                if act.len() >= 1 {
                    let a = self.get(act[0]);
                    out.push(ActivityGroup {
                        activity: Some(Child::remove(parentid, relationship,
                                                     a.clone())),
                        concepts: vec![cc],
                        hint_concepts: a.new_concepts.iter()
                            .filter(|&&x| x != cid)
                            .map(|&id| self.concept_representation_view(parentid,
                                                                        relationship,
                                                                        id)
                                 .set_verb("Add"))
                            .collect(),
                    });
                } else {
                    out.push(ActivityGroup {
                        activity: None,
                        concepts: vec![cc],
                        hint_concepts: Vec::new(),
                    });
                }
            }
        }
        out.sort_unstable_by_key(|g| {
            if let Some(ref a) = g.activity {
                a.id
            } else {
                ActivityID(100000)
            }
        });
        for g in out.iter_mut() {
            g.concepts.sort_unstable_by_key(|c| c.id);
        }
        out
    }
    fn extend_groups_with_activity(&self, gs: &mut Vec<ActivityGroup>, a: Activity,
                                   parentid: impl Copy+DisplayAs<HTML>,
                                   relationship: &'static str) {
        for existing_id in gs.iter().flat_map(|g| g.activity.iter().map(|x| x.id)) {
            if existing_id == a.id {
                return;
            }
        }
        gs.push(ActivityGroup {
            hint_concepts: self.get(a.id).new_concepts.iter()
                .map(|&c| self.concept_representation_view(parentid, relationship, c)
                     .set_verb("Add"))
                .collect(),
            activity: Some(Child::remove(parentid, relationship, a)),
            concepts: Vec::new(),
        });
    }

    pub fn spawn_dump_mirror(self) {
        use std::sync::atomic::{AtomicBool, Ordering};

        static AM_MIRRORING: AtomicBool = AtomicBool::new(false);
        if !AM_MIRRORING.compare_and_swap(false, true, Ordering::Relaxed) {
            std::thread::spawn(move || {
                self.dump_mirror();
                AM_MIRRORING.store(false, Ordering::Relaxed);
            });
        }
    }
    pub fn dump_mirror(self) {
        std::fs::create_dir_all("mirror/concept").unwrap();
        println!("creating concepts...");
        for a in self.concepts.iter() {
            std::fs::write(format_as!(URL, "mirror/" a),
                           format_as!(HTML, self.concept_view(a.id))).ok();
        }
        println!("creating activities...");
        std::fs::create_dir_all("mirror/activity").unwrap();
        for a in self.activities.iter() {
            std::fs::write(format_as!(URL, "mirror/" a),
                           format_as!(HTML, self.activity_view(a.id))).ok();
        }
        println!("creating courses...");
        std::fs::create_dir_all("mirror/course").unwrap();
        for a in self.courses.iter() {
            std::fs::write(format_as!(URL, "mirror/" a),
                           format_as!(HTML, self.course_view(a.id))).ok();
        }
        println!("creating representations...");
        std::fs::create_dir_all("mirror/representation").unwrap();
        for a in self.representations.iter() {
            std::fs::write(format_as!(URL, "mirror/" a),
                           format_as!(HTML, self.representation_view(a.id))).ok();
        }
        {
            std::fs::write(format_as!(URL, "mirror/index.html"),
                           format_as!(HTML, self.progression_view())).ok();
        }
        std::fs::create_dir_all("mirror/concept-map").unwrap();
        for max_width in 1..18 {
            std::fs::write(format_as!(URL, "mirror/concept-map/" max_width),
                           format_as!(HTML, self.concept_map(max_width, 1 << 20))).ok();
        }
        std::fs::write(format_as!(URL, "mirror/concept-map/index.html"),
                       format_as!(HTML, self.concept_map(4, 1 << 20))).ok();
        println!("All done with the mirror!");
    }
}

/// This is a course and concepts it teaches.
#[derive(Debug, Clone)]
pub struct PrereqCourse {
    pub course: Course,
    pub concepts: Vec<Child<ConceptRepresentationView>>,
}
#[with_template("[%" "%]" "prereq-course.html")]
impl DisplayAs<HTML> for PrereqCourse {}

/// This is an activity and concepts it teaches.
#[derive(Debug, Clone)]
pub struct ActivityGroup {
    pub activity: Option<Child<Activity>>,
    pub concepts: Vec<Child<ConceptRepresentationView>>,
    pub hint_concepts: Vec<Child<ConceptRepresentationView>>
}
impl ActivityGroup {
    pub fn with_detail(self) -> Self
    {
        ActivityGroup { concepts: self.concepts.into_iter()
                        .map(|x| x.with_detail()).collect(),
                        .. self}
    }
}

#[with_template("[%" "%]" "activity-group.html")]
impl DisplayAs<HTML> for ActivityGroup {}

/// This is an activity and concepts it teaches, but displayed in a progression.
#[derive(Debug, Clone)]
pub struct ProgressionGroup {
    pub activity: Child<ActivityView>,
    pub concepts: Vec<Child<ConceptRepresentationView>>,
}
#[with_template("[%" "%]" "progression-group.html")]
impl DisplayAs<HTML> for ProgressionGroup {}

pub struct CourseSequence {
    course: Course,
    prereq_courses: Vec<PrereqCourse>,
    new_activity: ActivityChoice,
    groups: Vec<ProgressionGroup>,
}
#[with_template("[%" "%]" "course-sequence.html")]
impl DisplayAs<HTML> for CourseSequence {}

pub struct ProgressionView {
    prereq_courses: Vec<PrereqCourse>,
    courses: Vec<CourseSequence>,
}
#[with_template("[%" "%]" "progression.html")]
impl DisplayAs<HTML> for ProgressionView {}

/// Represents a choice between concepts!
pub struct ConceptChoice {
    pub id: String,
    pub field: String,
    pub choices: Vec<Concept>,
}
#[with_template("[%" "%]" "concept-choice.html")]
impl DisplayAs<HTML> for ConceptChoice {}

/// Represents a choice between concepts!
#[derive(Debug, Clone)]
pub struct ConceptRepresentationChoice {
    pub id: String,
    pub field: String,
    pub choices: Vec<String>,
}
#[with_template("[%" "%]" "concept-representation-choice.html")]
impl DisplayAs<HTML> for ConceptRepresentationChoice {}

/// Represents a choice between activities!
pub struct ActivityChoice {
    pub id: String,
    pub field: String,
    pub choices: Vec<Activity>,
}
#[with_template("[%" "%]" "activity-choice.html")]
impl DisplayAs<HTML> for ActivityChoice {}

/// Represents a choice between anything!
#[derive(Debug, Clone)]
pub struct AnyChoice {
    pub class: String,
    pub id: String,
    pub field: String,
    pub choices: Vec<String>,
}
#[with_template("[%" "%]" "any-choice.html")]
impl DisplayAs<HTML> for AnyChoice {}

/// Represents adding or removing a thingy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct ChangeRelationship {
    /// This is the containing object
    pub parentid: String,
    /// verb is most likely add or remove
    pub verb: String,
    pub childid: String,
    /// relationship is "prereq" or similar.
    pub relationship: String,
}
#[with_template("[%" "%]" "change-relationship.html")]
impl DisplayAs<HTML> for ChangeRelationship {}

impl ChangeRelationship {
    pub fn none() -> Self {
        ChangeRelationship {
            parentid: "".to_string(),
            childid: "".to_string(),
            verb: "".to_string(),
            relationship: "".to_string(),
        }
    }
    pub fn parent(parentid: impl DisplayAs<HTML>,
                  verb: &'static str, relationship: &'static str) -> Self {
        ChangeRelationship {
            parentid: format_as!(HTML, parentid),
            childid: "".to_string(),
            verb: verb.to_string(),
            relationship: relationship.to_string(),
        }
    }
    pub fn child(&self, childid: impl DisplayAs<HTML>) -> ChangeRelationship {
        ChangeRelationship { childid: format_as!(HTML, childid), .. self.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct Child<T> {
    child: T,
    /// This is the containing object
    pub parentid: String,
    /// verb is most likely add or remove
    pub verb: String,
    /// relationship is "prereq" or similar.
    pub relationship: String,
    /// Show extra detail
    pub show_detail: bool,
    /// Extra class to display
    pub extra_class: String,
    /// The id in HTML
    pub html_id: Option<String>,
}

impl<T> Child<T> {
    pub fn new(parentid: impl DisplayAs<HTML>, verb: &'static str,
               relationship: &'static str, child: T) -> Self
    {
        Child {
            child,
            parentid: format_as!(HTML, parentid),
            verb: verb.to_string(),
            relationship: relationship.to_string(),
            show_detail: false,
            extra_class: "".to_string(),
            html_id: None,
        }
    }
    pub fn none(child: T) -> Self
    {
        Child {
            child,
            parentid: "".to_string(),
            verb: "".to_string(),
            relationship: "".to_string(),
            show_detail: false,
            extra_class: "".to_string(),
            html_id: None,
        }
    }
    pub fn add(parentid: impl DisplayAs<HTML>,
               relationship: &'static str, child: T) -> Self
    {
        Child {
            child,
            parentid: format_as!(HTML, parentid),
            verb: "Add".to_string(),
            relationship: relationship.to_string(),
            show_detail: false,
            extra_class: "".to_string(),
            html_id: None,
        }
    }
    pub fn remove(parentid: impl DisplayAs<HTML>,
                  relationship: &'static str, child: T) -> Self
    {
        Child {
            child,
            parentid: format_as!(HTML, parentid),
            verb: "Remove".to_string(),
            relationship: relationship.to_string(),
            show_detail: false,
            extra_class: "".to_string(),
            html_id: None,
        }
    }
    pub fn with_detail(self) -> Self
    {
        Child { show_detail: true, .. self }
    }
    pub fn set_verb(self, verb: &'static str) -> Self
    {
        Child { verb: verb.to_string(), .. self }
    }
    pub fn with_class(self, extra_class: String) -> Self
    {
        Child { extra_class, .. self }
    }
    pub fn with_html_id(self, html_id: String) -> Self
    {
        Child { html_id: Some(html_id), .. self }
    }
}
macro_rules! impl_child_addremove{
    ($t:ty) => {
        impl Child<$t> {
            pub fn addremove(&self) -> ChangeRelationship {
                ChangeRelationship {
                    parentid: self.parentid.clone(),
                    childid: format_as!(HTML, self.id),
                    verb: self.verb.clone(),
                    relationship: self.relationship.clone(),
                }
            }
        }
        impl PartialEq for Child<$t> {
            fn eq(&self, x: &Child<$t>) -> bool {
                self.id == x.id
            }
        }
    }
}
impl_child_addremove!(Concept);
impl_child_addremove!(Activity);
impl_child_addremove!(ActivityView);
impl_child_addremove!(Representation);
impl_child_addremove!(ConceptRepresentationView);

impl<T> std::ops::Deref for Child<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.child
    }
}

#[with_template( self.id )]
impl DisplayAs<URL> for Child<Concept> {}
#[with_template("[%" "%]" "concept.html")]
impl DisplayAs<HTML> for Child<Concept> {}

#[with_template( self.id )]
impl DisplayAs<URL> for Child<Activity> {}

#[with_template( self.id )]
impl DisplayAs<URL> for Child<Representation> {}
#[with_template("[%" "%]" "representation.html")]
impl DisplayAs<HTML> for Child<Representation> {}


// The following enables visualization of graphs.
impl<'a> dot::GraphWalk<'a,ConceptID,(ConceptID,ConceptID)> for Data {
    fn nodes(&'a self) -> dot::Nodes<'a,ConceptID> {
        // Only include concepts that have *some* dependency.
        let mut out: Vec<_> = self.concepts.iter().flat_map(|c| c.prereq_concepts.clone()).collect();
        for c in self.concepts.iter() {
            if c.prereq_concepts.len() > 0 {
                out.push(c.id);
            }
        }
        out.sort();
        out.dedup();
        out.into()
    }
    fn edges(&'a self) -> dot::Nodes<'a,(ConceptID,ConceptID)> {
        let mut out = Vec::new();
        for c in self.concepts.iter() {
            out.extend(c.prereq_concepts.iter().map(|&pre| (pre, c.id)));
        }
        out.into()
    }
    fn source(&self, edge: &(ConceptID, ConceptID)) -> ConceptID {
        edge.0
    }
    fn target(&self, edge: &(ConceptID, ConceptID)) -> ConceptID {
        edge.1
    }
}

/// An attempt at Coffman-Graham
pub fn layer_concepts(edges: Vec<(ConceptID, ConceptID)>,
                      max_width: usize)
    -> Vec<Vec<ConceptID>>
{
    let mut concepts: Vec<ConceptID> = edges.iter()
        .flat_map(|x| vec![x.0,x.1].into_iter()).collect();
    concepts.sort();
    concepts.dedup();
    // FIXME I should first ensure there is no cycle in the
    // edges... to avoid an infinite loop.
    use std::collections::BTreeMap;
    let mut children_map = BTreeMap::new();
    let mut parents_map = BTreeMap::new();
    for c in concepts.iter().cloned() {
        children_map.insert(c, Vec::new());
        parents_map.insert(c, Vec::new());
    }
    for (parent, child) in edges.into_iter() {
        children_map.entry(parent).or_insert(Vec::new()).push(child);
        parents_map.entry(child).or_insert(Vec::new()).push(parent);
    }
    // out is a map from ConceptID to the order it should have in the
    // sort.  This representation makes it easy to look up a the
    // location of a given concept in an ordered thingy.
    let mut out: BTreeMap<ConceptID, isize> = BTreeMap::new();
    // Find possible nodes to start with...
    let mut starts: Vec<_> =
        concepts.iter().cloned().filter(|c| parents_map[c].len() == 0).collect();
    let mut buggy_concepts: Vec<ConceptID> = Vec::new();
    if starts.len() > 0 {
        use std::collections::BTreeSet;
        let descendents_of = |c: ConceptID| -> BTreeSet<ConceptID> {
            let mut d = BTreeSet::new();
            d.insert(c);
            for _ in 0..30 { // just sloppily look at 30 generations of descendents
                let vc: Vec<_> = d.iter().cloned().collect();
                for cc in vc.into_iter() {
                    for x in children_map.get(&cc).iter().flat_map(|x| x.iter()) {
                        d.insert(*x);
                    }
                }
            }
            d.remove(&c);
            // println!("\n    descendents of {} are", c.0);
            // for dd in d.iter() {
            //     println!("        {}", dd.0);
            // }
            d
        };
        starts.sort_unstable_by_key(|c| -(descendents_of(*c).len() as isize));
        // start with the concept that has the most descendents in
        // total that doesn't need anything else.
        out.insert(starts[0], 0);
        concepts.retain(|&x| x != starts[0]);
        while concepts.len() > 0 {
            let mut nexts: Vec<_> = concepts.iter().cloned()
                .filter(|c| parents_map[c].iter().all(|p| out.contains_key(p)))
                .collect();
            let outsize = out.len() as isize;
            let urgency = |c: &ConceptID| -> isize {
                let urg = -parents_map[c].iter()
                    .map(|p| outsize - out.get(p).unwrap_or(&outsize))
                    .max().unwrap_or(0);
                let d_urg = -descendents_of(*c).iter()
                    .map(|cc| parents_map[cc].iter()
                         .map(|p| outsize - out.get(p).unwrap_or(&outsize))
                         .max().unwrap_or(0))
                    .max().unwrap_or(0);
                // println!("urgency of {} is {} from {} and {}", c.0, urg*4 + d_urg, urg, d_urg);
                urg*16 + d_urg
            };
            // println!("\nsorting by urgency...");
            nexts.sort_by_key(urgency);
            if nexts.len() == 0 {
                println!("Interesting problem, some unreachable concepts:");
                buggy_concepts.extend(concepts.iter());
                for c in concepts.iter() {
                    println!("   Concept {}", c.0);
                    for p in parents_map[c].iter().filter(|p| !out.contains_key(p)) {
                        println!("      missing parent: {}", p.0);
                    }
                }
                break;
            } else {
                // FIXME should pick from next ones with equal urgency
                // via proper algorithm...
                // println!("picking {}", nexts[0].0);
                out.insert(nexts[0], out.len() as isize);
                concepts.retain(|&cc| cc != nexts[0]);
            }
        }
    }
    let mut concepts: Vec<_> = out.into_iter().map(|(k,v)| (v,k)).collect();
    concepts.sort();
    let mut out: Vec<Vec<ConceptID>> = Vec::new();
    for concept in concepts.into_iter().map(|(_,v)| v) {
        let mut where_to_push: Option<usize> = None;
        let starting_row = if out.len() > 2 { out.len() - 2 } else { 0 };
        for i in (starting_row..out.len()).rev() {
            if out[i].iter().any(|x| children_map[x].contains(&concept)) {
                break;
            }
            if out[i].len() < max_width {
                where_to_push = Some(i);
                if children_map[&concept].len() > 0 {
                    break;
                }
            } else {
                // don't go up *above* a full row, or you're just
                // asking for way-too-long lines.
                break;
            }
        }
        if let Some(i) = where_to_push {
            out[i].push(concept);
        } else {
            out.push(vec![concept]);
        }
    }
    for c in buggy_concepts.into_iter() {
        out.push(vec![c]);
    }
    out
}

#[derive(Copy, Clone,Eq, PartialEq, PartialOrd, Ord)]
pub struct NodeID(usize);
impl From<ConceptID> for NodeID {
    fn from(x: ConceptID) -> Self { NodeID(x.0) }
}

/// A node in the concept map
#[derive(Clone,Eq, PartialEq)]
pub enum ConceptNode {
    /// An actual concept
    Concept {
        concept: ConceptView,
        children: Vec<NodeID>,
    },
    /// A "cheap" concept that just stores its ID
    CheapConcept {
        concept: ConceptID,
        children: Vec<NodeID>,
    },
    /// A fake node carrying a connection between concepts.
    Fake {
        fakeid: NodeID,
        child: NodeID,
    }
}
#[with_template("[%" "%]" "concept-node.html")]
impl DisplayAs<HTML> for ConceptNode {}

impl ConceptNode {
    fn is_fake(&self) -> bool {
        match self {
            ConceptNode::Concept{..} => false,
            ConceptNode::CheapConcept{..} => false,
            ConceptNode::Fake{..} => true,
        }
    }
    fn make_expensive(self, data: &Data) -> Self {
        match self {
            ConceptNode::Concept{..} => self,
            ConceptNode::CheapConcept{ concept, children} => {
                ConceptNode::Concept{ concept: data.concept_view(concept), children }
            },
            ConceptNode::Fake{..} => self,
        }
    }
    fn id(&self) -> NodeID {
        match self {
            ConceptNode::Concept{concept,..} => NodeID(concept.id.0),
            ConceptNode::CheapConcept{concept,..} => NodeID(concept.0),
            ConceptNode::Fake{fakeid,..} => *fakeid,
        }
    }
    fn children(&self) -> impl Iterator<Item=NodeID> {
        match self {
            ConceptNode::Concept{children,..} => children.clone().into_iter(),
            ConceptNode::CheapConcept{children,..} => children.clone().into_iter(),
            ConceptNode::Fake{child,..} => vec![*child].into_iter(),
        }
    }
    fn replace_child(&mut self, old: NodeID, new: NodeID) {
        match self {
            ConceptNode::Concept{ref mut children,..} => {
                for child in children.iter_mut() {
                    if *child == old { *child = new; }
                }
            }
            ConceptNode::CheapConcept{ref mut children,..} => {
                for child in children.iter_mut() {
                    if *child == old { *child = new; }
                }
            }
            ConceptNode::Fake{ref mut child,..} => {
                if *child == old { *child = new; }
            }
        }
    }
}

#[derive(Clone)]
pub struct ConceptMap {
    rows: Vec<Vec<ConceptNode>>,
}
#[with_template("[%" "%]" "concept-map.html")]
impl DisplayAs<HTML> for ConceptMap {}
const SCALE: usize = 2;
const DISTANCE_SCALE: usize = 1000000;
impl ConceptMap {
    pub fn make_expensive(self, data: &Data) -> Self {
        ConceptMap {
            rows: self.rows.into_iter()
                .map(|r| r.into_iter()
                     .map(|node| node.make_expensive(data))
                     .collect())
                .collect()
        }
    }
    pub fn crossings(&self, verbose: bool) -> usize {
        let mut cross = 0;
        let mut distance = 0;
        let findx = |ww: &[ConceptNode], ix| {
            let mut width = 0;
            let mut x = 0;
            for ii in 0..ix {
                if !ww[ii].is_fake() {
                    x += 1;
                }
            }
            width += x;
            for ii in ix..ww.len() {
                if !ww[ii].is_fake() {
                    width += 1;
                }
            }
            ((500 + x*1000)/(1+width)) as isize
        };
        for w in self.rows.windows(2) {
            let mut after = std::collections::BTreeMap::new();
            for (i,c) in w[1].iter().enumerate() {
                after.insert(c.id(), i);
            }
            for (i1,x1) in w[0].iter().enumerate() {
                for c1 in x1.children().flat_map(|x| after.get(&x)) {
                    let p2 = findx(&w[1], *c1);
                    let p1 = findx(&w[0], i1);
                    distance += ((p2-p1)*(p2-p1)) as usize;
                    for x2 in w[0].iter().enumerate().filter(|(i2,_)| i2 > &i1).map(|(_,x)| x) {
                        for c2 in x2.children().flat_map(|x| after.get(&x)) {
                            if c2 < c1 {
                                cross += 1;
                            }
                        }
                    }
                }
            }
        }
        if verbose {
            let d = distance as f64/DISTANCE_SCALE as f64;
            println!("   {} + {} = {}", cross, d, cross as f64 + d);
        }
        cross*SCALE + distance*SCALE/DISTANCE_SCALE
    }
    pub fn random_change(&self) -> Self {
        let mut out = self.clone();
        let to_change: usize = rand::random::<usize>() % out.rows.len();
        let rowlen: usize = out.rows[to_change].len();
        let to_swap1 = rand::random::<usize>() % rowlen;
        let to_swap2 = rand::random::<usize>() % rowlen;
        out.rows[to_change].swap(to_swap1, to_swap2);
        out
    }
    pub fn optimize(&self, num_iters: usize) -> Self {
        let mut best = self.clone();
        let mut current = self.clone();
        let mut e_best = best.crossings(false);
        let mut e = e_best;
        let mut logw = std::collections::BTreeMap::new();
        logw.insert(e_best, 1);
        for i in 0..num_iters {
            let trial = current.random_change();
            let e_trial = trial.crossings(false);
            let logw_old = logw.get(&e).unwrap_or(&0);
            let logw_new = logw.get(&e_trial).unwrap_or(&0);
            if logw_new < logw_old
              //  || (logw_new - logw_old < 8
              //      && rand::random::<usize>() % (1<<32) < (1<<32) >> (logw_new - logw_old) as u8)
            {
                // accept the move
                e = e_trial;
                current = trial;
                if e < e_best {
                    e_best = e;
                    best = current.clone();
                    if e_best == 0 {
                        // With our discretized result, there is no
                        // improvement to be made!
                        println!(" All done after {}% (best {})", i*100/num_iters, e_best as f64/SCALE as f64);
                        best.crossings(true);
                        return best;
                    }
                }
            }
            // rather than a flat histogram, attempt a 1/e histogram.
            *logw.entry(e).or_insert(0) += e;
            if i % (num_iters/20) == 1 {
                println!(" {:2}% done (current {}, best {})", i*100/num_iters,
                         e as f64/SCALE as f64, e_best as f64/SCALE as f64);
                current.crossings(true);
                best.crossings(true);
            }
        }
        println!(" 100% done (best {})", e_best as f64/SCALE as f64);
        best.crossings(true);
        best
    }
}

impl<'a> dot::Labeller<'a, ConceptID, (ConceptID, ConceptID)> for Data {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("concept_map").expect("trouble with graph_id?")
    }
    fn node_id(&'a self, n: &ConceptID) -> dot::Id<'a> {
        dot::Id::new(format_as!(HTML, n)).expect("trouble with node_id??")
    }
    fn node_label<'b>(&'b self, n: &ConceptID) -> dot::LabelText<'b> {
        dot::LabelText::HtmlStr(format_as!(HTML, self.get(*n).name).into())
    }
    fn edge_label<'b>(&'b self, _: &(ConceptID, ConceptID)) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr("".into())
    }
}

struct List<T>(Vec<T>);

#[with_template("[%" "%]" "list.html")]
impl<T: DisplayAs<HTML>>  DisplayAs<HTML> for List<T> {}
