//! Functional core: layout preset definitions and builder
//!
//! This module provides a preset system for saving and loading common layout configurations,
//! as well as a builder API for creating custom layouts programmatically.

use super::{
    layout::DockingLayout,
    panels::PanelConfig,
    position::{DockPosition, DockSize},
    zones::DockZone,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur when working with presets
#[derive(Error, Debug)]
pub enum PresetError {
    #[error("Preset '{name}' not found")]
    PresetNotFound { name: String },

    #[error("Preset '{name}' already exists")]
    PresetAlreadyExists { name: String },

    #[error("Invalid preset configuration: {reason}")]
    InvalidConfiguration { reason: String },

    #[error("Failed to serialize preset: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// A saved layout preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutPreset {
    /// Unique identifier for the preset
    pub id: String,
    /// Display name for the preset
    pub name: String,
    /// Description of the preset
    pub description: String,
    /// Whether this is a system preset (cannot be deleted)
    pub is_system: bool,
    /// The actual layout configuration
    pub layout: DockingLayout,
    /// Metadata about when this preset was created/modified
    pub metadata: PresetMetadata,
}

/// Metadata for a preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetMetadata {
    /// When the preset was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the preset was last modified
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Version of the preset format
    pub version: String,
    /// Tags for categorizing presets
    pub tags: Vec<String>,
}

impl PresetMetadata {
    /// Pure constructor accepting an explicit timestamp (functional core)
    pub fn new_with_timestamp(now: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            created_at: now,
            modified_at: now,
            version: "1.0.0".to_string(),
            tags: Vec::new(),
        }
    }
}

impl Default for PresetMetadata {
    fn default() -> Self {
        // FCIS: timing at shell/I/O boundary — delegates to pure constructor
        Self::new_with_timestamp(chrono::Utc::now())
    }
}

/// Registry for managing layout presets
#[derive(Debug, Clone)]
pub struct PresetRegistry {
    presets: HashMap<String, LayoutPreset>,
}

impl Default for PresetRegistry {
    fn default() -> Self {
        let mut registry = Self {
            presets: HashMap::new(),
        };

        // Register default presets
        registry.register_default_presets();
        registry
    }
}

impl PresetRegistry {
    /// Create a new empty preset registry
    pub fn new() -> Self {
        Self {
            presets: HashMap::new(),
        }
    }

    /// Register all default system presets
    fn register_default_presets(&mut self) {
        // Default Seaglass layout
        let default_preset = LayoutPreset {
            id: "default".to_string(),
            name: "Default Layout".to_string(),
            description: "The standard Seaglass layout with all panels visible".to_string(),
            is_system: true,
            layout: create_default_layout(),
            metadata: PresetMetadata {
                tags: vec!["default".to_string(), "standard".to_string()],
                ..Default::default()
            },
        };
        self.presets.insert("default".to_string(), default_preset);

        // Minimal layout
        let minimal_preset = LayoutPreset {
            id: "minimal".to_string(),
            name: "Minimal".to_string(),
            description: "Simplified layout with only essential panels".to_string(),
            is_system: true,
            layout: create_minimal_layout(),
            metadata: PresetMetadata {
                tags: vec!["minimal".to_string(), "simple".to_string()],
                ..Default::default()
            },
        };
        self.presets.insert("minimal".to_string(), minimal_preset);

        // Wide layout for large screens
        let wide_preset = LayoutPreset {
            id: "wide".to_string(),
            name: "Wide Screen".to_string(),
            description: "Optimized for wide monitors with side-by-side panels".to_string(),
            is_system: true,
            layout: create_wide_layout(),
            metadata: PresetMetadata {
                tags: vec!["wide".to_string(), "ultrawide".to_string()],
                ..Default::default()
            },
        };
        self.presets.insert("wide".to_string(), wide_preset);

        // Focus mode
        let focus_preset = LayoutPreset {
            id: "focus".to_string(),
            name: "Focus Mode".to_string(),
            description: "Maximum space for data table with hidden sidebars".to_string(),
            is_system: true,
            layout: create_focus_layout(),
            metadata: PresetMetadata {
                tags: vec!["focus".to_string(), "data".to_string()],
                ..Default::default()
            },
        };
        self.presets.insert("focus".to_string(), focus_preset);
    }

