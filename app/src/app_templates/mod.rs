pub mod pages;

use axum::body::Body;
use axum::http::{HeaderMap, Response};
use axum::response::IntoResponse;
use templr::{FnTemplate, Template, templ, templ_ret};
use utils::datastar;

pub fn index<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <!DOCTYPE html>
        <html>
            <head>
                <title>Rust Starter</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link href="/public/css/index.css" rel="stylesheet" />
                <script type="module" src="/public/js/datastar.js"></script>
            </head>
            <body>
                #children;
            </body>
        </html>
    }
}

pub fn render<'a>(f: Box<dyn Template + Send>, headers: &HeaderMap) -> Response<Body> {
    let datastar_req = datastar::is_request(headers);

    FnTemplate::new(move |w, ctx, _| {
        if datastar_req {
            f.render_into(w, ctx)
        } else {
            index().render_with_children_into(w, ctx, &*f)
        }
    })
    .into_response()
}
