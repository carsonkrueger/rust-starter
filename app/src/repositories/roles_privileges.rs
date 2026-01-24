use crate::repositories::DbConn;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper, TextExpressionMethods,
};
use models::{
    api::search_params::SearchParams,
    db::auth::{
        privilege::Privilege,
        role::Role,
        role_privilege::{RolePrivilege, RolePrivilegeJoin},
    },
};

use diesel_async::RunQueryDsl;
use schemas::app::auth::{privileges, roles, roles_privileges};
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
    ) -> RepositoryResult<Option<RolePrivilege>>;
    async fn join_list(
        &self,
        db: &mut DbConn,
        params: &SearchParams,
    ) -> RepositoryResult<Vec<RolePrivilegeJoin>>;
    async fn join_one(
        &self,
        db: &mut DbConn,
        role_id: i16,
        privilege_id: i64,
    ) -> RepositoryResult<Option<RolePrivilegeJoin>>;
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
    ) -> RepositoryResult<Option<RolePrivilege>> {
        trace!("->> delete");
        let res = diesel::delete(roles_privileges::table)
            .filter(
                roles_privileges::role_id
                    .eq(role_id)
                    .and(roles_privileges::privilege_id.eq(privilege_id)),
            )
            .returning(RolePrivilege::as_returning())
            .get_result(db)
            .await;
        if let Err(diesel::result::Error::NotFound) = res {
            return Ok(None);
        }
        Ok(Some(res?))
    }
    async fn join_list(
        &self,
        db: &mut DbConn,
        params: &SearchParams,
    ) -> RepositoryResult<Vec<RolePrivilegeJoin>> {
        trace!("->> join_list");
        let mut sql = roles_privileges::table
            .inner_join(privileges::table)
            .inner_join(roles::table)
            .select((Role::as_select(), Privilege::as_select()))
            .into_boxed();
        if let Some(query) = &params.query {
            let query = format!("%{}%", query);
            sql = sql.filter(
                roles::name
                    .like(query.clone())
                    .or(privileges::name.like(query)),
            );
        }
        let joins = sql.load::<(Role, Privilege)>(db).await?;
        Ok(joins.into_iter().map(|t| t.into()).collect())
    }
    async fn join_one(
        &self,
        db: &mut DbConn,
        role_id: i16,
        privilege_id: i64,
    ) -> RepositoryResult<Option<RolePrivilegeJoin>> {
        trace!("->> join_one");
        let join = roles_privileges::table
            .inner_join(privileges::table)
            .inner_join(roles::table)
            .select((Role::as_select(), Privilege::as_select()))
            .filter(
                roles_privileges::role_id
                    .eq(role_id)
                    .and(roles_privileges::privilege_id.eq(privilege_id)),
            )
            .first::<(Role, Privilege)>(db)
            .await;
        // handle if none is found
        let res = match join {
            Ok(r) => Some(r.into()),
            Err(diesel::result::Error::NotFound) => None,
            e => Some(e?.into()),
        };
        Ok(res)
    }
}
