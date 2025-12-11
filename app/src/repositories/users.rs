use crate::{models::user::*, schema::auth::users};
use diesel::SelectableHelper;
use diesel::prelude::*;

use crate::repositories::{DBConnection, Result};

pub trait UsersRepository {
    fn new() -> Self;
    fn insert_user(&self, _db: &mut DBConnection, user: &mut User) -> Result<()>;
}

#[derive(Debug)]
pub struct Users;

impl UsersRepository for Users {
    fn new() -> Self {
        Self {}
    }
    fn insert_user(&self, conn: &mut DBConnection, user: &mut User) -> Result<()> {
        *user = diesel::insert_into(users::table)
            .values(&*user)
            .returning(User::as_returning())
            .get_result(conn)?;
        Ok(())
    }
}
