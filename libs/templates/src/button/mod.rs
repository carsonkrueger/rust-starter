use templr::{templ, templ_ret};
use tw_merge::{AsTailwindClass, tw_merge};

pub struct ButtonProps<'a> {
    pub id: String,
    pub class: String,
    pub variant: Variant,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub enum Variant {
    Primary,
}

impl Default for ButtonProps<'_> {
    fn default() -> Self {
        ButtonProps {
            id: String::default(),
            class: String::default(),
            variant: Variant::default(),
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
        }
    }
}

pub fn button<'a>(props: ButtonProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <button
            class={tw_merge!("flex px-3 py-2 rounded-sm cursor-pointer hover:shadow-md hover:-translate-y-[2px] animate-all duration-300", &props.variant, &props.class)}
            {..props.attrs}
        >
            #children;
        </button>
    }
}
