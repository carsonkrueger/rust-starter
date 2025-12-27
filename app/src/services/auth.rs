use std::sync::Arc;

use crate::{
    repositories::{
        DBPool, RepositoryManager, sessions::SessionsRepository, users::UsersRepository,
    },
    services::ServiceResult,
};
use models::{
    api::auth::{Login, SignUp},
    db::auth::{session::Session, user::User},
};
use tracing::trace;
use utils::auth::{self, verify_password};

// #[async_trait]
pub trait AuthService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn login(&self, login: Login) -> ServiceResult<(User, Session)>;
    async fn sign_up(&self, sign_up: SignUp) -> ServiceResult<User>;
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
    async fn login(&self, login: Login) -> ServiceResult<(User, Session)> {
        trace!("->> login");
        let mut conn = self.pool.get().await?;
        let user = self
            .repos
            .users
            .get_by_email(&mut conn, &login.email)
            .await?;

        verify_password(&login.password, &user.password)?;

        let mut session = Session {
            user_id: user.id,
            token: uuid::Uuid::now_v7().to_string(),
            ..Default::default()
        };

        self.repos.sessions.insert(&mut conn, &mut session).await?;
        return Ok((user, session));
    }
    async fn get_user_by_auth(&self, parts: &auth::AuthParts) -> ServiceResult<User> {
        trace!("->> get_user_by_auth");
        let mut conn = self.pool.get().await?;
        Ok(self
            .repos
            .sessions
            .get_user(&mut conn, parts.id, &parts.token)
            .await?)
    }
    async fn sign_up(&self, sign_up: SignUp) -> ServiceResult<User> {
        trace!("->> sign_up");
        let mut user = User {
            email: sign_up.email,
            first_name: sign_up.first_name,
            last_name: sign_up.last_name,
            password: utils::auth::hash_password(&sign_up.password)?,
            ..Default::default()
        };
        dbg!(&user);
        let mut conn = self.pool.get().await?;

        self.repos.users.insert(&mut conn, &mut user).await?;
        dbg!(&user);
        Ok(user)
    }
}
