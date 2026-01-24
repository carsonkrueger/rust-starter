use crate::repositories::DbConn;
use diesel::prelude::*;
use models::api::search_params::SearchParams;
use models::db::auth::role::Role;

use diesel_async::RunQueryDsl;
use schemas::app::auth::roles;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait RolesRepository {
    fn new() -> Self;
    async fn index(&self, db: &mut DbConn, params: &SearchParams) -> RepositoryResult<Vec<Role>>;
}

#[derive(Debug)]
pub struct Roles;

impl RolesRepository for Roles {
    fn new() -> Self {
        Self {}
    }
    async fn index(&self, db: &mut DbConn, params: &SearchParams) -> RepositoryResult<Vec<Role>> {
        trace!("->> index");
        let mut query = roles::table
            .offset(params.offset())
            .limit(params.limit as i64)
            .into_boxed();
        if let Some(q) = &params.query
            && !q.is_empty()
        {
            query = query.filter(roles::name.ilike(format!("%{}%", q)));
        }
        Ok(query.load(db).await?)
    }
}
