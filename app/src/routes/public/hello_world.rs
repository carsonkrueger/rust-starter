use axum::{Router, response::IntoResponse, routing::get};
use tracing::trace;

use crate::{
    context::AppState,
    routes::{NestedRouter, NestedRouterPath},
};

#[derive(Clone)]
pub struct HelloWorldRoute {}

impl NestedRouterPath for HelloWorldRoute {
    const PATH: &str = "/hello_world";
}

impl NestedRouter<AppState> for HelloWorldRoute {
    fn router() -> Router<AppState> {
        axum::Router::new().route("/", get(hello_world))
    }
}

async fn hello_world() -> impl IntoResponse {
    trace!("->> hello_world");
    "Hello World!"
}
