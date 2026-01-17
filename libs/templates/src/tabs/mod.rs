use templr::{templ, templ_ret};
use tw_merge::AsTailwindClass;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy)]
pub struct TabGroupProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub direction: Direction,
    pub attrs: &'a [(&'a str, &'a str)],
}

#[derive(Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

impl AsTailwindClass for Direction {
    fn as_class(&self) -> &str {
        match self {
            Self::Horizontal => "",
            Self::Vertical => "flex-col",
        }
    }
}

pub fn tab_group<'a>(props: TabGroupProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("w-full flex", props.direction, props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default, Clone, Copy)]
pub struct TabProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
    pub selected: bool,
}

pub fn tab<'a>(props: TabProps<'a>) -> templ_ret!['a] {
    let selected_class = match props.selected {
        true => "bg-muted/10 shadow-md",
        false => "",
    };
    templ! {
        #use children;
        <div
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("px-4 py-2 cursor-pointer hover:bg-muted/10 hover:shadow-md", selected_class, props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}
