use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;
use templates::button;

pub fn page<'a>() -> templ_ret!['a] {
    let show_it = true;
    templ! {
        #page_layout() {
            <div data-signals:show-it={ show_it }/>
            Home!
            <div data-on:click="alert('omega')">click me</div>
            #button::button(button::ButtonProps {
                attrs: &[("data-on:click", "$showIt = false"), ("data-show", "$showIt")],
                ..Default::default()
            }) {
                Hello World!
            }
            #button::button(button::ButtonProps {
                attrs: &[("data-on:click", "$showIt = true"), ("data-show", "!$showIt")],
                ..Default::default()
            }) {
                Hello World 2!
            }
        }
    }
}
