//! Functional core: layout tree rendering calculations
//!
//! This module provides utilities for rendering the flexible layout tree
//! into UI components, calculating positions and sizes.

use super::{
    constraints::LayoutConstraints,
    node::{ContainerLayout, LayoutNode, NodeId, SplitDirection},
    tree::LayoutTree,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a rectangular area
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Split horizontally at the given ratio
    pub fn split_horizontal(&self, ratio: f32) -> (Rect, Rect) {
        let split_x = self.x + self.width * ratio;
        let first_width = self.width * ratio;
        let second_width = self.width * (1.0 - ratio);

        (
            Rect::new(self.x, self.y, first_width, self.height),
            Rect::new(split_x, self.y, second_width, self.height),
        )
    }

    /// Split vertically at the given ratio
    pub fn split_vertical(&self, ratio: f32) -> (Rect, Rect) {
        let split_y = self.y + self.height * ratio;
        let first_height = self.height * ratio;
        let second_height = self.height * (1.0 - ratio);

        (
            Rect::new(self.x, self.y, self.width, first_height),
            Rect::new(self.x, split_y, self.width, second_height),
        )
    }

    /// Apply padding to the rectangle
    pub fn with_padding(&self, padding: f32) -> Rect {
        Rect::new(
            self.x + padding,
            self.y + padding,
            self.width - 2.0 * padding,
            self.height - 2.0 * padding,
        )
    }
}

/// Rendered layout information
#[derive(Debug, Clone, PartialEq)]
pub struct RenderedLayout {
    /// Position and size of each zone
    pub zone_rects: HashMap<NodeId, Rect>,

    /// Position and size of each split handle
    pub split_handles: Vec<SplitHandle>,

    /// Container information
    pub containers: HashMap<NodeId, ContainerInfo>,

    /// Constraint violations found during rendering
    pub violations: Vec<(NodeId, String)>,
}

/// Information about a split handle for resizing
#[derive(Debug, Clone, PartialEq)]
pub struct SplitHandle {
    pub id: NodeId,
    pub rect: Rect,
    pub direction: SplitDirection,
    pub min_position: f32,
    pub max_position: f32,
    /// Bounding rect of the parent split node that this handle divides.
    /// Used to compute the correct resize ratio relative to the parent,
    /// not the full viewport.
    pub parent_bounds: Rect,
}

/// Information about a container
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerInfo {
    pub id: NodeId,
    pub container_layout: ContainerLayout,
    pub active_child: Option<usize>,
    pub child_zones: Vec<NodeId>,
}

/// Layout renderer
pub struct LayoutRenderer<'a> {
    tree: &'a LayoutTree,
    constraints: Option<&'a LayoutConstraints>,
}

impl<'a> LayoutRenderer<'a> {
    /// Create a new renderer
    pub fn new(tree: &'a LayoutTree) -> Self {
        Self {
            tree,
            constraints: None,
        }
    }

