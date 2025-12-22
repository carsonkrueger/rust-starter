use crate::{
    context::AppState, middlewares::auth::auth_middleware,
    routes::public::hello_world::HelloWorldRoute,
};
use axum::{
    Router,
    middleware::{self},
};
use tower_http::services::ServeDir;

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
        .nest(HelloWorldRoute::PATH, HelloWorldRoute::router())
        .nest_service("/public", ServeDir::new("public"))
        // ^^^ Public Routes Above ^^^
        .with_state(ctx)
}
