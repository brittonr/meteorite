use dioxus::prelude::*;

use crate::column::{Column, SortDirection, SortState};

/// Cell edit event: (row_index, column_key, new_value).
#[derive(Debug, Clone)]
pub struct CellEdit {
    pub row: usize,
    pub column: String,
    pub value: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct DataTableProps {
    pub columns: Vec<Column>,
    /// Each row is a Vec of cell strings, one per column.
    pub rows: Vec<Vec<String>>,
    /// Current sort state (controlled).
    #[props(default)]
    pub sort: Option<SortState>,
    /// Called when a column header is clicked for sorting.
    #[props(default)]
    pub on_sort: Option<EventHandler<SortState>>,
    /// Called when a cell is edited. The handler receives (row, col_key, new_value).
    #[props(default)]
    pub on_cell_edit: Option<EventHandler<CellEdit>>,
    /// Index of the currently selected row.
    #[props(default)]
    pub selected_row: Option<usize>,
    /// Called when a row is clicked.
    #[props(default)]
    pub on_row_click: Option<EventHandler<usize>>,
    /// Show row numbers.
    #[props(default = false)]
    pub row_numbers: bool,
    /// Striped rows.
    #[props(default = true)]
    pub striped: bool,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn DataTable(props: DataTableProps) -> Element {
    let mut editing = use_signal(|| None::<(usize, usize)>); // (row, col)

    let mut table_class = format!("met-table {}", props.class);
    if props.striped {
        table_class.push_str(" met-table-striped");
    }

    rsx! {
        table { class: "{table_class}",
            thead {
                tr {
                    if props.row_numbers {
                        th { class: "met-table-rownum-header", "#" }
                    }
                    for col in props.columns.iter() {
                        {
                            let key = col.key.clone();
                            let label = col.label.clone();
                            let sortable = col.sortable;
                            let on_sort = props.on_sort.clone();
                            let current_sort = props.sort.clone();

                            let is_sorted = current_sort.as_ref().map(|s| s.column_key == key).unwrap_or(false);
                            let sort_dir = current_sort.as_ref().filter(|s| s.column_key == key).map(|s| s.direction);

                            let mut th_class = String::from("met-table-th");
                            if sortable { th_class.push_str(" met-table-th-sortable"); }
                            if is_sorted { th_class.push_str(" met-table-th-sorted"); }

                            let w = col.width.as_ref().map(|w| format!("width: {w};")).unwrap_or_default();
                            let mw = col.min_width.as_ref().map(|w| format!("min-width: {w};")).unwrap_or_default();
                            let style = format!("{w}{mw}");

                            rsx! {
                                th {
                                    class: "{th_class}",
                                    style: "{style}",
                                    onclick: move |_| {
                                        if sortable {
                                            if let Some(ref handler) = on_sort {
                                                let dir = sort_dir
                                                    .map(|d| d.toggle())
                                                    .unwrap_or(SortDirection::Ascending);
                                                handler.call(SortState {
                                                    column_key: key.clone(),
                                                    direction: dir,
                                                });
                                            }
                                        }
                                    },
                                    span { "{label}" }
                                    if sortable {
                                        span { class: "met-table-sort-icon",
                                            if let Some(dir) = sort_dir {
                                                " {dir.icon()}"
                                            } else {
                                                " ↕"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            tbody {
                for (ri, row) in props.rows.iter().enumerate() {
                    {
                        let is_selected = props.selected_row == Some(ri);
                        let mut row_class = String::from("met-table-row");
                        if is_selected { row_class.push_str(" met-table-row-selected"); }

                        let on_row_click = props.on_row_click.clone();

                        rsx! {
                            tr {
                                class: "{row_class}",
                                onclick: move |_| {
                                    if let Some(ref h) = on_row_click {
                                        h.call(ri);
                                    }
                                },

                                if props.row_numbers {
                                    td { class: "met-table-rownum", "{ri + 1}" }
                                }

                                for (ci, cell) in row.iter().enumerate() {
                                    {
                                        let cell_val = cell.clone();
                                        let col_editable = props.columns.get(ci).map(|c| c.editable).unwrap_or(false);
                                        let col_key = props.columns.get(ci).map(|c| c.key.clone()).unwrap_or_default();
                                        let is_editing = *editing.read() == Some((ri, ci));
                                        let on_cell_edit = props.on_cell_edit.clone();

                                        if is_editing && col_editable {
                                            rsx! {
                                                td { class: "met-table-cell met-table-cell-editing",
                                                    input {
                                                        class: "met-table-cell-input",
                                                        r#type: "text",
                                                        value: "{cell_val}",
                                                        autofocus: true,
                                                        onblur: move |_| editing.set(None),
                                                        onkeydown: move |evt: KeyboardEvent| {
                                                            match evt.key() {
                                                                Key::Enter => editing.set(None),
                                                                Key::Escape => editing.set(None),
                                                                _ => {}
                                                            }
                                                        },
                                                        oninput: {
                                                            let col_key = col_key.clone();
                                                            let on_edit = on_cell_edit.clone();
                                                            move |evt: FormEvent| {
                                                                if let Some(ref h) = on_edit {
                                                                    h.call(CellEdit {
                                                                        row: ri,
                                                                        column: col_key.clone(),
                                                                        value: evt.value().clone(),
                                                                    });
                                                                }
                                                            }
                                                        },
                                                    }
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                td {
                                                    class: "met-table-cell",
                                                    ondoubleclick: move |_| {
                                                        if col_editable {
                                                            editing.set(Some((ri, ci)));
                                                        }
                                                    },
                                                    "{cell_val}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
