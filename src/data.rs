use crate::atomicfile::AtomicFile;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::rc::Rc;
use std::cell::RefCell;
use display_as::{with_template, HTML, URL, DisplayAs};
use simple_error::bail;

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub field: String,
    pub content: String,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConceptID(usize);
#[with_template("c" self.0)]
impl DisplayAs<HTML> for ConceptID {}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActivityID(usize);
#[with_template("a" self.0)]
impl DisplayAs<HTML> for ActivityID {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RepresentationID(usize);
#[with_template("FIXME-RepresentationID:" self.0)]
impl DisplayAs<URL> for RepresentationID {}
#[with_template("r" self.0)]
impl DisplayAs<HTML> for RepresentationID {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CourseID(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Course {
    pub id: CourseID,
    pub number: String,
}
#[with_template("/course/" slug::slugify(&self.number))]
impl DisplayAs<URL> for Course {}
#[with_template(r#"<a href=""# self as URL r#"" class="course">"# self.number r#"</a>"#)]
impl DisplayAs<HTML> for Course {}

pub use crate::concept::{Concept, ConceptView};
pub use crate::activity::{Activity, ActivityView};
pub use crate::representation::{Representation, RepresentationView};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    concepts: RefCell<Vec<Concept>>,
    activities: RefCell<Vec<Activity>>,
    representations: RefCell<Vec<Representation>>,
    courses: RefCell<Vec<Course>>,
    // activities: Vec<Activity>,
    #[serde(skip)]
    concept_views: RefCell<Vec<Option<Rc<RefCell<ConceptView>>>>>,
    #[serde(skip)]
    activity_views: RefCell<Vec<Option<Rc<RefCell<ActivityView>>>>>,
}

impl Data {
    pub fn save(&self) {
        let f = AtomicFile::create("progression.yaml").expect("error creating save file");
        serde_yaml::to_writer(&f, self).expect("error writing yaml");
    }
    pub fn new() -> Self {
        if let Ok(f) = std::fs::File::open("progression.yaml") {
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
            concept_views: RefCell::new(Vec::new()),
            activity_views: RefCell::new(Vec::new()),
        }
    }
    pub fn change(&mut self, c: Change) -> Result<(), Box<std::error::Error>> {
        match c.id.chars().next() {
            Some('c') => {
                let id: usize = c.id[1..].parse()?;
                match &c.field as &str {
                    "long_description" => {
                        self.concepts.borrow_mut()[id].long_description = c.content.trim().to_string();
                    }
                    "name" => {
                        self.concepts.borrow_mut()[id].name = c.content.trim().to_string();
                    }
                    _ => bail!("Unknown field of concept: {}", c.field),
                }
            }
            Some('a') => {
                let id: usize = c.id[1..].parse()?;
                match &c.field as &str {
                    "long_description" => {
                        self.activities.borrow_mut()[id].long_description = c.content.trim().to_string();
                    }
                    "name" => {
                        self.activities.borrow_mut()[id].name = c.content.trim().to_string();
                    }
                    _ => bail!("Unknown field of activity: {}", c.field),
                }
            }
            Some('r') => {
                let id: usize = c.id[1..].parse()?;
                match &c.field as &str {
                    "icon" => {
                        self.representations.borrow_mut()[id].icon = c.content.trim().to_string();
                    }
                    "name" => {
                        self.representations.borrow_mut()[id].name = c.content.trim().to_string();
                    }
                    _ => bail!("Unknown field of representation: {}", c.field),
                }
            }
            _ => bail!("Crazy kind: {}", c.id),
        }
        self.save();
        Ok(())
    }
    pub fn concept_by_name(&self, name: &str) -> ConceptID {
        let name = name.trim();
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
            icon: name.to_string(),
        });
        newid
    }
    pub fn course_by_name(&self, name: &str) -> CourseID {
        let name = name.trim();
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

    pub fn concept_view(&self, id: ConceptID) -> Rc<RefCell<ConceptView>> {
        while id.0 >= self.concept_views.borrow().len() {
            self.concept_views.borrow_mut().push(None);
        }
        if let Some(ref c) = self.concept_views.borrow()[id.0] {
            return c.clone();
        }
        let c = &self.concepts.borrow()[id.0];
        let my_prereq_concepts: Vec<_> =
            self.concepts.borrow().iter().filter(|x| c.prereq_concepts.contains(&x.id)).cloned().collect();
        let view = Rc::new(RefCell::new(ConceptView {
            id,
            name: c.name.clone(),

            activities: Vec::new(),
            prereq_courses: Vec::new(),

            prereq_concepts: Vec::new(),
            prereq_groups: Vec::new(),
            needed_for_concepts: Vec::new(),

            output_groups: Vec::new(),

            representations: c.representations.iter().map(|rid| self.representations.borrow()[rid.0].clone()).collect(),
            courses: c.courses.iter().map(|cid| self.courses.borrow()[cid.0].clone()).collect(),
            figure: c.figure.clone(),
            long_description: c.long_description.clone(),
            external_url: c.external_url.clone(),
            status: c.status.clone(),
            notes: c.notes.clone(),
        }));
        self.concept_views.borrow_mut()[id.0] = Some(view.clone());
        // We haven't generated this view yet, so we need to add the
        // related concepts.
        let prereq_courses: Vec<_> = self.courses.borrow().iter().cloned()
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: my_prereq_concepts.iter()
                    .filter(|c| c.courses.contains(&course.id))
                    .map(|c| self.concept_view(c.id))
                    .collect(),
            })
            .filter(|xx| xx.concepts.len() > 0 && !c.courses.contains(&xx.course.id))
            .collect();
        let the_prereq_courses: Vec<_> = prereq_courses.iter().map(|x| x.course.clone()).collect();

