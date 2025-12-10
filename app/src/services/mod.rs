use std::sync::Arc;

use crate::{
    repositories::{DBPool, RepositoryManager},
    services::{hello_world::HelloWorldService, hello_world_two::HelloWorldTwoService},
};

pub mod hello_world;
pub mod hello_world_two;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct ServiceManager<HW: HelloWorldService, HW2: HelloWorldTwoService<HW>> {
    pub hello_world: Arc<HW>,
    pub hello_world_two: Arc<HW2>,
}

impl<HW, HW2> ServiceManager<HW, HW2>
where
    HW: HelloWorldService,
    HW2: HelloWorldTwoService<HW>,
{
    pub fn default(pool: DBPool, repos: RepositoryManager) -> Self {
        let repos = Arc::new(repos);
        let hw = Arc::new(HW::new());
        let hw2 = Arc::new(HW2::new(pool, repos, hw.clone()));
        Self {
            hello_world: hw,
            hello_world_two: hw2,
        }
    }
}
