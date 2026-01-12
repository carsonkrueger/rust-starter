use templr::{templ, templ_ret};
use tw_merge::{AsTailwindClass, tw_merge};

use crate::label::{LabelProps, label};

#[derive(Default)]
pub struct AnchoredProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
    pub label: &'a str,
    pub for_: Option<&'a str>,
    pub anchor: Anchor,
    pub label_props: Option<&'a LabelProps<'a>>,
}

#[derive(Default)]
pub enum Anchor {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl AsTailwindClass for Anchor {
    fn as_class(&self) -> &str {
        match self {
            Anchor::Top => "flex-col",
            Anchor::Right => "flex-row-reverse items-center",
            Anchor::Bottom => "flex-col-reverse",
            Anchor::Left => "flex-row text-center items-center",
        }
    }
}

pub fn anchored<'a>(props: &'a AnchoredProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if let Some(id) = props.id {
                id={id}
            }
            #if let Some(for_) = props.for_ {
                for={for_}
            }
            class={tw_merge!("flex gap-2 w-full text-primary-foreground", &props.anchor, props.class)}
            {..props.attrs}
        >
            #label(props.label_props.unwrap_or(&LabelProps::default())) {
                {props.label}
            }
            #children;
        </div>
    }
}
