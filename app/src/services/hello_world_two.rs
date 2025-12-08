use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    repositories::DBPool,
    services::hello_world::{HelloWorld, HelloWorldService},
};

#[async_trait]
pub trait HelloWorldTwoService<HW>
where
    HW: HelloWorldService,
{
    fn new(pool: DBPool, hw: Arc<HW>) -> Self;
    async fn hello_world_two(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct HelloWorldTwo<HW = HelloWorld>
where
    HW: HelloWorldService,
{
    pool: DBPool,
    hw: Arc<HW>,
}

#[async_trait]
impl<HW> HelloWorldTwoService<HW> for HelloWorldTwo<HW>
where
    HW: HelloWorldService + Sync + Send,
{
    fn new(pool: DBPool, hw: Arc<HW>) -> Self {
        Self { pool, hw }
    }
    async fn hello_world_two(&self) -> String {
        let res = self.hw.hello_world().await;
        format!("{} 2", res)
    }
}
