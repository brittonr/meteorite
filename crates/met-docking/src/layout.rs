//! Functional core: docking layout management
//!
//! This module contains the core DockingLayout struct and its operations.

use super::{
    drag_drop::DropResult,
    panels::{FloatingPanel, PanelConfig},
    position::{DockPosition, DockSize},
    zones::DockZone,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during docking layout operations
#[derive(Error, Debug)]
pub enum DockingLayoutError {
    #[error("Target zone '{zone_id}' not found")]
    ZoneNotFound { zone_id: String },

    #[error("Panel '{panel_id}' not found")]
    PanelNotFound { panel_id: String },

    #[error("Panel '{panel_id}' not found in zone '{zone_id}'")]
    PanelNotFoundInZone { panel_id: String, zone_id: String },

    #[error("Floating panel '{panel_id}' not found")]
    FloatingPanelNotFound { panel_id: String },

    #[error("Panel not found in zone")]
    PanelNotInZone,
}

/// Main docking layout containing all zones and panels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockingLayout {
    pub zones: HashMap<String, DockZone>,
    pub floating_panels: HashMap<String, FloatingPanel>,
    pub panel_configs: HashMap<String, PanelConfig>,
}

impl Default for DockingLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Default zone configuration data
struct ZoneConfig {
    id: &'static str,
    position: DockPosition,
    width: f32,
    height: f32,
    min_width: f32,
    min_height: f32,
    default_panel: Option<&'static str>,
}

/// Default panel configuration data
struct DefaultPanelConfig {
    id: &'static str,
    title: &'static str,
    icon: &'static str,
    position: DockPosition,
    can_float: bool,
}

/// Tiger Style: Data-driven configuration for default zones
const DEFAULT_ZONES: &[ZoneConfig] = &[
    ZoneConfig {
        id: "left",
        position: DockPosition::Left,
        width: 20.0,
        height: 60.0,
        min_width: 10.0,
        min_height: 30.0,
        default_panel: Some("nodes_panel"),
    },
    ZoneConfig {
        id: "left_bottom",
        position: DockPosition::LeftBottom,
        width: 20.0,
        height: 40.0,
        min_width: 10.0,
        min_height: 20.0,
        default_panel: Some("validation_results_panel"),
    },
    ZoneConfig {
        id: "right",
        position: DockPosition::Right,
        width: 25.0,
        height: 100.0,
        min_width: 15.0,
        min_height: 100.0,
        default_panel: Some("transformations_panel"),
    },
    ZoneConfig {
        id: "center",
        position: DockPosition::Center,
        width: 55.0,
        height: 70.0,
        min_width: 30.0,
        min_height: 30.0,
        default_panel: Some("main_content"),
    },
    ZoneConfig {
        id: "bottom_left",
        position: DockPosition::BottomLeft,
        width: 50.0,
        height: 30.0,
        min_width: 25.0,
        min_height: 15.0,
        default_panel: Some("flow_view"),
    },
    ZoneConfig {
        id: "bottom_right",
        position: DockPosition::BottomRight,
        width: 50.0,
        height: 30.0,
        min_width: 25.0,
        min_height: 15.0,
        default_panel: Some("validation_rules_panel"),
    },
];

/// Tiger Style: Data-driven configuration for default panels
const DEFAULT_PANELS: &[DefaultPanelConfig] = &[
    DefaultPanelConfig {
        id: "nodes_panel",
        title: "Nodes",
        icon: "📦",
        position: DockPosition::Left,
        can_float: true,
    },
    DefaultPanelConfig {
        id: "transformations_panel",
        title: "Transformations",
        icon: "🔧",
        position: DockPosition::Right,
        can_float: true,
    },
    DefaultPanelConfig {
        id: "main_content",
        title: "Data",
        icon: "📊",
        position: DockPosition::Center,
        can_float: false,
    },
    DefaultPanelConfig {
        id: "flow_view",
        title: "Flow",
        icon: "🔄",
        position: DockPosition::BottomLeft,
        can_float: true,
    },
    DefaultPanelConfig {
        id: "validation_rules_panel",
        title: "Validation Rules",
        icon: "📋",
        position: DockPosition::BottomRight,
        can_float: true,
    },
    DefaultPanelConfig {
        id: "validation_results_panel",
        title: "Validation Results",
        icon: "⚠️",
        position: DockPosition::LeftBottom,
        can_float: true,
    },
];

