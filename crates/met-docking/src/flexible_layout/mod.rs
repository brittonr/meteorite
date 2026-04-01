//! Functional core: tree-based flexible layout system
//!
//! This module provides a flexible, tree-based layout system that replaces
//! the hardcoded zone structure with a dynamic, user-configurable layout.

pub mod builder;
pub mod constraints;
pub mod integration;
pub mod migration;
pub mod node;
pub mod operations;
pub mod presets;
pub mod renderer;
pub mod tree;

pub use builder::{BuilderError, LayoutBuilder};
pub use constraints::{LayoutConstraints, SizeConstraints};
pub use integration::{FlexibleLayoutAdapter, IntegrationError, LegacyZoneState};
pub use migration::LayoutMigration;
pub use node::{ContainerLayout, LayoutNode, NodeId, NodePath, SplitDirection, ZoneContent};
pub use operations::{LayoutOperation, LayoutOperationExecutor, OperationError, OperationResult};
pub use presets::{
    CustomPresetData, LayoutPreset, PresetInfo, PresetStorage, PRESET_STORAGE_KEY_PREFIX,
};
pub use renderer::{LayoutRenderer, Rect, RenderedLayout, SplitHandle};
pub use tree::{LayoutTree, TreeError};

/// Re-export commonly used types
pub mod prelude {
    pub use super::{
        ContainerLayout, LayoutBuilder, LayoutConstraints, LayoutNode, LayoutOperation,
        LayoutOperationExecutor, LayoutRenderer, LayoutTree, NodeId, RenderedLayout,
        SizeConstraints, SplitDirection, ZoneContent,
    };
}
