use std::{convert::Infallible, thread::sleep, time::Duration};

use axum::{
    Router, debug_handler,
    extract::{Query, State},
    middleware::from_fn_with_state,
    response::{IntoResponse, Sse, sse::Event},
    routing::get,
};
use futures_util::{Stream, stream};
use models::api::query_params::QueryParams;
use templr::Template;
use tracing::trace;
use utils::{auth::privileges::Privilege, extensions::privileges::RequiredPrivileges};

use crate::{
    app_templates::{pages::management_users, tables},
    context::{
        AppState,
        datastar::{DatastarElements, DatastarEvent, DatastarMode},
    },
    middlewares::privileges::privileges_middleware,
    routes::{self, NestedRouter, NestedRouterPath, RouteResult},
    services::{ServiceManager, users::UsersService},
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

#[debug_handler]
async fn users_rows(
    State(AppState {
        svc: ServiceManager { users, .. },
        ..
    }): State<AppState>,
    Query(query_params): Query<QueryParams>,
) -> RouteResult<Sse<impl Stream<Item = Result<Event, routes::Error>>>> {
    trace!("->> users_rows");
    let users = users.search(&query_params).await?;

    let event = {
        let mut event = Event::default()
            .event::<&'static str>(DatastarEvent::DatastarPatchElements.into())
            .data::<&'static str>(DatastarMode::Before.into());
        for u in &users {
            event = event.data(tables::management::user_row(u).render(&())?);
        }
        Ok(event)
    };

    sleep(Duration::from_secs(1));

    let stream = stream::once(async move { event });
    Ok(Sse::new(stream))
}
