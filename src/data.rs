use crate::atomicfile::AtomicFile;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use rcu_clean::RcRcu;
use std::cell::RefCell;
use display_as::{with_template, format_as, HTML, URL, DisplayAs};
use simple_error::bail;
use crate::markdown::Markdown;

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
    pub name: String,
}
#[with_template("/course/" slug::slugify(&self.number))]
impl DisplayAs<URL> for Course {}
#[with_template(r#"<a href=""# self as URL r#"" class="course">"# self.name r#"</a>"#)]
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
    concept_views: RefCell<Vec<Option<RcRcu<ConceptView>>>>,
    #[serde(skip)]
    activity_views: RefCell<Vec<Option<RcRcu<ActivityView>>>>,
}

enum AnyID {
    Concept(ConceptID),
    Activity(ActivityID),
    Representation(RepresentationID),
}
impl AnyID {
    fn parse(s: &str) -> Result<Self, Box<std::error::Error>> {
        match s.chars().next() {
            Some('c') => Ok(AnyID::Concept(ConceptID(s[1..].parse()?))),
            Some('a') => Ok(AnyID::Activity(ActivityID(s[1..].parse()?))),
            Some('r') => Ok(AnyID::Representation(RepresentationID(s[1..].parse()?))),
            _ => bail!("Crazy kind: {}", s),
        }
    }
}

trait ID: Copy+Clone {
    type Target;
    fn get(self, data: &Data) -> std::cell::Ref<Self::Target>;
    fn get_mut(self, data: &mut Data) -> std::cell::RefMut<Self::Target>;
}
impl ID for ConceptID {
    type Target = Concept;
    fn get(self, data: &Data) -> std::cell::Ref<Self::Target> {
        std::cell::Ref::map(data.concepts.borrow(), |c| &c[self.0])
    }
    fn get_mut(self, data: &mut Data) -> std::cell::RefMut<Self::Target> {
        std::cell::RefMut::map(data.concepts.borrow_mut(), |c| &mut c[self.0])
    }
}
impl ID for ActivityID {
    type Target = Activity;
    fn get(self, data: &Data) -> std::cell::Ref<Self::Target> {
        std::cell::Ref::map(data.activities.borrow(), |a| &a[self.0])
    }
    fn get_mut(self, data: &mut Data) -> std::cell::RefMut<Self::Target> {
        std::cell::RefMut::map(data.activities.borrow_mut(), |c| &mut c[self.0])
    }
}
impl ID for RepresentationID {
    type Target = Representation;
    fn get(self, data: &Data) -> std::cell::Ref<Self::Target> {
        std::cell::Ref::map(data.representations.borrow(), |r| &r[self.0])
    }
    fn get_mut(self, data: &mut Data) -> std::cell::RefMut<Self::Target> {
        std::cell::RefMut::map(data.representations.borrow_mut(), |c| &mut c[self.0])
    }
}
impl ID for CourseID {
    type Target = Course;
    fn get(self, data: &Data) -> std::cell::Ref<Self::Target> {
        std::cell::Ref::map(data.courses.borrow(), |c| &c[self.0])
    }
    fn get_mut(self, data: &mut Data) -> std::cell::RefMut<Self::Target> {
        std::cell::RefMut::map(data.courses.borrow_mut(), |c| &mut c[self.0])
    }
}

