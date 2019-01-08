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
            display(HTML, &*data.concept_view(data.concept_by_name(&name))).http_response()
        });
    let course = path!("course" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &data.course_view(&name)).http_response()
        });
    let representation = path!("representation" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &data.representation_view(data.representation_by_name(&name))).http_response()
        });
    let activity = path!("activity" / String)
        .map(|name: String| {
            let data = Data::new();
            display(HTML, &*data.activity_view(data.activity_by_name(&name))).http_response()
        });
    let index = (warp::path::end().or(path!("index.html")))
        .map(|_| {
            display(HTML, &Data::new().progression_view()).http_response()
        });
    let style_css = path!("style.css").and(warp::fs::file("style.css"));
    let libraries = path!("libraries").and(warp::fs::dir("libraries"));
    let figs = path!("figs").and(warp::fs::dir("figs"));

    warp::serve(index
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
