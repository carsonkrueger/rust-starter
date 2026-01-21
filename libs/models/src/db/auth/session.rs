use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemas::app;

#[derive(Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = app::auth::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id, token))]
pub struct Session {
    pub user_id: i64,
    pub token: String,
    #[diesel(skip_update)]
    #[diesel(skip_insertion)]
    pub expires_at: Option<NaiveDateTime>,
    #[diesel(skip_insertion)]
    pub created_at: Option<NaiveDateTime>,
}
