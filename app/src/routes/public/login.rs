use axum::{
    Form, Router,
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use datastar::{elements::DatastarElement, modes::DatastarMode};
use models::api::auth::Login;
use tracing::trace;
use utils::auth::AuthParts;

use crate::{
    app_templates::{self, Layout, pages},
    context::AppState,
    routes::{NestedRouter, NestedRouterPath, RouteResult},
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
    let cookie = AuthParts {
        id: user.id,
        token: session.token,
    };

    let el = DatastarElement::redirect_element("/management/users")?;
    let sse = datastar::patch_elements()
        .selector("#login")
        .mode(DatastarMode::After)
        .elements(el)
        .axum_stream();

    Ok((jar.add(cookie), sse))
}
