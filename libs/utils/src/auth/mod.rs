use std::fmt;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::Duration;

pub mod hash;
pub mod privileges;
pub mod roles;

pub type AuthResult<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    InvalidCookie,
    MissingCookie,
}

impl Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidCookie => "invalid cookie",
            Error::MissingCookie => "missing cookie",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())?;
        Ok(())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        match self {
            Error::InvalidCookie | Error::MissingCookie => StatusCode::UNAUTHORIZED,
        }
        .into_response()
    }
}

const COOKIE_NAME: &str = "rs-auth";

#[derive(Clone, Debug)]
pub struct AuthParts {
    pub token: String,
    pub id: i64,
}

impl Default for AuthParts {
    fn default() -> Self {
        AuthParts {
            token: String::new(),
            id: 0,
        }
    }
}

impl TryFrom<&str> for AuthParts {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (left, right) = value.rsplit_once("|").ok_or(Error::InvalidCookie)?;
        let parts = AuthParts {
            id: right.parse().or(Err(Error::InvalidCookie))?,
            token: left.to_string(),
        };
        Ok(parts)
    }
}

impl TryFrom<CookieJar> for AuthParts {
    type Error = Error;

    fn try_from(cookie_jar: CookieJar) -> Result<Self, Self::Error> {
        let cookie = cookie_jar.get(COOKIE_NAME).ok_or(Error::InvalidCookie)?;
        AuthParts::try_from(cookie.value())
    }
}

impl<'c> Into<Cookie<'c>> for AuthParts {
    fn into(self) -> Cookie<'c> {
        let mut cookie = Cookie::new(COOKIE_NAME, format!("{}|{}", self.token, self.id));
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);
        cookie.set_max_age(Duration::days(30));
        cookie
    }
}
