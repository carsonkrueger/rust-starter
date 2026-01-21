use std::{
    sync::OnceLock,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use templr::{FnTemplate, Template, templ, templ_ret};

pub mod layouts;
pub mod pages;

static VERSION: OnceLock<u128> = OnceLock::new();

fn version() -> u128 {
    *VERSION.get_or_init(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    })
}

pub fn index<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <title>Rust Starter</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link href={format!("/public/css/index.css?v={}", version())} rel="stylesheet" />
                <script type="module" src="/public/js/datastar.js" />
            </head>
            <body class="min-h-screen bg-background">
                #children;
            </body>
        </html>
    }
}

#[allow(unused)]
pub enum Layout {
    Main,
    Management,
}

/// Renders the template wrapping it with the index.html and the layout.
pub fn render<'a>(f: Box<dyn Template + Send + 'a>, layout: Layout) -> Response<Body> {
    let layout_template = FnTemplate::new(move |w, ctx, _| {
        let l: Box<dyn Template> = match layout {
            Layout::Main => Box::new(layouts::main::main()),
            Layout::Management => Box::new(layouts::management::management()),
        };
        l.render_with_children_into(w, ctx, &*f)
    });
    FnTemplate::new(move |w, ctx, _| index().render_with_children_into(w, ctx, &layout_template))
        .into_response()
}
