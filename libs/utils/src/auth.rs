use std::fmt;

use argon2::{
    PasswordHash, PasswordVerifier,
    password_hash::{self, PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};

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
        Cookie::new(COOKIE_NAME, format!("{}|{}", self.token, self.id))
    }
}

pub fn hash_password<'a>(password: &str) -> password_hash::Result<String> {
    let argon = argon2::Argon2::default();
    let salt = &SaltString::generate(&mut OsRng);
    let hash = argon.hash_password(password.as_bytes(), salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> password_hash::Result<()> {
    let argon = argon2::Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;
    argon.verify_password(password.as_bytes(), &parsed_hash)
}