        let output_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|x| x.prereq_concepts.contains(&id))
            .map(|x| self.concept_view(x.id))
            .collect();
        let mut output_groups = group_concepts(output_concepts);
        let activities: Vec<_> = output_groups.iter()
            .flat_map(|g| g.activity.iter().cloned())
            .collect();
        output_groups.extend(self.activities.borrow().iter()
                             .filter(|a| a.prereq_concepts.contains(&id))
                             .map(|a| self.activity_view(a.id))
                             .filter(|a| !activities.contains(a))
                             .map(|a| ActivityGroup {
                                 activity: Some(a),
                                 concepts: Vec::new(),
                             }));
        let activities: Vec<_> =
            self.activities.borrow().iter()
            .filter(|a| a.new_concepts.contains(&id))
            .map(|a| self.activity_view(a.id))
            .collect();
        let prereq_concepts: Vec<_> =
            c.prereq_concepts.iter()
            .map(|x| self.concept_view(*x))
            .filter(|x| !the_prereq_courses.iter().any(|z| x.borrow().courses.contains(&z)))
            .collect();
        let prereq_groups = group_concepts(prereq_concepts.clone());
        let needed_for_concepts: Vec<_> =
            self.concepts.borrow().iter()
            .filter(|x| x.prereq_concepts.contains(&id))
            .map(|x| self.concept_view(x.id))
            .collect();

        {
            let mut v = view.borrow_mut();
            v.prereq_courses = prereq_courses;
            v.activities = activities;
            for p in c.prereq_concepts.iter() {
                let pre = self.concept_view(*p);
                v.prereq_concepts.push(pre);
            }
            v.prereq_concepts = prereq_concepts;
            v.needed_for_concepts = needed_for_concepts;
            v.prereq_groups = prereq_groups;
            v.output_groups = output_groups;
        }
        view
    }

    pub fn activity_view(&self, id: ActivityID) -> Rc<RefCell<ActivityView>> {
        while id.0 >= self.activity_views.borrow().len() {
            self.activity_views.borrow_mut().push(None);
        }
        if let Some(ref a) = self.activity_views.borrow()[id.0] {
            return a.clone();
        }
        let a = &self.activities.borrow()[id.0];
        let my_prereq_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|x| a.prereq_concepts.contains(&x.id)).cloned().collect();
        let view = Rc::new(RefCell::new(ActivityView {
            id,
            name: a.name.clone(),

            prereq_courses: Vec::new(),

            prereq_concepts: Vec::new(),
            prereq_groups: Vec::new(),
            new_concepts: Vec::new(),

            output_groups: Vec::new(),

            representations: a.representations.iter().map(|rid| self.representations.borrow()[rid.0].clone()).collect(),
            courses: a.courses.iter().map(|cid| self.courses.borrow()[cid.0].clone()).collect(),
            figure: a.figure.clone(),
            long_description: a.long_description.clone(),
            external_url: a.external_url.clone(),
            status: a.status.clone(),
            notes: a.notes.clone(),
        }));
        // We haven't generated this view yet, so we need to add the
        // related concepts.
        let other_courses: Vec<_> = self.courses.borrow().iter()
            .filter(|xx| !a.courses.contains(&xx.id))
            .cloned()
            .collect();
        let prereq_courses: Vec<_> = self.courses.borrow().iter().cloned()
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: my_prereq_concepts.iter()
                    .filter(|c| c.courses.contains(&course.id))
                    .map(|c| self.concept_view(c.id))
                    .collect(),
            })
            .filter(|xx| xx.concepts.len() > 0 && other_courses.contains(&xx.course))
            .collect();

        let output_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|x| a.new_concepts.contains(&x.id))
            .map(|x| self.concept_view(x.id))
            .collect();
        let mut output_groups = group_concepts(output_concepts);
        output_groups.extend(self.activities.borrow().iter()
                             .filter(|aa| a.new_concepts.iter()
                                     .any(|cc| aa.prereq_concepts.contains(&cc)))
                             .map(|a| ActivityGroup {
                                 activity: Some(self.activity_view(a.id)),
                                 concepts: Vec::new(),
                             }));

        let new_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|x| a.new_concepts.contains(&x.id))
            .map(|x| self.concept_view(x.id))
            .collect();
        let prereq_concepts: Vec<_> = a.prereq_concepts.iter()
            .map(|x| self.concept_view(*x))
            .collect();
        let prereq_concepts_in_this_course: Vec<_> =
            prereq_concepts.iter()
            .filter(|c| !c.borrow().courses.iter().any(|cc| other_courses.contains(cc)))
            .cloned()
            .collect();
        let prereq_groups: Vec<_> = group_concepts(prereq_concepts_in_this_course);
        {
            let mut v = view.borrow_mut();

            v.prereq_courses = prereq_courses;
            for p in a.prereq_concepts.iter() {
                let pre = self.concept_view(*p);
                v.prereq_concepts.push(pre);
            }
            v.prereq_concepts = prereq_concepts;
            v.new_concepts = new_concepts;
            v.prereq_groups = prereq_groups;

            v.output_groups = output_groups;
        }
        view
    }

    pub fn representation_view(&self, id: RepresentationID) -> RepresentationView {
        let r = self.representations.borrow()[id.0].clone();
        let all_concepts_using_r: Vec<_> = self.concepts.borrow().iter()
            .filter(|c| c.representations.contains(&id))
            .cloned()
            .collect();
        let all_activities_using_r: Vec<_> = self.activities.borrow().iter()
            .filter(|c| c.representations.contains(&id))
            .cloned()
            .collect();
        let activity_concepts: Vec<_> = all_concepts_using_r.iter()
            .filter(|c| all_activities_using_r.iter().any(|a| a.new_concepts.contains(&c.id)))
            .cloned()
            .collect();
        let other_concepts: Vec<_> = all_concepts_using_r.into_iter()
            .filter(|c| !activity_concepts.contains(&c))
            .collect();
        let mut groups: Vec<_> = group_concepts(activity_concepts.iter().map(|c| self.concept_view(c.id)).collect());
        groups.extend(self.activities.borrow().iter()
                      .filter(|a| a.representations.contains(&id))
                      .map(|a| self.activity_view(a.id))
                      .filter(|a| !all_activities_using_r.contains(a))
                      .map(|a| ActivityGroup {
                          activity: Some(a),
                          concepts: Vec::new(),
                      }));
        RepresentationView {
            id,
            name: r.name,
            icon: r.icon,
            other_concepts,
            groups,
        }
    }

    pub fn progression_view(&self) -> ProgressionView {
        let courses: Vec<_> = self.courses.borrow().iter().map(|c| self.course_sequence(c.id))
            .filter(|x| x.groups.len() > 1) // FIXME should handle prereqs better?
            .collect();
        let prereq_courses: Vec<_> = self.courses.borrow().iter().take(courses[0].course.id.0).cloned()
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: self.concepts.borrow().iter()
                    .filter(|c| c.courses.contains(&course.id))
                    .map(|c| self.concept_view(c.id))
                    .collect(),
            })
            .collect();
        ProgressionView {
            prereq_courses,
            courses,
        }
    }

    pub fn course_sequence(&self, id: CourseID) -> CourseSequence {
        let course = self.courses.borrow()[id.0].clone();
        let course_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|c| c.courses.contains(&id))
            .map(|c| self.concept_view(c.id))
            .collect();
        let groups: Vec<ProgressionGroup> = group_concepts(course_concepts)
            .into_iter().map(|g| ProgressionGroup(g)).collect();
        CourseSequence { course, prereq_courses: Vec::new(), groups }
    }

    pub fn course_view(&self, name: &str) -> CourseSequence {
        let id = self.course_by_name(name);
        let mut cs = self.course_sequence(id);

        let my_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|c| c.courses.contains(&id))
            .cloned()
            .collect();
        let my_activities: Vec<_> = self.activities.borrow().iter()
            .filter(|a| a.courses.contains(&id))
            .cloned()
            .collect();

        let mut my_prereq_concepts: Vec<ConceptID> = my_concepts.iter()
            .flat_map(|c| c.prereq_concepts.clone())
            .collect();
        my_prereq_concepts.extend(my_activities.iter().flat_map(|a| a.prereq_concepts.clone()));
        let my_prereq_concepts: Vec<Concept> = self.concepts.borrow().iter()
            .filter(|c| my_prereq_concepts.contains(&c.id))
            .cloned()
            .collect();

        cs.prereq_courses = self.courses.borrow().iter()
            .filter(|course| course.id != id)
            .map(|course| PrereqCourse {
                course: course.clone(),
                concepts: my_prereq_concepts.iter()
                    .filter(|c| c.courses.contains(&course.id))
                    .map(|c| self.concept_view(c.id))
                    .collect(),
            })
            .filter(|xx| xx.concepts.len() > 0)
            .collect();
        cs
    }
}

