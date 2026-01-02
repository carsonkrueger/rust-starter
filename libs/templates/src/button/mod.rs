use strum::IntoStaticStr;
use templr::{templ, templ_ret};
use tw_merge::{AsTailwindClass, tw_merge};

#[derive(Default)]
pub struct ButtonProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub variant: Variant,
    pub btn_type: ButtonType,
    pub attrs: &'a [(&'a str, &'a str)],
}

#[derive(Default)]
pub enum Variant {
    #[default]
    Primary,
    Transparent,
}

#[derive(Default, IntoStaticStr)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
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
            #if let Some(id) = props.id {
                id={id}
            }
            type={Into::<&'static str>::into(&props.btn_type)}
            class={tw_merge!("flex justify-center items-center px-3 py-2 rounded-sm cursor-pointer hover:shadow-md hover:-translate-y-[2px] animate-all duration-300", &props.variant, &props.class)}
            {..props.attrs}
        >
            #children;
        </button>
    }
}
