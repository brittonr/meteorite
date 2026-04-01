//! Functional core: layout tree management

use super::node::{LayoutNode, NodeId, NodePath, SplitDirection, ZoneContent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Tiger Style: Fixed limit on recursion depth to prevent stack overflow.
/// This prevents malicious or malformed deeply nested layouts from crashing.
/// A depth of 100 allows for extremely complex layouts while staying well within stack limits.
const MAX_TREE_DEPTH: u32 = 100;

/// Errors that can occur during tree operations
#[derive(Error, Debug)]
pub enum TreeError {
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Panel not found: {0}")]
    PanelNotFound(String),

    #[error("Zone is not a content zone")]
    NotAZone,

    #[error("Cannot split a container node")]
    CannotSplitContainer,

    #[error("Invalid split ratio: {0}")]
    InvalidSplitRatio(f32),

    #[error("Layout tree exceeds maximum depth of {0}")]
    MaxDepthExceeded(u32),
}

/// The layout tree structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutTree {
    /// Root node of the tree
    root: LayoutNode,

    /// Cache for quick zone lookups
    #[serde(skip)]
    zone_cache: HashMap<NodeId, NodePath>,

    /// Cache for panel to zone mappings
    #[serde(skip)]
    panel_cache: HashMap<String, NodeId>,
}

impl LayoutTree {
    /// Create a new layout tree with a single root zone
    pub fn new() -> Self {
        let root_id = NodeId::from_string("root");
        let root = LayoutNode::Zone {
            id: root_id,
            content: ZoneContent::new(),
            constraints: None,
        };

        let mut tree = Self {
            root,
            zone_cache: HashMap::new(),
            panel_cache: HashMap::new(),
        };

        tree.rebuild_caches();
        tree
    }

    /// Create a layout tree from an existing root node
    pub fn from_root(root: LayoutNode) -> Self {
        let mut tree = Self {
            root,
            zone_cache: HashMap::new(),
            panel_cache: HashMap::new(),
        };

        tree.rebuild_caches();
        tree
    }

    /// Get the root node
    pub fn root(&self) -> &LayoutNode {
        &self.root
    }

    /// Get the root node (mutable)
    pub fn root_mut(&mut self) -> &mut LayoutNode {
        &mut self.root
    }

    /// Rebuild internal caches
    fn rebuild_caches(&mut self) {
        self.zone_cache.clear();
        self.panel_cache.clear();
        // Tiger Style: Start depth tracking at 0 for bounded recursion.
        self.build_cache_recursive(&self.root.clone(), &NodePath::root(), 0);
    }

    /// Tiger Style: Add depth parameter to track and limit recursion.
    /// Panics if depth exceeds MAX_TREE_DEPTH (programmer error if tree is malformed).
    fn build_cache_recursive(&mut self, node: &LayoutNode, path: &NodePath, depth: u32) {
        // Tiger Style: Assert bounded recursion to prevent stack overflow.
        assert!(
            depth < MAX_TREE_DEPTH,
            "Layout tree recursion depth {depth} exceeds maximum {MAX_TREE_DEPTH}"
        );

        if let LayoutNode::Zone { id, content, .. } = node {
            self.zone_cache.insert(id.clone(), path.clone());

            for panel in &content.panels {
                self.panel_cache.insert(panel.clone(), id.clone());
            }
        }

        match node {
            LayoutNode::Split { first, second, .. } => {
                self.build_cache_recursive(first, &path.child(0), depth + 1);
                self.build_cache_recursive(second, &path.child(1), depth + 1);
            }
            LayoutNode::Container { children, .. } => {
                for (i, child) in children.iter().enumerate() {
                    self.build_cache_recursive(child, &path.child(i), depth + 1);
                }
            }
            _ => {}
        }
    }

    /// Find a zone by ID
    pub fn find_zone(&self, zone_id: &NodeId) -> Option<&LayoutNode> {
        self.root.find_node(zone_id).filter(|n| n.is_zone())
    }

    /// Find a zone by ID (mutable)
    pub fn find_zone_mut(&mut self, zone_id: &NodeId) -> Option<&mut LayoutNode> {
        self.root.find_node_mut(zone_id).filter(|n| n.is_zone())
    }

    /// Get all zone IDs
    pub fn get_all_zone_ids(&self) -> Vec<NodeId> {
        self.root.collect_zone_ids()
    }

    /// Find which zone contains a panel
    pub fn find_panel_zone(&self, panel_id: &str) -> Option<NodeId> {
        self.panel_cache.get(panel_id).cloned()
    }

