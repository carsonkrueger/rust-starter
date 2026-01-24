use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{delete, get, post},
};
use datastar::{
    modes::DatastarMode,
    templates::table::{
        DatastarRowsProps, IntoTableData, search_params::DatastarSearchParams, table_patch_stream,
    },
};
use models::{api::search_params::SearchParams, db::auth::role_privilege::RolePrivilegeJoin};
use serde::Deserialize;
use templr::Template;
use tracing::trace;
use utils::auth::privileges::Privilege;

use crate::{
    app_templates::{
        Layout,
        pages::{management_roles_privileges, management_users},
        render,
    },
    context::AppState,
    middlewares::privileges::privileges_middleware,
    routes::{Error, NestedRouter, NestedRouterPath, RouteResult},
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
                    vec![Privilege::UsersRead, Privilege::UsersDelete],
                    privileges_middleware,
                )),
            )
            .route(
                "/users/rows",
                get(users_rows).route_layer(from_fn_with_state(
                    vec![Privilege::UsersRead],
                    privileges_middleware,
                )),
            )
            .route(
                "/roles_privileges",
                get(roles_privileges_page).route_layer(from_fn_with_state(
                    vec![
                        Privilege::RolesPrivilegeRead,
                        Privilege::RolesPrivilegeDelete,
                    ],
                    privileges_middleware,
                )),
            )
            .route(
                "/roles_privileges/rows",
                get(roles_privileges_rows).route_layer(from_fn_with_state(
                    vec![Privilege::RolesPrivilegeCreate],
                    privileges_middleware,
                )),
            )
            .route(
                "/roles/{role_id}/privileges/{privilege_id}",
                delete(delete_role_privilege).route_layer(from_fn_with_state(
                    vec![Privilege::RolesPrivilegeDelete],
                    privileges_middleware,
                )),
            )
            .route(
                "/roles/privileges",
                post(create_role_privilege).route_layer(from_fn_with_state(
                    vec![Privilege::RolesPrivilegeCreate],
                    privileges_middleware,
                )),
            )
    }
}
// /roles/{}/privilege/{}

async fn users_page() -> RouteResult<impl IntoResponse> {
    trace!("->> users_page");
    let page = management_users::page();
    Ok(render(Box::new(page), Layout::Management))
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

async fn roles_privileges_page(
    State(AppState {
        svc: ServiceManager { privileges, .. },
        ..
    }): State<AppState>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> roles_privileges_page");

    let params = SearchParams {
        limit: 9999,
        page: 1,
        query: None,
    };
    let roles = privileges.roles(&params).await?;
    let privileges = privileges.privileges(&params).await?;

    let page = management_roles_privileges::page(&roles, &privileges);
    Ok(render(Box::new(page), Layout::Management))
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
    let roles = privileges
        .list_roles_privileges(search_params.clone())
        .await?;
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

    let join = privileges
        .one_role_privilege(role_id, privilege_id)
        .await?
        .ok_or(Error::GenericError(
            StatusCode::NOT_FOUND,
            "Role privilege not found".to_string(),
        ))?;
    privileges
        .disassociate_auth(role_id, privilege_id)
        .await?
        .ok_or(Error::GenericError(
            StatusCode::NOT_FOUND,
            "Role privilege not found".to_string(),
        ))?;

    let selector = format!("#{}", join.row_id());
    Ok(datastar::patch_elements()
        .mode(DatastarMode::Remove)
        .selector(selector)
        .axum_stream())
}

#[derive(Deserialize)]
struct RolePrivilegeCreate {
    role_id: i16,
    privilege_id: i64,
}

async fn create_role_privilege(
    State(AppState {
        svc: ServiceManager { privileges, .. },
        ..
    }): State<AppState>,
    Form(role_privilege_create): Form<RolePrivilegeCreate>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> create_role_privilege");

    privileges
        .create_role_privilege(
            role_privilege_create.role_id,
            role_privilege_create.privilege_id,
        )
        .await?;
    let join = privileges
        .one_role_privilege(
            role_privilege_create.role_id,
            role_privilege_create.privilege_id,
        )
        .await?
        .ok_or(Error::GenericError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch role privilege".to_string(),
        ))?;

    let rows = &[join];
    let row_template =
        datastar::templates::table::datastar_rows(DatastarRowsProps { rows }).render(&())?;
    let sse = datastar::patch_elements()
        .selector(format!("#{}", RolePrivilegeJoin::TABLE_BODY_ID))
        .mode(DatastarMode::Prepend)
        .elements(row_template)
        .axum_stream();

    Ok(sse)
}
