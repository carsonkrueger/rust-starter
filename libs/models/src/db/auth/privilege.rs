use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemas::app;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = app::auth::privileges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Privilege {
    #[diesel(skip_update)]
    #[diesel(skip_insertion)]
    pub id: i64,
    pub name: String,
    #[diesel(skip_insertion)]
    #[diesel(skip_update)]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(skip_insertion)]
    pub updated_at: Option<NaiveDateTime>,
}
