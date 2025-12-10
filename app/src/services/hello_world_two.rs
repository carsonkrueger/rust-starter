use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    repositories::{DBPool, RepositoryManager, users::UsersRepository},
    services::hello_world::{HelloWorld, HelloWorldService},
};

#[async_trait]
pub trait HelloWorldTwoService<HW>
where
    HW: HelloWorldService,
{
    fn new(pool: DBPool, repos: Arc<RepositoryManager>, hw: Arc<HW>) -> Self;
    async fn hello_world_two(&self) -> String;
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
    async fn hello_world_two(&self) -> String {
        let res = self.hw.hello_world().await;
        let user = self.repos.users.get_user(self.pool.clone());
        format!("{} 2", res)
    }
}
