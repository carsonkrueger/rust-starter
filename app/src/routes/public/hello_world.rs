use axum::{Router, extract::State, http::HeaderMap, response::IntoResponse, routing::get};

use crate::{
    app_router::{NestedRouter, NestedRouterPath},
    app_templates::{self, pages::home},
    context::AppState,
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

#[axum::debug_handler]
async fn hello_world(
    State(AppState {
        svc: ServiceManager { hello_world, .. },
        ..
    }): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let _ = hello_world.hello_world().await;
    let _ = "haha3";
    let page = home::page();
    app_templates::render(Box::new(page), &headers)
}
