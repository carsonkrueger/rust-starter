use crate::repositories::DbConn;
use diesel::SelectableHelper;
use diesel::prelude::*;
use models::db::auth::session::Session;
use models::db::auth::user::User;
use schemas::app::auth::{sessions, users};

use diesel_async::RunQueryDsl;
use tracing::trace;

use crate::repositories::RepositoryResult;

#[allow(unused)]
pub trait SessionsRepository {
    fn new() -> Self;
    async fn insert(&self, db: &mut DbConn, session: &mut Session) -> RepositoryResult<()>;
    async fn get_one(&self, db: &mut DbConn, pk: (i64, String)) -> RepositoryResult<Session>;
    async fn get_user(&self, db: &mut DbConn, user_id: i64, token: &str) -> RepositoryResult<User>;
}

#[derive(Debug)]
pub struct Sessions;

impl SessionsRepository for Sessions {
    fn new() -> Self {
        Self {}
    }
    async fn insert(&self, db: &mut DbConn, session: &mut Session) -> RepositoryResult<()> {
        trace!("->> insert");
        *session = diesel::insert_into(sessions::table)
            .values(&*session)
            .returning(Session::as_returning())
            .get_result(db)
            .await?;
        Ok(())
    }
    async fn get_one(&self, db: &mut DbConn, pk: (i64, String)) -> RepositoryResult<Session> {
        trace!("->> get_one");
        Ok(sessions::table.find(pk).first(db).await?)
    }
    async fn get_user(&self, db: &mut DbConn, user_id: i64, token: &str) -> RepositoryResult<User> {
        trace!("->> get_user");
        Ok(users::table
            .select(User::as_select())
            .inner_join(sessions::table)
            .filter(sessions::token.eq(token))
            .filter(sessions::user_id.eq(user_id))
            .first(db)
            .await?)
    }
}
