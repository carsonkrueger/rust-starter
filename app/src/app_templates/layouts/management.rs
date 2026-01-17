use templates::tabs::{self, TabGroupProps, TabProps};
use templr::{templ, templ_ret};

use crate::app_templates::layouts::main;

pub fn management<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        #main::main() {
            #tabs::tab_group(TabGroupProps::default()) {
                <a href="/management/users">
                    #tabs::tab(TabProps{
                        ..Default::default()
                    }) {
                        Users
                    }
                </a>
                <a href="/management/roles_privileges">
                    #tabs::tab(TabProps{
                        ..Default::default()
                    }) {
                        Roles
                    }
                </a>
            }
            #children;
        }
    }
}
