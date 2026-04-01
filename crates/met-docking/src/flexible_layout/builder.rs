//! Functional core: layout tree builder API
use super::node::{LayoutNode, NodeId, SplitDirection, ZoneContent};
use super::tree::LayoutTree;
use std::collections::HashMap;

/// Builder for creating layout trees
pub struct LayoutBuilder {
    current: Option<LayoutNode>,
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutBuilder {
    /// Create a new layout builder
    pub fn new() -> Self {
        Self { current: None }
    }

    /// Create a zone node
    pub fn zone(self, id: &str) -> ZoneBuilder {
        ZoneBuilder {
            id: id.to_string(),
            panels: Vec::new(),
            active_panel: None,
            metadata: HashMap::new(),
            parent_builder: self,
        }
    }

    /// Create a horizontal split
    pub fn split_horizontal(self) -> SplitBuilder {
        SplitBuilder {
            direction: SplitDirection::Horizontal,
            ratio: 0.5,
            first: None,
            second: None,
            parent_builder: self,
        }
    }

    /// Create a vertical split
    pub fn split_vertical(self) -> SplitBuilder {
        SplitBuilder {
            direction: SplitDirection::Vertical,
            ratio: 0.5,
            first: None,
            second: None,
            parent_builder: self,
        }
    }

    /// Build the final tree
    pub fn build(self) -> Result<LayoutTree, BuilderError> {
        match self.current {
            Some(root) => Ok(LayoutTree::from_root(root)),
            None => Err(BuilderError::EmptyLayout),
        }
    }
}

pub struct ZoneBuilder {
    id: String,
    panels: Vec<String>,
    active_panel: Option<String>,
    metadata: HashMap<String, String>,
    parent_builder: LayoutBuilder,
}

impl ZoneBuilder {
    /// Add a panel to the zone
    pub fn add_panel(mut self, panel_id: &str) -> Self {
        self.panels.push(panel_id.to_string());
        // Set first panel as active if none set
        if self.active_panel.is_none() && self.panels.len() == 1 {
            self.active_panel = Some(panel_id.to_string());
        }
        self
    }

    /// Set the active panel
    pub fn set_active(mut self, panel_id: &str) -> Self {
        self.active_panel = Some(panel_id.to_string());
        self
    }

    /// Add metadata to the zone
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Build the zone and return to parent builder
    pub fn build(mut self) -> LayoutBuilder {
        let mut content = ZoneContent::new();
        for panel in self.panels {
            content.add_panel(panel);
        }
        if let Some(active) = self.active_panel {
            content.set_active_panel(Some(active));
        }
        content.metadata = self.metadata;

        let zone = LayoutNode::Zone {
            id: NodeId::from_string(self.id),
            content,
            constraints: None,
        };
        self.parent_builder.current = Some(zone);
        self.parent_builder
    }
}

pub struct SplitBuilder {
    direction: SplitDirection,
    ratio: f32,
    first: Option<Box<LayoutNode>>,
    second: Option<Box<LayoutNode>>,
    parent_builder: LayoutBuilder,
}

impl SplitBuilder {
    /// Set the split ratio (0.0 to 1.0)
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.1, 0.9);
        self
    }

    /// Build the first child
    pub fn first<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(LayoutBuilder) -> LayoutBuilder,
    {
        let child_builder = builder_fn(LayoutBuilder::new());
        if let Ok(tree) = child_builder.build() {
            self.first = Some(Box::new(tree.root().clone()));
        }
        self
    }

    /// Build the second child
    pub fn second<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(LayoutBuilder) -> LayoutBuilder,
    {
        let child_builder = builder_fn(LayoutBuilder::new());
        if let Ok(tree) = child_builder.build() {
            self.second = Some(Box::new(tree.root().clone()));
        }
        self
    }

    /// Build the split and return to parent builder
    pub fn build(mut self) -> Result<LayoutBuilder, BuilderError> {
        match (self.first, self.second) {
            (Some(first), Some(second)) => {
                let split = LayoutNode::Split {
                    id: NodeId::new(),
                    direction: self.direction,
                    ratio: self.ratio,
                    first,
                    second,
                };
                self.parent_builder.current = Some(split);
                Ok(self.parent_builder)
            }
            _ => Err(BuilderError::IncompleteSplit),
        }
    }
}

/// Errors that can occur during layout building
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Layout is empty")]
    EmptyLayout,

    #[error("Split node is incomplete")]
    IncompleteSplit,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

// Convenience methods for common layouts
impl LayoutBuilder {
    /// Create a simple single-zone layout
    pub fn simple_zone(zone_id: &str, panels: Vec<&str>) -> Result<LayoutTree, BuilderError> {
        let mut builder = Self::new().zone(zone_id);
        for panel in panels {
            builder = builder.add_panel(panel);
        }
        builder.build().build()
    }

    /// Create a two-column layout
    pub fn two_column(
        left_zone: &str,
        left_panels: Vec<&str>,
        right_zone: &str,
        right_panels: Vec<&str>,
        ratio: f32,
    ) -> Result<LayoutTree, BuilderError> {
        Self::new()
            .split_horizontal()
            .ratio(ratio)
            .first(|b| {
                let mut zone = b.zone(left_zone);
                for panel in left_panels {
                    zone = zone.add_panel(panel);
                }
                zone.build()
            })
            .second(|b| {
                let mut zone = b.zone(right_zone);
                for panel in right_panels {
                    zone = zone.add_panel(panel);
                }
                zone.build()
            })
            .build()?
            .build()
    }

