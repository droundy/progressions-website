use warp::{Filter, path};
use progression_website::data::Data;
use display_as::{DisplayAs, HTML};

fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("concept" / String)
        .map(move |name: String| {
            let data = Data::new();
            data.concept_view(data.concept_by_name(&name))
                .display_as(HTML).into_reply()
        });

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030));
}
