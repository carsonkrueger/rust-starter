use templr::{templ, templ_ret};
use tw_merge::{AsTailwindClass, tw_merge};

pub struct ButtonProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub variant: Variant,
    pub btn_type: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub enum Variant {
    Primary,
    Transparent,
}

impl Default for ButtonProps<'_> {
    fn default() -> Self {
        ButtonProps {
            id: "",
            class: "",
            variant: Variant::default(),
            btn_type: "button",
            attrs: &[],
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Primary
    }
}

impl AsTailwindClass for Variant {
    fn as_class(&self) -> &str {
        match self {
            Self::Primary => "bg-primary hover:bg-primary/80 text-primary-foreground",
            Self::Transparent => "hover:bg-transparent/80 text-primary-foreground",
        }
    }
}

pub fn button<'a>(props: ButtonProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <button
            #if !props.id.is_empty() {
                id={props.id}
            }
            type={props.btn_type}
            class={tw_merge!("flex justify-center items-center px-3 py-2 rounded-sm cursor-pointer hover:shadow-md hover:-translate-y-[2px] animate-all duration-300", &props.variant, &props.class)}
            {..props.attrs}
        >
            #children;
        </button>
    }
}
