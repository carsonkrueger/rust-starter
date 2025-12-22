use crate::repositories::{DbConn, Repository};
use diesel::SelectableHelper;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use models::db::auth::user::User;
use schemas::auth::users;

use crate::repositories::RepositoryResult;

pub trait UsersRepository: Repository<User, i64> {
    async fn get_by_email(&self, db: &mut DbConn, email: &str) -> RepositoryResult<User>;
}

#[derive(Debug)]
pub struct Users;

impl Repository<User, i64> for Users {
    fn new() -> Self {
        Self {}
    }
    async fn insert(&self, db: &mut DbConn, user: &mut User) -> RepositoryResult<()> {
        *user = diesel::insert_into(users::table)
            .values(&*user)
            .returning(User::as_returning())
            .get_result(db)
            .await?;
        Ok(())
    }
    async fn get_one(&self, db: &mut DbConn, pk: i64) -> RepositoryResult<User> {
        Ok(users::table.find(pk).first(db).await?)
    }
}

impl UsersRepository for Users {
    async fn get_by_email(&self, db: &mut DbConn, email: &str) -> RepositoryResult<User> {
        Ok(users::table
            .filter(users::email.eq(email))
            .first(db)
            .await?)
    }
}
