//! Tree data model: items, flattening, and visible-row computation.

use std::collections::{BTreeMap, BTreeSet};

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

/// Build the flat visible-row list from items + expanded set.
pub fn compute_visible_rows(items: &[TreeItem], expanded: &BTreeSet<String>) -> Vec<VisibleRow> {
    // Index: parent_id → ordered children
    let mut children_map: BTreeMap<Option<&str>, Vec<&TreeItem>> = BTreeMap::new();
    for item in items {
        children_map
            .entry(item.parent_id.as_deref())
            .or_default()
            .push(item);
    }

    let child_ids: BTreeSet<&str> = items
        .iter()
        .filter_map(|i| i.parent_id.as_deref())
        .collect();

    let mut rows = Vec::new();
    let roots: Vec<&TreeItem> = children_map.get(&None).cloned().unwrap_or_default();
    let root_count = roots.len();

    for (i, root) in roots.iter().enumerate() {
        let is_last = i == root_count - 1;
        walk(
            root,
            &children_map,
            &child_ids,
            expanded,
            0,
            is_last,
            &[],
            &mut rows,
        );
    }

    rows
}

fn walk(
    item: &TreeItem,
    children_map: &BTreeMap<Option<&str>, Vec<&TreeItem>>,
    parent_ids: &BTreeSet<&str>,
    expanded: &BTreeSet<String>,
    depth: usize,
    is_last_sibling: bool,
    ancestors_last: &[bool],
    rows: &mut Vec<VisibleRow>,
) {
    let has_children = parent_ids.contains(item.id.as_str());
    let is_expanded = has_children && expanded.contains(&item.id);

    rows.push(VisibleRow {
        id: item.id.clone(),
        label: item.label.clone(),
        icon: item.icon.clone(),
        depth,
        has_children,
        is_expanded,
        is_last_sibling,
        ancestors_last: ancestors_last.to_vec(),
    });

    if is_expanded {
        let kids: Vec<&TreeItem> = children_map
            .get(&Some(item.id.as_str()))
            .cloned()
            .unwrap_or_default();
        let kid_count = kids.len();
        let mut child_ancestors: Vec<bool> = ancestors_last.to_vec();
        child_ancestors.push(is_last_sibling);

        for (ci, kid) in kids.iter().enumerate() {
            let kid_is_last = ci == kid_count - 1;
            walk(
                kid,
                children_map,
                parent_ids,
                expanded,
                depth + 1,
                kid_is_last,
                &child_ancestors,
                rows,
            );
        }
    }
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
        // a (index 1) is not last sibling, b (index 4) is
        assert!(!rows[1].is_last_sibling);
        assert!(rows[4].is_last_sibling);
        // a1 ancestors_last: [true(root), false(a is not last)]
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
