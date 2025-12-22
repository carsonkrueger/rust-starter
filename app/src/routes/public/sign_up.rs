use axum::{
    Form, Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get,
};
use models::api::auth::SignUp;

use crate::{
    app_router::{NestedRouter, NestedRouterPath},
    context::AppState,
    routes::RouteResult,
    services::{ServiceManager, auth::AuthService},
};

#[derive(Clone)]
pub struct SignUpRoute {}

impl NestedRouterPath for SignUpRoute {
    const PATH: &str = "/sign_up";
}

impl NestedRouter<AppState> for SignUpRoute {
    fn router() -> Router<AppState> {
        axum::Router::new().route("/", get(sign_up))
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
