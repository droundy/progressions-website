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
        let activities: Vec<_> = self.activities.borrow().iter()
            .filter(|a| a.new_concepts.contains(&id)).cloned().collect();
        let my_prereq_concepts: Vec<_> =
            self.concepts.borrow().iter().filter(|x| c.prereq_concepts.contains(&x.id)).cloned().collect();
        let prereq_courses: Vec<_> = self.courses.borrow().iter().cloned()
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: my_prereq_concepts.iter().filter(|c| c.courses.contains(&course.id)).cloned().collect(),
            })
            .filter(|xx| xx.concepts.len() > 0 && !c.courses.contains(&xx.course.id))
            .collect();
        let the_prereq_courses: Vec<_> = prereq_courses.iter().map(|x| x.course.clone()).collect();
        let view = Intern::new(ConceptView {
            id,
            name: c.name.clone(),

            activities,
            prereq_courses,

            prereq_concepts: RefCell::new(Vec::new()),
            prereq_groups: RefCell::new(Vec::new()),
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
                .filter(|x| !the_prereq_courses.iter().any(|z| x.courses.contains(&z.id)))
                .collect();
            *view.needed_for_concepts.borrow_mut() =
                self.concepts.borrow().iter()
                .filter(|x| x.prereq_concepts.contains(&id))
                .map(|x| self.concept_view(x.id))
                .collect();
            *view.prereq_groups.borrow_mut() =
                group_concepts(view.prereq_concepts.borrow().clone());
        }
        view
    }
}

/// This is a course and concepts it teaches.
#[derive(Debug, Clone, Serialize)]
pub struct PrereqCourse {
    pub course: Course,
    pub concepts: Vec<Concept>,
}
#[with_template("prereq-course.html")]
impl DisplayAs<HTML> for PrereqCourse {}

/// This is a concept, but with all the relationships filled in.
#[derive(Debug, Clone, Serialize)]
pub struct ConceptView {
    pub id: ConceptID,
    pub name: String,

    pub activities: Vec<Activity>,

    pub prereq_courses: Vec<PrereqCourse>,
    pub prereq_concepts: RefCell<Vec<Intern<ConceptView>>>,
    pub prereq_groups: RefCell<Vec<ActivityGroup>>,
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

/// This is an activity and concepts it teaches.
#[derive(Debug, Clone, Serialize)]
pub struct ActivityGroup {
    pub activity: Activity,
    pub concepts: Vec<Intern<ConceptView>>,
}
#[with_template("activity-group.html")]
impl DisplayAs<HTML> for ActivityGroup {}
fn group_concepts(x: Vec<Intern<ConceptView>>) -> Vec<ActivityGroup> {
    let mut out: Vec<ActivityGroup> = Vec::new();
    for c in x.into_iter() {
        let act: Vec<_> = c.activities.iter().map(|x| x.id).collect();
        if let Some(ref mut group) = out.iter_mut()
            .filter(|x: &&mut ActivityGroup| act.contains(&x.activity.id))
            .next()
        {
            group.concepts.push(c);
        } else {
            if act.len() >= 1 {
                out.push(ActivityGroup { activity: c.activities[0].clone(), concepts: vec![c] });
            } else {
                println!("There is an orphan concept! What should I do?");
            }
        }
    }
    out
}
