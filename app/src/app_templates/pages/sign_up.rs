use templr::{templ, templ_ret};

use crate::app_templates::pages::page_layout;
use templates::{
    button::{ButtonProps, button},
    form::{FormColProps, FormRowProps, TitleProps, col, row, title},
    input::{InputProps, InputType, input},
    label::anchored::{AnchoredProps, anchored},
};

pub fn page<'a>() -> templ_ret!['a] {
    templ! {
        #page_layout() {
            <div class="px-8 min-h-screen text-main flex flex-col items-center justify-center">
                <form class="max-w-96 w-full">
                    #col(&FormColProps::default()) {
                        #title(&TitleProps::default()) {
                            Sign Up
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
                        #row(&FormRowProps::default()) {
                            #anchored(&AnchoredProps{
                                label: "First Name",
                                ..AnchoredProps::default()
                            }) {
                                #input(&InputProps{
                                    name: "first_name",
                                    ..Default::default()
                                });
                            }
                            #anchored(&AnchoredProps{
                                label: "Last Name",
                                ..AnchoredProps::default()
                            }) {
                                #input(&InputProps{
                                    name: "last_name",
                                    ..Default::default()
                                });
                            }
                        }
                        #anchored(&AnchoredProps{
                            label: "Password",
                            ..AnchoredProps::default()
                        }) {
                            #input(&InputProps{
                                name: "password",
                                input_type: InputType::Password,
                                ..Default::default()
                            });
                        }
                        #anchored(&AnchoredProps{
                            label: "Confirm Password",
                            ..AnchoredProps::default()
                        }) {
                            #input(&InputProps{
                                input_type: InputType::Password,
                                ..Default::default()
                            });
                        }
                        <div class="w-full flex justify-between items-end">
                            <p class="text-sm text-primary-foreground">
                                Already have an account?
                                <a href="/login">
                                    <span class="hover:underline cursor-pointer">login</span>
                                </a>
                            </p>
                            #button(ButtonProps {
                                attrs: &[("data-on:click", "@post('/sign_up', {contentType: 'form'})")],
                                btn_type: templates::button::ButtonType::Submit,
                                ..Default::default()
                            }) {
                                Sign Up
                            }
                        </div>
                    }
                </form>
            </div>
        }
    }
}
