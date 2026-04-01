//! Data table component for Dioxus with sorting, filtering, and pagination.

pub mod column;
pub mod table;

pub use column::{Column, SortDirection};
pub use table::DataTable;
