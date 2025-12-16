use crate::repositories::{DbConn, Repository};
use crate::{models::auth::user::User, schema::auth::users};
use diesel::SelectableHelper;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::repositories::RepositoryResult;

pub trait UsersRepository: Repository<User, i64> {}

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

impl UsersRepository for Users {}
