use datastar::templates::table::DatastarTableProps;
use models::db::auth::user::User;
use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;

pub fn page<'a>() -> templ_ret!['a] {
    templ! {
        #page_layout() {
            <div class="min-h-screen text-main flex flex-col items-center">
                #datastar::templates::table::datastar_table::<User>(DatastarTableProps::default());
            </div>
        }
    }
}
