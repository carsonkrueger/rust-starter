use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;
use templates::button;

pub fn page<'a>() -> templ_ret!['a] {
    templ! {
        #page_layout() {
            Home!
            #button::button(None) {
                Hello World!
            }
        }
    }
}
