use crate::repositories::DbConn;
use diesel::SelectableHelper;
use diesel::prelude::*;
use models::db::auth::role::Role;

use diesel_async::RunQueryDsl;
use schemas::auth::roles;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait RolesRepository {
    fn new() -> Self;
    async fn list(&self, db: &mut DbConn) -> RepositoryResult<Vec<Role>>;
}

#[derive(Debug)]
pub struct Roles;

impl RolesRepository for Roles {
    fn new() -> Self {
        Self {}
    }
    async fn list(&self, db: &mut DbConn) -> RepositoryResult<Vec<Role>> {
        trace!("->> list");
        let roles = roles::table.select(Role::as_select()).load(db).await?;
        Ok(roles)
    }
}
