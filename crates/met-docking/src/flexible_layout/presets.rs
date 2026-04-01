//! Functional core: predefined layout presets

use super::{BuilderError, LayoutBuilder, LayoutTree};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// Available layout presets
#[derive(Debug, Clone)]
pub enum LayoutPreset {
    /// Default Seaglass layout (matches legacy layout)
    Default,
    /// Focus mode - single large center area
    Focus,
    /// Split view - two equal panels
    SplitView,
    /// Debug layout - multiple panels for debugging
    Debug,
    /// Custom preset loaded from storage
    Custom(String),
}

/// Key prefix for preset storage
pub const PRESET_STORAGE_KEY_PREFIX: &str = "seaglass_layout_preset_";

/// Serializable custom preset data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPresetData {
    /// Name of the preset
    pub name: String,
    /// The layout tree
    pub layout: LayoutTree,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
    /// Timestamp when saved (Unix millis)
    #[serde(default)]
    pub saved_at: Option<u64>,
}

impl CustomPresetData {
    /// Pure constructor accepting an explicit timestamp (functional core)
    pub fn new_with_timestamp(name: String, layout: LayoutTree, timestamp_millis: u64) -> Self {
        Self {
            name,
            layout,
            description: None,
            saved_at: Some(timestamp_millis),
        }
    }

    /// Create new custom preset data (imperative shell — captures current time)
    pub fn new(name: String, layout: LayoutTree) -> Self {
        // FCIS: timing at shell/I/O boundary — delegates to pure constructor
        Self::new_with_timestamp(name, layout, current_timestamp_millis())
    }

    /// Create with description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get the storage key for this preset
    pub fn storage_key(&self) -> String {
        format!("{}{}", PRESET_STORAGE_KEY_PREFIX, self.name)
    }
}

/// Get current timestamp in milliseconds (cross-platform)
// FCIS: impure — captures wall-clock time
fn current_timestamp_millis() -> u64 {
    chrono::Utc::now().timestamp_millis() as u64
}

impl LayoutPreset {
    /// Build the layout tree for this preset
    pub fn build(&self) -> Result<LayoutTree, BuilderError> {
        match self {
            LayoutPreset::Default => build_default_layout(),
            LayoutPreset::Focus => build_focus_layout(),
            LayoutPreset::SplitView => build_split_layout(),
            LayoutPreset::Debug => build_debug_layout(),
            LayoutPreset::Custom(name) => load_custom_preset(name),
        }
    }

    /// Get all available presets
    pub fn all() -> Vec<(String, LayoutPreset)> {
        vec![
            ("default".to_string(), LayoutPreset::Default),
            ("focus".to_string(), LayoutPreset::Focus),
            ("split".to_string(), LayoutPreset::SplitView),
            ("debug".to_string(), LayoutPreset::Debug),
        ]
    }

    /// Get preset by name
    pub fn by_name(name: &str) -> Option<LayoutPreset> {
        match name {
            "default" => Some(LayoutPreset::Default),
            "focus" => Some(LayoutPreset::Focus),
            "split" => Some(LayoutPreset::SplitView),
            "debug" => Some(LayoutPreset::Debug),
            custom => Some(LayoutPreset::Custom(custom.to_string())),
        }
    }
}

/// Build the default Seaglass layout
fn build_default_layout() -> Result<LayoutTree, BuilderError> {
    LayoutBuilder::default_seaglass_layout()
}

/// Build focus mode layout - single center panel with minimal sidebars
fn build_focus_layout() -> Result<LayoutTree, BuilderError> {
    LayoutBuilder::new()
        .split_horizontal()
        .ratio(0.9) // 90% main area, 10% right sidebar
        .first(|b| {
            b.split_vertical()
                .ratio(0.85) // 85% main, 15% bottom
                .first(|b| {
                    b.zone("main")
                        .add_panel("flow_panel")
                        .with_metadata("focus", "true")
                        .build()
                })
                .second(|b| b.zone("bottom").add_panel("table_tabs_panel").build())
                .build()
                .unwrap_or_else(|_| LayoutBuilder::new())
        })
        .second(|b| {
            b.zone("sidebar")
                .add_panel("validation_results_panel")
                .build()
        })
        .build()?
        .build()
}

