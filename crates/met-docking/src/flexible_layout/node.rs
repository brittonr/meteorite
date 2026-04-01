//! Functional core: layout tree node types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

/// Tiger Style: Fixed limit on recursion depth to prevent stack overflow.
/// This prevents malicious or malformed deeply nested layouts from crashing.
const MAX_NODE_DEPTH: u32 = 100;

/// Unique identifier for a node in the layout tree
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    /// Create a new unique node ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a node ID from a string (useful for legacy zones)
    pub fn from_string(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for NodeId {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

/// Path to a node in the tree (for navigation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePath(pub Vec<usize>);

impl NodePath {
    /// Create an empty path (root)
    pub fn root() -> Self {
        Self(Vec::new())
    }

    /// Add a child index to the path
    pub fn child(&self, index: usize) -> Self {
        let mut path = self.0.clone();
        path.push(index);
        Self(path)
    }

    /// Get parent path
    pub fn parent(&self) -> Option<Self> {
        if self.0.is_empty() {
            None
        } else {
            let mut path = self.0.clone();
            path.pop();
            Some(Self(path))
        }
    }
}

/// Direction for splitting a zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal, // Left/Right split
    Vertical,   // Top/Bottom split
}

/// Layout strategy for container zones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerLayout {
    Tabs,      // Only one child visible at a time
    Stack,     // All children stacked in one direction
    Grid(u32), // Grid with specified columns
}

/// Content within a zone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneContent {
    /// Panels in this zone
    pub panels: Vec<String>,
    /// Currently active panel
    pub active_panel: Option<String>,
    /// Zone-specific metadata (for custom behaviors)
    pub metadata: HashMap<String, String>,
}

impl ZoneContent {
    /// Create new empty zone content
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            active_panel: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a panel to the zone (appends to end)
    pub fn add_panel(&mut self, panel_id: String) {
        if !self.panels.contains(&panel_id) {
            self.panels.push(panel_id.clone());
            if self.active_panel.is_none() {
                self.active_panel = Some(panel_id);
            }
        }
    }

    /// Insert a panel at a specific index
    ///
    /// If the index is greater than the number of panels, the panel is appended.
    /// If the panel already exists in the zone, it is moved to the new position.
    pub fn insert_panel_at(&mut self, panel_id: String, index: u32) {
        // If panel already exists, remove it first (we'll re-insert at new position)
        if let Some(pos) = self.panels.iter().position(|p| p == &panel_id) {
            self.panels.remove(pos);
        }

        // Insert at the specified index (clamped to bounds)
        let index_usize = index as usize;
        let insert_idx = index_usize.min(self.panels.len());
        self.panels.insert(insert_idx, panel_id.clone());

        // Set as active if no active panel
        if self.active_panel.is_none() {
            self.active_panel = Some(panel_id);
        }
    }

    /// Remove a panel from the zone
    pub fn remove_panel(&mut self, panel_id: &str) -> bool {
        if let Some(pos) = self.panels.iter().position(|p| p == panel_id) {
            self.panels.remove(pos);
            if self.active_panel.as_ref() == Some(&panel_id.to_string()) {
                self.active_panel = self.panels.first().cloned();
            }
            true
        } else {
            false
        }
    }

    /// Set the active panel
    pub fn set_active_panel(&mut self, panel_id: Option<String>) {
        if let Some(ref id) = panel_id {
            if self.panels.contains(id) {
                self.active_panel = panel_id;
            }
        } else {
            self.active_panel = None;
        }
    }
}

impl Default for ZoneContent {
    fn default() -> Self {
        Self::new()
    }
}

/// A node in the layout tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayoutNode {
    /// A zone that contains panels
    Zone {
        id: NodeId,
        content: ZoneContent,
        constraints: Option<super::constraints::SizeConstraints>,
    },

    /// A split that divides space between two children
    Split {
        id: NodeId,
        direction: SplitDirection,
        ratio: f32, // 0.0-1.0, position of the split
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },

    /// A container that holds multiple children
    Container {
        id: NodeId,
        layout: ContainerLayout,
        children: Vec<LayoutNode>,
        active_child: Option<usize>,
    },
}

impl LayoutNode {
    /// Get the ID of this node
    pub fn id(&self) -> &NodeId {
        match self {
            LayoutNode::Zone { id, .. } => id,
            LayoutNode::Split { id, .. } => id,
            LayoutNode::Container { id, .. } => id,
        }
    }

    /// Check if this is a zone node
    pub fn is_zone(&self) -> bool {
        matches!(self, LayoutNode::Zone { .. })
    }

    /// Check if this is a split node
    pub fn is_split(&self) -> bool {
        matches!(self, LayoutNode::Split { .. })
    }

    /// Check if this is a container node
    pub fn is_container(&self) -> bool {
        matches!(self, LayoutNode::Container { .. })
    }

