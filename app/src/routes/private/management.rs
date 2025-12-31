use axum::{
    Router,
    extract::{Query, State},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::get,
};
use models::api::query_params::QueryParams;
use tracing::trace;
use utils::{auth::privileges::Privilege, extensions::privileges::RequiredPrivileges};

use crate::{
    app_templates::pages::management_users,
    context::AppState,
    middlewares::privileges::privileges_middleware,
    routes::{NestedRouter, NestedRouterPath, RouteResult},
    services::{ServiceManager, users::UsersService},
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
        svc: ServiceManager { users, .. },
        ..
    }): State<AppState>,
    Query(query_params): Query<QueryParams>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> users_page");
    let query_params = query_params.sanitize();
    let users = users.search(&query_params).await?;
    Ok(management_users::page(users.as_slice()).into_response())
}
