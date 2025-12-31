use templr::{templ, templ_ret};
use tw_merge::tw_merge;

#[derive(Default)]
pub struct TableProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn table<'a>(props: TableProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <table
            #if !props.id.is_empty() {
                id={props.id}
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
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn thead<'a>(props: THeadProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <thead
            #if !props.id.is_empty() {
                id={props.id}
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
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn tbody<'a>(props: TBodyProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <tbody
            #if !props.id.is_empty() {
                id={props.id}
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
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn row<'a>(props: RowProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <tr
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("border-b transition-colors hover:bg-muted/50", props.class)}
            {..props.attrs}
        >
            #children;
        </tr>
    }
}

#[derive(Default)]
pub struct ThProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn th<'a>(props: ThProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <th
            #if !props.id.is_empty() {
                id={props.id}
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
    pub id: &'a str,
    pub class: &'a str,
    pub attrs: &'a [(&'a str, &'a str)],
}

pub fn td<'a>(props: TdProps<'a>) -> templ_ret!['a] {
    templ! {
        #use children;
        <td
            #if !props.id.is_empty() {
                id={props.id}
            }
            class={tw_merge!("p-2 align-middle", props.class)}
            {..props.attrs}
        >
            #children;
        </td>
    }
}
