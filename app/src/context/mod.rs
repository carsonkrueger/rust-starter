use crate::{
    env::config::Config,
    services::{ServiceManager, hello_world::HelloWorld},
};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub cfg: Config,
    pub svc: ServiceManager<HelloWorld>,
}
