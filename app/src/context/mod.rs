use crate::{
    env::config::Config,
    services::{
        ServiceManager,
        hello_world::{HelloWorld, HelloWorldService},
        hello_world_two::{HelloWorldTwo, HelloWorldTwoService},
    },
};

#[derive(Clone, Debug)]
pub struct AppContext<HW = HelloWorld, HW2 = HelloWorldTwo<HW>>
where
    HW: HelloWorldService,
    HW2: HelloWorldTwoService<HW>,
{
    pub cfg: Config,
    pub svc: ServiceManager<HW, HW2>,
}
