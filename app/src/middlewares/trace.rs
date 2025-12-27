use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use tracing::Span;
use utils::ctx::{Ctx, CtxError};

use crate::{
    context::AppState,
    routes::RouteResult,
    services::{self, ServiceManager, auth::AuthService},
};

pub async fn trace_middleware(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> RouteResult<impl IntoResponse> {
    let parts_res = jar.try_into();

    let user_res = match parts_res {
        Ok(parts) => auth.get_user_by_auth(&parts).await,
        Err(_) => Err(services::Error::InvalidCredentials),
    };

    let ctx_res = match user_res {
        Ok(u) => {
            let span = Span::current();
            span.record("user_id", u.id);
            req.extensions_mut().insert(span);
            Ok(Ctx { user: u })
        }
        Err(e) => Err(CtxError::Invalid(e.to_string())),
    };

    req.extensions_mut().insert(ctx_res);

    Ok(next.run(req).await)
}
