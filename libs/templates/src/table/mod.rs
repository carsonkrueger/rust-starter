use templr::{templ, templ_ret};
use tw_merge::tw_merge;

#[derive(Default)]
pub struct TableProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn table<'a>(props: &'a TableProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <table
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("w-full caption-bottom text-sm", props.class)}
            {..props.attrs}
        >
            #children;
        </table>
    }
}

#[derive(Default)]
pub struct THeadProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn thead<'a>(props: &'a THeadProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <thead
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("", props.class)}
            {..props.attrs}
        >
            #children;
        </thead>
    }
}

#[derive(Default)]
pub struct TBodyProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn tbody<'a>(props: &'a TBodyProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <tbody
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("[&_tr:last-child]:border-0", props.class)}
            {..props.attrs}
        >
            #children;
        </tbody>
    }
}

#[derive(Default)]
pub struct RowProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn row<'a>(props: &'a RowProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <tr
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("border-b transition-colors hover:bg-muted/50", props.class)}
            {..props.attrs}
        >
            #children;
        </tr>
    }
}

#[derive(Default, Clone, Copy)]
pub struct ThProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn th<'a>(props: &'a ThProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <th
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("h-10 px-2 text-left align-middle font-medium text-muted-foreground", props.class)}
            {..props.attrs}
        >
            #children;
        </th>
    }
}

#[derive(Default)]
pub struct TdProps<'a> {
    pub id: Option<&'a str>,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn td<'a>(props: &'a TdProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <td
            #if let Some(id) = props.id {
                id={id}
            }
            class={tw_merge!("p-5 align-middle", props.class)}
            {..props.attrs}
        >
            #children;
        </td>
    }
}
