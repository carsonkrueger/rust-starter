use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Clone, Debug, Serialize)]
#[diesel(table_name = schemas::auth::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[diesel(skip_update)]
    #[diesel(skip_insertion)]
    pub id: i64,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i16,
    #[diesel(skip_update)]
    #[diesel(skip_insertion)]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(skip_insertion)]
    pub updated_at: Option<NaiveDateTime>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            email: String::new(),
            password: String::new(),
            phone: None,
            first_name: String::new(),
            last_name: String::new(),
            role_id: 0,
            created_at: None,
            updated_at: None,
        }
    }
}
