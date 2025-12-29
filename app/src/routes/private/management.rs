use axum::{
    Router, extract::State, middleware::from_fn_with_state, response::IntoResponse, routing::get,
};
use tracing::trace;
use utils::{auth::privileges::Privilege, extensions::privileges::RequiredPrivileges};

use crate::{
    context::AppState,
    middlewares::privileges::privileges_middleware,
    routes::{NestedRouter, NestedRouterPath},
    services::{ServiceManager, hello_world::HelloWorldService},
};

#[derive(Clone)]
pub struct ManagementRoute {}

impl NestedRouterPath for ManagementRoute {
    const PATH: &str = "/management";
}

impl NestedRouter<AppState> for ManagementRoute {
    fn router() -> Router<AppState> {
        axum::Router::new().route(
            "/users",
            get(users_page).route_layer(from_fn_with_state(
                RequiredPrivileges(vec![Privilege::UsersRead, Privilege::UsersDelete]),
                privileges_middleware,
            )),
        )
    }
}

async fn users_page(
    State(AppState {
        svc: ServiceManager { hello_world, .. },
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    trace!("->> users_page");
    hello_world.hello_world().await
}
