//! Functional core: flexible layout sizing constraints

use serde::{Deserialize, Serialize};

/// Size constraints for a layout node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeConstraints {
    /// Minimum width (percentage 0.0-1.0 or pixels)
    pub min_width: SizeValue,
    /// Maximum width (percentage 0.0-1.0 or pixels)
    pub max_width: Option<SizeValue>,
    /// Minimum height (percentage 0.0-1.0 or pixels)
    pub min_height: SizeValue,
    /// Maximum height (percentage 0.0-1.0 or pixels)
    pub max_height: Option<SizeValue>,
    /// Preferred width
    pub preferred_width: Option<SizeValue>,
    /// Preferred height  
    pub preferred_height: Option<SizeValue>,
    /// Aspect ratio constraint (width/height)
    pub aspect_ratio: Option<f32>,
}

impl SizeConstraints {
    /// Create new constraints with default minimums
    pub fn new() -> Self {
        Self {
            min_width: SizeValue::Percentage(0.1), // 10%
            max_width: None,
            min_height: SizeValue::Percentage(0.1), // 10%
            max_height: None,
            preferred_width: None,
            preferred_height: None,
            aspect_ratio: None,
        }
    }

    /// Set minimum width
    pub fn with_min_width(mut self, value: SizeValue) -> Self {
        self.min_width = value;
        self
    }

    /// Set maximum width
    pub fn with_max_width(mut self, value: SizeValue) -> Self {
        self.max_width = Some(value);
        self
    }

    /// Set minimum height
    pub fn with_min_height(mut self, value: SizeValue) -> Self {
        self.min_height = value;
        self
    }

    /// Set maximum height
    pub fn with_max_height(mut self, value: SizeValue) -> Self {
        self.max_height = Some(value);
        self
    }

    /// Set preferred width
    pub fn with_preferred_width(mut self, value: SizeValue) -> Self {
        self.preferred_width = Some(value);
        self
    }

    /// Set preferred height
    pub fn with_preferred_height(mut self, value: SizeValue) -> Self {
        self.preferred_height = Some(value);
        self
    }

    /// Set aspect ratio
    pub fn with_aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }

    /// Apply constraints to a proposed size
    pub fn constrain(
        &self,
        width: f32,
        height: f32,
        parent_width: f32,
        parent_height: f32,
    ) -> (f32, f32) {
        let mut final_width = width;
        let mut final_height = height;

        // Apply min/max width
        let min_w = self.min_width.to_pixels(parent_width);
        final_width = final_width.max(min_w);

        if let Some(max_width) = &self.max_width {
            let max_w = max_width.to_pixels(parent_width);
            final_width = final_width.min(max_w);
        }

        // Apply min/max height
        let min_h = self.min_height.to_pixels(parent_height);
        final_height = final_height.max(min_h);

        if let Some(max_height) = &self.max_height {
            let max_h = max_height.to_pixels(parent_height);
            final_height = final_height.min(max_h);
        }

        // Apply aspect ratio if set
        if let Some(ratio) = self.aspect_ratio {
            // Try to maintain aspect ratio while respecting other constraints
            let current_ratio = final_width / final_height;
            if (current_ratio - ratio).abs() > 0.01 {
                // Adjust to maintain ratio
                if current_ratio > ratio {
                    // Too wide, reduce width
                    final_width = final_height * ratio;
                } else {
                    // Too tall, reduce height
                    final_height = final_width / ratio;
                }

                // Re-apply constraints after aspect ratio adjustment
                final_width = final_width.max(min_w);
                if let Some(max_width) = &self.max_width {
                    let max_w = max_width.to_pixels(parent_width);
                    final_width = final_width.min(max_w);
                }

                final_height = final_height.max(min_h);
                if let Some(max_height) = &self.max_height {
                    let max_h = max_height.to_pixels(parent_height);
                    final_height = final_height.min(max_h);
                }
            }
        }

        (final_width, final_height)
    }
}

impl Default for SizeConstraints {
    fn default() -> Self {
        Self::new()
    }
}

/// Size value that can be percentage or pixels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SizeValue {
    /// Percentage of parent size (0.0-1.0)
    Percentage(f32),
    /// Fixed pixel size
    Pixels(f32),
}

impl SizeValue {
    /// Convert to pixels given parent size
    pub fn to_pixels(&self, parent_size: f32) -> f32 {
        match self {
            SizeValue::Percentage(p) => parent_size * p,
            SizeValue::Pixels(px) => *px,
        }
    }

    /// Convert to percentage given parent size
    pub fn to_percentage(&self, parent_size: f32) -> f32 {
        match self {
            SizeValue::Percentage(p) => *p,
            SizeValue::Pixels(px) => px / parent_size,
        }
    }
}

/// Global layout constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutConstraints {
    /// Minimum zone size in pixels
    pub min_zone_size: f32,
    /// Gutter size for splits in pixels
    pub gutter_size: f32,
    /// Minimum split ratio
    pub min_split_ratio: f32,
    /// Maximum split ratio
    pub max_split_ratio: f32,
    /// Enable snapping to edges
    pub snap_to_edges: bool,
    /// Snap threshold in pixels
    pub snap_threshold: f32,
}

impl LayoutConstraints {
    /// Create default layout constraints
    pub fn new() -> Self {
        Self {
            min_zone_size: 50.0,
            gutter_size: 6.0,
            min_split_ratio: 0.1,
            max_split_ratio: 0.9,
            snap_to_edges: true,
            snap_threshold: 10.0,
        }
    }

    /// Validate and constrain a split ratio
    pub fn constrain_split_ratio(&self, ratio: f32) -> f32 {
        ratio.clamp(self.min_split_ratio, self.max_split_ratio)
    }

    /// Check if a position should snap to an edge
    pub fn should_snap(&self, position: f32, edge: f32) -> Option<f32> {
        if !self.snap_to_edges {
            return None;
        }

        let distance = (position - edge).abs();
        if distance <= self.snap_threshold {
            Some(edge)
        } else {
            None
        }
    }
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self::new()
    }
}
