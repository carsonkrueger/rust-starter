use http::Method;
use templr::{templ, templ_ret};

pub struct IntersectRowProps<'a> {
    pub id: Option<&'a str>,
    pub method: Method,
    pub endpoint: &'a str,
    pub indicator: Option<&'a str>,
}

pub fn intersect_trigger<'a>(props: &'a IntersectRowProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <tfoot
            #if let Some(id) = props.id {
                id={id}
            }
            class={"w-full caption-bottom text-sm"}
            data-on-intersect={format!("@{}('{}')", props.method.to_string().to_lowercase(), props.endpoint)}
            #if let Some(indicator) = props.indicator {
                data-indicator={indicator}
            }
        >
            #children;
        </tfoot>
    }
}
