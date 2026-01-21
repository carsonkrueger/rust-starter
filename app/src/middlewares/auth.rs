use axum::{Extension, body::Body, extract::Request, middleware::Next, response::IntoResponse};

use utils::extensions::ctx::CtxResult;

use crate::routes::RouteResult;

/// Enforces that the user is authenticated.
pub async fn auth_middleware(
    Extension(res_ctx): Extension<CtxResult>,
    mut req: Request<Body>,
    next: Next,
) -> RouteResult<impl IntoResponse> {
    req.extensions_mut().insert(res_ctx?);
    Ok(next.run(req).await)
}
