use std::error::Error;
use std::io;
use std::process;
use serde_derive::Deserialize;

use progression_website::types::{Data, Concept, Activity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum ThingType {
    Activity, Concept, Neither
}

#[derive(Deserialize)]
struct Row<'a> {
    thingtype: &'a str,
    name: &'a str,
    rownum: &'a str,
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
    x.split(',').filter(|y| y.len() > 0).map(|y| y.to_string()).collect()
}

fn read_progression_csv() -> Result<(), Box<Error>> {
    let mut data = Data::new();
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let record = result?;
        let datum: Row = record.deserialize(None)?;
        let prereqs: Vec<_> = parse_list(datum.prereq_concepts).iter()
            .map(|c| data.concept_by_name(c)).collect();
        let new_concepts: Vec<_> = parse_list(datum.new_concepts).iter()
            .map(|c| data.concept_by_name(c)).collect();
        let reps: Vec<_> = parse_list(datum.representations).iter()
            .map(|c| data.representation_by_name(c)).collect();
        if datum.thingtype == "Concept" || datum.thingtype == "concept" {
            let c = Concept {
                id: data.concept_by_name(datum.name),
                name: datum.name.to_string(),
                prereq_concepts: prereqs,
                representations: reps,
                courses: Vec::new(),
                figure: None,
                long_description: "".to_string(),
                external_url: None,
                status: None,
                notes: None,
            };
            data.set_concept(c.id, c);
        } else if datum.thingtype == "Activity" || datum.thingtype == "activity" {
            println!("{:?}", record);
            let c = Activity {
                id: data.activity_by_name(datum.name),
                name: datum.name.to_string(),
                prereq_concepts: prereqs,
                new_concepts: new_concepts,
                representations: reps,
                courses: Vec::new(),
                figure: None,
                long_description: "".to_string(),
                external_url: None,
                status: None,
                notes: None,
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
