use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

use crate::services;

pub mod public;

#[allow(unused)]
pub type RouteResult<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Service(#[from] services::Error),
    #[error(transparent)]
    Auth(#[from] auth::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::new(format!("{}", self)))
    }
}
