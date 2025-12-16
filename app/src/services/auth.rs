use std::sync::Arc;

use crate::{
    models::auth::user::User,
    repositories::{DBPool, RepositoryManager, sessions::SessionsRepository},
    services::ServiceResult,
};

// #[async_trait]
pub trait AuthService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn login(&self) -> String;
    async fn get_user_by_auth(&self, parts: &auth::AuthParts) -> ServiceResult<User>;
}

#[derive(Debug, Clone)]
pub struct Auth {
    pool: DBPool,
    repos: Arc<RepositoryManager>,
}

// #[async_trait]
impl AuthService for Auth {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self {
        Self { pool, repos }
    }
    async fn login(&self) -> String {
        "Hello World!".into()
    }
    async fn get_user_by_auth(&self, parts: &auth::AuthParts) -> ServiceResult<User> {
        let mut conn = self.pool.get().await?;
        Ok(self
            .repos
            .sessions
            .get_user(&mut conn, parts.id, &parts.token)
            .await?)
    }
}
