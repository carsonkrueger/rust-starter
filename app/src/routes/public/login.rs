use axum::{
    Form, Router,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Sse, sse::Event},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use futures_util::stream;
use models::api::auth::Login;
use tracing::trace;
use utils::auth::AuthParts;

use crate::{
    app_templates::{self, Layout, pages},
    context::{
        AppState,
        datastar::{DatastarElement, DatastarMode},
    },
    routes::{self, NestedRouter, NestedRouterPath, RouteResult},
    services::{ServiceManager, auth::AuthService},
};

#[derive(Clone)]
pub struct LoginRoute {}

impl NestedRouterPath for LoginRoute {
    const PATH: &str = "/login";
}

impl NestedRouter<AppState> for LoginRoute {
    fn router() -> Router<AppState> {
        axum::Router::new()
            .route("/", get(login_page))
            .route("/", post(login))
    }
}

async fn login_page(headers: HeaderMap) -> impl IntoResponse {
    trace!("->> login_page");
    let page = pages::login::page();
    app_templates::render(Box::new(page), Layout::Main, &headers)
}

async fn login(
    State(AppState {
        svc: ServiceManager { auth, .. },
        ..
    }): State<AppState>,
    jar: CookieJar,
    Form(login): Form<Login>,
) -> RouteResult<impl IntoResponse> {
    trace!("->> login");
    let (user, session) = auth.login(login).await?;
    let cookie: Cookie = AuthParts {
        id: user.id,
        token: session.token,
    }
    .into();

    let el = DatastarElement::redirect("/management/users")?;
    let stream = stream::once(async move {
        Ok::<Event, routes::Error>(el.event_with_mode("#login", DatastarMode::After))
    });
    let sse = Sse::new(stream);

    Ok((jar.add(cookie), sse))
}