/// Build split view layout - two equal panels side by side
fn build_split_layout() -> Result<LayoutTree, BuilderError> {
    LayoutBuilder::new()
        .split_horizontal()
        .ratio(0.5) // Equal split
        .first(|b| {
            b.split_vertical()
                .ratio(0.7) // 70% top, 30% bottom
                .first(|b| b.zone("left_main").add_panel("flow_panel").build())
                .second(|b| b.zone("left_bottom").add_panel("table_tabs_panel").build())
                .build()
                .unwrap_or_else(|_| LayoutBuilder::new())
        })
        .second(|b| {
            b.split_vertical()
                .ratio(0.7) // 70% top, 30% bottom
                .first(|b| {
                    b.zone("right_main")
                        .add_panel("flow_panel_secondary")
                        .build()
                })
                .second(|b| {
                    b.zone("right_bottom")
                        .add_panel("table_tabs_panel_secondary")
                        .build()
                })
                .build()
                .unwrap_or_else(|_| LayoutBuilder::new())
        })
        .build()?
        .build()
}

/// Build debug layout - multiple panels for debugging
fn build_debug_layout() -> Result<LayoutTree, BuilderError> {
    LayoutBuilder::new()
        .split_horizontal()
        .ratio(0.25) // 25% left sidebar
        .first(|b| {
            b.split_vertical()
                .ratio(0.33) // Three equal panels
                .first(|b| b.zone("debug_nodes").add_panel("nodes_panel").build())
                .second(|b| {
                    b.split_vertical()
                        .ratio(0.5)
                        .first(|b| {
                            b.zone("debug_validation")
                                .add_panel("validation_results_panel")
                                .build()
                        })
                        .second(|b| b.zone("debug_errors").add_panel("error_panel").build())
                        .build()
                        .unwrap_or_else(|_| LayoutBuilder::new())
                })
                .build()
                .unwrap_or_else(|_| LayoutBuilder::new())
        })
        .second(|b| {
            b.split_horizontal()
                .ratio(0.66) // 66% center, 34% right
                .first(|b| {
                    b.split_vertical()
                        .ratio(0.6) // 60% flow, 40% data
                        .first(|b| b.zone("debug_flow").add_panel("flow_panel").build())
                        .second(|b| b.zone("debug_data").add_panel("table_tabs_panel").build())
                        .build()
                        .unwrap_or_else(|_| LayoutBuilder::new())
                })
                .second(|b| {
                    b.zone("debug_inspector")
                        .add_panel("node_configuration_panel")
                        .with_metadata("debug_mode", "true")
                        .build()
                })
                .build()
                .unwrap_or_else(|_| LayoutBuilder::new())
        })
        .build()?
        .build()
}

/// Load a custom preset from storage
///
/// Note: This function cannot directly access the web storage service since
/// seaglass-core doesn't depend on seaglass-web. Custom preset loading should
/// be handled by the UI layer (layout_customization_ui.rs) using PresetStorageService.
///
/// This function is kept for API compatibility but will return an error directing
/// the caller to use the UI-layer storage service instead.
fn load_custom_preset(name: &str) -> Result<LayoutTree, BuilderError> {
    // Custom presets must be loaded via PresetStorageService in seaglass-web
    // This is a core library limitation - storage access is platform-specific
    Err(BuilderError::InvalidConfiguration(format!(
        "Custom preset '{name}' must be loaded via PresetStorageService. \
         Use seaglass_web::services::preset_storage_service::PresetStorageService::load_preset()"
    )))
}

/// Preset metadata for UI display
#[derive(Debug, Clone)]
pub struct PresetInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
}

