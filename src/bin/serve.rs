use warp::{Filter, path};
use progression_website::data::Data;
use display_as::{DisplayAs, HTML};

fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let concept = path!("concept" / String)
        .map(|name: String| {
            let data = Data::new();
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

    warp::serve(concept
                .or(activity)
                .or(style_css)
                .or(libraries)
                .or(figs)
                .or(index))
        .run(([0, 0, 0, 0], 3030));
}
