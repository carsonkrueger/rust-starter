use templr::{templ, templ_ret};

pub mod home;

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
