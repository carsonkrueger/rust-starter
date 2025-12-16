use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::auth::roles_privileges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RolePrivilege {
    pub role_id: i16,
    pub privilege_id: i64,
    pub created_at: Option<NaiveDateTime>,
}
