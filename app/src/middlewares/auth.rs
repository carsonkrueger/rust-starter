use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use utils::{auth::AuthParts, ctx::Ctx};

use crate::{
    context::AppState,
    routes::RouteResult,
    services::{ServiceManager, auth::AuthService},
};

pub async fn auth_middleware(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> RouteResult<impl IntoResponse> {
    let parts: AuthParts = jar.try_into()?;
    let user = auth.get_user_by_auth(&parts).await?;
    let ctx = Ctx { auth: parts, user };
    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}
