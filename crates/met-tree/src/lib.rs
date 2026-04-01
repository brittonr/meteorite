//! Tree view component for Dioxus with expand/collapse, keyboard navigation,
//! and guide lines.
//!
//! # Usage
//!
//! ```rust,ignore
//! use met_tree::{Tree, TreeItem};
//!
//! let items = vec![
//!     TreeItem::new("0", "Documents"),
//!     TreeItem::new("1", "Photos").parent("0"),
//!     TreeItem::new("2", "Vacation").parent("1"),
//!     TreeItem::new("3", "Work").parent("0"),
//!     TreeItem::new("4", "Music"),
//! ];
//!
//! rsx! {
//!     Tree {
//!         items: items,
//!         on_select: move |id: String| log::info!("selected {id}"),
//!     }
//! }
//! ```

pub mod model;
pub mod tree;

pub use model::TreeItem;
pub use tree::Tree;