    /// Add a panel to a zone
    pub fn add_panel_to_zone(
        &mut self,
        zone_id: &NodeId,
        panel_id: String,
    ) -> Result<(), TreeError> {
        let zone = self
            .find_zone_mut(zone_id)
            .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;

        let content = zone.zone_content_mut().ok_or(TreeError::NotAZone)?;

        content.add_panel(panel_id.clone());
        self.panel_cache.insert(panel_id, zone_id.clone());

        Ok(())
    }

    /// Add a panel to a zone at a specific index
    ///
    /// If the index is greater than the number of panels, the panel is appended.
    pub fn add_panel_to_zone_at(
        &mut self,
        zone_id: &NodeId,
        panel_id: String,
        index: usize,
    ) -> Result<(), TreeError> {
        let zone = self
            .find_zone_mut(zone_id)
            .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;

        let content = zone.zone_content_mut().ok_or(TreeError::NotAZone)?;

        debug_assert!(index <= u32::MAX as usize);
        content.insert_panel_at(panel_id.clone(), index as u32);
        self.panel_cache.insert(panel_id, zone_id.clone());

        Ok(())
    }

    /// Remove a panel from its zone
    pub fn remove_panel(&mut self, panel_id: &str) -> Result<(), TreeError> {
        let zone_id = self
            .find_panel_zone(panel_id)
            .ok_or_else(|| TreeError::PanelNotFound(panel_id.to_string()))?;

        let zone = self
            .find_zone_mut(&zone_id)
            .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;

        let content = zone.zone_content_mut().ok_or(TreeError::NotAZone)?;

        if content.remove_panel(panel_id) {
            self.panel_cache.remove(panel_id);
            Ok(())
        } else {
            Err(TreeError::PanelNotFound(panel_id.to_string()))
        }
    }

    /// Move a panel from one zone to another
    ///
    /// If `insert_index` is `Some(idx)`, the panel is inserted at that position.
    /// If `None`, the panel is appended to the end.
    pub fn move_panel(
        &mut self,
        panel_id: &str,
        target_zone_id: &NodeId,
        insert_index: Option<usize>,
    ) -> Result<(), TreeError> {
        // Remove from current zone
        self.remove_panel(panel_id)?;

        // Add to target zone at specified index or append
        if let Some(idx) = insert_index {
            self.add_panel_to_zone_at(target_zone_id, panel_id.to_string(), idx)?;
        } else {
            self.add_panel_to_zone(target_zone_id, panel_id.to_string())?;
        }

        // Set the moved panel as active in the target zone
        if let Some(zone) = self.find_zone_mut(target_zone_id) {
            if let Some(content) = zone.zone_content_mut() {
                content.active_panel = Some(panel_id.to_string());
            }
        }

        Ok(())
    }

