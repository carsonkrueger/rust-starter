use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::repositories::users::{Users, UsersRepository};

pub mod users;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

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
