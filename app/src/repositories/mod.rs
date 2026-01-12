use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::repositories::{
    privileges::{Privileges, PrivilegesRepository},
    roles::{Roles, RolesRepository},
    roles_privileges::{RolesPrivileges, RolesPrivilegesRepository},
    sessions::{Sessions, SessionsRepository},
    users::{Users, UsersRepository},
};

pub mod privileges;
pub mod roles;
pub mod roles_privileges;
pub mod sessions;
pub mod users;

pub type DbConn = AsyncPgConnection;
pub type DBPool = Pool<AsyncDieselConnectionManager<DbConn>>;

#[allow(unused)]
#[derive(Debug)]
pub struct RepositoryManager<
    UR = Users,
    SR = Sessions,
    PR = Privileges,
    RPR = RolesPrivileges,
    RR = Roles,
> where
    UR: UsersRepository,
    SR: SessionsRepository,
    PR: PrivilegesRepository,
    RPR: RolesPrivilegesRepository,
    RR: RolesRepository,
{
    pub users: UR,
    pub sessions: SR,
    pub privileges: PR,
    pub roles_privileges: RPR,
    pub roles: RR,
}

impl<UR, SR, PR, RPR, RR> RepositoryManager<UR, SR, PR, RPR, RR>
where
    UR: UsersRepository,
    SR: SessionsRepository,
    PR: PrivilegesRepository,
    RPR: RolesPrivilegesRepository,
    RR: RolesRepository,
{
    pub fn default() -> Self {
        let users = UR::new();
        let sessions = SR::new();
        let privileges = PR::new();
        let roles_privileges = RPR::new();
        let roles = RR::new();
        Self {
            users,
            sessions,
            privileges,
            roles_privileges,
            roles,
        }
    }
}

#[allow(unused)]
pub type RepositoryResult<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
}