impl Data {
    fn get<I: ID>(&self, id: I) -> std::cell::Ref<I::Target> {
        id.get(self)
    }
    fn get_mut<I: ID>(&mut self, id: I) -> std::cell::RefMut<I::Target> {
        id.get_mut(self)
    }
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
        match AnyID::parse(&c.id)? {
            AnyID::Concept(id) => {
                match &c.field as &str {
                    "long_description" => {
                        self.get_mut(id).long_description = c.content.trim().to_string();
                    }
                    "name" => {
                        self.get_mut(id).name = c.content.trim().to_string();
                    }
                    "needed for" => {
                        let needed_for_id = self.concept_by_name(&c.content);
                        self.get_mut(needed_for_id).prereq_concepts.push(id);
                    }
                    "prereq" => {
                        let prereq_id = self.concept_by_name(&c.content);
                        self.get_mut(id).prereq_concepts.push(prereq_id);
                    }
                    "Add" => {
                        match c.html.as_ref() {
                            "needed for" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(needed_for_id) => {
                                        self.get_mut(needed_for_id).prereq_concepts.push(id)
                                    }
                                    _ => bail!("Cannot yet handle needed for with other types"),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(prereq_id) => {
                                        self.get_mut(id).prereq_concepts.push(prereq_id)
                                    }
                                    _ => bail!("prereq must be a concept"),
                                }
                            }
                            _ => bail!("Unknown relationship: {}", c.html),
                        }
                    }
                    "Remove" => {
                        match c.html.as_ref() {
                            "needed for" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(needed_for_id) => {
                                        self.get_mut(needed_for_id).prereq_concepts.retain(|&x| x != id);
                                    }
                                    _ => bail!("Cannot yet remove needed for with other types"),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(prereq_id) => {
                                        self.get_mut(id).prereq_concepts.retain(|&x| x != prereq_id);
                                    }
                                    _ => bail!("prereq must be a concept in remove"),
                                }
                            }
                            "taught by" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Activity(a_id) => {
                                        self.get_mut(a_id).new_concepts.retain(|&x| x != id);
                                    }
                                    _ => bail!("taughtby must be an activity in remove"),
                                }
                            }
                            _ => bail!("Unknown relationship on remove: {}", c.html),
                        }
                    }
                    _ => bail!("Unknown field of concept: {}", c.field),
                }
            }
            AnyID::Activity(id) => {
                match &c.field as &str {
                    "long_description" => {
                        self.get_mut(id).long_description = c.content.trim().to_string();
                    }
                    "name" => {
                        self.get_mut(id).name = c.content.trim().to_string();
                    }
                    "Remove" => {
                        match c.html.as_ref() {
                            "new_concept" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(c_id) => {
                                        self.get_mut(id).new_concepts.retain(|&x| x != c_id);
                                    }
                                    _ => bail!("No new_concept as other type"),
                                }
                            }
                            "prereq" => {
                                match AnyID::parse(&c.content)? {
                                    AnyID::Concept(c_id) => {
                                        self.get_mut(id).prereq_concepts.retain(|&x| x != c_id);
                                    }
                                    _ => bail!("No new_concept as other type"),
                                }
                            }
                            _ => bail!("Unknown relationship on remove from activity: {}", c.html),
                        }
                    }
                    _ => bail!("Unknown field of activity: {}", c.field),
                }
            }
            AnyID::Representation(id) => {
                match &c.field as &str {
                    "icon" => {
                        self.get_mut(id).icon = c.html.trim().to_string();
                    }
                    "name" => {
                        self.get_mut(id).name = c.content.trim().to_string();
                    }
                    "description" => {
                        self.get_mut(id).description = Markdown::from_html(&c.html);
                    }
                    _ => bail!("Unknown field of representation: {}", c.field),
                }
            }
        }
        self.save();
        Ok(())
    }
    pub fn concept_by_name(&self, name: &str) -> ConceptID {
        let name = name.trim();
        if let Some(c) = self.concepts.borrow().iter()
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
            addremove: ChangeRelationship::none(),
        });
        newid
    }
    pub fn activity_by_name(&self, name: &str) -> ActivityID {
        if let Some(c) = self
            .activities.borrow().iter()
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
            addremove: ChangeRelationship::none(),
        });
        newid
    }
    pub fn representation_by_name(&self, name: &str) -> RepresentationID {
        if let Some(c) = self
            .representations.borrow().iter()
            .filter(|c| &c.name == name || &slug::slugify(&c.name) == name)
            .next()
        {
            return c.id;
        }
        let newid = RepresentationID(self.representations.borrow().len());
        self.representations.borrow_mut().push(Representation {
            id: newid,
            name: name.to_string(),
            description: Default::default(),
            icon: name.to_string(),
        });
        newid
    }
    pub fn course_by_name(&self, name: &str) -> CourseID {
        let name = name.trim();
        if let Some(c) = self
            .courses.borrow().iter()
            .filter(|c| &c.number == name || &c.name == name || &slug::slugify(&c.number) == name)
            .next()
        {
            return c.id;
        }
        let newid = CourseID(self.courses.borrow().len());
        let (number, name) = match name {
            "MTH 251" => (name, "Differential Calculus"),
            "MTH 254" => (name, "Multivariable Calculus"),
            "MTH 255" => (name, "Vector Calculus"),
            "PH 423" => (name, "Energy and Entropy"),
            "PH 422" => (name, "Static Fields"),
            _ => (name, name),
        };
        self.courses.borrow_mut().push(Course {
            id: newid,
            number: number.to_string(),
            name: name.to_string(),
        });
        newid
    }
    pub fn set_concept(&mut self, id: ConceptID, c: Concept) {
        *self.get_mut(id) = c;
    }
    pub fn set_activity(&mut self, id: ActivityID, c: Activity) {
        *self.get_mut(id) = c;
    }

    pub fn concept_view(&self, id: ConceptID) -> RcRcu<ConceptView> {
        while id.0 >= self.concept_views.borrow().len() {
            self.concept_views.borrow_mut().push(None);
        }
        if let Some(ref c) = self.concept_views.borrow()[id.0] {
            return c.clone();
        }
        let c = &self.get(id);
        let my_prereq_concepts: Vec<_> =
            self.concepts.borrow().iter().filter(|x| c.prereq_concepts.contains(&x.id)).cloned().collect();
        let view = RcRcu::new(ConceptView {
            id,
            name: c.name.clone(),

            all_concepts: self.concepts.borrow().clone(),

            activities: Vec::new(),
            prereq_courses: Vec::new(),

            prereq_concepts: Vec::new(),
            prereq_groups: Vec::new(),
            needed_for_concepts: Vec::new(),

            output_groups: Vec::new(),

            representations: c.representations.iter().map(|&rid| self.get(rid).clone()).collect(),
            courses: c.courses.iter().map(|&cid| self.get(cid).clone()).collect(),
            figure: c.figure.clone(),
            long_description: c.long_description.clone(),
            external_url: c.external_url.clone(),
            status: c.status.clone(),
            notes: c.notes.clone(),
        });
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
        let mut output_groups = self.group_concepts(output_concepts, id, "needed for");
        for a in self.activities.borrow().iter().filter(|a| a.prereq_concepts.contains(&id))
        {
            self.extend_groups_with_activity(&mut output_groups, a.clone(), id, "needed for");
        }
        let activities: Vec<_> =
            self.activities.borrow().iter()
            .filter(|a| a.new_concepts.contains(&id))
            .map(|a| self.activity_view(a.id)) // RcRcu::new( .remove(id, "taught by"))
            .collect();
        let prereq_concepts: Vec<_> =
            c.prereq_concepts.iter()
            .map(|x| self.concept_view(*x))
            .filter(|x| !the_prereq_courses.iter().any(|z| x.courses.contains(&z)))
            .collect();
        let prereq_groups = self.group_concepts(prereq_concepts.clone(), id, "prereq");
        let needed_for_concepts: Vec<_> =
            self.concepts.borrow().iter()
            .filter(|x| x.prereq_concepts.contains(&id))
            .map(|x| self.concept_view(x.id))
            .collect();

        {
            let mut v = view.update();
            v.prereq_courses = prereq_courses;
            v.activities = activities;
            v.prereq_concepts = prereq_concepts;
            v.needed_for_concepts = needed_for_concepts;
            v.prereq_groups = prereq_groups;
            v.output_groups = output_groups;
        }
        view
    }

    pub fn activity_view(&self, id: ActivityID) -> RcRcu<ActivityView> {
        while id.0 >= self.activity_views.borrow().len() {
            self.activity_views.borrow_mut().push(None);
        }
        if let Some(ref a) = self.activity_views.borrow()[id.0] {
            return a.clone();
        }
        let a = &self.get(id);
        let my_prereq_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|x| a.prereq_concepts.contains(&x.id)).cloned().collect();
        let view = RcRcu::new(ActivityView {
            id,
            name: a.name.clone(),

            prereq_courses: Vec::new(),

            prereq_concepts: Vec::new(),
            prereq_groups: Vec::new(),
            new_concepts: Vec::new(),

            output_groups: Vec::new(),

            representations: a.representations.iter().map(|&rid| self.get(rid).clone()).collect(),
            courses: a.courses.iter().map(|&cid| self.get(cid).clone()).collect(),
            figure: a.figure.clone(),
            long_description: a.long_description.clone(),
            external_url: a.external_url.clone(),
            status: a.status.clone(),
            notes: a.notes.clone(),
            addremove: ChangeRelationship::none(),
        });
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
        let mut output_groups = self.group_concepts(output_concepts, id, "new concept");
        for a in self.activities.borrow().iter()
            .filter(|aa| a.new_concepts.iter().any(|cc| aa.prereq_concepts.contains(&cc)))
        {
            self.extend_groups_with_activity(&mut output_groups, a.clone(), id, "new concept");
        }

        let new_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|x| a.new_concepts.contains(&x.id))
            .map(|x| self.concept_view(x.id))
            .collect();
        let prereq_concepts: Vec<_> = a.prereq_concepts.iter()
            .map(|x| self.concept_view(*x))
            .collect();
        let prereq_concepts_in_this_course: Vec<_> =
            prereq_concepts.iter()
            .filter(|c| !c.courses.iter().any(|cc| other_courses.contains(cc)))
            .cloned()
            .collect();
        let prereq_groups: Vec<_> = self.group_concepts(prereq_concepts_in_this_course,
                                                        id, "prereq");
        {
            let mut v = view.update();

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
        let r = self.get(id).clone();
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
        let mut groups: Vec<_> = self.group_concepts(activity_concepts.iter().map(|c| self.concept_view(c.id)).collect(),
                                                     id, "used by");
        for a in self.activities.borrow().iter()
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
        let course = self.get(id).clone();
        let course_concepts: Vec<_> = self.concepts.borrow().iter()
            .filter(|c| c.courses.contains(&id))
            .map(|c| self.concept_view(c.id))
            .collect();
        let groups: Vec<ProgressionGroup> =
            self.progression_group_concepts(course_concepts);
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

    fn group_concepts(&self, x: Vec<RcRcu<ConceptView>>,
                      parentid: impl Copy+DisplayAs<HTML>,
                      relationship: &'static str) -> Vec<ActivityGroup> {
        self.progression_group_concepts(x).into_iter()
            .map(|g| {
                let concepts: Vec<_> = g.concepts.iter()
                    .map(|c| Child::remove(parentid, relationship, self.get(c.id).clone())).collect();
                if let Some(a) = g.activity {
                    let hint_concepts: Vec<_> = a.new_concepts.iter()
                        .map(|c| Child::add(parentid, relationship, self.get(c.id).clone()))
                        .filter(|c| !concepts.iter().any(|x| x.id == c.id))
                        .collect();
                    let activity = Some(self.get(a.id).clone());
                    ActivityGroup { activity, concepts, hint_concepts, }
                } else {
                    ActivityGroup {
                        activity: None,
                        concepts,
                        hint_concepts: Vec::new(),
                    }
                }
            })
            .collect()
    }
    fn extend_groups_with_activity(&self, gs: &mut Vec<ActivityGroup>, a: Activity,
                                   parentid: impl Copy+DisplayAs<HTML>,
                                   relationship: &'static str) {
        if gs.iter().any(|g| g.activity == Some(a.clone())) {
            return; // it is already here
        }
        gs.push(ActivityGroup {
            hint_concepts: self.get(a.id).new_concepts.iter()
                .map(|c| Child::add(parentid, relationship, self.get(*c).clone()))
                .collect(),
            activity: Some(a), // .remove(parentid, relationship)),
            concepts: Vec::new(),
        });
    }

    fn progression_group_concepts(&self, x: Vec<RcRcu<ConceptView>>) -> Vec<ProgressionGroup> {
        let mut out: Vec<ProgressionGroup> = Vec::new();
        for c in x.into_iter() {
            let mut act: Vec<_> = c.activities.iter().map(|x| Some(x.clone())).collect();
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
                    out.push(ProgressionGroup { activity: act[0].clone(), concepts: vec![c] });
                } else {
                    out.push(ProgressionGroup { activity: None, concepts: vec![c] });
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
}

/// This is a course and concepts it teaches.
#[derive(Debug, Clone)]
pub struct PrereqCourse {
    pub course: Course,
    pub concepts: Vec<RcRcu<ConceptView>>,
}
#[with_template("[%" "%]" "prereq-course.html")]
impl DisplayAs<HTML> for PrereqCourse {}

/// This is an activity and concepts it teaches.
#[derive(Debug, Clone)]
pub struct ActivityGroup {
    pub activity: Option<Activity>,
    pub concepts: Vec<Child<Concept>>,
    pub hint_concepts: Vec<Child<Concept>>
}
#[with_template("[%" "%]" "activity-group.html")]
impl DisplayAs<HTML> for ActivityGroup {}

/// This is an activity and concepts it teaches, but displayed in a progression.
#[derive(Debug, Clone)]
pub struct ProgressionGroup {
    pub activity: Option<RcRcu<ActivityView>>,
    pub concepts: Vec<RcRcu<ConceptView>>,
}
#[with_template("[%" "%]" "progression-group.html")]
impl DisplayAs<HTML> for ProgressionGroup {}

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

/// Represents a choice between concepts!
pub struct ConceptChoice {
    pub id: String,
    pub field: String,
    pub choices: Vec<Concept>,
}
#[with_template("[%" "%]" "concept-choice.html")]
impl DisplayAs<HTML> for ConceptChoice {}

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
        }
    }
}

impl<T> std::ops::Deref for Child<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.child
    }
}

#[with_template("/concept/" slug::slugify(&self.name))]
impl DisplayAs<URL> for Child<Concept> {}
#[with_template("[%" "%]" "concept.html")]
impl DisplayAs<HTML> for Child<Concept> {}
