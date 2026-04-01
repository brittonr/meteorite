//! Fuzzy-search command palette overlay for Dioxus.
//!
//! Provides a ⌘K / Ctrl+K style command palette with fuzzy matching,
//! match highlighting, and keyboard navigation.
//!
//! # Usage
//!
//! ```rust,ignore
//! use met_command_palette::{CommandPalette, PaletteItem};
//!
//! let items = vec![
//!     PaletteItem::new("save", "Save file"),
//!     PaletteItem::new("open", "Open file").shortcut("⌘O"),
//!     PaletteItem::new("quit", "Quit application").shortcut("⌘Q"),
//! ];
//!
//! rsx! {
//!     CommandPalette {
//!         open: true,
//!         items: items,
//!         on_select: move |id: String| log::info!("selected {id}"),
//!         on_close: move |_| {},
//!     }
//! }
//! ```

pub mod fuzzy;
pub mod palette;

pub use fuzzy::{fuzzy_score, ScoredMatch};
pub use palette::{CommandPalette, PaletteItem};
