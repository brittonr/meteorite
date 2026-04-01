//! Helix-style leader key popup menu for Dioxus.
//!
//! Press a leader key (e.g. Space) to open a which-key overlay.
//! A single keypress executes an action or opens a submenu.
//! Escape or any unrecognized key dismisses the menu.
//!
//! The state machine and builder live in `ratcore::leaderkey` (shared
//! with the ratatui TUI). This crate provides the Dioxus component.
//!
//! # Example
//!
//! ```rust,ignore
//! use met_leaderkey::*;
//! use ratcore::leaderkey::*;
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum Action { Save, Open, Quit }
//!
//! struct AppBindings;
//! impl MenuContributor<Action> for AppBindings {
//!     fn menu_items(&self) -> Vec<MenuContribution<Action>> {
//!         vec![
//!             MenuContribution {
//!                 key: 's', label: "save".into(),
//!                 action: LeaderAction::Action(Action::Save),
//!                 placement: MenuPlacement::Root,
//!                 priority: PRIORITY_BUILTIN, source: "app".into(),
//!             },
//!         ]
//!     }
//! }
//!
//! fn App() -> Element {
//!     let mut menu = use_signal(|| {
//!         let (m, _) = build(&[&AppBindings], &HashSet::new());
//!         m
//!     });
//!     rsx! {
//!         div {
//!             onkeydown: move |evt| {
//!                 if evt.key() == Key::Character(" ".into()) && !menu.read().visible {
//!                     menu.write().open();
//!                 }
//!             },
//!             LeaderKeyOverlay {
//!                 menu: menu,
//!                 on_action: move |action| { /* dispatch */ },
//!             }
//!         }
//!     }
//! }
//! ```

mod overlay;

pub use overlay::LeaderKeyOverlay;

// Re-export everything from ratcore so consumers don't need a direct dep.
pub use ratcore::leaderkey::*;
