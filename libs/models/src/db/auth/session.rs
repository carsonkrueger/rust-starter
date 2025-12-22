use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = schemas::auth::sessions)]
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

impl Default for Session {
    fn default() -> Self {
        Self {
            user_id: 0,
            token: String::new(),
            expires_at: None,
            created_at: None,
        }
    }
}
