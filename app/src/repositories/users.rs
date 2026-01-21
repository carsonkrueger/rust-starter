use crate::repositories::DbConn;
use diesel::SelectableHelper;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use models::api::search_params::SearchParams;
use models::db::auth::user::User;
use schemas::app::auth::users;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait UsersRepository {
    fn new() -> Self;
    async fn insert(&self, db: &mut DbConn, user: &mut User) -> RepositoryResult<()>;
    async fn get_one(&self, db: &mut DbConn, pk: i64) -> RepositoryResult<User>;
    async fn get_by_email(&self, db: &mut DbConn, email: &str) -> RepositoryResult<User>;
    async fn index(&self, db: &mut DbConn, params: &SearchParams) -> RepositoryResult<Vec<User>>;
}

#[derive(Debug)]
pub struct Users;

impl UsersRepository for Users {
    fn new() -> Self {
        Self {}
    }
    async fn insert(&self, db: &mut DbConn, user: &mut User) -> RepositoryResult<()> {
        trace!("->> insert");
        *user = diesel::insert_into(users::table)
            .values(&*user)
            .returning(User::as_returning())
            .get_result(db)
            .await?;
        Ok(())
    }
    async fn get_one(&self, db: &mut DbConn, pk: i64) -> RepositoryResult<User> {
        trace!("->> get_one");
        Ok(users::table.find(pk).first(db).await?)
    }
    async fn get_by_email(&self, db: &mut DbConn, email: &str) -> RepositoryResult<User> {
        trace!("->> get_by_email");
        Ok(users::table
            .filter(users::email.eq(email))
            .first(db)
            .await?)
    }
    async fn index(&self, db: &mut DbConn, params: &SearchParams) -> RepositoryResult<Vec<User>> {
        trace!("->> index");
        let mut query = users::table
            .offset(params.offset())
            .limit(params.limit as i64)
            .into_boxed();
        if let Some(q) = &params.query
            && !q.is_empty()
        {
            query = query.filter(
                users::email.ilike(format!("%{}%", q)).or(users::first_name
                    .ilike(format!("%{}%", q))
                    .or(users::last_name.ilike(format!("%{}%", q)))),
            );
        }
        Ok(query.load(db).await?)
    }
}
