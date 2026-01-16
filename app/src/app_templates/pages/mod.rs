use templr::{templ, templ_ret};

pub mod home;
pub mod login;
pub mod management_roles_privileges;
pub mod management_users;
pub mod sign_up;

const PAGE_LAYOUT_ID: &'static str = "page-layout";

pub fn page_layout<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <main
            id={PAGE_LAYOUT_ID}
            class="min-h-screen"
        >
            #children;
        </main>
    }
}
