use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

use crate::repositories::users::{Users, UsersRepository};

pub mod users;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub struct RepositoryManager<UR = Users>
where
    UR: UsersRepository,
{
    pub users: UR,
}

impl<UR> RepositoryManager<UR>
where
    UR: UsersRepository,
{
    pub fn default() -> Self {
        let users = UR::new();
        Self { users }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
}
