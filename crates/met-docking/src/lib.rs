//! Framework-agnostic panel tiling and docking engine.
//!
//! Computes layout geometry for UI panels that can be split, docked, floated,
//! dragged, and resized. Produces position/size data that any rendering
//! framework (Dioxus, egui, Yew, leptos) consumes. Does not render anything.
//!
//! ## Modules
//!
//! - **position** — core types for positioning and sizing
//! - **zones** — zone management for docked panels
//! - **panels** — panel configuration and floating panel state
//! - **drag_drop** — drag-and-drop operation state and results
//! - **layout** — main docking layout orchestration
//! - **flexible_layout** — tree-based flexible layout system
//! - **presets** — named layout presets with serialization

pub mod drag_drop;
pub mod flexible_layout;
pub mod layout;
pub mod panels;
pub mod position;
pub mod presets;
pub mod zones;

// Re-export commonly used types for convenience
pub use drag_drop::{DragState, DropResult, DropTarget, DropZone};
pub use layout::DockingLayout;
pub use panels::{FloatingPanel, PanelConfig, PanelState};
pub use position::{DockPosition, DockSize};
pub use zones::DockZone;

// Re-export preset system types
pub use presets::{LayoutBuilder, LayoutPreset, PresetError, PresetMetadata, PresetRegistry};
