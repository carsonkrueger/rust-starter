use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    repositories::{self, DBPool, RepositoryManager},
    services::{auth::AuthService, hello_world::HelloWorldService, privileges::PrivilegesService},
};

pub mod auth;
pub mod hello_world;
pub mod privileges;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct ServiceManager<HW: HelloWorldService, AT: AuthService, PS: PrivilegesService> {
    pub hello_world: Arc<HW>,
    pub auth: Arc<AT>,
    pub privileges: Arc<PS>,
}

impl<HW, AT, PS> ServiceManager<HW, AT, PS>
where
    HW: HelloWorldService,
    AT: AuthService,
    PS: PrivilegesService,
{
    pub fn default(pool: DBPool, repos: RepositoryManager) -> Self {
        let repos = Arc::new(repos);
        let hello_world = Arc::new(HW::new());
        let auth = Arc::new(AT::new(pool.clone(), repos.clone()));
        let privileges = Arc::new(PS::new(pool, repos.clone()));
        Self {
            hello_world,
            auth,
            privileges,
        }
    }
}

#[allow(unused)]
pub type ServiceResult<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("err: {0}")]
    Generic(String),
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("repository err: {0}")]
    Repository(#[from] repositories::Error),
    #[error("bb8 err: {0}")]
    Bb8(String),
    #[error("hash err: {0}")]
    Hash(#[from] utils::prelude::Error),
}

impl<E: std::error::Error + 'static> From<bb8::RunError<E>> for Error {
    fn from(err: bb8::RunError<E>) -> Self {
        Error::Bb8(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Generic(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            Error::InvalidCredentials | Error::Hash(_) => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
            Error::Repository(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            Error::Bb8(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
