use axum::{
    Extension,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use utils::extensions::{ctx::Ctx, privileges::RequiredPrivileges};

pub async fn privileges_middleware(
    State(RequiredPrivileges(privs)): State<RequiredPrivileges>,
    Extension(ctx): Extension<Ctx>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    dbg!("required:", &privs);
    dbg!("contains:", &ctx.privileges);
    for p in &privs {
        if !ctx.privileges.contains(p) {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }
    next.run(req).await
}
