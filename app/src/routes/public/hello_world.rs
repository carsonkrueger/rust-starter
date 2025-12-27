use axum::{Router, extract::State, response::IntoResponse, routing::get};
use tracing::trace;

use crate::{
    context::AppState,
    routes::{NestedRouter, NestedRouterPath},
    services::{ServiceManager, hello_world::HelloWorldService},
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

async fn hello_world(
    State(AppState {
        svc: ServiceManager { hello_world, .. },
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    trace!("->> hello_world");
    hello_world.hello_world().await
}