    /// Get mutable access to zone content if this is a zone
    pub fn zone_content_mut(&mut self) -> Option<&mut ZoneContent> {
        match self {
            LayoutNode::Zone { content, .. } => Some(content),
            _ => None,
        }
    }

    /// Get zone content if this is a zone
    pub fn zone_content(&self) -> Option<&ZoneContent> {
        match self {
            LayoutNode::Zone { content, .. } => Some(content),
            _ => None,
        }
    }

    /// Find a node by ID
    pub fn find_node(&self, target_id: &NodeId) -> Option<&LayoutNode> {
        if self.id() == target_id {
            return Some(self);
        }

        match self {
            LayoutNode::Split { first, second, .. } => first
                .find_node(target_id)
                .or_else(|| second.find_node(target_id)),
            LayoutNode::Container { children, .. } => {
                children.iter().find_map(|child| child.find_node(target_id))
            }
            _ => None,
        }
    }

    /// Find a node by ID (mutable)
    pub fn find_node_mut(&mut self, target_id: &NodeId) -> Option<&mut LayoutNode> {
        if self.id() == target_id {
            return Some(self);
        }

        match self {
            LayoutNode::Split { first, second, .. } => {
                if let Some(node) = first.find_node_mut(target_id) {
                    Some(node)
                } else {
                    second.find_node_mut(target_id)
                }
            }
            LayoutNode::Container { children, .. } => children
                .iter_mut()
                .find_map(|child| child.find_node_mut(target_id)),
            _ => None,
        }
    }

    /// Count the number of splits in the tree
    pub fn count_splits(&self) -> u32 {
        match self {
            LayoutNode::Split { first, second, .. } => {
                1 + first.count_splits() + second.count_splits()
            }
            LayoutNode::Container { children, .. } => {
                children.iter().map(|child| child.count_splits()).sum()
            }
            LayoutNode::Zone { .. } => 0,
        }
    }

    /// Get the ratio of the first split found (for debugging)
    pub fn get_first_split_ratio(&self) -> Option<f32> {
        match self {
            LayoutNode::Split { ratio, .. } => Some(*ratio),
            LayoutNode::Container { children, .. } => children
                .iter()
                .find_map(|child| child.get_first_split_ratio()),
            LayoutNode::Zone { .. } => None,
        }
    }

    /// Get node at path
    pub fn get_at_path(&self, path: &NodePath) -> Option<&LayoutNode> {
        let mut current = self;

        for &index in &path.0 {
            match current {
                LayoutNode::Split { first, second, .. } => {
                    current = match index {
                        0 => first,
                        1 => second,
                        _ => return None,
                    };
                }
                LayoutNode::Container { children, .. } => {
                    current = children.get(index)?;
                }
                _ => return None,
            }
        }

        Some(current)
    }

    /// Get node at path (mutable)
    pub fn get_at_path_mut(&mut self, path: &NodePath) -> Option<&mut LayoutNode> {
        let mut current = self;

        for &index in &path.0 {
            match current {
                LayoutNode::Split { first, second, .. } => {
                    current = match index {
                        0 => first,
                        1 => second,
                        _ => return None,
                    };
                }
                LayoutNode::Container { children, .. } => {
                    current = children.get_mut(index)?;
                }
                _ => return None,
            }
        }

        Some(current)
    }

    /// Collect all zone IDs in the tree
    pub fn collect_zone_ids(&self) -> Vec<NodeId> {
        let mut ids = Vec::new();
        // Tiger Style: Start depth tracking at 0 for bounded recursion.
        self.collect_zone_ids_recursive(&mut ids, 0);
        ids
    }

    /// Tiger Style: Add depth parameter to track and limit recursion.
    fn collect_zone_ids_recursive(&self, ids: &mut Vec<NodeId>, depth: u32) {
        // Tiger Style: Assert bounded recursion to prevent stack overflow.
        assert!(
            depth < MAX_NODE_DEPTH,
            "Node recursion depth {depth} exceeds maximum {MAX_NODE_DEPTH}"
        );

        match self {
            LayoutNode::Zone { id, .. } => {
                ids.push(id.clone());
            }
            LayoutNode::Split { first, second, .. } => {
                first.collect_zone_ids_recursive(ids, depth + 1);
                second.collect_zone_ids_recursive(ids, depth + 1);
            }
            LayoutNode::Container { children, .. } => {
                for child in children {
                    child.collect_zone_ids_recursive(ids, depth + 1);
                }
            }
        }
    }

    /// Count total nodes in the tree
    pub fn count_nodes(&self) -> u32 {
        match self {
            LayoutNode::Zone { .. } => 1,
            LayoutNode::Split { first, second, .. } => {
                1 + first.count_nodes() + second.count_nodes()
            }
            LayoutNode::Container { children, .. } => {
                1 + children.iter().map(|c| c.count_nodes()).sum::<u32>()
            }
        }
    }
}
