use axum::{
    Form, Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use models::api::auth::{Login, SignUp};

use crate::{
    context::AppState,
    routes::RouteResult,
    routes::{NestedRouter, NestedRouterPath},
    services::{ServiceManager, auth::AuthService},
};

#[derive(Clone)]
pub struct MeRoute {}

impl NestedRouterPath for MeRoute {
    const PATH: &str = "/me";
}

impl NestedRouter<AppState> for MeRoute {
    fn router() -> Router<AppState> {
        axum::Router::new()
            .route("/sign_up", get(sign_up_page))
            .route("/sign_up", post(sign_up))
            .route("/login", get(login_page))
            .route("/login", post(login))
    }
}

async fn sign_up(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    Form(sign_up): Form<SignUp>,
) -> RouteResult<impl IntoResponse> {
    let user = auth.sign_up(sign_up).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn sign_up_page() -> RouteResult<impl IntoResponse> {
    // todo
    Ok(())
}

async fn login(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    Form(login): Form<Login>,
) -> RouteResult<impl IntoResponse> {
    let _ = auth.login(login).await?;
    Ok(StatusCode::OK)
}

async fn login_page() -> RouteResult<impl IntoResponse> {
    Ok(())
}
