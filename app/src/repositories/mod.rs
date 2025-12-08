use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

pub mod users;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct RepositoryManager {}
