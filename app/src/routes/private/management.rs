use axum::{
    Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{delete, get},
};
use datastar::templates::table::{search_params::DatastarSearchParams, table_patch_stream};
use tracing::trace;
use utils::{auth::privileges::Privilege, extensions::privileges::RequiredPrivileges};

use crate::{
    app_templates::{
        Layout,
        pages::{management_roles_privileges, management_users},
        render,
    },
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
            .route(
                "/roles_privileges",
                get(roles_privileges_page).route_layer(from_fn_with_state(
                    RequiredPrivileges(vec![
                        Privilege::RolesPrivilegeRead,
                        Privilege::RolesPrivilegeDelete,
                    ]),
                    privileges_middleware,
                )),
            )
            .route(
                "/roles_privileges/rows",
                get(roles_privileges_rows).route_layer(from_fn_with_state(
                    RequiredPrivileges(vec![Privilege::RolesPrivilegeCreate]),
                    privileges_middleware,
                )),
            )
            .route(
                "/roles/{role_id}/privileges/{privilege_id}",
                delete(delete_role_privilege).route_layer(from_fn_with_state(
                    RequiredPrivileges(vec![Privilege::RolesPrivilegeDelete]),
                    privileges_middleware,
                )),
            )
    }
}
// /roles/{}/privilege/{}

async fn users_page(header_map: HeaderMap) -> RouteResult<impl IntoResponse> {
    trace!("->> users_page");
    let page = management_users::page();
    Ok(render(Box::new(page), Layout::Main, &header_map))
}

async fn users_rows(
    State(AppState {
        svc: ServiceManager { users, .. },
        ..
    }): State<AppState>,
    Query(DatastarSearchParams {
        data: search_params,
    }): Query<DatastarSearchParams>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> users_rows");
    let users = users.search(search_params.clone()).await?;
    let stream = table_patch_stream(&users, search_params)?;
    Ok(stream)
}

async fn roles_privileges_page(header_map: HeaderMap) -> RouteResult<impl IntoResponse> {
    trace!("->> roles_privileges_page");
    let page = management_roles_privileges::page();
    Ok(render(Box::new(page), Layout::Main, &header_map))
}

async fn roles_privileges_rows(
    State(AppState {
        svc: ServiceManager { privileges, .. },
        ..
    }): State<AppState>,
    Query(DatastarSearchParams {
        data: search_params,
    }): Query<DatastarSearchParams>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> roles_privileges_rows");
    let roles = privileges.list_roles_privileges().await?;
    let stream = table_patch_stream(&roles, search_params)?;
    Ok(stream)
}

async fn delete_role_privilege(
    State(AppState {
        svc: ServiceManager { privileges, .. },
        ..
    }): State<AppState>,
    Path((role_id, privilege_id)): Path<(i16, i64)>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> delete_role_privilege");
    privileges.disassociate(role_id, privilege_id).await?;
    Ok("")
}