    /// Get a preset by ID
    pub fn get(&self, id: &str) -> Option<&LayoutPreset> {
        self.presets.get(id)
    }

    /// Get all presets
    pub fn list(&self) -> Vec<&LayoutPreset> {
        self.presets.values().collect()
    }

    /// Get presets filtered by tags
    pub fn list_by_tags(&self, tags: &[String]) -> Vec<&LayoutPreset> {
        self.presets
            .values()
            .filter(|preset| tags.iter().any(|tag| preset.metadata.tags.contains(tag)))
            .collect()
    }

    /// Add a custom preset
    pub fn add_preset(&mut self, preset: LayoutPreset) -> Result<(), PresetError> {
        if self.presets.contains_key(&preset.id) {
            return Err(PresetError::PresetAlreadyExists {
                name: preset.id.clone(),
            });
        }

        self.presets.insert(preset.id.clone(), preset);
        Ok(())
    }

    /// Update an existing preset with an explicit timestamp (functional core)
    pub fn update_preset_with_timestamp(
        &mut self,
        id: &str,
        layout: DockingLayout,
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), PresetError> {
        match self.presets.get_mut(id) {
            Some(preset) => {
                if preset.is_system {
                    return Err(PresetError::InvalidConfiguration {
                        reason: "Cannot modify system presets".to_string(),
                    });
                }
                preset.layout = layout;
                preset.metadata.modified_at = now;
                Ok(())
            }
            None => Err(PresetError::PresetNotFound {
                name: id.to_string(),
            }),
        }
    }

    /// Update an existing preset (imperative shell — captures current time)
    pub fn update_preset(&mut self, id: &str, layout: DockingLayout) -> Result<(), PresetError> {
        // FCIS: timing at shell/I/O boundary — delegates to pure variant
        self.update_preset_with_timestamp(id, layout, chrono::Utc::now())
    }

    /// Remove a custom preset
    pub fn remove_preset(&mut self, id: &str) -> Result<(), PresetError> {
        match self.presets.get(id) {
            Some(preset) => {
                if preset.is_system {
                    return Err(PresetError::InvalidConfiguration {
                        reason: "Cannot remove system presets".to_string(),
                    });
                }
                self.presets.remove(id);
                Ok(())
            }
            None => Err(PresetError::PresetNotFound {
                name: id.to_string(),
            }),
        }
    }

    /// Export presets to JSON
    pub fn export(&self) -> Result<String, PresetError> {
        let custom_presets: HashMap<_, _> = self
            .presets
            .iter()
            .filter(|(_, preset)| !preset.is_system)
            .collect();

        serde_json::to_string_pretty(&custom_presets).map_err(PresetError::SerializationError)
    }

    /// Import presets from JSON
    pub fn import(&mut self, json: &str) -> Result<usize, PresetError> {
        let imported: HashMap<String, LayoutPreset> =
            serde_json::from_str(json).map_err(PresetError::SerializationError)?;

        let mut count = 0;
        for (_, mut preset) in imported {
            preset.is_system = false; // Ensure imported presets are not system presets
            if !self.presets.contains_key(&preset.id) {
                self.presets.insert(preset.id.clone(), preset);
                count += 1;
            }
        }

        Ok(count)
    }
}

