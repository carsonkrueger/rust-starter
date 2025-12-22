use crate::auth::{AuthParts, AuthResult, Error};
use axum::{extract::FromRequestParts, http::request::Parts};
use models::db::auth::user::User;

#[derive(Clone, Debug)]
pub struct Ctx {
    pub auth: AuthParts,
    pub user: User,
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AuthResult<Self> {
        Ok(parts
            .extensions
            .get::<AuthResult<Ctx>>()
            .ok_or(Error::InvalidCookie)?
            .as_ref()
            .or(Err(Error::InvalidCookie))?
            .clone())
    }
}
