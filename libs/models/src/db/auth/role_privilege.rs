use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schemas::auth::roles_privileges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RolePrivilege {
    #[diesel(skip_update)]
    pub role_id: i16,
    #[diesel(skip_update)]
    pub privilege_id: i64,
    #[diesel(skip_insertion)]
    #[diesel(skip_update)]
    pub created_at: Option<NaiveDateTime>,
}
