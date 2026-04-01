//! Data table component for Dioxus with sorting, inline editing, column resize,
//! and row selection.

pub mod column;
pub mod table;

pub use column::{Column, SortDirection, SortState};
pub use table::DataTable;
