use axum::{
    Json, Router,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use models::api::auth::SignUp;
use tracing::trace;

use crate::{
    app_templates::{self, Layout, pages},
    context::AppState,
    routes::{NestedRouter, NestedRouterPath, RouteResult},
    services::{ServiceManager, auth::AuthService},
};

#[derive(Clone)]
pub struct SignUpRoute {}

impl NestedRouterPath for SignUpRoute {
    const PATH: &str = "/sign_up";
}

impl NestedRouter<AppState> for SignUpRoute {
    fn router() -> Router<AppState> {
        axum::Router::new()
            .route("/", get(sign_up_page))
            .route("/", post(sign_up))
    }
}

async fn sign_up_page(headers: HeaderMap) -> impl IntoResponse {
    trace!("->> sign_up_page");
    let page = pages::sign_up::page();
    app_templates::render(Box::new(page), Layout::Main, &headers)
}

async fn sign_up(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    Json(sign_up): Json<SignUp>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> sign_up");
    auth.sign_up(sign_up).await?;
    Ok(Redirect::temporary("/login"))
}
