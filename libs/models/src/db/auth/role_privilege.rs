use chrono::NaiveDateTime;
use datastar::templates::table::IntoTableData;
use diesel::prelude::*;
use templates::{
    button::{self, ButtonProps},
    icon::{self, IconProps, icon_data::Icon},
    table::{TdProps, ThProps, td, th},
};
use templr::templ;

use crate::db::auth::{privilege::Privilege, role::Role};

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

pub struct RolePrivilegeJoin(pub Role, pub Privilege);

impl From<(Role, Privilege)> for RolePrivilegeJoin {
    fn from((role, privilege): (Role, Privilege)) -> Self {
        RolePrivilegeJoin(role, privilege)
    }
}

impl IntoTableData for RolePrivilegeJoin {
    const ENDPOINT: &'static str = "/management/roles_privileges/rows";
    const TABLE_ID: &'static str = "rolePrivilegeTable";
    const TABLE_BODY_ID: &'static str = "rolePrivilegeTableBody";
    fn thead_row<'a>() -> templr::templ_ret!['a] {
        templ! {
            #th(ThProps::default()) {
                Role
            }
            #th(ThProps::default()) {
                Privilege
            }
            #th(ThProps::default());
        }
    }
    fn table_data<'a>(&'a self) -> templr::templ_ret!['a] {
        templ! {
            #td(TdProps::default()) {
                {self.0.name}
            }
            #td(TdProps::default()) {
                {self.1.name}
            }
            #td(TdProps::default()) {
                <div class="flex items-center">
                    #button::button(ButtonProps{
                        variant: button::Variant::Transparent,
                        attrs: &[("data-on:click", &format!("@delete('/management/roles/{}/privileges/{}')", self.0.id, self.1.id))],
                        ..Default::default()
                    }) {
                        #icon::icon(IconProps{
                            icon: Icon::Trash2,
                            ..Default::default()
                        });
                    }
                </div>
            }
        }
    }
}
