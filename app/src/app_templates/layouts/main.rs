use templates::icon::{IconProps, icon, icon_data::Icon};
use templr::{templ, templ_ret};

pub fn main<'a>() -> templ_ret!['a] {
    templ! {
        #use children;
        <div class="relative bg-[url(/public/img/wave_background.svg)] bg-no-repeat bg-cover bg-center bg-fixed">
            <header class="backdrop-blur-xs hover:backdrop-blur-lg fixed lg:p-4 p-6 top-0 z-10 w-30 h-screen drop-shadow-lg hover:drop-shadow-2xl animate-all duration-300">
                <h1 class="">
                    Rust Starter Kit
                </h1>
                <div class="">
                    #icon(Icon::House, IconProps::default());
                </div>
            </header>
            <div class="pl-30">
                #children;
            </div>
            <footer />
        </div>
    }
}
