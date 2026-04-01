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
                    for col in props.columns.clone().into_iter() {
                        {
                            let key = col.key.clone();
                            let sortable = col.sortable;
                            let on_sort = props.on_sort.clone();
                            let w = col.width.as_ref().map(|w| format!("width: {w}")).unwrap_or_default();
                            rsx! {
                                th {
                                    style: "{w}",
                                    onclick: move |_| {
                                        if sortable {
                                            if let Some(ref handler) = on_sort {
                                                handler.call((key.clone(), SortDirection::Ascending));
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
