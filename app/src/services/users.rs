use std::sync::Arc;

use crate::{
    repositories::{DBPool, RepositoryManager, users::UsersRepository},
    services::ServiceResult,
};
use models::{api::query_params::QueryParams, db::auth::user::User};
use tracing::trace;

pub trait UsersService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn search<'a>(&self, params: &QueryParams) -> ServiceResult<Vec<User>>;
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
    async fn search<'a>(&self, params: &QueryParams) -> ServiceResult<Vec<User>> {
        trace!("->> search");
        let mut db = self.pool.get().await?;
        let users = self.repos.users.index(&mut db, params).await?;
        Ok(users)
    }
}
