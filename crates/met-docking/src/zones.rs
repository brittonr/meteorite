//! Functional core: dock zone types and management
//!
//! This module handles dock zones where panels can be placed and managed.

use super::position::{DockPosition, DockSize};
use serde::{Deserialize, Serialize};

/// A zone where panels can be docked
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockZone {
    pub position: DockPosition,
    pub panels: Vec<String>,
    pub active_panel: Option<String>,
    pub size: DockSize,
}

impl DockZone {
    /// Create a new dock zone
    pub fn new(position: DockPosition, size: DockSize) -> Self {
        Self {
            position,
            panels: Vec::new(),
            active_panel: None,
            size,
        }
    }

    /// Add a panel to this zone
    pub fn add_panel(&mut self, panel_id: String) {
        debug_assert!(!panel_id.is_empty());
        if !self.panels.contains(&panel_id) {
            self.panels.push(panel_id.clone());
            // Set as active if it's the first panel
            if self.active_panel.is_none() {
                self.active_panel = Some(panel_id);
            }
        }
        debug_assert!(!self.panels.is_empty());
    }

    /// Remove a panel from this zone
    pub fn remove_panel(&mut self, panel_id: &str) -> bool {
        if let Some(pos) = self.panels.iter().position(|p| p == panel_id) {
            self.panels.remove(pos);

            // Update active panel if removed
            if self.active_panel.as_ref() == Some(&panel_id.to_string()) {
                self.active_panel = self.panels.first().cloned();
            }
            true
        } else {
            false
        }
    }

    /// Set the active panel
    pub fn set_active_panel(&mut self, panel_id: String) -> bool {
        if self.panels.contains(&panel_id) {
            self.active_panel = Some(panel_id);
            true
        } else {
            false
        }
    }

    /// Get the active panel ID
    pub fn get_active_panel(&self) -> Option<&String> {
        self.active_panel.as_ref()
    }

    /// Check if zone is empty
    pub fn is_empty(&self) -> bool {
        self.panels.is_empty()
    }

    /// Get panel count
    pub fn panel_count(&self) -> u32 {
        debug_assert!(self.panels.len() <= u32::MAX as usize);
        self.panels.len() as u32
    }

    /// Check if panel exists in zone
    pub fn contains_panel(&self, panel_id: &str) -> bool {
        self.panels.iter().any(|p| p == panel_id)
    }

    /// Get index of panel in zone
    pub fn get_panel_index(&self, panel_id: &str) -> Option<u32> {
        self.panels
            .iter()
            .position(|p| p == panel_id)
            .map(|index| {
                debug_assert!(index <= u32::MAX as usize);
                index as u32
            })
    }

    /// Insert panel at specific index
    pub fn insert_panel(&mut self, panel_id: String, index: u32) {
        debug_assert!(!panel_id.is_empty());
        let initial_count = self.panels.len();
        if !self.panels.contains(&panel_id) {
            let index_usize = index as usize;
            let insert_index = index_usize.min(self.panels.len());
            self.panels.insert(insert_index, panel_id.clone());

            // Set as active if it's the first panel
            if self.active_panel.is_none() {
                self.active_panel = Some(panel_id);
            }
        }
        debug_assert!(self.panels.len() >= initial_count);
    }

    /// Move panel to different index within zone
    pub fn move_panel(&mut self, panel_id: &str, new_index: u32) -> bool {
        if let Some(current_index) = self.get_panel_index(panel_id) {
            let current_index_usize = current_index as usize;
            let panel = self.panels.remove(current_index_usize);
            let new_index_usize = new_index as usize;
            let insert_index = new_index_usize.min(self.panels.len());
            self.panels.insert(insert_index, panel);
            true
        } else {
            false
        }
    }
}
