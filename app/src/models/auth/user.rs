use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::auth::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i16,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
