use warp::{Filter, path};
use progression_website::data::{ Data, ConceptEdit, Change };
use display_as::{DisplayAs, HTML};

fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let edit_concept = path!("concept" / "edit" / String)
        .map(|name: String| {
            let data = Data::new();
            ConceptEdit::new((*data.concept_view(data.concept_by_name(&name))).clone())
                .display_as(HTML).into_reply()
        });
    let change = path!("change")
        .and(warp::filters::body::form())
        .map(|change: Change| {
            println!("change: {:?}", change);
            if let Err(e) = Data::new().change(change.clone()) {
                println!("Error {} while changing {:?}", e, change);
            }
            "okay"
        });
    let concept = path!("concept" / String)
        .map(|name: String| {
            let data = Data::new();
            println!("My concept is {:?}", data.concept_by_name(&name));
            println!("My concept view is {:?}",
                     data.concept_view(data.concept_by_name(&name)).long_description);
            data.concept_view(data.concept_by_name(&name))
                .display_as(HTML).into_reply()
        });
    let activity = path!("activity" / String)
        .map(|name: String| {
            let data = Data::new();
            data.activity_view(data.activity_by_name(&name))
                .display_as(HTML).into_reply()
        });
    let index = (warp::path::end().or(path!("index.html")))
        .map(|_| {
            Data::new().progression_view().display_as(HTML).into_reply()
        });
    let style_css = path!("style.css").and(warp::fs::file("style.css"));
    let libraries = path!("libraries").and(warp::fs::dir("libraries"));
    let figs = path!("figs").and(warp::fs::dir("figs"));

    warp::serve(index
                .or(change)
                .or(edit_concept)
                .or(concept)
                .or(activity)
                .or(style_css)
                .or(libraries)
                .or(figs)
                .with(warp::reply::with::default_header("Cache-Control", "no-store")))
        .run(([0, 0, 0, 0], 3030));
}
