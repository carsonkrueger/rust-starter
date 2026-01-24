use std::sync::Arc;

use chrono::Utc;
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use models::{
    api::search_params::SearchParams,
    db::auth::role_privilege::{RolePrivilege, RolePrivilegeJoin},
};
use tracing::trace;
// use utils::auth::privileges::Privilege;

use crate::{
    repositories::{
        self, DBPool, RepositoryManager, privileges::PrivilegesRepository, roles::RolesRepository,
        roles_privileges::RolesPrivilegesRepository,
    },
    services::ServiceResult,
};

pub trait PrivilegesService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn create_role_privilege(&self, role_id: i16, privilege_id: i64) -> ServiceResult<()>;
    async fn associate_auth(
        &self,
        role_id: i16,
        privileges: &[utils::auth::privileges::Privilege],
    ) -> ServiceResult<Vec<RolePrivilege>>;
    async fn list_roles_privileges<SP: Into<SearchParams>>(
        &self,
        search_params: SP,
    ) -> ServiceResult<Vec<RolePrivilegeJoin>>;
    async fn one_role_privilege(
        &self,
        role_id: i16,
        privilege_id: i64,
    ) -> ServiceResult<Option<RolePrivilegeJoin>>;
    async fn disassociate_auth(
        &self,
        role_id: i16,
        privilege_id: i64,
    ) -> ServiceResult<Option<RolePrivilege>>;
    async fn privileges(
        &self,
        params: &SearchParams,
    ) -> ServiceResult<Vec<models::db::auth::privilege::Privilege>>;
    async fn roles(
        &self,
        params: &SearchParams,
    ) -> ServiceResult<Vec<models::db::auth::role::Role>>;
}

#[derive(Debug, Clone)]
pub struct Privileges {
    pool: DBPool,
    repos: Arc<RepositoryManager>,
}

impl PrivilegesService for Privileges {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self {
        Self { pool, repos }
    }
    async fn create_role_privilege(&self, role_id: i16, privilege_id: i64) -> ServiceResult<()> {
        trace!("->> create_role_privilege");

        let utc = Utc::now();
        let now = utc.naive_utc();

        let role_privilege = RolePrivilege {
            role_id,
            privilege_id,
            created_at: Some(now),
        };

        let mut db = self.pool.get().await?;
        self.repos
            .roles_privileges
            .add_many(&mut db, &[role_privilege])
            .await?;

        Ok(())
    }
    async fn associate_auth(
        &self,
        role_id: i16,
        privileges: &[utils::auth::privileges::Privilege],
    ) -> ServiceResult<Vec<RolePrivilege>> {
        trace!("->> associate");

        let mut db_privileges: Vec<models::db::auth::privilege::Privilege> =
            Vec::with_capacity(privileges.len());
        for p in privileges {
            db_privileges.push(models::db::auth::privilege::Privilege {
                id: 0,
                name: Into::<&'static str>::into(p).to_string(),
                created_at: None,
                updated_at: None,
            });
        }

        let mut db = self.pool.get().await?;
        let role_privileges = db
            .transaction::<'_, '_, _, repositories::Error, _>(|mut db| {
                async move {
                    let privileges = self
                        .repos
                        .privileges
                        .add_many(&mut db, &db_privileges)
                        .await?;
                    let mut role_privileges: Vec<RolePrivilege> =
                        Vec::with_capacity(privileges.len());
                    for p in privileges {
                        role_privileges.push(RolePrivilege {
                            role_id,
                            privilege_id: p.id,
                            created_at: None,
                        });
                    }
                    let role_privileges = self
                        .repos
                        .roles_privileges
                        .add_many(&mut db, &role_privileges)
                        .await?;

                    Ok(role_privileges)
                }
                .scope_boxed()
            })
            .await?;

        Ok(role_privileges)
    }
    async fn list_roles_privileges<SP: Into<SearchParams>>(
        &self,
        search_params: SP,
    ) -> ServiceResult<Vec<RolePrivilegeJoin>> {
        let mut db = self.pool.get().await?;
        let roles_privileges = self
            .repos
            .roles_privileges
            .join_list(&mut db, &search_params.into())
            .await?;
        Ok(roles_privileges)
    }
    async fn disassociate_auth(
        &self,
        role_id: i16,
        privilege_id: i64,
    ) -> ServiceResult<Option<RolePrivilege>> {
        let mut db = self.pool.get().await?;
        let row = self
            .repos
            .roles_privileges
            .delete(&mut db, role_id, privilege_id)
            .await?;
        Ok(row)
    }
    async fn privileges(
        &self,
        params: &SearchParams,
    ) -> ServiceResult<Vec<models::db::auth::privilege::Privilege>> {
        let mut db = self.pool.get().await?;
        let privileges = self.repos.privileges.index(&mut db, params).await?;
        Ok(privileges)
    }
    async fn roles(
        &self,
        params: &SearchParams,
    ) -> ServiceResult<Vec<models::db::auth::role::Role>> {
        let mut db = self.pool.get().await?;
        let roles = self.repos.roles.index(&mut db, params).await?;
        Ok(roles)
    }
    async fn one_role_privilege(
        &self,
        role_id: i16,
        privilege_id: i64,
    ) -> ServiceResult<Option<RolePrivilegeJoin>> {
        let mut db = self.pool.get().await?;
        let row = self
            .repos
            .roles_privileges
            .join_one(&mut db, role_id, privilege_id)
            .await?;
        Ok(row)
    }
}
