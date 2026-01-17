use templr::{templ, templ_ret};
use tw_merge::tw_merge;

pub mod anchored;

#[derive(Default, Copy, Clone)]
pub struct LabelProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn label<'a>(props: LabelProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <label
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("text-sm font-medium leading-none inline-block", props.class)}
            {..props.attrs}
        >
            #children;
        </label>
    }
}
