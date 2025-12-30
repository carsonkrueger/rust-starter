use std::sync::Arc;

use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use models::db::auth::role_privilege::RolePrivilege;
use tracing::trace;
use utils::auth::privileges::Privilege;

use crate::{
    repositories::{
        self, DBPool, RepositoryManager, privileges::PrivilegesRepository,
        roles_privileges::RolesPrivilegesRepository,
    },
    services::ServiceResult,
};

pub trait PrivilegesService {
    fn new(pool: DBPool, repos: Arc<RepositoryManager>) -> Self;
    async fn associate(
        &self,
        role_id: i16,
        privileges: &[Privilege],
    ) -> ServiceResult<Vec<RolePrivilege>>;
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
    async fn associate(
        &self,
        role_id: i16,
        privileges: &[Privilege],
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
}
