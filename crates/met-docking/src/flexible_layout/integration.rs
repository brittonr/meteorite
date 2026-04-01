//! Functional core: legacy-to-flexible layout conversion
//!
//! This module provides utilities for converting between the old fixed-zone
//! layout system and the new flexible tree-based layout system.

use super::{
    builder::LayoutBuilder,
    node::{NodeId, ZoneContent},
    tree::{LayoutTree, TreeError},
};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during integration
#[derive(Error, Debug)]
pub enum IntegrationError {
    #[error("Tree error: {0}")]
    TreeError(#[from] TreeError),

    #[error("Legacy zone not found: {0}")]
    LegacyZoneNotFound(String),

    #[error("Invalid mapping: {0}")]
    InvalidMapping(String),
}

/// Legacy zone state for migration
#[derive(Debug, Clone)]
pub struct LegacyZoneState {
    /// Panels in each zone
    pub zone_panels: HashMap<String, Vec<String>>,

    /// Active panels per zone
    pub active_panels: HashMap<String, String>,

    /// Zone sizes (percentages)
    pub zone_sizes: HashMap<String, f32>,
}

impl Default for LegacyZoneState {
    fn default() -> Self {
        Self::new()
    }
}

impl LegacyZoneState {
    /// Create empty legacy state
    pub fn new() -> Self {
        Self {
            zone_panels: HashMap::new(),
            active_panels: HashMap::new(),
            zone_sizes: HashMap::new(),
        }
    }

    /// Add a panel to a zone
    pub fn add_panel(&mut self, zone: String, panel: String) {
        self.zone_panels.entry(zone).or_default().push(panel);
    }

    /// Set active panel for a zone
    pub fn set_active(&mut self, zone: String, panel: String) {
        self.active_panels.insert(zone, panel);
    }

    /// Set zone size
    pub fn set_size(&mut self, zone: String, size: f32) {
        self.zone_sizes.insert(zone, size);
    }
}

/// Convert a legacy zone state to a flexible LayoutTree
pub fn convert_legacy_state(legacy: &LegacyZoneState) -> Result<LayoutTree, IntegrationError> {
    // Create the default layout
    let mut tree = LayoutBuilder::default_seaglass_layout()
        .map_err(|e| IntegrationError::InvalidMapping(e.to_string()))?;

    // Migrate panels to their zones
    for (zone_name, panels) in &legacy.zone_panels {
        // Find the zone by metadata
        let zone_id = find_zone_by_legacy_name(&tree, zone_name)
            .ok_or_else(|| IntegrationError::LegacyZoneNotFound(zone_name.clone()))?;

        // Add panels
        for panel in panels {
            tree.add_panel_to_zone(&zone_id, panel.clone())?;
        }

        // Set active panel if exists
        if let Some(active) = legacy.active_panels.get(zone_name) {
            tree.set_active_panel(&zone_id, Some(active.clone()))?;
        }
    }

    Ok(tree)
}

/// Find zone by legacy name in tree
fn find_zone_by_legacy_name(tree: &LayoutTree, legacy_name: &str) -> Option<NodeId> {
    for zone_id in tree.get_all_zone_ids() {
        if let Some(zone) = tree.find_zone(&zone_id) {
            if let Some(content) = zone.zone_content() {
                if content.metadata.get("legacy_name") == Some(&legacy_name.to_string()) {
                    return Some(zone_id);
                }
            }
        }
    }
    None
}

/// Adapter to use flexible layout with existing zone-based code
pub struct FlexibleLayoutAdapter {
    /// The flexible layout tree
    pub tree: LayoutTree,

    /// Mapping from legacy zone names to flexible NodeIds
    legacy_mappings: HashMap<String, NodeId>,

    /// Reverse mapping from NodeIds to legacy names
    node_to_legacy: HashMap<NodeId, String>,

    /// Floating panels (not in the tree)
    floating_panels: HashMap<String, (f32, f32)>, // panel_id -> (x, y)
}

impl FlexibleLayoutAdapter {
    /// Create a new adapter with default Seaglass layout
    pub fn new() -> Result<Self, IntegrationError> {
        let tree = LayoutBuilder::default_seaglass_layout()
            .map_err(|e| IntegrationError::InvalidMapping(e.to_string()))?;

        let mut adapter = Self {
            tree,
            legacy_mappings: HashMap::new(),
            node_to_legacy: HashMap::new(),
            floating_panels: HashMap::new(),
        };

        adapter.build_mappings();
        Ok(adapter)
    }

    /// Create adapter from existing tree
    pub fn from_tree(tree: LayoutTree) -> Self {
        let mut adapter = Self {
            tree,
            legacy_mappings: HashMap::new(),
            node_to_legacy: HashMap::new(),
            floating_panels: HashMap::new(),
        };

        adapter.build_mappings();
        adapter
    }

    /// Create adapter from legacy state
    pub fn from_legacy(legacy: &LegacyZoneState) -> Result<Self, IntegrationError> {
        let tree = convert_legacy_state(legacy)?;
        Ok(Self::from_tree(tree))
    }

