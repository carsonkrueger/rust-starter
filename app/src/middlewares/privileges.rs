use axum::{
    Extension,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tracing::warn;
use utils::{auth::privileges::Privilege, extensions::ctx::Ctx};

pub async fn privileges_middleware(
    State(privs): State<Vec<Privilege>>,
    Extension(ctx): Extension<Ctx>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    for p in &privs {
        if !ctx.privileges.contains(p) {
            warn!("User does not have required privilege: {:?}", p);
            return StatusCode::FORBIDDEN.into_response();
        }
    }
    next.run(req).await
}
