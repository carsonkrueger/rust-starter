use http::Method;
use templr::{templ, templ_ret};
use tw_merge::tw_merge;

#[derive(Default)]
pub struct IntersectRowProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub endpoint: &'a str,
    pub col_span: Option<usize>,
    pub method: Option<Method>,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn intersect_row<'a>(props: &'a IntersectRowProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <tr
            #if let Some(id) = props.id {
                id={id}
            }
            #if let Some(col_span) = props.col_span {
                colspan={col_span}
            }
            class={tw_merge!("w-full caption-bottom text-sm", props.class)}
            data-on-intersect={format!("@{}('{}', {{ filterSignals: {{ exclude: '*' }} }})", props.method.clone().unwrap_or_default().to_string().to_lowercase(), props.endpoint)}
            {..props.attrs}
        >
            #children;
        </tr>
    }
}
