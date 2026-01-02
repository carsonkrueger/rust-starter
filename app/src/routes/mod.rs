use crate::{
    middlewares::trace::trace_middleware,
    routes::{
        private::management::ManagementRoute,
        public::{login::LoginRoute, sign_up::SignUpRoute},
    },
    services,
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
};
use strum::ParseError;
use tracing::{error, info_span, warn};
use utils::{auth, extensions::ctx};
use uuid::Uuid;

use crate::{
    context::AppState,
    middlewares::auth::auth_middleware,
    routes::public::{hello_world::HelloWorldRoute, home::HomeRoute},
};
use axum::{
    Router,
    middleware::{self},
};
use tower_http::{services::ServeDir, trace::TraceLayer};

pub mod private;
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
    #[error(transparent)]
    Ctx(#[from] ctx::CtxError),
    #[error(transparent)]
    Strum(#[from] ParseError),
    #[error(transparent)]
    Templr(#[from] templr::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Service(e) => {
                error!(error = %e, "service error");
                e.into_response()
            }
            Error::Auth(e) => {
                warn!(error = %e, "auth error");
                e.into_response()
            }
            Error::Ctx(e) => {
                warn!(error = %e, "ctx error");
                StatusCode::UNAUTHORIZED.into_response()
            }
            Error::Strum(e) => {
                error!(error = %e, "strum error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Error::Templr(e) => {
                error!(error = %e, "templr error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
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

pub fn build_router(app_state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http().make_span_with(|_req: &Request<Body>| {
        info_span!(
            "http.request",
            req_id = Uuid::now_v7().to_string(),
            user_id = tracing::field::Empty
        )
    });

    Router::new()
        // === Private Routes Begin ===
        .nest(ManagementRoute::PATH, ManagementRoute::router())
        // === Private Routes End ===
        .layer(middleware::from_fn(auth_middleware))
        // === Public Routes Begin ===
        .nest(HomeRoute::PATH, HomeRoute::router())
        .nest(LoginRoute::PATH, LoginRoute::router())
        .nest(SignUpRoute::PATH, SignUpRoute::router())
        .nest(HelloWorldRoute::PATH, HelloWorldRoute::router())
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(|| async { Redirect::permanent("/home") }))
        // === Public Routes End ===
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            trace_middleware,
        ))
        .layer(trace_layer)
        .with_state(app_state)
}