    /// Split a zone into two zones
    pub fn split_zone(
        &mut self,
        zone_id: &NodeId,
        direction: SplitDirection,
        ratio: f32,
        new_zone_first: bool,
    ) -> Result<NodeId, TreeError> {
        if !(0.1..=0.9).contains(&ratio) {
            return Err(TreeError::InvalidSplitRatio(ratio));
        }

        // Find the zone to split
        let path = self
            .zone_cache
            .get(zone_id)
            .cloned()
            .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;

        // Create new zone
        let new_zone_id = NodeId::new();
        let new_zone = LayoutNode::Zone {
            id: new_zone_id.clone(),
            content: ZoneContent::new(),
            constraints: None,
        };

        // Get the node to replace
        let node_to_split = if path.0.is_empty() {
            // Splitting root
            std::mem::replace(
                &mut self.root,
                LayoutNode::Zone {
                    id: NodeId::new(),
                    content: ZoneContent::new(),
                    constraints: None,
                },
            )
        } else {
            // Navigate to parent and extract the node
            // Safety: parent() and last() are guaranteed to return Some for non-root paths.
            let parent_path = path.parent().ok_or_else(|| {
                TreeError::InvalidOperation("Cannot get parent of root path".to_string())
            })?;
            let last_index = *path.0.last().ok_or_else(|| {
                TreeError::InvalidOperation("Empty path has no last index".to_string())
            })?;

            let parent = self
                .root
                .get_at_path_mut(&parent_path)
                .ok_or_else(|| TreeError::NodeNotFound("parent".to_string()))?;

            match parent {
                LayoutNode::Split { first, second, .. } => match last_index {
                    0 => std::mem::replace(first.as_mut(), new_zone.clone()),
                    1 => std::mem::replace(second.as_mut(), new_zone.clone()),
                    _ => return Err(TreeError::InvalidOperation("Invalid index".to_string())),
                },
                LayoutNode::Container { children, .. } => {
                    if last_index >= children.len() {
                        return Err(TreeError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                    std::mem::replace(&mut children[last_index], new_zone.clone())
                }
                _ => {
                    return Err(TreeError::InvalidOperation(
                        "Parent is not a split or container".to_string(),
                    ))
                }
            }
        };

        // Create the split node
        let split_id = NodeId::new();
        let split = LayoutNode::Split {
            id: split_id,
            direction,
            ratio,
            first: if new_zone_first {
                Box::new(new_zone.clone())
            } else {
                Box::new(node_to_split.clone())
            },
            second: if new_zone_first {
                Box::new(node_to_split)
            } else {
                Box::new(new_zone)
            },
        };

        // Replace the node
        if path.0.is_empty() {
            self.root = split;
        } else {
            // Safety: path is non-empty so parent() and last() are guaranteed to return Some.
            let parent_path = path
                .parent()
                .expect("non-empty path verified above has parent");
            let last_index = *path
                .0
                .last()
                .expect("non-empty path verified above has last index");

            let parent = self
                .root
                .get_at_path_mut(&parent_path)
                .ok_or_else(|| TreeError::NodeNotFound("parent".to_string()))?;

            match parent {
                LayoutNode::Split { first, second, .. } => match last_index {
                    0 => **first = split,
                    1 => **second = split,
                    _ => return Err(TreeError::InvalidOperation("Invalid index".to_string())),
                },
                LayoutNode::Container { children, .. } => {
                    children[last_index] = split;
                }
                _ => {
                    return Err(TreeError::InvalidOperation(
                        "Parent is not a split or container".to_string(),
                    ))
                }
            }
        }

        // Rebuild caches
        self.rebuild_caches();

        Ok(new_zone_id)
    }

    /// Remove a zone and promote its sibling (if in a split)
    pub fn remove_zone(&mut self, zone_id: &NodeId) -> Result<(), TreeError> {
        let (_path, parent_path, last_index) = self.remove_zone_validate(zone_id)?;
        let sibling = self.remove_zone_find_sibling(&parent_path, last_index)?;
        self.remove_zone_update_tree(sibling, &parent_path)?;
        self.rebuild_caches();
        Ok(())
    }

    /// Validate that the zone can be removed and get path information
    fn remove_zone_validate(
        &self,
        zone_id: &NodeId,
    ) -> Result<(NodePath, NodePath, usize), TreeError> {
        let path = self
            .zone_cache
            .get(zone_id)
            .cloned()
            .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;

        if path.0.is_empty() {
            return Err(TreeError::InvalidOperation(
                "Cannot remove root zone".to_string(),
            ));
        }

        // Safety: we verified path is non-empty above, so parent() and last() return Some.
        let parent_path = path
            .parent()
            .expect("non-empty path verified above has parent");
        let last_index = *path
            .0
            .last()
            .expect("non-empty path verified above has last index");

        Ok((path, parent_path, last_index))
    }

    /// Find the sibling node that will replace the parent split
    fn remove_zone_find_sibling(
        &self,
        parent_path: &NodePath,
        last_index: usize,
    ) -> Result<LayoutNode, TreeError> {
        let parent = self
            .root
            .get_at_path(parent_path)
            .ok_or_else(|| TreeError::NodeNotFound("parent".to_string()))?;

        if let LayoutNode::Split { first, second, .. } = parent {
            let sibling = match last_index {
                0 => second.as_ref().clone(),
                1 => first.as_ref().clone(),
                _ => return Err(TreeError::InvalidOperation("Invalid index".to_string())),
            };
            Ok(sibling)
        } else {
            Err(TreeError::InvalidOperation(
                "Parent is not a split".to_string(),
            ))
        }
    }

    /// Update the tree structure by replacing the parent split with the sibling
    fn remove_zone_update_tree(
        &mut self,
        sibling: LayoutNode,
        parent_path: &NodePath,
    ) -> Result<(), TreeError> {
        if parent_path.0.is_empty() {
            self.root = sibling;
        } else {
            let grandparent_path = parent_path
                .parent()
                .expect("non-empty parent_path has grandparent for zone removal");
            let parent_index = *parent_path
                .0
                .last()
                .expect("non-empty parent_path has last index for zone removal");

            let grandparent = self
                .root
                .get_at_path_mut(&grandparent_path)
                .ok_or_else(|| TreeError::NodeNotFound("grandparent".to_string()))?;

            match grandparent {
                LayoutNode::Split { first, second, .. } => match parent_index {
                    0 => **first = sibling,
                    1 => **second = sibling,
                    _ => return Err(TreeError::InvalidOperation("Invalid index".to_string())),
                },
                LayoutNode::Container { children, .. } => {
                    children[parent_index] = sibling;
                }
                _ => {
                    return Err(TreeError::InvalidOperation(
                        "Grandparent is not a split or container".to_string(),
                    ))
                }
            }
        }
        Ok(())
    }

    /// Update split ratio
    pub fn update_split_ratio(
        &mut self,
        split_id: &NodeId,
        new_ratio: f32,
    ) -> Result<(), TreeError> {
        if !(0.1..=0.9).contains(&new_ratio) {
            return Err(TreeError::InvalidSplitRatio(new_ratio));
        }

        let node = self
            .root
            .find_node_mut(split_id)
            .ok_or_else(|| TreeError::NodeNotFound(split_id.0.clone()))?;

        if let LayoutNode::Split { ratio, .. } = node {
            *ratio = new_ratio;

            Ok(())
        } else {
            Err(TreeError::InvalidOperation(
                "Node is not a split".to_string(),
            ))
        }
    }

    /// Set active panel in a zone
    pub fn set_active_panel(
        &mut self,
        zone_id: &NodeId,
        panel_id: Option<String>,
    ) -> Result<(), TreeError> {
        let zone = self
            .find_zone_mut(zone_id)
            .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;

        let content = zone.zone_content_mut().ok_or(TreeError::NotAZone)?;

        content.set_active_panel(panel_id);
        Ok(())
    }

    /// Merge two adjacent zones into one
    ///
    /// The zones must be siblings (children of the same Split node).
    /// All panels from both zones are combined into a single zone.
    /// Returns the ID of the merged zone.
    pub fn merge_zones(
        &mut self,
        zone1_id: &NodeId,
        zone2_id: &NodeId,
    ) -> Result<NodeId, TreeError> {
        // Find paths to both zones
        let path1 = self
            .zone_cache
            .get(zone1_id)
            .cloned()
            .ok_or_else(|| TreeError::NodeNotFound(zone1_id.0.clone()))?;

        let path2 = self
            .zone_cache
            .get(zone2_id)
            .cloned()
            .ok_or_else(|| TreeError::NodeNotFound(zone2_id.0.clone()))?;

        // Check that both zones have the same parent (are siblings)
        let parent_path1 = path1
            .parent()
            .ok_or_else(|| TreeError::InvalidOperation("Cannot merge root zone".to_string()))?;
        let parent_path2 = path2
            .parent()
            .ok_or_else(|| TreeError::InvalidOperation("Cannot merge root zone".to_string()))?;

        if parent_path1 != parent_path2 {
            return Err(TreeError::InvalidOperation(
                "Zones must be siblings (have the same parent) to merge".to_string(),
            ));
        }

        // Get panels from both zones
        let zone1 = self
            .find_zone(zone1_id)
            .ok_or_else(|| TreeError::NodeNotFound(zone1_id.0.clone()))?;
        let zone2 = self
            .find_zone(zone2_id)
            .ok_or_else(|| TreeError::NodeNotFound(zone2_id.0.clone()))?;

        let panels1 = zone1
            .zone_content()
            .map(|c| c.panels.clone())
            .unwrap_or_default();
        let panels2 = zone2
            .zone_content()
            .map(|c| c.panels.clone())
            .unwrap_or_default();

        // Determine which zone to keep (prefer zone1, keep its ID)
        let merged_zone_id = zone1_id.clone();

        // Create merged zone content
        let mut merged_content = ZoneContent::new();
        for panel in panels1 {
            merged_content.add_panel(panel);
        }
        for panel in panels2 {
            merged_content.add_panel(panel);
        }

        // Create the merged zone node
        let merged_zone = LayoutNode::Zone {
            id: merged_zone_id.clone(),
            content: merged_content,
            constraints: None,
        };

        // Replace the parent split with the merged zone
        if parent_path1.0.is_empty() {
            // Parent is root
            self.root = merged_zone;
        } else {
            let grandparent_path = parent_path1.parent().ok_or_else(|| {
                TreeError::InvalidOperation("Cannot get grandparent path".to_string())
            })?;
            let parent_index = *parent_path1
                .0
                .last()
                .ok_or_else(|| TreeError::InvalidOperation("Empty parent path".to_string()))?;

            let grandparent = self
                .root
                .get_at_path_mut(&grandparent_path)
                .ok_or_else(|| TreeError::NodeNotFound("grandparent".to_string()))?;

            match grandparent {
                LayoutNode::Split { first, second, .. } => match parent_index {
                    0 => **first = merged_zone,
                    1 => **second = merged_zone,
                    _ => {
                        return Err(TreeError::InvalidOperation(
                            "Invalid parent index".to_string(),
                        ))
                    }
                },
                LayoutNode::Container { children, .. } => {
                    if parent_index >= children.len() {
                        return Err(TreeError::InvalidOperation(
                            "Parent index out of bounds".to_string(),
                        ));
                    }
                    children[parent_index] = merged_zone;
                }
                _ => {
                    return Err(TreeError::InvalidOperation(
                        "Grandparent is not a split or container".to_string(),
                    ))
                }
            }
        }

        // Rebuild caches
        self.rebuild_caches();

        Ok(merged_zone_id)
    }

    /// Create a container from multiple zones
    ///
    /// Groups the specified zones into a Container node with the given layout.
    /// The zones must all be siblings (children of the same parent).
    /// Returns the ID of the new container.
    pub fn create_container(
        &mut self,
        zone_ids: &[NodeId],
        layout: super::node::ContainerLayout,
    ) -> Result<NodeId, TreeError> {
        if zone_ids.is_empty() {
            return Err(TreeError::InvalidOperation(
                "Cannot create container from empty zone list".to_string(),
            ));
        }

        if zone_ids.len() == 1 {
            return Err(TreeError::InvalidOperation(
                "Cannot create container from single zone".to_string(),
            ));
        }

        // Collect all zones and verify they exist
        let mut zones_to_container: Vec<LayoutNode> = Vec::new();
        for zone_id in zone_ids {
            let zone = self
                .find_zone(zone_id)
                .ok_or_else(|| TreeError::NodeNotFound(zone_id.0.clone()))?;
            zones_to_container.push(zone.clone());
        }

        // Create the container
        let container_id = NodeId::new();
        let container = LayoutNode::Container {
            id: container_id.clone(),
            layout,
            children: zones_to_container,
            active_child: Some(0),
        };

        // For simplicity, we replace the first zone's position with the container
        // and remove the other zones
        let first_zone_id = &zone_ids[0];
        let first_path = self
            .zone_cache
            .get(first_zone_id)
            .cloned()
            .ok_or_else(|| TreeError::NodeNotFound(first_zone_id.0.clone()))?;

        if first_path.0.is_empty() {
            // First zone is root
            self.root = container;
        } else {
            let parent_path = first_path
                .parent()
                .ok_or_else(|| TreeError::InvalidOperation("Cannot get parent path".to_string()))?;
            let first_index = *first_path
                .0
                .last()
                .ok_or_else(|| TreeError::InvalidOperation("Empty path".to_string()))?;

            let parent = self
                .root
                .get_at_path_mut(&parent_path)
                .ok_or_else(|| TreeError::NodeNotFound("parent".to_string()))?;

            match parent {
                LayoutNode::Split { first, second, .. } => match first_index {
                    0 => **first = container,
                    1 => **second = container,
                    _ => return Err(TreeError::InvalidOperation("Invalid index".to_string())),
                },
                LayoutNode::Container { children, .. } => {
                    if first_index >= children.len() {
                        return Err(TreeError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                    children[first_index] = container;
                }
                _ => {
                    return Err(TreeError::InvalidOperation(
                        "Parent is not a split or container".to_string(),
                    ))
                }
            }
        }

        // Rebuild caches
        self.rebuild_caches();

        Ok(container_id)
    }

    /// Check if two zones are siblings (have the same parent)
    pub fn are_siblings(&self, zone1_id: &NodeId, zone2_id: &NodeId) -> bool {
        let path1 = match self.zone_cache.get(zone1_id) {
            Some(p) => p,
            None => return false,
        };
        let path2 = match self.zone_cache.get(zone2_id) {
            Some(p) => p,
            None => return false,
        };

        match (path1.parent(), path2.parent()) {
            (Some(p1), Some(p2)) => p1 == p2,
            _ => false,
        }
    }

    /// Get the parent split of a zone (if any)
    pub fn get_parent_split(&self, zone_id: &NodeId) -> Option<&LayoutNode> {
        let path = self.zone_cache.get(zone_id)?;
        let parent_path = path.parent()?;
        let parent = self.root.get_at_path(&parent_path)?;
        if parent.is_split() {
            Some(parent)
        } else {
            None
        }
    }
}

impl Default for LayoutTree {
    fn default() -> Self {
        Self::new()
    }
}