impl DockingLayout {
    /// Create default zones from configuration
    fn create_default_zones() -> HashMap<String, DockZone> {
        let mut zones = HashMap::with_capacity(DEFAULT_ZONES.len());

        for config in DEFAULT_ZONES {
            let mut zone = DockZone::new(
                config.position,
                DockSize::with_constraints(
                    config.width,
                    config.height,
                    config.min_width,
                    config.min_height,
                    None,
                    None,
                ),
            );

            if let Some(panel_id) = config.default_panel {
                zone.add_panel(panel_id.to_string());
            }

            zones.insert(config.id.to_string(), zone);
        }

        zones
    }

    /// Create default panel configurations from configuration
    fn create_default_panel_configs() -> HashMap<String, PanelConfig> {
        let mut panel_configs = HashMap::with_capacity(DEFAULT_PANELS.len());

        for config in DEFAULT_PANELS {
            let mut panel = PanelConfig::new(config.id, config.title)
                .with_icon(config.icon)
                .with_close_button(false)
                .with_default_position(config.position);

            if !config.can_float {
                panel = panel.with_float_capability(false);
            }

            panel_configs.insert(config.id.to_string(), panel);
        }

        panel_configs
    }

    /// Create a new docking layout with default configuration
    pub fn new() -> Self {
        Self {
            zones: Self::create_default_zones(),
            floating_panels: HashMap::new(),
            panel_configs: Self::create_default_panel_configs(),
        }
    }

    /// Move a floating panel to a zone
    fn move_floating_to_zone(
        &mut self,
        panel_id: &str,
        target_zone_id: &str,
        insert_index: Option<usize>,
    ) -> Result<DropResult, DockingLayoutError> {
        self.floating_panels.remove(panel_id);

        if let Some(target_zone) = self.zones.get_mut(target_zone_id) {
            target_zone.add_panel(panel_id.to_string());
            Ok(DropResult::ZoneChange {
                panel_id: panel_id.to_string(),
                from_zone: None,
                to_zone: target_zone_id.to_string(),
                insert_index,
            })
        } else {
            Err(DockingLayoutError::ZoneNotFound {
                zone_id: target_zone_id.to_string(),
            })
        }
    }

    /// Reorder a panel within its current zone
    fn reorder_within_zone(
        &mut self,
        panel_id: &str,
        zone_id: &str,
        insert_index: Option<usize>,
    ) -> Result<DropResult, DockingLayoutError> {
        if let (Some(zone), Some(index)) = (self.zones.get_mut(zone_id), insert_index) {
            let current_index = zone
                .get_panel_index(panel_id)
                .ok_or(DockingLayoutError::PanelNotInZone)?;

            debug_assert!(index <= u32::MAX as usize);
            if zone.move_panel(panel_id, index as u32) {
                return Ok(DropResult::Reorder {
                    panel_id: panel_id.to_string(),
                    zone_id: zone_id.to_string(),
                    from_index: current_index as usize,
                    to_index: index,
                });
            }
        }
        Ok(DropResult::Cancel {
            panel_id: panel_id.to_string(),
        })
    }

    /// Transfer a panel between different zones
    fn transfer_between_zones(
        &mut self,
        panel_id: &str,
        source_zone_id: &str,
        target_zone_id: &str,
        insert_index: Option<usize>,
    ) -> Result<DropResult, DockingLayoutError> {
        // Remove from source zone
        if let Some(source_zone) = self.zones.get_mut(source_zone_id) {
            source_zone.remove_panel(panel_id);
        }

        // Add to target zone
        if let Some(target_zone) = self.zones.get_mut(target_zone_id) {
            if let Some(index) = insert_index {
                debug_assert!(index <= u32::MAX as usize);
                target_zone.insert_panel(panel_id.to_string(), index as u32);
            } else {
                target_zone.add_panel(panel_id.to_string());
            }

            Ok(DropResult::ZoneChange {
                panel_id: panel_id.to_string(),
                from_zone: Some(source_zone_id.to_string()),
                to_zone: target_zone_id.to_string(),
                insert_index,
            })
        } else {
            Err(DockingLayoutError::ZoneNotFound {
                zone_id: target_zone_id.to_string(),
            })
        }
    }

