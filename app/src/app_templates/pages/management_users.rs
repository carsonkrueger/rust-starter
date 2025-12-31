use models::db::auth::user::User;
use templr::{templ, templ_ret};

use crate::app_templates::{pages::page_layout, tables};

pub fn page<'a>(users: &'a [User]) -> templ_ret!['a] {
    templ! {
        #page_layout() {
            <div class="min-h-screen text-main flex flex-col items-center">
                #tables::management::management_users_table("users", users);
            </div>
        }
    }
}
