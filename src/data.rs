use crate::atomicfile::AtomicFile;
use internment::Intern;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::cell::RefCell;
use std::hash::Hash;
use display_as::{with_template, HTML, DisplayAs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConceptID(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActivityID(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RepresentationID(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CourseID(usize);

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
}
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Representation {
    pub id: RepresentationID,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Course {
    pub id: CourseID,
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    concepts: RefCell<Vec<Concept>>,
    activities: RefCell<Vec<Activity>>,
    representations: RefCell<Vec<Representation>>,
    courses: RefCell<Vec<Course>>,
    // activities: Vec<Activity>,
}

impl Data {
    pub fn save(&self) {
        let f = AtomicFile::create("progression.yaml").expect("error creating save file");
        serde_yaml::to_writer(&f, self).expect("error writing yaml")
    }
    pub fn new() -> Self {
        if let Ok(f) = ::std::fs::File::open("progression.yaml") {
            if let Ok(s) = serde_yaml::from_reader::<_,Self>(&f) {
                return s;
            }
        }
        Data {
            concepts: RefCell::new(Vec::new()),
            activities: RefCell::new(Vec::new()),
            representations: RefCell::new(Vec::new()),
            courses: RefCell::new(Vec::new()),
            // activities: Vec::new(),
        }
    }
    pub fn concept_by_name(&self, name: &str) -> ConceptID {
        if let Some(c) = self
            .concepts
            .borrow()
            .iter()
            .filter(|c| &c.name == name || &slug::slugify(&c.name) == name)
            .next()
        {
            return c.id;
        }
        let newid = ConceptID(self.concepts.borrow().len());
        self.concepts.borrow_mut().push(Concept {
            id: newid,
            name: name.to_string(),
            prereq_concepts: Vec::new(),
            representations: Vec::new(),
            courses: Vec::new(),
            figure: None,
            long_description: "".to_string(),
            external_url: None,
            status: None,
            notes: None,
        });
        newid
    }
    pub fn activity_by_name(&self, name: &str) -> ActivityID {
        if let Some(c) = self
            .activities
            .borrow()
            .iter()
            .filter(|c| &c.name == name || &slug::slugify(&c.name) == name)
            .next()
        {
            return c.id;
        }
        let newid = ActivityID(self.activities.borrow().len());
        self.activities.borrow_mut().push(Activity {
            id: newid,
            name: name.to_string(),
            prereq_concepts: Vec::new(),
            new_concepts: Vec::new(),
            representations: Vec::new(),
            courses: Vec::new(),
            figure: None,
            long_description: "".to_string(),
            external_url: None,
            status: None,
            notes: None,
        });
        newid
    }
    pub fn representation_by_name(&self, name: &str) -> RepresentationID {
        if let Some(c) = self
            .representations
            .borrow()
            .iter()
            .filter(|c| &c.name == name || &slug::slugify(&c.name) == name)
            .next()
        {
            return c.id;
        }
        let newid = RepresentationID(self.representations.borrow().len());
        self.representations.borrow_mut().push(Representation {
            id: newid,
            name: name.to_string(),
        });
        newid
    }
    pub fn course_by_name(&self, name: &str) -> CourseID {
        if let Some(c) = self
            .courses
            .borrow()
            .iter()
            .filter(|c| &c.number == name || &slug::slugify(&c.number) == name)
            .next()
        {
            return c.id;
        }
        let newid = CourseID(self.courses.borrow().len());
        self.courses.borrow_mut().push(Course {
            id: newid,
            number: name.to_string(),
        });
        newid
    }
    pub fn set_concept(&mut self, id: ConceptID, c: Concept) {
        self.concepts.borrow_mut()[id.0] = c;
    }
    pub fn set_activity(&mut self, id: ActivityID, c: Activity) {
        self.activities.borrow_mut()[id.0] = c;
    }

    pub fn concept_view(&self, id: ConceptID) -> Intern<ConceptView> {
        let c = &self.concepts.borrow()[id.0];
        let view = Intern::new(ConceptView {
            id,
            name: c.name.clone(),

            prereq_concepts: RefCell::new(Vec::new()),
            needed_for_concepts: RefCell::new(Vec::new()),

            representations: c.representations.clone(),
            courses: c.courses.clone(),
            figure: c.figure.clone(),
            long_description: c.long_description.clone(),
            external_url: c.external_url.clone(),
            status: c.status.clone(),
            notes: c.notes.clone(),

            am_initialized: RefCell::new(false),
        });
        if !*view.am_initialized.borrow() {
            // We haven't generated this view yet, so we need to add
            // the related concepts.  am_initialized allows me to
            // avoid any infinite loops where we keep generating the
            // same views.
            *view.am_initialized.borrow_mut() = true;
            for p in c.prereq_concepts.iter() {
                let pre = self.concept_view(*p);
                view.prereq_concepts.borrow_mut().push(pre);
            }
            *view.prereq_concepts.borrow_mut() =
                c.prereq_concepts.iter()
                .map(|x| self.concept_view(*x))
                .collect();
            *view.needed_for_concepts.borrow_mut() =
                self.concepts.borrow().iter()
                .filter(|x| x.prereq_concepts.contains(&id))
                .map(|x| self.concept_view(x.id))
                .collect();
        }
        view
    }
}

/// This is a concept, but with all the relationships filled in.
#[derive(Debug, Clone, Serialize)]
pub struct ConceptView {
    pub id: ConceptID,
    pub name: String,

    pub prereq_concepts: RefCell<Vec<Intern<ConceptView>>>,
    pub needed_for_concepts: RefCell<Vec<Intern<ConceptView>>>,

    pub representations: Vec<RepresentationID>,
    pub courses: Vec<CourseID>,
    pub figure: Option<String>,
    pub long_description: String,
    pub external_url: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,

    pub am_initialized: RefCell<bool>,
}
impl Hash for ConceptView {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.0.hash(state);
    }
}
#[with_template("concept.html")]
impl DisplayAs<HTML> for ConceptView {}

impl PartialEq for ConceptView {
    fn eq(&self, other: &ConceptView) -> bool {
        self.id == other.id
    }
}
impl Eq for ConceptView {}