    /// Set constraints for validation
    pub fn with_constraints(mut self, constraints: &'a LayoutConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }

    /// Render the layout tree into rectangles
    pub fn render(&self, bounds: Rect) -> RenderedLayout {
        debug_assert!(bounds.width > 0.0);
        debug_assert!(bounds.height > 0.0);
        let mut layout = RenderedLayout {
            zone_rects: HashMap::new(),
            split_handles: Vec::new(),
            containers: HashMap::new(),
            violations: Vec::new(),
        };

        self.render_node(self.tree.root(), bounds, &mut layout);
        debug_assert!(!layout.zone_rects.is_empty());

        layout
    }

    /// Render a single node and its children
    fn render_node(&self, node: &LayoutNode, bounds: Rect, layout: &mut RenderedLayout) {
        match node {
            LayoutNode::Split {
                id,
                direction,
                ratio,
                first,
                second,
            } => {
                self.render_node_split(id, *direction, *ratio, (first, second), bounds, layout);
            }
            LayoutNode::Zone {
                id, constraints, ..
            } => {
                self.render_node_zone(id, constraints, bounds, layout);
            }
            LayoutNode::Container {
                id,
                layout: container_layout,
                children,
                active_child,
            } => {
                self.render_node_container(
                    id,
                    *container_layout,
                    children,
                    *active_child,
                    bounds,
                    layout,
                );
            }
        }
    }

    /// Render a split node with its handle and children
    fn render_node_split(
        &self,
        id: &NodeId,
        direction: SplitDirection,
        ratio: f32,
        children: (&LayoutNode, &LayoutNode),
        bounds: Rect,
        layout: &mut RenderedLayout,
    ) {
        let (first, second) = children;
        let (first_rect, second_rect) = match direction {
            SplitDirection::Horizontal => bounds.split_horizontal(ratio),
            SplitDirection::Vertical => bounds.split_vertical(ratio),
        };

        // Add split handle
        let handle_rect = self.calculate_split_handle_rect(bounds, direction, ratio);

        layout.split_handles.push(SplitHandle {
            id: id.clone(),
            rect: handle_rect,
            direction,
            min_position: match direction {
                SplitDirection::Horizontal => bounds.x + 50.0, // Min 50px
                SplitDirection::Vertical => bounds.y + 50.0,
            },
            max_position: match direction {
                SplitDirection::Horizontal => bounds.x + bounds.width - 50.0,
                SplitDirection::Vertical => bounds.y + bounds.height - 50.0,
            },
            parent_bounds: bounds,
        });

        // Render children
        self.render_node(first, first_rect, layout);
        self.render_node(second, second_rect, layout);
    }

    /// Render a zone node with its constraints and padding
    fn render_node_zone(
        &self,
        id: &NodeId,
        constraints: &Option<super::constraints::SizeConstraints>,
        bounds: Rect,
        layout: &mut RenderedLayout,
    ) {
        // Apply padding to zones
        let zone_rect = bounds.with_padding(2.0);

        // Validate size constraints if available
        if let Some(size_constraints) = constraints {
            let parent_width = bounds.width;
            let parent_height = bounds.height;
            let (_, _) = size_constraints.constrain(
                zone_rect.width,
                zone_rect.height,
                parent_width,
                parent_height,
            );
            // Could add violation tracking here if needed
        }

        layout.zone_rects.insert(id.clone(), zone_rect);
    }

    /// Render a container node with its children based on layout type
    fn render_node_container(
        &self,
        id: &NodeId,
        container_layout: ContainerLayout,
        children: &[LayoutNode],
        active_child: Option<usize>,
        bounds: Rect,
        layout: &mut RenderedLayout,
    ) {
        // Handle different container types
        match container_layout {
            ContainerLayout::Tabs => {
                self.render_node_container_tabs(children, active_child, bounds, layout);
            }
            ContainerLayout::Stack => {
                self.render_node_container_stack(children, bounds, layout);
            }
            ContainerLayout::Grid(cols) => {
                self.render_node_container_grid(children, cols, bounds, layout);
            }
        }

        // Store container info
        let child_zones: Vec<NodeId> = children
            .iter()
            .filter_map(|child| {
                if child.is_zone() {
                    Some(child.id().clone())
                } else {
                    None
                }
            })
            .collect();

        layout.containers.insert(
            id.clone(),
            ContainerInfo {
                id: id.clone(),
                container_layout,
                active_child,
                child_zones,
            },
        );
    }

    /// Render tabs layout showing only the active child
    fn render_node_container_tabs(
        &self,
        children: &[LayoutNode],
        active_child: Option<usize>,
        bounds: Rect,
        layout: &mut RenderedLayout,
    ) {
        if let Some(active) = active_child {
            if let Some(child) = children.get(active) {
                self.render_node(child, bounds, layout);
            }
        }
    }

    /// Render stack layout showing all children vertically
    fn render_node_container_stack(
        &self,
        children: &[LayoutNode],
        bounds: Rect,
        layout: &mut RenderedLayout,
    ) {
        let child_height = bounds.height / children.len() as f32;
        for (i, child) in children.iter().enumerate() {
            let child_rect = Rect::new(
                bounds.x,
                bounds.y + (i as f32 * child_height),
                bounds.width,
                child_height,
            );
            self.render_node(child, child_rect, layout);
        }
    }

    /// Render grid layout with specified columns
    fn render_node_container_grid(
        &self,
        children: &[LayoutNode],
        cols: u32,
        bounds: Rect,
        layout: &mut RenderedLayout,
    ) {
        let cols = cols as usize;
        let rows = children.len().div_ceil(cols);
        let cell_width = bounds.width / cols as f32;
        let cell_height = bounds.height / rows as f32;

        for (i, child) in children.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let child_rect = Rect::new(
                bounds.x + (col as f32 * cell_width),
                bounds.y + (row as f32 * cell_height),
                cell_width,
                cell_height,
            );
            self.render_node(child, child_rect, layout);
        }
    }

    /// Calculate the rectangle for a split handle
    fn calculate_split_handle_rect(
        &self,
        bounds: Rect,
        direction: SplitDirection,
        ratio: f32,
    ) -> Rect {
        const HANDLE_SIZE: f32 = 12.0; // Increased for better usability across platforms

        match direction {
            SplitDirection::Horizontal => {
                let handle_center_x = bounds.x + bounds.width * ratio - HANDLE_SIZE / 2.0;
                Rect::new(handle_center_x, bounds.y, HANDLE_SIZE, bounds.height)
            }
            SplitDirection::Vertical => {
                let handle_center_y = bounds.y + bounds.height * ratio - HANDLE_SIZE / 2.0;
                Rect::new(bounds.x, handle_center_y, bounds.width, HANDLE_SIZE)
            }
        }
    }
}

/// Find which zone contains a point
pub fn find_zone_at_point(layout: &RenderedLayout, point_x: f32, point_y: f32) -> Option<NodeId> {
    for (zone_id, rect) in &layout.zone_rects {
        if point_x >= rect.x
            && point_x <= rect.x + rect.width
            && point_y >= rect.y
            && point_y <= rect.y + rect.height
        {
            return Some(zone_id.clone());
        }
    }
    None
}

/// Find which split handle contains a point
/// Check if a point is contained within a rectangle
fn rect_contains_point(rect: &Rect, x: f32, y: f32) -> bool {
    debug_assert!(rect.width >= 0.0);
    debug_assert!(rect.height >= 0.0);
    let within_horizontal = x >= rect.x && x <= rect.x + rect.width;
    let within_vertical = y >= rect.y && y <= rect.y + rect.height;
    within_horizontal && within_vertical
}

pub fn find_split_at_point(layout: &RenderedLayout, x: f32, y: f32) -> Option<&SplitHandle> {
    layout
        .split_handles
        .iter()
        .find(|&handle| rect_contains_point(&handle.rect, x, y))
}

/// Calculate new split ratio based on mouse position
pub fn calculate_split_ratio(handle: &SplitHandle, mouse_pos: f32, bounds: Rect) -> f32 {
    match handle.direction {
        SplitDirection::Horizontal => {
            let relative_pos = mouse_pos - bounds.x;
            (relative_pos / bounds.width).clamp(0.1, 0.9)
        }
        SplitDirection::Vertical => {
            let relative_pos = mouse_pos - bounds.y;
            (relative_pos / bounds.height).clamp(0.1, 0.9)
        }
    }
}
