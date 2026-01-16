use crate::repositories::DbConn;
use diesel::SelectableHelper;
use diesel::prelude::*;
use models::db::auth::privilege::Privilege;
use models::db::auth::role::Role;

use diesel_async::RunQueryDsl;
use models::db::auth::role_privilege::RolePrivilegeJoin;
use schemas::auth::privileges;
use schemas::auth::roles;
use schemas::auth::roles_privileges;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait RolesRepository {
    fn new() -> Self;
    async fn join_list(&self, db: &mut DbConn) -> RepositoryResult<Vec<RolePrivilegeJoin>>;
}

#[derive(Debug)]
pub struct Roles;

impl RolesRepository for Roles {
    fn new() -> Self {
        Self {}
    }
    async fn join_list(&self, db: &mut DbConn) -> RepositoryResult<Vec<RolePrivilegeJoin>> {
        trace!("->> join_list");
        let roles = roles_privileges::table
            .inner_join(privileges::table)
            .inner_join(roles::table)
            .select((Role::as_select(), Privilege::as_select()))
            .load::<(Role, Privilege)>(db)
            .await?;
        Ok(roles.into_iter().map(|t| t.into()).collect())
    }
}
