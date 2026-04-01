//! Functional core: panel configuration and floating panel types
//!
//! This module handles panel configuration and floating panel state.

use super::position::DockPosition;
use serde::{Deserialize, Serialize};

/// Configuration for a panel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelConfig {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub can_close: bool,
    pub can_float: bool,
    pub default_position: DockPosition,
}

impl PanelConfig {
    /// Create a new panel configuration
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            can_close: true,
            can_float: true,
            default_position: DockPosition::default(),
        }
    }

    /// Set the panel icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set whether panel can be closed
    pub fn with_close_button(mut self, can_close: bool) -> Self {
        self.can_close = can_close;
        self
    }

    /// Set whether panel can float
    pub fn with_float_capability(mut self, can_float: bool) -> Self {
        self.can_float = can_float;
        self
    }

    /// Set default position
    pub fn with_default_position(mut self, position: DockPosition) -> Self {
        self.default_position = position;
        self
    }
}

/// A floating panel that exists outside of dock zones
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatingPanel {
    pub panel_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub z_index: i32,
}

impl FloatingPanel {
    /// Create a new floating panel
    pub fn new(panel_id: impl Into<String>, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            panel_id: panel_id.into(),
            x,
            y,
            width,
            height,
            z_index: 1,
        }
    }

    /// Set the z-index for layering
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    /// Move the floating panel (functional — returns new value)
    pub fn moved_to(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Resize the floating panel (functional — returns new value)
    pub fn resized(mut self, width: f32, height: f32) -> Self {
        self.width = width.max(100.0); // Minimum width
        self.height = height.max(50.0); // Minimum height
        self
    }

    /// Get panel bounds as (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }

    /// Check if point is within panel bounds
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Bring panel to front by setting highest z-index (functional — returns new value)
    pub fn brought_to_front(mut self, max_z_index: i32) -> Self {
        self.z_index = max_z_index + 1;
        self
    }
}

/// Panel state tracking
#[derive(Debug, Clone, PartialEq, Default)]
pub enum PanelState {
    Docked {
        zone_id: String,
    },
    Floating,
    #[default]
    Hidden,
}