    /// Create a three-column layout
    pub fn three_column(
        left_zone: &str,
        left_panels: Vec<&str>,
        center_zone: &str,
        center_panels: Vec<&str>,
        right_zone: &str,
        right_panels: Vec<&str>,
    ) -> Result<LayoutTree, BuilderError> {
        let inner_split = Self::new()
            .split_horizontal()
            .ratio(0.75) // 75% of remaining for center (60% total)
            .first(|b2| {
                let mut zone = b2.zone(center_zone);
                for panel in center_panels {
                    zone = zone.add_panel(panel);
                }
                zone.build()
            })
            .second(|b2| {
                let mut zone = b2.zone(right_zone);
                for panel in right_panels {
                    zone = zone.add_panel(panel);
                }
                zone.build()
            })
            .build()
            .map_err(|e| {
                BuilderError::InvalidConfiguration(format!(
                    "Failed to build inner center/right split: {e}"
                ))
            })?
            .build()?;

        Self::new()
            .split_horizontal()
            .ratio(0.2) // 20% left
            .first(|b| {
                let mut zone = b.zone(left_zone);
                for panel in left_panels {
                    zone = zone.add_panel(panel);
                }
                zone.build()
            })
            .second(|_b| {
                // Use the pre-built inner tree
                let mut builder = LayoutBuilder::new();
                builder.current = Some(inner_split.root().clone());
                builder
            })
            .build()?
            .build()
    }

    /// Create the default Seaglass layout with menu bar at top
    pub fn default_seaglass_layout() -> Result<LayoutTree, BuilderError> {
        let left_sidebar = Self::default_seaglass_layout_left_sidebar()?;
        let right_sidebar = Self::default_seaglass_layout_right_sidebar()?;
        let center_right = Self::default_seaglass_layout_center_right(right_sidebar)?;
        let main_content = Self::default_seaglass_layout_main_content(left_sidebar, center_right)?;

        // Build final layout (menu bar and main content)
        Self::new()
            .split_vertical()
            .ratio(0.02) // 2% for menu bar, 98% for main content
            .first(|b| {
                // Menu bar zone at the top
                b.zone("menu_zone")
                    .add_panel("menu_bar")
                    .with_metadata("legacy_name", "menu_zone")
                    .build()
            })
            .second(|_b| {
                let mut builder = LayoutBuilder::new();
                builder.current = Some(main_content.root().clone());
                builder
            })
            .build()?
            .build()
    }

    /// Build the left sidebar structure
    ///
    /// Two zones: nodes (40%) and validation results (60%).
    /// The old layout had a third empty "left_bottom" drop zone that
    /// wasted ~33% of the sidebar height — removed.
    fn default_seaglass_layout_left_sidebar() -> Result<LayoutTree, BuilderError> {
        Self::new()
            .split_vertical()
            .ratio(0.4) // 40% nodes, 60% validation results
            .first(|b| {
                b.zone("left_top")
                    .add_panel("nodes_panel")
                    .with_metadata("legacy_name", "left_top")
                    .build()
            })
            .second(|b| {
                b.zone("left_middle")
                    .add_panel("validation_results_panel")
                    .with_metadata("legacy_name", "left_middle")
                    .build()
            })
            .build()
            .map_err(|e| {
                BuilderError::InvalidConfiguration(format!("Failed to build left sidebar: {e}"))
            })?
            .build()
    }

    /// Build the right sidebar structure
    ///
    /// Two zones: transformations (50%) and validation rules (50%).
    /// The old layout had a third empty "right_bottom" drop zone that
    /// wasted ~33% of the sidebar height — removed.
    fn default_seaglass_layout_right_sidebar() -> Result<LayoutTree, BuilderError> {
        Self::new()
            .split_vertical()
            .ratio(0.5) // 50% transformations, 50% validation rules
            .first(|b| {
                b.zone("right_top")
                    .add_panel("transformations_panel")
                    .with_metadata("legacy_name", "right_top")
                    .build()
            })
            .second(|b| {
                b.zone("right_middle")
                    .add_panel("validation_rules_panel")
                    .with_metadata("legacy_name", "right_middle")
                    .build()
            })
            .build()
            .map_err(|e| {
                BuilderError::InvalidConfiguration(format!("Failed to build right sidebar: {e}"))
            })?
            .build()
    }

    /// Build the center and right split structure
    fn default_seaglass_layout_center_right(
        right_sidebar: LayoutTree,
    ) -> Result<LayoutTree, BuilderError> {
        Self::new()
            .split_horizontal()
            .ratio(0.7) // 70% center, 30% right sidebar
            .first(|b| {
                // Center zone (main table view)
                b.zone("center")
                    .add_panel("main_content")
                    .add_panel("flow_view")
                    .with_metadata("legacy_name", "center")
                    .build()
            })
            .second(|_b| {
                let mut builder = LayoutBuilder::new();
                builder.current = Some(right_sidebar.root().clone());
                builder
            })
            .build()
            .map_err(|e| {
                BuilderError::InvalidConfiguration(format!(
                    "Failed to build center/right split: {e}"
                ))
            })?
            .build()
    }

    /// Build the main content area combining left sidebar and center/right
    fn default_seaglass_layout_main_content(
        left_sidebar: LayoutTree,
        center_right: LayoutTree,
    ) -> Result<LayoutTree, BuilderError> {
        Self::new()
            .split_horizontal()
            .ratio(0.2) // 20% left sidebar, 80% rest
            .first(|_b| {
                let mut builder = LayoutBuilder::new();
                builder.current = Some(left_sidebar.root().clone());
                builder
            })
            .second(|_b| {
                let mut builder = LayoutBuilder::new();
                builder.current = Some(center_right.root().clone());
                builder
            })
            .build()
            .map_err(|e| {
                BuilderError::InvalidConfiguration(format!(
                    "Failed to build main content area: {e}"
                ))
            })?
            .build()
    }
}
