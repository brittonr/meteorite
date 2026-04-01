//! Functional core: drag and drop state types
//!
//! This module handles the state and logic for dragging panels between zones.

use super::position::DockPosition;

/// State information during panel drag operation
#[derive(Debug, Clone, PartialEq)]
pub struct DragState {
    pub panel_id: String,
    pub start_x: f32,
    pub start_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub original_zone: Option<String>,
}

impl DragState {
    /// Create a new drag state
    pub fn new(
        panel_id: impl Into<String>,
        start_x: f32,
        start_y: f32,
        offset_x: f32,
        offset_y: f32,
    ) -> Self {
        Self {
            panel_id: panel_id.into(),
            start_x,
            start_y,
            offset_x,
            offset_y,
            original_zone: None,
        }
    }

    /// Set the original zone the panel was dragged from
    pub fn with_original_zone(mut self, zone_id: impl Into<String>) -> Self {
        self.original_zone = Some(zone_id.into());
        self
    }

    /// Calculate current drag position
    pub fn current_position(&self, mouse_x: f32, mouse_y: f32) -> (f32, f32) {
        (mouse_x - self.offset_x, mouse_y - self.offset_y)
    }

    /// Calculate distance dragged from start position
    pub fn drag_distance(&self, mouse_x: f32, mouse_y: f32) -> f32 {
        let dx = mouse_x - self.start_x;
        let dy = mouse_y - self.start_y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if drag distance exceeds threshold
    pub fn exceeds_threshold(&self, mouse_x: f32, mouse_y: f32, threshold: f32) -> bool {
        self.drag_distance(mouse_x, mouse_y) > threshold
    }
}

/// Possible drop target for a dragged panel
#[derive(Debug, Clone, PartialEq)]
pub struct DropTarget {
    pub zone_id: String,
    pub position: DockPosition,
    pub insert_index: Option<usize>,
}

impl DropTarget {
    /// Create a new drop target
    pub fn new(zone_id: impl Into<String>, position: DockPosition) -> Self {
        Self {
            zone_id: zone_id.into(),
            position,
            insert_index: None,
        }
    }

    /// Set the insertion index for tab ordering
    pub fn with_insert_index(mut self, index: usize) -> Self {
        self.insert_index = Some(index);
        self
    }

    /// Check if this is a zone drop (vs tab reordering)
    pub fn is_zone_drop(&self) -> bool {
        self.insert_index.is_none()
    }

    /// Check if this is a tab reordering drop
    pub fn is_tab_reorder(&self) -> bool {
        self.insert_index.is_some()
    }
}

/// Drop zone detection for UI feedback
#[derive(Debug, Clone, PartialEq)]
pub struct DropZone {
    pub zone_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub position: DockPosition,
    pub is_active: bool,
}

impl DropZone {
    /// Create a new drop zone
    pub fn new(
        zone_id: impl Into<String>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        position: DockPosition,
    ) -> Self {
        Self {
            zone_id: zone_id.into(),
            x,
            y,
            width,
            height,
            position,
            is_active: false,
        }
    }

    /// Check if point is within drop zone
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Activate drop zone for visual feedback
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate drop zone
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Get drop zone bounds
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

/// Result of a drop operation
#[derive(Debug, Clone, PartialEq)]
pub enum DropResult {
    /// Panel was dropped in a new zone
    ZoneChange {
        panel_id: String,
        from_zone: Option<String>,
        to_zone: String,
        insert_index: Option<usize>,
    },
    /// Panel was reordered within the same zone
    Reorder {
        panel_id: String,
        zone_id: String,
        from_index: usize,
        to_index: usize,
    },
    /// Panel became floating
    Float {
        panel_id: String,
        from_zone: Option<String>,
        x: f32,
        y: f32,
    },
    /// Drop was cancelled or invalid
    Cancel { panel_id: String },
}
