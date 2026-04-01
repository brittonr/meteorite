//! Functional core: layout operation command types
//!
//! This module defines high-level operations that can be performed on the layout tree,
//! providing a command pattern for layout manipulation.

use super::{
    node::{ContainerLayout, NodeId, SplitDirection},
    tree::{LayoutTree, TreeError},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during layout operations
#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Tree error: {0}")]
    TreeError(#[from] TreeError),

    #[error("Invalid target: {0}")]
    InvalidTarget(String),

    #[error("Operation not allowed: {0}")]
    NotAllowed(String),

    #[error("Undo stack is empty")]
    NothingToUndo,

    #[error("Redo stack is empty")]
    NothingToRedo,
}

/// Result of an operation
pub type OperationResult<T> = Result<T, OperationError>;

/// High-level layout operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayoutOperation {
    /// Split a zone into two zones
    SplitZone {
        zone_id: NodeId,
        direction: SplitDirection,
        ratio: f32,
        new_zone_first: bool,
    },

    /// Merge two adjacent zones
    MergeZones { zone1_id: NodeId, zone2_id: NodeId },

    /// Move a panel to a different zone
    MovePanel {
        panel_id: String,
        target_zone_id: NodeId,
        insert_index: Option<usize>,
    },

    /// Add a new panel to a zone
    AddPanel { panel_id: String, zone_id: NodeId },

    /// Remove a panel from the layout
    RemovePanel { panel_id: String },

    /// Resize a split
    ResizeSplit { split_id: NodeId, new_ratio: f32 },

    /// Create a container from zones
    CreateContainer {
        zone_ids: Vec<NodeId>,
        container_layout: ContainerLayout,
    },

    /// Float a panel (remove from layout)
    FloatPanel { panel_id: String },

    /// Dock a floating panel
    DockPanel {
        panel_id: String,
        target_zone_id: NodeId,
    },

    /// Set active panel in a zone
    SetActivePanel { zone_id: NodeId, panel_id: String },

    /// Swap two zones
    SwapZones { zone1_id: NodeId, zone2_id: NodeId },

    /// Create a new empty zone
    CreateZone {
        zone_id: NodeId,
        parent_id: NodeId,
        position: ZonePosition,
    },
}

/// Position for inserting a new zone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ZonePosition {
    /// Replace the parent node with a split containing the new zone
    AsSibling {
        direction: SplitDirection,
        ratio: f32,
        new_zone_first: bool,
    },
    /// Add as a child in a container (if parent is a container)
    AsChild { index: Option<usize> },
}

/// Result data from a layout operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationResultData {
    /// Zone was split, returns new zone ID
    ZoneSplit { new_zone_id: NodeId },

    /// Zones were merged, returns resulting zone ID
    ZonesMerged { result_zone_id: NodeId },

    /// Panel was moved between zones
    PanelMoved {
        panel_id: String,
        from_zone: NodeId,
        to_zone: NodeId,
    },

    /// Panel was added to a zone
    PanelAdded { panel_id: String, zone_id: NodeId },

    /// Panel was removed from a zone
    PanelRemoved { panel_id: String, from_zone: NodeId },

    /// Split was resized
    SplitResized {
        split_id: NodeId,
        old_ratio: f32,
        new_ratio: f32,
    },

    /// Container was created
    ContainerCreated {
        container_id: NodeId,
        zone_ids: Vec<NodeId>,
    },

    /// Panel was floated
    PanelFloated { panel_id: String, from_zone: NodeId },

    /// Panel was docked
    PanelDocked { panel_id: String, to_zone: NodeId },

    /// Active panel was changed
    ActivePanelSet { zone_id: NodeId, panel_id: String },

    /// Zones were swapped
    ZonesSwapped { zone1_id: NodeId, zone2_id: NodeId },

    /// Zone was created
    ZoneCreated { zone_id: NodeId, parent_id: NodeId },
}

/// Executor for layout operations with undo/redo support
pub struct LayoutOperationExecutor {
    undo_stack: Vec<(LayoutOperation, LayoutTree)>,
    redo_stack: Vec<(LayoutOperation, LayoutTree)>,
    max_undo_levels: usize,
}

