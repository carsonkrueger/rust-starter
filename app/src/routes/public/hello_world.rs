use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::{
    app_router::{NestedRouter, NestedRouterPath},
    context::AppContext,
    services::hello_world::HelloWorldService,
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

#[axum::debug_handler]
async fn hello_world(State(ctx): State<Arc<AppContext>>) -> impl IntoResponse {
    ctx.svc.hello_world.hello_world().await
}