    /// Move a panel between zones
    pub fn move_panel(
        &mut self,
        panel_id: &str,
        target_zone_id: &str,
        insert_index: Option<usize>,
    ) -> Result<DropResult, DockingLayoutError> {
        let source_zone_id = self.find_panel_zone(panel_id);

        // Handle floating panel to zone
        if source_zone_id.is_none() && self.floating_panels.contains_key(panel_id) {
            return self.move_floating_to_zone(panel_id, target_zone_id, insert_index);
        }

        let source_zone_id = source_zone_id.ok_or_else(|| DockingLayoutError::PanelNotFound {
            panel_id: panel_id.to_string(),
        })?;

        // Handle reordering within same zone
        if source_zone_id == target_zone_id {
            return self.reorder_within_zone(panel_id, &source_zone_id, insert_index);
        }

        // Move between different zones
        self.transfer_between_zones(panel_id, &source_zone_id, target_zone_id, insert_index)
    }

    /// Find which zone contains a panel
    pub fn find_panel_zone(&self, panel_id: &str) -> Option<String> {
        for (zone_id, zone) in &self.zones {
            if zone.contains_panel(panel_id) {
                return Some(zone_id.clone());
            }
        }
        None
    }

    /// Set the active panel in a zone
    pub fn set_active_panel(
        &mut self,
        zone_id: &str,
        panel_id: &str,
    ) -> Result<(), DockingLayoutError> {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            if zone.set_active_panel(panel_id.to_string()) {
                Ok(())
            } else {
                Err(DockingLayoutError::PanelNotFoundInZone {
                    panel_id: panel_id.to_string(),
                    zone_id: zone_id.to_string(),
                })
            }
        } else {
            Err(DockingLayoutError::ZoneNotFound {
                zone_id: zone_id.to_string(),
            })
        }
    }

    /// Float a panel from a zone
    pub fn float_panel(
        &mut self,
        panel_id: &str,
        x: f32,
        y: f32,
    ) -> Result<DropResult, DockingLayoutError> {
        let source_zone_id = self.find_panel_zone(panel_id);

        // Remove from zone if found
        if let Some(zone_id) = &source_zone_id {
            if let Some(zone) = self.zones.get_mut(zone_id) {
                zone.remove_panel(panel_id);
            }
        }

        // Create floating panel
        let floating_panel = FloatingPanel::new(panel_id, x, y, 300.0, 200.0);
        self.floating_panels
            .insert(panel_id.to_string(), floating_panel);

        Ok(DropResult::Float {
            panel_id: panel_id.to_string(),
            from_zone: source_zone_id,
            x,
            y,
        })
    }

    /// Dock a floating panel to a zone
    pub fn dock_floating_panel(
        &mut self,
        panel_id: &str,
        target_zone_id: &str,
    ) -> Result<DropResult, DockingLayoutError> {
        if !self.floating_panels.contains_key(panel_id) {
            return Err(DockingLayoutError::FloatingPanelNotFound {
                panel_id: panel_id.to_string(),
            });
        }

        self.floating_panels.remove(panel_id);

        if let Some(target_zone) = self.zones.get_mut(target_zone_id) {
            target_zone.add_panel(panel_id.to_string());
            Ok(DropResult::ZoneChange {
                panel_id: panel_id.to_string(),
                from_zone: None,
                to_zone: target_zone_id.to_string(),
                insert_index: None,
            })
        } else {
            Err(DockingLayoutError::ZoneNotFound {
                zone_id: target_zone_id.to_string(),
            })
        }
    }

    /// Add a new panel configuration
    pub fn add_panel_config(&mut self, config: PanelConfig) {
        self.panel_configs.insert(config.id.clone(), config);
    }

    /// Get panel configuration
    pub fn get_panel_config(&self, panel_id: &str) -> Option<&PanelConfig> {
        self.panel_configs.get(panel_id)
    }

    /// Get all zone IDs
    pub fn get_zone_ids(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }

    /// Get zone by ID
    pub fn get_zone(&self, zone_id: &str) -> Option<&DockZone> {
        self.zones.get(zone_id)
    }

    /// Get mutable zone by ID
    pub fn get_zone_mut(&mut self, zone_id: &str) -> Option<&mut DockZone> {
        self.zones.get_mut(zone_id)
    }

    /// Check if panel is floating
    pub fn is_panel_floating(&self, panel_id: &str) -> bool {
        self.floating_panels.contains_key(panel_id)
    }

    /// Get floating panel
    pub fn get_floating_panel(&self, panel_id: &str) -> Option<&FloatingPanel> {
        self.floating_panels.get(panel_id)
    }

    /// Get mutable floating panel
    pub fn get_floating_panel_mut(&mut self, panel_id: &str) -> Option<&mut FloatingPanel> {
        self.floating_panels.get_mut(panel_id)
    }
}