impl LayoutOperationExecutor {
    /// Create a new operation executor
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_levels: 50,
        }
    }

    /// Execute an operation
    pub fn execute(
        &mut self,
        operation: LayoutOperation,
        tree: &mut LayoutTree,
    ) -> OperationResult<()> {
        // Save current state for undo
        let previous_state = tree.clone();

        // Execute the operation
        match &operation {
            LayoutOperation::SplitZone {
                zone_id,
                direction,
                ratio,
                new_zone_first,
            } => {
                tree.split_zone(zone_id, *direction, *ratio, *new_zone_first)?;
            }

            LayoutOperation::MergeZones { zone1_id, zone2_id } => {
                tree.merge_zones(zone1_id, zone2_id)?;
            }

            LayoutOperation::MovePanel {
                panel_id,
                target_zone_id,
                insert_index,
            } => {
                tree.move_panel(panel_id, target_zone_id, *insert_index)?;
            }

            LayoutOperation::AddPanel { panel_id, zone_id } => {
                tree.add_panel_to_zone(zone_id, panel_id.clone())?;
            }

            LayoutOperation::RemovePanel { panel_id } => {
                tree.remove_panel(panel_id)?;
            }

            LayoutOperation::ResizeSplit {
                split_id,
                new_ratio,
            } => {
                tree.update_split_ratio(split_id, *new_ratio)?;
            }

            LayoutOperation::CreateContainer {
                zone_ids,
                container_layout,
            } => {
                tree.create_container(zone_ids, *container_layout)?;
            }

            LayoutOperation::FloatPanel { panel_id } => {
                tree.remove_panel(panel_id)?;
            }

            LayoutOperation::DockPanel {
                panel_id,
                target_zone_id,
            } => {
                tree.add_panel_to_zone(target_zone_id, panel_id.clone())?;
            }

            LayoutOperation::SetActivePanel { zone_id, panel_id } => {
                tree.set_active_panel(zone_id, Some(panel_id.clone()))?;
            }

            LayoutOperation::SwapZones { zone1_id, zone2_id } => {
                self.swap_zones(tree, zone1_id, zone2_id)?;
            }

            LayoutOperation::CreateZone {
                zone_id,
                parent_id,
                position,
            } => {
                self.create_zone(tree, zone_id, parent_id, position)?;
            }
        }

        // Add to undo stack
        self.undo_stack.push((operation, previous_state));
        if self.undo_stack.len() > self.max_undo_levels {
            self.undo_stack.remove(0);
        }

        // Clear redo stack
        self.redo_stack.clear();

        Ok(())
    }

    /// Undo the last operation
    pub fn undo(&mut self, tree: &mut LayoutTree) -> OperationResult<()> {
        let (operation, previous_state) =
            self.undo_stack.pop().ok_or(OperationError::NothingToUndo)?;

        let current_state = tree.clone();
        *tree = previous_state;

        self.redo_stack.push((operation, current_state));

        Ok(())
    }

    /// Redo the last undone operation
    pub fn redo(&mut self, tree: &mut LayoutTree) -> OperationResult<()> {
        let (operation, next_state) = self.redo_stack.pop().ok_or(OperationError::NothingToRedo)?;

        let current_state = tree.clone();
        *tree = next_state;

        self.undo_stack.push((operation, current_state));

        Ok(())
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear undo/redo history
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Create a new zone (helper method)
    fn create_zone(
        &self,
        tree: &mut LayoutTree,
        _zone_id: &NodeId,
        parent_id: &NodeId,
        position: &ZonePosition,
    ) -> OperationResult<()> {
        match position {
            ZonePosition::AsSibling {
                direction,
                ratio,
                new_zone_first,
            } => {
                // Check if parent_id is a zone
                if tree.find_zone(parent_id).is_some() {
                    // Use the existing split_zone method, which handles all the complexity
                    tree.split_zone(parent_id, *direction, *ratio, *new_zone_first)?;

                    // The split_zone method returns the new zone ID, but we want to use our specified ID
                    // For now, we'll accept the auto-generated ID from split_zone
                    // In the future, we could enhance split_zone to accept a custom ID
                } else {
                    return Err(OperationError::InvalidTarget(format!(
                        "Parent {} is not a zone",
                        parent_id.0
                    )));
                }
            }

            ZonePosition::AsChild { index: _ } => {
                // For containers, add as a child
                // This would require modifying the tree structure to support adding zones to containers
                return Err(OperationError::NotAllowed(
                    "Adding zones as container children not yet implemented".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Swap two zones (helper method)
    fn swap_zones(
        &self,
        tree: &mut LayoutTree,
        zone_a: &NodeId,
        zone_b: &NodeId,
    ) -> OperationResult<()> {
        // Get panels from both zones
        let zone_a_node = tree
            .find_zone(zone_a)
            .ok_or_else(|| OperationError::InvalidTarget(format!("Zone {} not found", zone_a.0)))?;
        let zone_b_node = tree
            .find_zone(zone_b)
            .ok_or_else(|| OperationError::InvalidTarget(format!("Zone {} not found", zone_b.0)))?;

        let panels_a = zone_a_node
            .zone_content()
            .map(|c| c.panels.clone())
            .unwrap_or_default();
        let panels_b = zone_b_node
            .zone_content()
            .map(|c| c.panels.clone())
            .unwrap_or_default();

        // Remove all panels from both zones
        for panel in &panels_a {
            tree.remove_panel(panel)?;
        }
        for panel in &panels_b {
            tree.remove_panel(panel)?;
        }

        // Add panels to opposite zones
        for panel in panels_a {
            tree.add_panel_to_zone(zone_b, panel)?;
        }
        for panel in panels_b {
            tree.add_panel_to_zone(zone_a, panel)?;
        }

        Ok(())
    }
}

impl Default for LayoutOperationExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common operations
impl LayoutTree {
    /// Create a preset layout
    pub fn from_preset(preset: LayoutPreset) -> OperationResult<Self> {
        match preset {
            LayoutPreset::SinglePane => Ok(Self::new()),

            LayoutPreset::TwoPaneHorizontal { ratio } => {
                let mut tree = Self::new();
                let root_id = tree.get_all_zone_ids()[0].clone();
                tree.split_zone(&root_id, SplitDirection::Horizontal, ratio, true)?;
                Ok(tree)
            }

            LayoutPreset::TwoPaneVertical { ratio } => {
                let mut tree = Self::new();
                let root_id = tree.get_all_zone_ids()[0].clone();
                tree.split_zone(&root_id, SplitDirection::Vertical, ratio, true)?;
                Ok(tree)
            }

            LayoutPreset::ThreeColumn {
                left_ratio,
                right_ratio,
            } => {
                let mut tree = Self::new();
                let root_id = tree.get_all_zone_ids()[0].clone();

                // First split for left column
                tree.split_zone(&root_id, SplitDirection::Horizontal, left_ratio, true)?;

                // Second split for right column
                let zones = tree.get_all_zone_ids();
                let center_zone = zones
                    .iter()
                    .find(|z| z != &&root_id)
                    .expect("split_zone created at least one new zone besides root");
                tree.split_zone(
                    center_zone,
                    SplitDirection::Horizontal,
                    1.0 - right_ratio,
                    false,
                )?;

                Ok(tree)
            }

            LayoutPreset::Seaglass => {
                use crate::flexible_layout::builder::LayoutBuilder;
                LayoutBuilder::default_seaglass_layout()
                    .map_err(|e| OperationError::InvalidTarget(e.to_string()))
            }
        }
    }
}

/// Common layout presets
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LayoutPreset {
    SinglePane,
    TwoPaneHorizontal { ratio: f32 },
    TwoPaneVertical { ratio: f32 },
    ThreeColumn { left_ratio: f32, right_ratio: f32 },
    Seaglass,
}

/// Builder for complex layout operations
pub struct LayoutOperationBuilder {
    operations: Vec<LayoutOperation>,
}

impl LayoutOperationBuilder {
    /// Create a new operation builder
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add a split zone operation
    pub fn split_zone(
        mut self,
        zone_id: NodeId,
        direction: SplitDirection,
        ratio: f32,
        new_zone_first: bool,
    ) -> Self {
        self.operations.push(LayoutOperation::SplitZone {
            zone_id,
            direction,
            ratio,
            new_zone_first,
        });
        self
    }

    /// Add a move panel operation (appends to end of zone)
    pub fn move_panel(mut self, panel_id: String, target_zone_id: NodeId) -> Self {
        self.operations.push(LayoutOperation::MovePanel {
            panel_id,
            target_zone_id,
            insert_index: None,
        });
        self
    }

    /// Add a move panel operation at a specific index
    pub fn move_panel_at(
        mut self,
        panel_id: String,
        target_zone_id: NodeId,
        insert_index: usize,
    ) -> Self {
        self.operations.push(LayoutOperation::MovePanel {
            panel_id,
            target_zone_id,
            insert_index: Some(insert_index),
        });
        self
    }

    /// Add a resize split operation
    pub fn resize_split(mut self, split_id: NodeId, new_ratio: f32) -> Self {
        self.operations.push(LayoutOperation::ResizeSplit {
            split_id,
            new_ratio,
        });
        self
    }

    /// Build the list of operations
    pub fn build(self) -> Vec<LayoutOperation> {
        self.operations
    }
}

impl Default for LayoutOperationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
