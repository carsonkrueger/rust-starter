use templr::{templ, templ_ret};
use tw_merge::tw_merge;

pub mod anchored;

pub struct LabelProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

impl Default for LabelProps<'_> {
    fn default() -> Self {
        LabelProps {
            id: "",
            class: "",
            attrs: &[],
        }
    }
}

pub fn label<'a>(props: LabelProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <label
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("text-sm font-medium leading-none inline-block", props.class)}
            {..props.attrs}
        >
            #children;
        </label>
    }
}
