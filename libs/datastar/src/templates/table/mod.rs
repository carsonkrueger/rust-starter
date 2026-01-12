use std::convert::Infallible;

use axum::response::{Sse, sse::Event};
use http::Method;
use templates::{
    icon::{self, IconProps, icon_data::Icon},
    input::{self, InputProps},
    label::anchored::{self, AnchoredProps},
    table::{RowProps, TBodyProps, THeadProps, TableProps, TdProps, row, table, tbody, td, thead},
};
use templr::{Template, templ, templ_ret};

use crate::{
    event_stream,
    modes::DatastarMode,
    templates::table::{
        infinite_scroll::{IntersectRowProps, intersect_trigger},
        search_params::TableSearchParams,
    },
};

pub mod infinite_scroll;
pub mod search_params;

pub trait IntoTableData {
    const ENDPOINT: &'static str;
    const TABLE_ID: &'static str;
    const TABLE_BODY_ID: &'static str;
    /// Returns only the <td> elements
    fn table_data<'a>(&'a self) -> templ_ret!['a];
    /// Returns only the <th> elements
    fn thead_row<'a>() -> templ_ret!['a];
}

fn indicator<TD: IntoTableData>() -> String {
    TD::TABLE_ID.to_string() + "Indicator"
}

#[derive(Default)]
pub struct DatastarTableProps {}

pub fn datastar_table<'a, TD: IntoTableData>(_props: &'a DatastarTableProps) -> templ_ret!['a] {
    templ! {
        <div class="flex flex-col gap-24 w-full">
            <div class="p-5">
                #anchored::anchored(&AnchoredProps {
                    label: "Search",
                    class: "max-w-96",
                    ..Default::default()
                }) {
                    #input::input(&InputProps{
                        attrs: &[("data-bind", "query"), ("data-on:input__debounce.300ms", &format!("@get('{}')", TD::ENDPOINT)), ("data-on:input", "$page = 1"), ("data-indicator", &indicator::<TD>())],
                        ..Default::default()
                    });
                }
            </div>
            #table(&TableProps{
                id: Some(TD::TABLE_ID),
                ..Default::default()
            }) {
                #thead(&THeadProps::default()) {
                    #TD::thead_row();
                }
                #tbody(&TBodyProps{
                    id: Some(TD::TABLE_BODY_ID),
                    ..Default::default()
                });
                #intersect_trigger(&IntersectRowProps{
                    id: None,
                    endpoint: TD::ENDPOINT,
                    method: Method::GET,
                    indicator: Some(&indicator::<TD>()),
                }) {
                    #row(&RowProps::default()) {
                        #td(&TdProps{
                            attrs: &[("colspan", "100%"), ("data-show", &format!("${}", &indicator::<TD>()))],
                            ..Default::default()
                        }) {
                            #icon::icon(IconProps{
                                icon: Icon::LoaderCircle,
                                class: "animate-spin m-auto",
                                size: 30,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        </div>
    }
}

pub struct DatastarRowsProps<'a, TD: IntoTableData> {
    pub rows: &'a [TD],
}

pub fn datastar_rows<'a, TD: IntoTableData>(
    props: &'a DatastarRowsProps<'a, TD>,
) -> templ_ret!['a] {
    templ! {
        #for r in props.rows {
            #row(&RowProps::default()) {
                #r.table_data();
            }
        }
    }
}

/// Automatically increments to next page
pub fn table_patch_events<'a, TD: IntoTableData, S: Into<TableSearchParams>>(
    rows: &[TD],
    search_params: S,
) -> Result<Vec<Event>, templr::Error> {
    let row_templates = datastar_rows(&DatastarRowsProps { rows }).render(&())?;
    let mut search_params = search_params.into();
    let mode = match search_params.page {
        1 => DatastarMode::Inner,
        _ => DatastarMode::Append,
    };
    let rows_event = crate::patch_elements()
        .selector(format!("#{}", TD::TABLE_BODY_ID))
        .mode(mode)
        .elements(row_templates)
        .axum_event();
    if !rows.is_empty() {
        search_params = search_params.next_page();
    }
    let signal_event = crate::patch_signals(search_params).axum_event();
    Ok(vec![signal_event, rows_event])
}

/// Automatically increments to next page
pub fn table_patch_stream<'a, TD: IntoTableData, S: Into<TableSearchParams>>(
    rows: &[TD],
    search_params: S,
) -> Result<
    Sse<impl futures_core::Stream<Item = Result<Event, Infallible>> + Send + use<TD, S>>,
    templr::Error,
> {
    let events = table_patch_events(rows, search_params)?;
    Ok(event_stream(events))
}
