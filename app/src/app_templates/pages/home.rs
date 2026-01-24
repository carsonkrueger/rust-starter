use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;
use templates::button::{self, ButtonProps};

pub fn page<'a>() -> templ_ret!['a] {
    templ! {
        #page_layout() {
            <div class="px-8 min-h-screen text-main">
                <div class="flex flex-col justify-center gap-8 pt-[22%]">
                    <p class="text-7xl font-thin">The ultimate kit <br/> for <b class="font-black">your next <br/> passion project.</b></p>
                    <a
                        href="https://github.com/carsonkrueger/rust-starter"
                        target="_blank"
                    >
                        #button::button(ButtonProps::default()) {
                            Follow the Project
                        }
                    </a>
                </div>
            </div>
        }
    }
}