/// Builder for creating custom layouts
pub struct LayoutBuilder {
    zones: HashMap<String, DockZone>,
    panel_configs: HashMap<String, PanelConfig>,
    floating_panels: HashMap<String, super::panels::FloatingPanel>,
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutBuilder {
    /// Create a new layout builder
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
            panel_configs: HashMap::new(),
            floating_panels: HashMap::new(),
        }
    }

    /// Add a zone to the layout
    pub fn add_zone(mut self, id: &str, position: DockPosition, size: DockSize) -> Self {
        self.zones
            .insert(id.to_string(), DockZone::new(position, size));
        self
    }

    /// Add a zone with specific size constraints
    pub fn add_zone_with_constraints(
        mut self,
        id: &str,
        position: DockPosition,
        width: f32,
        height: f32,
        min_width: f32,
        min_height: f32,
    ) -> Self {
        let size = DockSize::with_constraints(width, height, min_width, min_height, None, None);
        self.zones
            .insert(id.to_string(), DockZone::new(position, size));
        self
    }

    /// Add a panel configuration
    pub fn add_panel_config(mut self, config: PanelConfig) -> Self {
        self.panel_configs.insert(config.id.clone(), config);
        self
    }

    /// Add a panel to a zone
    pub fn add_panel_to_zone(mut self, panel_id: &str, zone_id: &str) -> Self {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            zone.add_panel(panel_id.to_string());
        }
        self
    }

    /// Set the active panel in a zone
    pub fn set_active_panel(mut self, zone_id: &str, panel_id: &str) -> Self {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            zone.set_active_panel(panel_id.to_string());
        }
        self
    }

    /// Build the final DockingLayout
    pub fn build(self) -> DockingLayout {
        DockingLayout {
            zones: self.zones,
            panel_configs: self.panel_configs,
            floating_panels: self.floating_panels,
        }
    }
}

/// Create the default Seaglass layout
fn create_default_layout() -> DockingLayout {
    let mut builder = LayoutBuilder::new();
    builder = create_default_layout_zones(builder);
    builder = create_default_layout_panels(builder);
    builder = create_default_layout_configs(builder);
    builder.build()
}

/// Add zones with constraints to the layout builder
fn create_default_layout_zones(builder: LayoutBuilder) -> LayoutBuilder {
    builder
        // Left sidebar - 20% width
        .add_zone_with_constraints("left", DockPosition::Left, 20.0, 60.0, 10.0, 30.0)
        .add_zone_with_constraints(
            "left_bottom",
            DockPosition::LeftBottom,
            20.0,
            40.0,
            10.0,
            20.0,
        )
        // Right sidebar - 25% width
        .add_zone_with_constraints("right", DockPosition::Right, 25.0, 100.0, 15.0, 100.0)
        // Center area - remaining space
        .add_zone_with_constraints("center", DockPosition::Center, 55.0, 70.0, 30.0, 30.0)
        // Bottom area - 30% height, split 50/50
        .add_zone_with_constraints(
            "bottom_left",
            DockPosition::BottomLeft,
            50.0,
            30.0,
            25.0,
            15.0,
        )
        .add_zone_with_constraints(
            "bottom_right",
            DockPosition::BottomRight,
            50.0,
            30.0,
            25.0,
            15.0,
        )
}

/// Add panels to their respective zones
fn create_default_layout_panels(builder: LayoutBuilder) -> LayoutBuilder {
    builder
        .add_panel_to_zone("nodes_panel", "left")
        .add_panel_to_zone("validation_results_panel", "left_bottom")
        .add_panel_to_zone("transformations_panel", "right")
        .add_panel_to_zone("main_content", "center")
        .add_panel_to_zone("flow_view", "bottom_left")
}

