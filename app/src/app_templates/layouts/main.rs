use templates::{
    button::{ButtonProps, Variant, button},
    icon::{IconProps, icon, icon_data::Icon},
};
use templr::{templ, templ_ret};

pub fn main<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <div class="dark:bg-[url(/public/img/wave_background_dark.svg)] bg-[url(/public/img/wave_background.svg)] bg-no-repeat bg-cover bg-center bg-fixed">
            <header class="flex flex-col justify-between items-center backdrop-blur-xs hover:backdrop-blur-lg fixed lg:p-4 p-6 top-0 z-10 w-30 h-screen drop-shadow-lg hover:drop-shadow-2xl animate-all duration-300">
                <img src="/public/img/rust.svg" />
                <div class="flex flex-col items-center justify-end gap-1">
                    <a href="/home">
                        #button(ButtonProps{
                            variant: Variant::Transparent,
                            class: &"rounded-full p-2",
                            ..Default::default()
                        }) {
                            #icon(IconProps{ icon: Icon::House, color: "var(--primary-foreground)", ..Default::default()});
                        }
                    </a>
                    <a href="/login">
                        #button(ButtonProps{
                            variant: Variant::Transparent,
                            class: &"rounded-full p-2",
                            ..Default::default()
                        }) {
                            #icon(IconProps{ icon: Icon::CircleUserRound, color: "var(--primary-foreground)", ..Default::default()});
                        }
                    </a>
                </div>
            </header>
            <div class="pl-30">
                #children;
            </div>
            <footer />
        </div>
    }
}
