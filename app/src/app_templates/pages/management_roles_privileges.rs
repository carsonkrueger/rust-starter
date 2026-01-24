use datastar::templates::table::DatastarTableProps;
use models::db::auth::{privilege::Privilege, role::Role, role_privilege::RolePrivilegeJoin};
use templates::{
    button::{ButtonProps, button},
    select::{self, SelectProps},
};
use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;

pub fn page<'a>(roles: &'a [Role], privileges: &'a [Privilege]) -> templ_ret!['a] {
    templ! {
        #let privilege_options = privileges.iter().map(|p| (p.name.clone(), p.id.to_string())).collect::<Vec<_>>();
        #let role_options = roles.iter().map(|r| (r.name.clone(), r.id.to_string())).collect::<Vec<_>>();
        #page_layout() {
            <div class="min-h-screen text-main flex flex-col items-center">
                #datastar::templates::table::datastar_table::<RolePrivilegeJoin>(DatastarTableProps::default());
                <form class="fixed bottom-0 flex items-center gap-3 py-2">
                    #select::select(SelectProps{
                        options: role_options.as_slice(),
                        name: "role_id",
                        attrs: &[("data-bind", "role")],
                        ..Default::default()
                    });
                    #select::select(SelectProps{
                        options: privilege_options.as_slice(),
                        name: "privilege_id",
                        ..Default::default()
                    });
                    #button(ButtonProps {
                        attrs: &[("data-on:click", "@post('/management/roles/privileges', { contentType: 'form' })")],
                        btn_type: templates::button::ButtonType::Submit,
                        ..Default::default()
                    }) {
                        Associate
                    }
                </form>
            </div>
        }
    }
}
