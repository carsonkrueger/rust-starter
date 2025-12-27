use axum::{Router, http::HeaderMap, response::IntoResponse, routing::get};
use tracing::trace;

use crate::{
    app_templates::{
        self, Layout,
        pages::{self},
    },
    context::AppState,
    routes::{NestedRouter, NestedRouterPath},
};

#[derive(Clone)]
pub struct HomeRoute {}

impl NestedRouterPath for HomeRoute {
    const PATH: &str = "/home";
}

impl NestedRouter<AppState> for HomeRoute {
    fn router() -> Router<AppState> {
        axum::Router::new().route("/", get(home_page))
    }
}

async fn home_page(headers: HeaderMap) -> impl IntoResponse {
    trace!("->> home_page");
    let page = pages::home::page();
    app_templates::render(Box::new(page), Layout::Main, &headers)
}
