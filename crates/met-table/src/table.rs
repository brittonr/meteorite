use dioxus::prelude::*;
use crate::column::{Column, SortDirection};

#[derive(Props, Clone, PartialEq)]
pub struct DataTableProps {
    pub columns: Vec<Column>,
    /// Each row is a Vec of cell strings, one per column.
    pub rows: Vec<Vec<String>>,
    #[props(default)]
    pub class: String,
    pub on_sort: Option<EventHandler<(String, SortDirection)>>,
}

#[component]
pub fn DataTable(props: DataTableProps) -> Element {
    let class = format!("met-table {}", props.class);

    rsx! {
        table { class: "{class}",
            thead {
                tr {
                    for col in props.columns.iter() {
                        th {
                            style: col.width.as_ref().map(|w| format!("width: {w}")).unwrap_or_default(),
                            onclick: move |_| {
                                if col.sortable {
                                    if let Some(handler) = &props.on_sort {
                                        handler.call((col.key.clone(), SortDirection::Ascending));
                                    }
                                }
                            },
                            "{col.label}"
                            if col.sortable {
                                span { class: "met-table-sort-icon", " ↕" }
                            }
                        }
                    }
                }
            }
            tbody {
                for row in props.rows.iter() {
                    tr {
                        for cell in row.iter() {
                            td { "{cell}" }
                        }
                    }
                }
            }
        }
    }
}
