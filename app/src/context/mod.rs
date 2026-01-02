use crate::{
    env::config::Config,
    services::{
        ServiceManager,
        auth::{Auth, AuthService},
        privileges::{Privileges, PrivilegesService},
        users::{Users, UsersService},
    },
};

pub mod datastar;

#[derive(Clone, Debug)]
pub struct AppState<AT = Auth, PS = Privileges, US = Users>
where
    AT: AuthService,
    PS: PrivilegesService,
    US: UsersService,
{
    pub cfg: Config,
    pub svc: ServiceManager<AT, PS, US>,
}
