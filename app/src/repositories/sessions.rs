use crate::repositories::{DbConn, Repository};
use diesel::SelectableHelper;
use diesel::prelude::*;
use models::db::auth::session::Session;
use models::db::auth::user::User;
use schemas::auth::{sessions, users};

use diesel_async::RunQueryDsl;

use crate::repositories::RepositoryResult;

pub trait SessionsRepository: Repository<Session, (i64, String)> {
    async fn get_user(&self, db: &mut DbConn, user_id: i64, token: &str) -> RepositoryResult<User>;
}

#[derive(Debug)]
pub struct Sessions;

impl Repository<Session, (i64, String)> for Sessions {
    fn new() -> Self {
        Self {}
    }
    async fn insert(&self, db: &mut DbConn, session: &mut Session) -> RepositoryResult<()> {
        *session = diesel::insert_into(sessions::table)
            .values(&*session)
            .returning(Session::as_returning())
            .get_result(db)
            .await?;
        Ok(())
    }
    async fn get_one(&self, db: &mut DbConn, pk: (i64, String)) -> RepositoryResult<Session> {
        Ok(sessions::table.find(pk).first(db).await?)
    }
}

impl SessionsRepository for Sessions {
    async fn get_user(&self, db: &mut DbConn, user_id: i64, token: &str) -> RepositoryResult<User> {
        Ok(users::table
            .select(User::as_select())
            .inner_join(sessions::table)
            .filter(sessions::token.eq(token))
            .filter(sessions::user_id.eq(user_id))
            .first(db)
            .await?)
    }
}
