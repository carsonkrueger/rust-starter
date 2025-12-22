use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::{
    app_router::{NestedRouter, NestedRouterPath},
    app_templates::pages,
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

async fn hello_world(
    State(AppState {
        svc: ServiceManager { hello_world, .. },
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    let _ = hello_world.hello_world().await;
    let _ = "haha3";
    pages::home::page().into_response()
}