impl PresetInfo {
    pub fn all() -> Vec<PresetInfo> {
        vec![
            PresetInfo {
                name: "default".to_string(),
                display_name: "Default".to_string(),
                description: "Standard Seaglass layout with all panels".to_string(),
                icon: "⊞".to_string(),
            },
            PresetInfo {
                name: "focus".to_string(),
                display_name: "Focus Mode".to_string(),
                description: "Minimalist layout for focused work".to_string(),
                icon: "□".to_string(),
            },
            PresetInfo {
                name: "split".to_string(),
                display_name: "Split View".to_string(),
                description: "Side-by-side panels for comparison".to_string(),
                icon: "⊟".to_string(),
            },
            PresetInfo {
                name: "debug".to_string(),
                display_name: "Debug Layout".to_string(),
                description: "Multiple panels for debugging workflows".to_string(),
                icon: "⊡".to_string(),
            },
        ]
    }
}

/// Storage for custom presets with in-memory cache and persistent backend
pub struct PresetStorage {
    /// In-memory cache of presets
    presets: RwLock<HashMap<String, CustomPresetData>>,
}

impl Default for PresetStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl PresetStorage {
    /// Create new preset storage
    pub fn new() -> Self {
        Self {
            presets: RwLock::new(HashMap::new()),
        }
    }

    /// Save a custom preset to in-memory cache
    /// Call `persist_preset` to save to storage backend
    pub fn save_preset(&self, name: String, layout: LayoutTree) {
        let preset_data = CustomPresetData::new(name.clone(), layout);
        if let Ok(mut presets) = self.presets.write() {
            presets.insert(name, preset_data);
        }
    }

    /// Save a preset with full metadata
    pub fn save_preset_data(&self, preset_data: CustomPresetData) {
        if let Ok(mut presets) = self.presets.write() {
            presets.insert(preset_data.name.clone(), preset_data);
        }
    }

    /// Load a custom preset from in-memory cache
    pub fn load_preset(&self, name: &str) -> Option<LayoutTree> {
        self.presets
            .read()
            .ok()
            .and_then(|presets| presets.get(name).map(|p| p.layout.clone()))
    }

    /// Load full preset data
    pub fn load_preset_data(&self, name: &str) -> Option<CustomPresetData> {
        self.presets
            .read()
            .ok()
            .and_then(|presets| presets.get(name).cloned())
    }

    /// List all custom preset names
    pub fn list_presets(&self) -> Vec<String> {
        self.presets
            .read()
            .ok()
            .map(|presets| presets.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// List all custom presets with metadata
    pub fn list_preset_infos(&self) -> Vec<CustomPresetData> {
        self.presets
            .read()
            .ok()
            .map(|presets| presets.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Delete a custom preset from in-memory cache
    pub fn delete_preset(&self, name: &str) -> bool {
        self.presets
            .write()
            .ok()
            .map(|mut presets| presets.remove(name).is_some())
            .unwrap_or(false)
    }

    /// Check if a preset exists
    pub fn has_preset(&self, name: &str) -> bool {
        self.presets
            .read()
            .ok()
            .map(|presets| presets.contains_key(name))
            .unwrap_or(false)
    }

    /// Get preset count
    pub fn count(&self) -> usize {
        self.presets
            .read()
            .ok()
            .map(|presets| presets.len())
            .unwrap_or(0)
    }

    /// Clear all presets from cache (doesn't affect storage)
    pub fn clear(&self) {
        if let Ok(mut presets) = self.presets.write() {
            presets.clear();
        }
    }

    /// Export all presets as JSON for backup
    pub fn export_all_as_json(&self) -> Result<String, String> {
        let presets = self
            .presets
            .read()
            .map_err(|e| format!("Failed to read presets: {e}"))?;

        let data: Vec<&CustomPresetData> = presets.values().collect();
        serde_json::to_string_pretty(&data).map_err(|e| format!("JSON serialization error: {e}"))
    }

    /// Import presets from JSON backup
    pub fn import_from_json(&self, json: &str) -> Result<usize, String> {
        let data: Vec<CustomPresetData> =
            serde_json::from_str(json).map_err(|e| format!("JSON parse error: {e}"))?;

        let count = data.len();
        let mut presets = self
            .presets
            .write()
            .map_err(|e| format!("Failed to write presets: {e}"))?;

        for preset in data {
            presets.insert(preset.name.clone(), preset);
        }

        Ok(count)
    }
}
