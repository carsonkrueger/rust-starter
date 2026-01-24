use crate::repositories::DbConn;
use diesel::SelectableHelper;
use diesel::prelude::*;
use diesel::upsert::excluded;
use models::api::search_params::SearchParams;
use models::db::auth::privilege::Privilege;
use schemas::app::auth::privileges;

use diesel_async::RunQueryDsl;
use schemas::app::auth::roles_privileges;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait PrivilegesRepository {
    fn new() -> Self;
    async fn index(&self, db: &mut DbConn, sp: &SearchParams) -> RepositoryResult<Vec<Privilege>>;
    async fn add_many(
        &self,
        db: &mut DbConn,
        privs: &[Privilege],
    ) -> RepositoryResult<Vec<Privilege>>;
    async fn get_by_role(&self, db: &mut DbConn, role_id: i16) -> RepositoryResult<Vec<Privilege>>;
}

#[derive(Debug)]
pub struct Privileges;

impl PrivilegesRepository for Privileges {
    fn new() -> Self {
        Self {}
    }
    async fn index(
        &self,
        db: &mut DbConn,
        params: &SearchParams,
    ) -> RepositoryResult<Vec<Privilege>> {
        trace!("->> index");
        let mut query = privileges::table
            .offset(params.offset())
            .limit(params.limit as i64)
            .into_boxed();
        if let Some(q) = &params.query
            && !q.is_empty()
        {
            query = query.filter(privileges::name.ilike(format!("%{}%", q)));
        }
        Ok(query.load(db).await?)
    }
    async fn add_many(
        &self,
        db: &mut DbConn,
        privs: &[Privilege],
    ) -> RepositoryResult<Vec<Privilege>> {
        let res = diesel::insert_into(privileges::table)
            .values(privs)
            .on_conflict(privileges::name)
            .do_update()
            .set(privileges::name.eq(excluded(privileges::name)))
            .returning(Privilege::as_returning())
            .get_results(db)
            .await?;
        Ok(res)
    }
    async fn get_by_role(&self, db: &mut DbConn, role_id: i16) -> RepositoryResult<Vec<Privilege>> {
        trace!("->> get_by_role");
        let privs = privileges::table
            .select(Privilege::as_select())
            .inner_join(roles_privileges::table)
            .filter(roles_privileges::role_id.eq(role_id))
            .load(db)
            .await?;
        Ok(privs)
    }
}
