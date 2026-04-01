//! Functional core: legacy layout migration helpers

use super::{FlexibleLayoutAdapter, IntegrationError, LayoutBuilder, LayoutTree, LegacyZoneState};
use crate::DockingLayout;
use std::collections::HashMap;

/// Migration helper for converting legacy layouts
pub struct LayoutMigration;

impl LayoutMigration {
    /// Convert a legacy DockingLayout to LegacyZoneState
    pub fn from_docking_layout(layout: &DockingLayout) -> LegacyZoneState {
        let mut state = LegacyZoneState::new();

        // Migrate zones and panels
        for (zone_id, zone) in &layout.zones {
            // Add panels to the zone
            for panel in &zone.panels {
                state.add_panel(zone_id.clone(), panel.clone());
            }

            // Set active panel if exists
            if let Some(active) = &zone.active_panel {
                state.set_active(zone_id.clone(), active.clone());
            }

            // Store zone size (simplified - in practice would calculate from positions)
            let size = match zone_id.as_str() {
                "left" => 20.0,                         // 20% width
                "right" => 25.0,                        // 25% width
                "bottom_left" | "bottom_right" => 50.0, // 50% of bottom
                _ => 100.0,
            };
            state.set_size(zone_id.clone(), size);
        }

        state
    }

    // NOTE: This method requires access to Signal values from outside a component context
    // which is not directly supported in Dioxus. If migration from a live DockingService
    // is needed, it should be done from within a component that has access to the signals.
    /*
    /// Convert current docking service state to flexible layout
    pub fn migrate_docking_service(
        docking_service: &crate::services::docking_service::DockingService // commented out block
    ) -> Result<FlexibleLayoutAdapter, IntegrationError> {
        let layout = docking_service.layout.peek().clone();
        let legacy_state = Self::from_docking_layout(&layout);
        FlexibleLayoutAdapter::from_legacy(&legacy_state)
    }
    */

    /// Create a flexible layout adapter with current panel positions
    pub fn create_with_current_panels(
        panel_positions: HashMap<String, String>,
    ) -> Result<FlexibleLayoutAdapter, IntegrationError> {
        // Start with default layout
        let mut adapter = FlexibleLayoutAdapter::new()?;

        // Add panels to their zones
        for (panel_id, zone_name) in panel_positions {
            adapter.add_panel_to_zone(&zone_name, panel_id)?;
        }

        Ok(adapter)
    }

    /// Check if a layout needs migration
    pub fn needs_migration(layout_json: &str) -> bool {
        // Check if the JSON contains old-style zone definitions
        layout_json.contains("\"zones\"") && !layout_json.contains("\"tree\"")
    }

    /// Migrate a serialized layout
    pub fn migrate_serialized(old_layout_json: &str) -> Result<String, IntegrationError> {
        // Parse old layout
        let old_layout: DockingLayout = serde_json::from_str(old_layout_json).map_err(|e| {
            IntegrationError::InvalidMapping(format!("Failed to parse old layout: {e}"))
        })?;

        // Convert to legacy state
        let legacy_state = Self::from_docking_layout(&old_layout);

        // Create flexible layout
        let adapter = FlexibleLayoutAdapter::from_legacy(&legacy_state)?;

        // Serialize the tree
        serde_json::to_string(&adapter.tree).map_err(|e| {
            IntegrationError::InvalidMapping(format!("Failed to serialize new layout: {e}"))
        })
    }

    /// Get default panel assignments for zones
    pub fn default_panel_assignments() -> HashMap<String, Vec<String>> {
        let mut assignments = HashMap::new();

        // Menu bar zone at the top
        assignments.insert("menu_zone".to_string(), vec!["menu_bar".to_string()]);

        // Left sidebar zones (3 zones)
        assignments.insert("left_top".to_string(), vec!["nodes_panel".to_string()]);
        assignments.insert(
            "left_middle".to_string(),
            vec!["validation_results_panel".to_string()],
        );
        assignments.insert("left_bottom".to_string(), vec![]); // Empty zone for future use

        // Center zone panels (main table view)
        assignments.insert(
            "center".to_string(),
            vec!["main_content".to_string(), "flow_view".to_string()],
        );

        // Right sidebar zones (3 zones)
        assignments.insert(
            "right_top".to_string(),
            vec!["transformations_panel".to_string()],
        );
        assignments.insert(
            "right_middle".to_string(),
            vec!["validation_rules_panel".to_string()],
        );
        assignments.insert("right_bottom".to_string(), vec![]); // Empty zone for future use

        assignments
    }

    /// Create a layout with specific panel arrangement
    pub fn create_custom_layout(
        panel_arrangement: HashMap<String, Vec<String>>,
    ) -> Result<LayoutTree, IntegrationError> {
        // Start with default structure
        let mut tree = LayoutBuilder::default_seaglass_layout()
            .map_err(|e| IntegrationError::InvalidMapping(e.to_string()))?;

        // Add panels to zones
        for (zone_name, panels) in panel_arrangement {
            if let Some(zone_id) = find_zone_by_legacy_name(&tree, &zone_name) {
                for panel in panels {
                    tree.add_panel_to_zone(&zone_id, panel)?;
                }
            }
        }

        Ok(tree)
    }
}

/// Helper function to find zone by legacy name
fn find_zone_by_legacy_name(tree: &LayoutTree, legacy_name: &str) -> Option<super::NodeId> {
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
