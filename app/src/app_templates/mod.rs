use axum::body::Body;
use axum::http::{HeaderMap, Response};
use axum::response::IntoResponse;
use templr::{FnTemplate, Template, templ, templ_ret};
use utils::datastar;

pub mod layouts;
pub mod pages;
pub mod tables;

pub fn index<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Rust Starter</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link href="/public/css/index.css" rel="stylesheet" />
                <script type="module" src="/public/js/datastar.js" />
            </head>
            <body class="min-h-screen bg-background">
                #children;
            </body>
        </html>
    }
}

pub enum Layout {
    Main,
}

pub fn render<'a>(
    f: Box<dyn Template + Send>,
    layout: Layout,
    headers: &HeaderMap,
) -> Response<Body> {
    match datastar::is_request(headers) {
        false => {
            let layout_template = FnTemplate::new(move |w, ctx, _| {
                match layout {
                    Layout::Main => layouts::main::main(),
                }
                .render_with_children_into(w, ctx, &*f)
            });
            FnTemplate::new(move |w, ctx, _| {
                index().render_with_children_into(w, ctx, &layout_template)
            })
            .into_response()
        }
        true => FnTemplate::new(move |w, ctx, _| f.render_into(w, ctx)).into_response(),
    }
}