/// This is a course and concepts it teaches.
#[derive(Debug, Clone)]
pub struct PrereqCourse {
    pub course: Course,
    pub concepts: Vec<Rc<RefCell<ConceptView>>>,
}
#[with_template("[%" "%]" "prereq-course.html")]
impl DisplayAs<HTML> for PrereqCourse {}

/// This is an activity and concepts it teaches.
#[derive(Debug, Clone)]
pub struct ActivityGroup {
    pub activity: Option<Rc<RefCell<ActivityView>>>,
    pub concepts: Vec<Rc<RefCell<ConceptView>>>,
}
#[with_template("[%" "%]" "activity-group.html")]
impl DisplayAs<HTML> for ActivityGroup {}

/// This is an activity and concepts it teaches, but displayed in a progression..
#[derive(Debug, Clone)]
pub struct ProgressionGroup(ActivityGroup);
#[with_template("[%" "%]" "progression-group.html")]
impl DisplayAs<HTML> for ProgressionGroup {}

fn group_concepts(x: Vec<Rc<RefCell<ConceptView>>>) -> Vec<ActivityGroup> {
    let mut out: Vec<ActivityGroup> = Vec::new();
    for c in x.into_iter() {
        let mut act: Vec<_> = c.borrow().activities.iter().map(|x| Some(x.clone())).collect();
        if act.len() == 0 {
            act.push(None);
        }
        if let Some(ref mut group) = out.iter_mut()
            .filter(|x| act.contains(&x.activity))
            .next()
        {
            group.concepts.push(c);
        } else {
            if act.len() >= 1 {
                out.push(ActivityGroup { activity: act[0].clone(), concepts: vec![c] });
            } else {
                out.push(ActivityGroup { activity: None, concepts: vec![c] });
            }
        }
    }
    out.sort_unstable_by_key(|g| {
        if let Some(ref a) = g.activity {
            a.borrow().id
        } else {
            ActivityID(100000)
        }
    });
    for g in out.iter_mut() {
        g.concepts.sort_unstable_by_key(|c| c.borrow().id);
    }
    out
}

pub struct CourseSequence {
    course: Course,
    prereq_courses: Vec<PrereqCourse>,
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
