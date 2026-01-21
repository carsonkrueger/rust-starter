use std::collections::HashSet;

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use tracing::Span;
use utils::{
    auth::privileges::Privilege,
    extensions::ctx::{Ctx, CtxError},
};

use crate::{
    context::AppState,
    routes::RouteResult,
    services::{self, ServiceManager, auth::AuthService},
};

/// Inserts user result and privileges into the request extensions. Also adds user_id into tracing span.
pub async fn ctx_middleware(
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
            let db_privileges = auth.get_privileges(u.role_id).await?;
            let privileges: Result<HashSet<Privilege>, _> = db_privileges
                .into_iter()
                .map(TryInto::<Privilege>::try_into)
                .collect();
            Ok(Ctx {
                user: u,
                privileges: privileges?,
            })
        }
        Err(e) => Err(CtxError::Invalid(e.to_string())),
    };

    req.extensions_mut().insert(ctx_res);

    Ok(next.run(req).await)
}
