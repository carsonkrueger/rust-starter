use chrono::NaiveDateTime;
use datastar::templates::table::IntoTableData;
use diesel::prelude::*;
use serde::Serialize;
use templates::table::{TdProps, ThProps, td, th};
use templr::{templ, templ_ret};

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

impl IntoTableData for User {
    const ENDPOINT: &'static str = "/management/users/rows";
    const TABLE_ID: &'static str = "usersTable";
    const TABLE_BODY_ID: &'static str = "usersTableBody";
    fn thead_row<'a>() -> templ_ret!['a] {
        templ! {
            #th(ThProps::default()) {
                ID
            }
            #th(ThProps::default()) {
                Email
            }
            #th(ThProps::default()) {
                First Name
            }
            #th(ThProps::default()) {
                Last Name
            }
            #th(ThProps::default()) {
                Role
            }
        }
    }
    fn table_data<'a>(&'a self) -> templ_ret!['a] {
        templ! {
            #td(TdProps::default()) {
                {self.id}
            }
            #td(TdProps::default()) {
                {self.email}
            }
            #td(TdProps::default()) {
                {self.first_name}
            }
            #td(TdProps::default()) {
                {self.last_name}
            }
            #td(TdProps::default()) {
                {self.role_id}
            }
        }
    }
    fn row_id(&self) -> String {
        format!("user{}", self.id)
    }
}
