use serde_derive::Deserialize;
use std::error::Error;
use std::io;
use std::process;

use progression_website::data::{Activity, Concept, Data};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum ThingType {
    Activity,
    Concept,
    Neither,
}

#[derive(Deserialize)]
struct Row<'a> {
    thingtype: &'a str,
    name: &'a str,
    _rownum: &'a str,
    prereq_concepts: &'a str,
    new_concepts: &'a str,
    representations: &'a str,
    course_number: &'a str,
    figure: &'a str,
    long_description: &'a str,
    external_url: &'a str,
    status: &'a str,
    notes: &'a str,
}

fn parse_list(s: &str) -> Vec<String> {
    if s.len() == 0 {
        return Vec::new();
    }
    if s.chars().next() != Some('[') {
        return Vec::new();
    }
    let mut x = s[1..].to_string();
    if x.pop() != Some(']') {
        return Vec::new();
    }
    x.split(',')
        .filter(|y| y.len() > 0)
        .map(|y| y.trim().to_string())
        .collect()
}

fn nonempty_string(s: &str) -> Option<String> {
    if s.len() == 0 {
        return None;
    }
    Some(s.to_string())
}

fn read_progression_csv() -> Result<(), Box<Error>> {
    let mut data = Data::new();
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let record = result?;
        let mut datum: Row = record.deserialize(None)?;
        datum.name = datum.name.trim();
        let prereqs: Vec<_> = parse_list(datum.prereq_concepts)
            .iter()
            .map(|c| data.concept_by_name(c))
            .collect();
        let new_concepts: Vec<_> = parse_list(datum.new_concepts)
            .iter()
            .map(|c| data.concept_by_name(c))
            .collect();
        let representations: Vec<_> = parse_list(datum.representations)
            .iter()
            .map(|c| data.representation_by_name(c))
            .collect();
        let courses: Vec<_> = if datum.course_number.len() == 0 {
            Vec::new()
        } else {
            vec![data.course_by_name(datum.course_number)]
        };
        if datum.thingtype == "Concept" || datum.thingtype == "concept" {
            if datum.name.trim().len() > 0 {
                let c = Concept {
                    id: data.concept_by_name(datum.name),
                    name: datum.name.to_string(),
                    prereq_concepts: prereqs,
                    representations,
                    courses,
                    figure: nonempty_string(datum.figure),
                    long_description: datum.long_description.to_string(),
                    external_url: nonempty_string(datum.external_url),
                    status: nonempty_string(datum.status),
                    notes: nonempty_string(datum.notes),
                };
                data.set_concept(c.id, c);
            }
        } else if datum.thingtype == "Activity" || datum.thingtype == "activity" {
            let c = Activity {
                id: data.activity_by_name(datum.name),
                name: datum.name.to_string(),
                prereq_concepts: prereqs,
                new_concepts,
                representations,
                courses,
                figure: nonempty_string(datum.figure),
                long_description: datum.long_description.to_string(),
                external_url: nonempty_string(datum.external_url),
                status: nonempty_string(datum.status),
                notes: nonempty_string(datum.notes),
            };
            data.set_activity(c.id, c);
        } else {
            println!("   {}: {}", datum.thingtype, datum.name);
        }
        data.save();
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_progression_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
