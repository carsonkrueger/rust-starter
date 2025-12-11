use std::sync::Arc;

use async_trait::async_trait;
use diesel::Connection;

use crate::{
    models::user::User,
    repositories::{DBPool, RepositoryManager, users::UsersRepository},
    services::{
        self,
        hello_world::{HelloWorld, HelloWorldService},
    },
};

#[async_trait]
pub trait HelloWorldTwoService<HW>
where
    HW: HelloWorldService,
{
    fn new(pool: DBPool, repos: Arc<RepositoryManager>, hw: Arc<HW>) -> Self;
    async fn hello_world_two(&self) -> services::Result<String>;
}

#[derive(Clone, Debug)]
pub struct HelloWorldTwo<HW = HelloWorld>
where
    HW: HelloWorldService,
{
    pool: DBPool,
    repos: Arc<RepositoryManager>,
    hw: Arc<HW>,
}

#[async_trait]
impl<HW> HelloWorldTwoService<HW> for HelloWorldTwo<HW>
where
    HW: HelloWorldService + Sync + Send,
{
    fn new(pool: DBPool, repos: Arc<RepositoryManager>, hw: Arc<HW>) -> Self {
        Self { pool, hw, repos }
    }
    async fn hello_world_two(&self) -> services::Result<String> {
        let res = self.hw.hello_world().await;
        let mut conn = self.pool.get()?;
        let mut user = User {
            id: 100,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            password: "password".to_string(),
            phone: None,
            role_id: 1,
        };
        conn.transaction(|c| self.repos.users.insert_user(c, &mut user))?;
        Ok(format!("{} 2", res))
    }
}