/// Add panel configurations
fn create_default_layout_configs(builder: LayoutBuilder) -> LayoutBuilder {
    builder
        .add_panel_config(
            PanelConfig::new("nodes_panel", "Nodes")
                .with_icon("📦")
                .with_close_button(false)
                .with_default_position(DockPosition::Left),
        )
        .add_panel_config(
            PanelConfig::new("transformations_panel", "Transformations")
                .with_icon("🔧")
                .with_close_button(false)
                .with_default_position(DockPosition::Right),
        )
        .add_panel_config(
            PanelConfig::new("main_content", "Data")
                .with_icon("📊")
                .with_close_button(false)
                .with_float_capability(false)
                .with_default_position(DockPosition::Center),
        )
        .add_panel_config(
            PanelConfig::new("flow_view", "Flow")
                .with_icon("🔄")
                .with_close_button(false)
                .with_default_position(DockPosition::BottomLeft),
        )
        .add_panel_config(
            PanelConfig::new("validation_results_panel", "Validation Results")
                .with_icon("⚠️")
                .with_close_button(false)
                .with_default_position(DockPosition::LeftBottom),
        )
        .add_panel_config(
            PanelConfig::new("validation_rules_panel", "Validation Rules")
                .with_icon("📋")
                .with_close_button(false)
                .with_default_position(DockPosition::BottomRight),
        )
}

/// Create a minimal layout with only essential panels
fn create_minimal_layout() -> DockingLayout {
    LayoutBuilder::new()
        // Left sidebar for nodes only
        .add_zone_with_constraints("left", DockPosition::Left, 20.0, 100.0, 15.0, 100.0)
        // Center takes most space
        .add_zone_with_constraints("center", DockPosition::Center, 80.0, 100.0, 50.0, 100.0)
        // Add panels
        .add_panel_to_zone("nodes_panel", "left")
        .add_panel_to_zone("main_content", "center")
        // Panel configs
        .add_panel_config(
            PanelConfig::new("nodes_panel", "Nodes")
                .with_icon("📦")
                .with_close_button(true)
                .with_default_position(DockPosition::Left),
        )
        .add_panel_config(
            PanelConfig::new("main_content", "Data")
                .with_icon("📊")
                .with_close_button(false)
                .with_float_capability(false)
                .with_default_position(DockPosition::Center),
        )
        .build()
}

/// Create a wide layout optimized for ultrawide monitors
fn create_wide_layout() -> DockingLayout {
    LayoutBuilder::new()
        // Narrow left sidebar
        .add_zone_with_constraints("left", DockPosition::Left, 15.0, 100.0, 10.0, 100.0)
        // Wide center area
        .add_zone_with_constraints("center", DockPosition::Center, 60.0, 100.0, 40.0, 100.0)
        // Right panel for transformations
        .add_zone_with_constraints("right", DockPosition::Right, 25.0, 100.0, 15.0, 100.0)
        // Add panels
        .add_panel_to_zone("nodes_panel", "left")
        .add_panel_to_zone("main_content", "center")
        .add_panel_to_zone("transformations_panel", "right")
        .add_panel_to_zone("flow_view", "right")
        // Panel configs
        .add_panel_config(
            PanelConfig::new("nodes_panel", "Nodes")
                .with_icon("📦")
                .with_close_button(false)
                .with_default_position(DockPosition::Left),
        )
        .add_panel_config(
            PanelConfig::new("main_content", "Data")
                .with_icon("📊")
                .with_close_button(false)
                .with_float_capability(false)
                .with_default_position(DockPosition::Center),
        )
        .add_panel_config(
            PanelConfig::new("transformations_panel", "Transformations")
                .with_icon("🔧")
                .with_close_button(true)
                .with_default_position(DockPosition::Right),
        )
        .add_panel_config(
            PanelConfig::new("flow_view", "Flow")
                .with_icon("🔄")
                .with_close_button(true)
                .with_default_position(DockPosition::Right),
        )
        .build()
}

/// Create a focus layout with maximum space for data
fn create_focus_layout() -> DockingLayout {
    LayoutBuilder::new()
        // Only center zone
        .add_zone_with_constraints("center", DockPosition::Center, 100.0, 100.0, 100.0, 100.0)
        // Add only data table
        .add_panel_to_zone("main_content", "center")
        // Panel config
        .add_panel_config(
            PanelConfig::new("main_content", "Data")
                .with_icon("📊")
                .with_close_button(false)
                .with_float_capability(false)
                .with_default_position(DockPosition::Center),
        )
        .build()
}
