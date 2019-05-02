use warp::{Filter, path};
use progression_website::data::{ set_base_url, Data, Change, CourseID, AnyID,
                                 ConceptID, ActivityID, RepresentationID };
use display_as::{HTML, display};
use clapme::ClapMe;

#[derive(Debug, ClapMe)]
struct Args {
    base_url: String,
}

fn main() {
    let args = Args::from_args();
    set_base_url(&args.base_url);
    let change = path!("change")
        .and(warp::filters::body::form())
        .map(|change: Change| {
            if let Err(e) = Data::new().change(change.clone()) {
                println!("Error {} while changing {:?}", e, change);
            }
            "okay"
        });
    let figure = path!("figure" / AnyID / String)
        .and(warp::body::content_length_limit(1024 * 1024 * 32))
        .and(warp::body::concat())
        .map(|id: AnyID, filename: String, full_body: warp::body::FullBody| {
            println!("got {:?} and {}", id, filename);
            // FullBody is a `Buf`, which could have several non-contiguous
            // slices of memory...
            use bytes::buf::Buf;
            let b: Vec<u8> = full_body.collect();
            std::fs::write(format!("figs/{}", filename), &b).unwrap();
            if let Err(e) = Data::new().uploaded_figure(id, &filename) {
                println!("Error {} while coping with new file {:?}", e, filename);
            }
            "okay"
        });
    let concept = path!("concept" / ConceptID)
        .map(|id: ConceptID| {
            let data = Data::new();
            display(HTML, &data.concept_view(id))
                .http_response()
        });
    let course = path!("course" / CourseID)
        .map(|id: CourseID| {
            let data = Data::new();
            display(HTML, &data.course_view(id)).http_response()
        });
    let map = path!("concept-map" / usize)
        .map(|max_width: usize| {
            let data = Data::new();
            display(HTML, &data.concept_map(max_width)).http_response()
        })
        .or(path!("concept-map")
            .map(|| {
                let data = Data::new();
                display(HTML, &data.concept_map(4)).http_response()
            }));
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
                stdin.write(b"digraph concept_map { ratio=compress; size=\"8,1000\"\n").expect("trouble writing to pipe");
                let mut out: Vec<u8> = Vec::new();
                dot::render(&data, &mut out).expect("Trouble rendering dot!");
                // 22 is the number of bytes in the first line of the
                // generated code.  This is way hokey.
                stdin.write(&out[22..]).expect("trouble foo");
                //println!("file: {}", String::from_utf8(Vec::from(&out[22..])).unwrap());
            }
            let output = child.wait_with_output().expect("Failed to read stdout");
            String::from_utf8(output.stdout).expect("Trouble utf8")
        });
    let representation = path!("representation" / RepresentationID)
        .map(|id: RepresentationID| {
            let data = Data::new();
            display(HTML, &data.representation_view(id)).http_response()
        });
    let representations = path!("representations")
        .map(|| {
            let data = Data::new();
            display(HTML, &data.all_representations()).http_response()
        });
    let activity = path!("activity" / ActivityID)
        .map(|id: ActivityID| {
            let data = Data::new();
            display(HTML, &data.activity_view(id)).http_response()
        });
    let index = (warp::path::end().or(path!("index.html")))
        .map(|_| {
            display(HTML, &Data::new().progression_view()).http_response()
        });
    let style_css = path!("style.css").and(warp::fs::file("style.css"));
    let libraries = path!("libraries").and(warp::fs::dir("libraries"));
    let figs = path!("figs").and(warp::fs::dir("figs"));

    warp::serve(index
                .or(map)
                .or(dot)
                .or(dotsvg)
                .or(change)
                .or(concept)
                .or(activity)
                .or(course)
                .or(representation)
                .or(representations)
                .or(libraries)
                .or(figs)
                .or(figure)
                .with(warp::reply::with::default_header("Cache-Control", "no-store"))
                .or(style_css))
        .run(([0, 0, 0, 0], 3030));
}
