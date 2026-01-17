use templr::{templ, templ_ret};
use tw_merge::tw_merge;

#[derive(Default, Copy, Clone)]
pub struct FormRowProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn row<'a>(props: FormRowProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("flex-1 flex items-center gap-3", &props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default, Copy, Clone)]
pub struct FormColProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn col<'a>(props: FormColProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("flex-1 flex flex-col items-center gap-4 w-full", &props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default, Copy, Clone)]
pub struct ItemProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn item<'a>(props: ItemProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <div
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("flex flex-col items-center gap-2", &props.class)}
            {..props.attrs}
        >
            #children;
        </div>
    }
}

#[derive(Default, Copy, Clone)]
pub struct TitleProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn title<'a>(props: TitleProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <h4
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("text-lg font-medium text-background pb-2", &props.class)}
            {..props.attrs}
        >
            #children;
        </h4>
    }
}