    /// Build mappings from zone metadata
    pub fn build_mappings(&mut self) {
        self.legacy_mappings.clear();
        self.node_to_legacy.clear();

        for zone_id in self.tree.get_all_zone_ids() {
            if let Some(zone) = self.tree.find_zone(&zone_id) {
                if let Some(content) = zone.zone_content() {
                    if let Some(legacy_name) = content.metadata.get("legacy_name") {
                        self.legacy_mappings
                            .insert(legacy_name.clone(), zone_id.clone());
                        self.node_to_legacy.insert(zone_id, legacy_name.clone());
                    }
                }
            }
        }
    }

    /// Get the layout tree
    pub fn tree(&self) -> &LayoutTree {
        &self.tree
    }

    /// Get mutable layout tree
    pub fn tree_mut(&mut self) -> &mut LayoutTree {
        &mut self.tree
    }

    /// Get zone ID by legacy name
    pub fn get_zone_id(&self, legacy_name: &str) -> Option<&NodeId> {
        self.legacy_mappings.get(legacy_name)
    }

    /// Get legacy name for zone ID
    pub fn get_legacy_name(&self, zone_id: &NodeId) -> Option<&String> {
        self.node_to_legacy.get(zone_id)
    }

    /// Move a panel (compatible with legacy API)
    ///
    /// If `insert_index` is `Some(idx)`, the panel is inserted at that position in the zone.
    /// If `None`, the panel is appended to the end.
    pub fn move_panel(
        &mut self,
        panel_id: &str,
        target_zone: &str,
        insert_index: Option<usize>,
    ) -> Result<(), IntegrationError> {
        // Check if panel is floating
        if self.floating_panels.contains_key(panel_id) {
            // Dock the floating panel
            self.floating_panels.remove(panel_id);
            let zone_id = self
                .get_zone_id(target_zone)
                .cloned()
                .ok_or_else(|| IntegrationError::LegacyZoneNotFound(target_zone.to_string()))?;

            // Add at specified index or append
            if let Some(idx) = insert_index {
                self.tree
                    .add_panel_to_zone_at(&zone_id, panel_id.to_string(), idx)?;
            } else {
                self.tree
                    .add_panel_to_zone(&zone_id, panel_id.to_string())?;
            }
            return Ok(());
        }

        // Regular panel move
        let target_id = self
            .get_zone_id(target_zone)
            .cloned()
            .ok_or_else(|| IntegrationError::LegacyZoneNotFound(target_zone.to_string()))?;
        self.tree.move_panel(panel_id, &target_id, insert_index)?;
        Ok(())
    }

    /// Float a panel (compatible with legacy API)
    pub fn float_panel(&mut self, panel_id: &str, x: f32, y: f32) -> Result<(), IntegrationError> {
        // Remove from tree
        self.tree.remove_panel(panel_id)?;

        // Add to floating panels
        self.floating_panels.insert(panel_id.to_string(), (x, y));
        Ok(())
    }

    /// Find which zone contains a panel (returns legacy name)
    pub fn find_panel_zone(&self, panel_id: &str) -> Option<String> {
        if let Some(zone_id) = self.tree.find_panel_zone(panel_id) {
            self.get_legacy_name(&zone_id).cloned()
        } else {
            None
        }
    }

    /// Get all legacy zone names
    pub fn get_legacy_zones(&self) -> Vec<String> {
        self.legacy_mappings.keys().cloned().collect()
    }

    /// Check if panel is floating
    pub fn is_panel_floating(&self, panel_id: &str) -> bool {
        self.floating_panels.contains_key(panel_id)
    }

    /// Add panel to zone by legacy name
    pub fn add_panel_to_zone(
        &mut self,
        legacy_name: &str,
        panel_id: String,
    ) -> Result<(), IntegrationError> {
        let zone_id = self
            .get_zone_id(legacy_name)
            .cloned()
            .ok_or_else(|| IntegrationError::LegacyZoneNotFound(legacy_name.to_string()))?;

        self.tree.add_panel_to_zone(&zone_id, panel_id)?;
        Ok(())
    }

    /// Remove panel
    pub fn remove_panel(&mut self, panel_id: &str) -> Result<(), IntegrationError> {
        // Check if floating first
        if self.floating_panels.remove(panel_id).is_some() {
            return Ok(());
        }

        self.tree.remove_panel(panel_id)?;
        Ok(())
    }

    /// Get zone content by legacy name
    pub fn get_zone_content(&self, legacy_name: &str) -> Option<&ZoneContent> {
        self.get_zone_id(legacy_name)
            .and_then(|id| self.tree.find_zone(id))
            .and_then(|node| node.zone_content())
    }

    /// Set active panel by legacy zone name
    pub fn set_active_panel(
        &mut self,
        legacy_name: &str,
        panel_id: Option<String>,
    ) -> Result<(), IntegrationError> {
        let zone_id = self
            .get_zone_id(legacy_name)
            .cloned()
            .ok_or_else(|| IntegrationError::LegacyZoneNotFound(legacy_name.to_string()))?;

        self.tree.set_active_panel(&zone_id, panel_id)?;
        Ok(())
    }

    /// Get a clone of the current layout tree
    pub fn get_layout_tree(&self) -> LayoutTree {
        self.tree.clone()
    }

    /// Replace the entire layout tree
    pub fn set_layout_tree(&mut self, tree: LayoutTree) {
        self.tree = tree;
        self.build_mappings();
    }
}

impl Default for FlexibleLayoutAdapter {
    fn default() -> Self {
        Self::new().expect("Failed to create default layout")
    }
}
