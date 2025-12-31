use templr::{templ, templ_ret};
use tw_merge::tw_merge;

#[derive(Default)]
pub struct FormRowProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn row<'a>(props: FormRowProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("flex-1 flex items-center gap-3", &props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default)]
pub struct FormColProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn col<'a>(props: FormColProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("flex-1 flex flex-col items-center gap-4 w-full", &props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default)]
pub struct ItemProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn item<'a>(props: ItemProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("flex flex-col items-center gap-2", &props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default)]
pub struct TitleProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn title<'a>(props: TitleProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <h4
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("text-lg font-medium text-background pb-2", &props.class)}
            {..props.attrs}
        >
            #children;
        </h4>
    }
}
