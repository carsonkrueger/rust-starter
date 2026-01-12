use std::sync::Arc;

use crate::{
    repositories::{DBPool, RepositoryManager, users::UsersRepository},
    services::ServiceResult,
};
use models::{api::search_params::SearchParams, db::auth::user::User};
use tracing::trace;

pub trait UsersService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn search<S: Into<SearchParams>>(&self, params: S) -> ServiceResult<Vec<User>>;
}

#[derive(Debug, Clone)]
pub struct Users {
    pool: DBPool,
    repos: Arc<RepositoryManager>,
}

impl UsersService for Users {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self {
        Self { pool, repos }
    }
    async fn search<S: Into<SearchParams>>(&self, params: S) -> ServiceResult<Vec<User>> {
        trace!("->> search");
        let mut db = self.pool.get().await?;
        let users = self.repos.users.index(&mut db, &params.into()).await?;
        Ok(users)
    }
}
