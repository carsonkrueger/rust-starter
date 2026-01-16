use crate::repositories::DbConn;
use diesel::{BoolExpressionMethods, ExpressionMethods, SelectableHelper};
use models::db::auth::role_privilege::RolePrivilege;

use diesel_async::RunQueryDsl;
use schemas::auth::roles_privileges;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait RolesPrivilegesRepository {
    fn new() -> Self;
    async fn add_many(
        &self,
        db: &mut DbConn,
        role_privs: &[RolePrivilege],
    ) -> RepositoryResult<Vec<RolePrivilege>>;
    async fn delete(
        &self,
        db: &mut DbConn,
        role_id: i16,
        privilege_id: i64,
    ) -> RepositoryResult<()>;
}

#[derive(Debug)]
pub struct RolesPrivileges;

impl RolesPrivilegesRepository for RolesPrivileges {
    fn new() -> Self {
        Self {}
    }
    async fn add_many(
        &self,
        db: &mut DbConn,
        role_privs: &[RolePrivilege],
    ) -> RepositoryResult<Vec<RolePrivilege>> {
        trace!("->> add_many");
        let res = diesel::insert_into(roles_privileges::table)
            .values(role_privs)
            .on_conflict((roles_privileges::role_id, roles_privileges::privilege_id))
            .do_nothing()
            .returning(RolePrivilege::as_returning())
            .get_results(db)
            .await?;
        Ok(res)
    }
    async fn delete(
        &self,
        db: &mut DbConn,
        role_id: i16,
        privilege_id: i64,
    ) -> RepositoryResult<()> {
        trace!("->> delete");
        diesel::delete(roles_privileges::table)
            .filter(
                roles_privileges::role_id
                    .eq(role_id)
                    .and(roles_privileges::privilege_id.eq(privilege_id)),
            )
            .execute(db)
            .await?;
        Ok(())
    }
}
