//! Tree data model: items, flattening, and visible-row computation.
//!
//! Uses ratcore's tree walk algorithm under the hood. The `TreeItem` struct
//! provides the Dioxus-friendly String-based API; a private adapter bridges
//! it to ratcore's `TreeData` trait.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use ratcore::tree::{self as rc, TreeData};

/// A single node in the tree.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeItem {
    pub id: String,
    pub label: String,
    pub parent_id: Option<String>,
    pub icon: Option<String>,
}

impl TreeItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            parent_id: None,
            icon: None,
        }
    }

    pub fn parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// A flattened visible row produced by walking expanded nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct VisibleRow {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub depth: usize,
    pub has_children: bool,
    pub is_expanded: bool,
    pub is_last_sibling: bool,
    /// Per-ancestor flag: true if that ancestor is the last sibling at its
    /// level (so the guide connector should be blank, not │).
    pub ancestors_last: Vec<bool>,
}

// ── Adapter: Vec<TreeItem> → ratcore::tree::TreeData ────────────────────────

struct TreeItemsAdapter {
    items: Vec<TreeItem>,
    roots: Vec<usize>,
    children: BTreeMap<usize, Vec<usize>>,
    id_to_idx: HashMap<String, usize>,
    parent_idx: Vec<Option<usize>>,
}

impl TreeItemsAdapter {
    fn new(items: &[TreeItem]) -> Self {
        let id_to_idx: HashMap<String, usize> = items
            .iter()
            .enumerate()
            .map(|(i, item)| (item.id.clone(), i))
            .collect();

        let mut roots = Vec::new();
        let mut children: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut parent_idx = vec![None; items.len()];

        for (i, item) in items.iter().enumerate() {
            match item.parent_id.as_ref().and_then(|pid| id_to_idx.get(pid.as_str())) {
                Some(&pidx) => {
                    children.entry(pidx).or_default().push(i);
                    parent_idx[i] = Some(pidx);
                }
                None if item.parent_id.is_none() => {
                    roots.push(i);
                }
                None => {
                    // Parent ID given but not found — treat as root.
                    roots.push(i);
                }
            }
        }

        Self {
            items: items.to_vec(),
            roots,
            children,
            id_to_idx,
            parent_idx,
        }
    }
}

impl TreeData for TreeItemsAdapter {
    fn root_count(&self) -> usize {
        self.roots.len()
    }

    fn root(&self, index: usize) -> usize {
        self.roots[index]
    }

    fn child_count(&self, node: usize) -> usize {
        self.children.get(&node).map(|v| v.len()).unwrap_or(0)
    }

    fn child(&self, node: usize, index: usize) -> usize {
        self.children[&node][index]
    }

    fn node_label(&self, node: usize) -> &str {
        &self.items[node].label
    }

    fn node_icon(&self, node: usize) -> Option<&str> {
        self.items[node].icon.as_deref()
    }

    fn parent(&self, node: usize) -> Option<usize> {
        self.parent_idx[node]
    }
}

/// Build the flat visible-row list from items + expanded set.
///
/// Delegates to ratcore's tree walk, then maps back to String-based rows.
pub fn compute_visible_rows(items: &[TreeItem], expanded: &BTreeSet<String>) -> Vec<VisibleRow> {
    let adapter = TreeItemsAdapter::new(items);

    // Convert String expanded set → usize expanded set.
    let expanded_idx: BTreeSet<usize> = expanded
        .iter()
        .filter_map(|id| adapter.id_to_idx.get(id.as_str()).copied())
        .collect();

    let rc_rows = rc::compute_visible_rows(&adapter, &expanded_idx);

    rc_rows
        .into_iter()
        .map(|r| {
            let item = &adapter.items[r.node_id];
            VisibleRow {
                id: item.id.clone(),
                label: item.label.clone(),
                icon: item.icon.clone(),
                depth: r.depth,
                has_children: r.has_children,
                is_expanded: r.is_expanded,
                is_last_sibling: r.is_last_sibling,
                ancestors_last: r.ancestors_last,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_items() -> Vec<TreeItem> {
        vec![
            TreeItem::new("0", "root"),
            TreeItem::new("1", "a").parent("0"),
            TreeItem::new("2", "b").parent("0"),
            TreeItem::new("3", "a1").parent("1"),
            TreeItem::new("4", "a2").parent("1"),
            TreeItem::new("5", "b1").parent("2"),
        ]
    }

    #[test]
    fn all_collapsed_shows_only_roots() {
        let rows = compute_visible_rows(&sample_items(), &BTreeSet::new());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "0");
        assert!(!rows[0].is_expanded);
    }

    #[test]
    fn expand_root() {
        let expanded = BTreeSet::from(["0".into()]);
        let rows = compute_visible_rows(&sample_items(), &expanded);
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, vec!["0", "1", "2"]);
        assert!(rows[0].is_expanded);
        assert_eq!(rows[1].depth, 1);
    }

    #[test]
    fn nested_expand() {
        let expanded = BTreeSet::from(["0".into(), "1".into()]);
        let rows = compute_visible_rows(&sample_items(), &expanded);
        let ids: Vec<&str> = rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, vec!["0", "1", "3", "4", "2"]);
        assert_eq!(rows[2].depth, 2);
    }

    #[test]
    fn last_sibling_flags() {
        let expanded = BTreeSet::from(["0".into(), "1".into()]);
        let rows = compute_visible_rows(&sample_items(), &expanded);
        assert!(!rows[1].is_last_sibling);
        assert!(rows[4].is_last_sibling);
        assert_eq!(rows[2].ancestors_last, vec![true, false]);
    }

    #[test]
    fn empty_items() {
        let rows = compute_visible_rows(&[], &BTreeSet::new());
        assert!(rows.is_empty());
    }

    #[test]
    fn multiple_roots() {
        let items = vec![
            TreeItem::new("a", "Alpha"),
            TreeItem::new("b", "Beta"),
        ];
        let rows = compute_visible_rows(&items, &BTreeSet::new());
        assert_eq!(rows.len(), 2);
        assert!(!rows[0].is_last_sibling);
        assert!(rows[1].is_last_sibling);
    }
}
