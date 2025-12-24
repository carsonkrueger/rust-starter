use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::repositories::{
    sessions::{Sessions, SessionsRepository},
    users::{Users, UsersRepository},
};

pub mod sessions;
pub mod users;

pub type DbConn = AsyncPgConnection;
pub type DBPool = Pool<AsyncDieselConnectionManager<DbConn>>;

#[allow(unused)]
#[derive(Debug)]
pub struct RepositoryManager<UR = Users, SR = Sessions>
where
    UR: UsersRepository,
    SR: SessionsRepository,
{
    pub users: UR,
    pub sessions: SR,
}

impl<UR, SR> RepositoryManager<UR, SR>
where
    UR: UsersRepository,
    SR: SessionsRepository,
{
    pub fn default() -> Self {
        let users = UR::new();
        let sessions = SR::new();
        Self { users, sessions }
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
