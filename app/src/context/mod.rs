use crate::{
    env::config::Config,
    services::{
        ServiceManager,
        auth::{Auth, AuthService},
        hello_world::{HelloWorld, HelloWorldService},
    },
};

#[derive(Clone, Debug)]
pub struct AppState<HW = HelloWorld, AT = Auth>
where
    HW: HelloWorldService,
    AT: AuthService,
{
    pub cfg: Config,
    pub svc: ServiceManager<HW, AT>,
}
