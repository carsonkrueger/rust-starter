use axum::{
    Router,
    extract::{Query, State},
    http::HeaderMap,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::get,
};
use datastar::templates::table::{search_params::DatastarSearchParams, table_patch_stream};
use tracing::trace;
use utils::{auth::privileges::Privilege, extensions::privileges::RequiredPrivileges};

use crate::{
    app_templates::{Layout, pages::management_users, render},
    context::AppState,
    middlewares::privileges::privileges_middleware,
    routes::{NestedRouter, NestedRouterPath, RouteResult},
    services::{ServiceManager, privileges::PrivilegesService, users::UsersService},
};

#[derive(Clone)]
pub struct ManagementRoute {}

impl NestedRouterPath for ManagementRoute {
    const PATH: &str = "/management";
}

impl NestedRouter<AppState> for ManagementRoute {
    fn router() -> Router<AppState> {
        axum::Router::new()
            .route(
                "/users",
                get(users_page).route_layer(from_fn_with_state(
                    RequiredPrivileges(vec![Privilege::UsersRead, Privilege::UsersDelete]),
                    privileges_middleware,
                )),
            )
            .route(
                "/users/rows",
                get(users_rows).route_layer(from_fn_with_state(
                    RequiredPrivileges(vec![Privilege::UsersRead]),
                    privileges_middleware,
                )),
            )
    }
}

async fn users_page(header_map: HeaderMap) -> RouteResult<impl IntoResponse> {
    trace!("->> users_page");
    let page = management_users::page();
    Ok(render(Box::new(page), Layout::Main, &header_map))
}

async fn users_rows(
    State(AppState {
        svc: ServiceManager {
            users, privileges, ..
        },
        ..
    }): State<AppState>,
    Query(DatastarSearchParams {
        data: search_params,
    }): Query<DatastarSearchParams>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> users_rows");
    let users = users.search(search_params.clone()).await?;
    let roles = privileges.list_roles().await?;
    let stream = table_patch_stream(&users, search_params)?;
    Ok(stream)
}
