use std::sync::Arc;

use crate::{
    repositories::{
        DBPool, RepositoryManager, privileges::PrivilegesRepository, sessions::SessionsRepository,
        users::UsersRepository,
    },
    services::{self, ServiceResult},
};
use chrono::{Days, Utc};
use models::{
    api::auth::{Login, SignUp},
    db::auth::{privilege::Privilege, session::Session, user::User},
};
use tracing::trace;
use utils::auth::{
    self, MAX_COOKIE_AGE_DAYS,
    hash::{hash_password, verify_password},
    roles::ROLE_ADMIN,
};

// #[async_trait]
pub trait AuthService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn login(&self, login: Login) -> ServiceResult<(User, Session)>;
    async fn sign_up(&self, sign_up: SignUp) -> ServiceResult<User>;
    async fn get_user_by_auth(&self, parts: &auth::AuthParts) -> ServiceResult<User>;
    async fn get_privileges(&self, role_id: i16) -> ServiceResult<Vec<Privilege>>;
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

        let expires_at = Utc::now()
            .checked_add_days(Days::new(MAX_COOKIE_AGE_DAYS))
            .ok_or(services::Error::Generic(
                "could not create expires at".to_string(),
            ))?;

        let mut session = Session {
            user_id: user.id,
            token: uuid::Uuid::now_v7().to_string(),
            created_at: None,
            expires_at: Some(expires_at.naive_utc()),
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
            password: hash_password(&sign_up.password)?,
            role_id: ROLE_ADMIN, // TODO: CHANGE TO ROLE_BASIC, this is admin for testing purposes only
            ..Default::default()
        };
        let mut conn = self.pool.get().await?;

        self.repos.users.insert(&mut conn, &mut user).await?;
        Ok(user)
    }
    async fn get_privileges(&self, role_id: i16) -> ServiceResult<Vec<Privilege>> {
        let mut conn = self.pool.get().await?;
        let privileges = self
            .repos
            .privileges
            .get_by_role(&mut conn, role_id)
            .await?;
        Ok(privileges)
    }
}
