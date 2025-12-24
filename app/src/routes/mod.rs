use crate::{routes::public::me::MeRoute, services};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use utils::auth;

use crate::{
    context::AppState,
    middlewares::auth::auth_middleware,
    routes::public::{hello_world::HelloWorldRoute, home::HomeRoute},
};
use axum::{
    Router,
    middleware::{self},
};
use tower_http::services::ServeDir;

pub mod public;

#[allow(unused)]
pub type RouteResult<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Service(#[from] services::Error),
    #[error(transparent)]
    Auth(#[from] auth::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Service(e) => e.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub trait NestedRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn router() -> Router<S>;
}

pub trait NestedRouterPath {
    const PATH: &str;
}

pub fn build_router(ctx: AppState) -> Router {
    Router::new()
        // === Private Routes Below ===
        // ^^^ Private Routes Above ^^^
        .layer(middleware::from_fn_with_state(ctx.clone(), auth_middleware))
        // === Public Routes Below ===
        .nest(MeRoute::PATH, MeRoute::router())
        .nest(HomeRoute::PATH, HomeRoute::router())
        .nest(HelloWorldRoute::PATH, HelloWorldRoute::router())
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(|| async { Redirect::permanent("/home") }))
        // ^^^ Public Routes Above ^^^
        .with_state(ctx)
}
