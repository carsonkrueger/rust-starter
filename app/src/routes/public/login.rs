use axum::{
    Form, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use models::api::auth::Login;
use tracing::trace;

use crate::{
    app_templates::{self, Layout, pages},
    context::AppState,
    routes::{NestedRouter, NestedRouterPath, RouteResult},
    services::{ServiceManager, auth::AuthService},
};

#[derive(Clone)]
pub struct LoginRoute {}

impl NestedRouterPath for LoginRoute {
    const PATH: &str = "/login";
}

impl NestedRouter<AppState> for LoginRoute {
    fn router() -> Router<AppState> {
        axum::Router::new()
            .route("/", get(login_page))
            .route("/", post(login))
    }
}

async fn login_page(headers: HeaderMap) -> impl IntoResponse {
    trace!("->> login_page");
    let page = pages::login::page();
    app_templates::render(Box::new(page), Layout::Main, &headers)
}

async fn login(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    Form(login): Form<Login>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> login");
    let _ = auth.login(login).await?;
    Ok(StatusCode::OK)
}
