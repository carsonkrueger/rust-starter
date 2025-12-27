use axum::{Extension, body::Body, extract::Request, middleware::Next, response::IntoResponse};

use utils::ctx::CtxResult;

use crate::routes::RouteResult;

pub async fn auth_middleware(
    Extension(res_ctx): Extension<CtxResult>,
    req: Request<Body>,
    next: Next,
) -> RouteResult<impl IntoResponse> {
    res_ctx?;
    Ok(next.run(req).await)
}
