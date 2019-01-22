use warp::{Filter, path};
use progression_website::data::{ Data, Change };
use display_as::{HTML, display};

fn main() {
    let change = path!("change")
        .and(warp::filters::body::form())
        .map(|change: Change| {
            if let Err(e) = Data::new().change(change.clone()) {
                println!("Error {} while changing {:?}", e, change);
            }
            "okay"
        });
    let concept = path!("concept" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &*data.concept_view(data.concept_by_name(&name)
                                              .expect("Nonexisting concept")))
                .http_response()
        });
    let course = path!("course" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &data.course_view(&name)).http_response()
        });
    let dot = path!("concept-map.dot")
        .map(|| {
            let data = Data::new();
            let mut out: Vec<u8> = Vec::new();
            dot::render(&data, &mut out).expect("Trouble rendering dot!");
            String::from_utf8(out).expect("trouble converting utf8?")
        });
    let dotsvg = path!("concept-map.svg")
        .map(|| {
            let data = Data::new();
            let mut child = std::process::Command::new("dot")
                .args(&["-Tsvg"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("failed to spawn dot");
            {
                use std::io::Write;
                let stdin = child.stdin.as_mut().expect("Failed to open stdin");
                stdin.write(b"digraph concept_map { ratio=compress;size=8,1000\n").expect("trouble writing to pipe");
                let mut out: Vec<u8> = Vec::new();
                dot::render(&data, &mut out).expect("Trouble rendering dot!");
                stdin.write(&out[22..]).expect("trouble foo");
                println!("file: {}", String::from_utf8(Vec::from(&out[22..])).unwrap());
                //dot::render(&data, &mut stdin).expect("Trouble rendering dot!");
            }
            let output = child.wait_with_output().expect("Failed to read stdout");
            String::from_utf8(output.stdout).expect("Trouble utf8")
        });
    let representation = path!("representation" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &data.representation_view(data.representation_by_name(&name)
                                                    .expect("Nonexisting representation")))
                .http_response()
        });
    let activity = path!("activity" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &*data.activity_view(data.activity_by_name(&name)
                                               .expect("Nonexisting activity")))
                .http_response()
        });
    let index = (warp::path::end().or(path!("index.html")))
        .map(|_| {
            display(HTML, &Data::new().progression_view()).http_response()
        });
    let style_css = path!("style.css").and(warp::fs::file("style.css"));
    let libraries = path!("libraries").and(warp::fs::dir("libraries"));
    let figs = path!("figs").and(warp::fs::dir("figs"));

    warp::serve(index
                .or(dot)
                .or(dotsvg)
                .or(change)
                .or(concept)
                .or(activity)
                .or(course)
                .or(representation)
                .or(style_css)
                .or(libraries)
                .or(figs)
                .with(warp::reply::with::default_header("Cache-Control", "no-store")))
        .run(([0, 0, 0, 0], 3030));
}
