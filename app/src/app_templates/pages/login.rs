use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;
use templates::{
    button::{ButtonProps, button},
    form::{FormColProps, TitleProps, col, title},
    input::{InputProps, input},
    label::anchored::{AnchoredProps, anchored},
};

pub fn page<'a>() -> templ_ret!['a] {
    templ! {
        #page_layout() {
            <div class="px-8 min-h-screen text-main flex flex-col items-center justify-center">
                <form id="login" class="max-w-96 w-full">
                    #col(&FormColProps::default()) {
                        #title(&TitleProps::default()) {
                            Login
                        }
                        #anchored(&AnchoredProps{
                            label: "Email",
                            ..AnchoredProps::default()
                        }) {
                            #input(&InputProps{
                                name: "email",
                                ..Default::default()
                            });
                        }
                        #anchored(&AnchoredProps{
                            label: "Password",
                            ..AnchoredProps::default()
                        }) {
                            #input(&InputProps{
                                name: "password",
                                ..Default::default()
                            });
                        }
                        <div class="w-full flex justify-between items-end">
                            <a href="/sign_up">
                                <span class="text-primary-foreground text-sm hover:underline cursor-pointer">Sign Up</span>
                            </a>
                            #button(ButtonProps {
                                attrs: &[("data-on:click", "@post('/login', {contentType: 'form'})")],
                                btn_type: templates::button::ButtonType::Submit,
                                ..Default::default()
                            }) {
                                Login
                            }
                        </div>
                    }
                </form>
            </div>
        }
    }
}
