use std::sync::Arc;

use crate::{context::AppContext, routes::public::hello_world::HelloWorldRoute};
use axum::Router;

pub trait NestedRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn router() -> Router<S>;
}

pub trait NestedRouterPath {
    const PATH: &str;
}

pub fn build_router(ctx: AppContext) -> Router {
    Router::new()
        .nest(HelloWorldRoute::PATH, HelloWorldRoute::router())
        .with_state(Arc::new(ctx))
}
