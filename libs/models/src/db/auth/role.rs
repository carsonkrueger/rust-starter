use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schemas::auth::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    #[diesel(skip_update)]
    #[diesel(skip_insertion)]
    pub id: i16,
    pub name: String,
    #[diesel(skip_update)]
    #[diesel(skip_insertion)]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(skip_insertion)]
    pub updated_at: Option<NaiveDateTime>,
}
