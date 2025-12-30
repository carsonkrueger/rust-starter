use crate::{
    env::config::Config,
    services::{
        ServiceManager,
        auth::{Auth, AuthService},
        hello_world::{HelloWorld, HelloWorldService},
        privileges::{Privileges, PrivilegesService},
    },
};

#[derive(Clone, Debug)]
pub struct AppState<HW = HelloWorld, AT = Auth, PS = Privileges>
where
    HW: HelloWorldService,
    AT: AuthService,
    PS: PrivilegesService,
{
    pub cfg: Config,
    pub svc: ServiceManager<HW, AT, PS>,
}
