//! Functional core: docking position and size types
//!
//! This module contains core types for defining dock positions and sizing constraints.

use serde::{Deserialize, Serialize};

/// Position where a panel can be docked
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum DockPosition {
    Left,
    LeftBottom,
    Right,
    Top,
    Bottom,
    BottomLeft,
    BottomRight,
    #[default]
    Center,
    Float {
        x: f32,
        y: f32,
    },
}

/// Size constraints for docked panels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockSize {
    pub width: f32,
    pub height: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
}

impl Default for DockSize {
    fn default() -> Self {
        Self {
            width: 30.0,
            height: 30.0,
            min_width: 15.0,
            min_height: 15.0,
            max_width: None,
            max_height: None,
        }
    }
}

impl DockSize {
    /// Create a new dock size with constraints
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            min_width: width * 0.5,
            min_height: height * 0.5,
            max_width: None,
            max_height: None,
        }
    }

    /// Create a size with explicit constraints
    pub fn with_constraints(
        width: f32,
        height: f32,
        min_width: f32,
        min_height: f32,
        max_width: Option<f32>,
        max_height: Option<f32>,
    ) -> Self {
        Self {
            width,
            height,
            min_width,
            min_height,
            max_width,
            max_height,
        }
    }

    /// Clamp width and height to constraints
    pub fn clamp(&mut self) {
        self.width = self.width.max(self.min_width);
        self.height = self.height.max(self.min_height);

        if let Some(max_width) = self.max_width {
            self.width = self.width.min(max_width);
        }

        if let Some(max_height) = self.max_height {
            self.height = self.height.min(max_height);
        }
    }

    /// Check if size is within constraints
    pub fn is_valid(&self) -> bool {
        self.width >= self.min_width
            && self.height >= self.min_height
            && self.max_width.is_none_or(|max| self.width <= max)
            && self.max_height.is_none_or(|max| self.height <= max)
    }
}
