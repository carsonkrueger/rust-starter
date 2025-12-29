use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::repositories::{
    privileges::{Privileges, PrivilegesRepository},
    roles_privileges::{RolesPrivileges, RolesPrivilegesRepository},
    sessions::{Sessions, SessionsRepository},
    users::{Users, UsersRepository},
};

pub mod privileges;
pub mod roles_privileges;
pub mod sessions;
pub mod users;

pub type DbConn = AsyncPgConnection;
pub type DBPool = Pool<AsyncDieselConnectionManager<DbConn>>;

#[allow(unused)]
#[derive(Debug)]
pub struct RepositoryManager<UR = Users, SR = Sessions, PR = Privileges, RPR = RolesPrivileges>
where
    UR: UsersRepository,
    SR: SessionsRepository,
    PR: PrivilegesRepository,
    RPR: RolesPrivilegesRepository,
{
    pub users: UR,
    pub sessions: SR,
    pub privileges: PR,
    pub roles_privileges: RPR,
}

impl<UR, SR, PR, RPR> RepositoryManager<UR, SR, PR, RPR>
where
    UR: UsersRepository,
    SR: SessionsRepository,
    PR: PrivilegesRepository,
    RPR: RolesPrivilegesRepository,
{
    pub fn default() -> Self {
        let users = UR::new();
        let sessions = SR::new();
        let privileges = PR::new();
        let roles_privileges = RPR::new();
        Self {
            users,
            sessions,
            privileges,
            roles_privileges,
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
