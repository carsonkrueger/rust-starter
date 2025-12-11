use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::{
    app_router::{NestedRouter, NestedRouterPath},
    context::AppContext,
    services::hello_world_two::HelloWorldTwoService,
};

#[derive(Clone)]
pub struct HelloWorldRoute {}

impl NestedRouterPath for HelloWorldRoute {
    const PATH: &str = "/hello_world";
}

impl NestedRouter<Arc<AppContext>> for HelloWorldRoute {
    fn router() -> Router<Arc<AppContext>> {
        axum::Router::new().route("/", get(hello_world))
    }
}

async fn hello_world(State(ctx): State<Arc<AppContext>>) -> impl IntoResponse {
    match ctx.svc.hello_world_two.hello_world_two().await {
        Ok(message) => message,
        Err(err) => format!("Error: {}", err),
    }
}
