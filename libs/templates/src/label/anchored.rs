use templr::{templ, templ_ret};
use tw_merge::{AsTailwindClass, tw_merge};

use crate::label::{LabelProps, label};

pub struct AnchoredProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
    pub label: &'a str,
    /// Where the label is anchored
    pub anchor: Anchor,
}

impl Default for AnchoredProps<'_> {
    fn default() -> Self {
        AnchoredProps {
            id: "",
            class: "",
            attrs: &[],
            label: "",
            anchor: Anchor::Top,
        }
    }
}

pub enum Anchor {
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

pub fn anchored<'a>(props: AnchoredProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div class={tw_merge!("flex gap-2 w-full text-primary-foreground", &props.anchor, props.class)}>
            #label(LabelProps::default()) {
                {props.label}
            }
            #children;
        </div>
    }
}
