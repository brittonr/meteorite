use met_docking::flexible_layout::{LayoutTree, SplitDirection};

#[test]
fn new_tree_has_single_root_zone() {
    let tree = LayoutTree::new();
    let zones = tree.get_all_zone_ids();
    assert_eq!(zones.len(), 1, "new tree should have exactly one zone");
    assert!(tree.root().is_zone(), "root should be a zone node");
}

#[test]
fn split_zone_creates_two_children() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    let new_zone_id = tree
        .split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .expect("split should succeed");

    let zones = tree.get_all_zone_ids();
    assert_eq!(zones.len(), 2, "split should produce two zones");
    assert!(tree.root().is_split(), "root should become a split node");
    // Both the original and new zone should exist
    assert!(tree.find_zone(&root_id).is_some(), "original zone should still exist");
    assert!(tree.find_zone(&new_zone_id).is_some(), "new zone should exist");
}

#[test]
fn split_zone_invalid_ratio_errors() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Ratio too low
    let result = tree.split_zone(&root_id, SplitDirection::Horizontal, 0.0, false);
    assert!(result.is_err(), "ratio 0.0 should fail");

    // Ratio too high
    let result = tree.split_zone(&root_id, SplitDirection::Horizontal, 1.0, false);
    assert!(result.is_err(), "ratio 1.0 should fail");

    // Ratio slightly below minimum
    let result = tree.split_zone(&root_id, SplitDirection::Horizontal, 0.05, false);
    assert!(result.is_err(), "ratio 0.05 should fail");

    // Ratio slightly above maximum
    let result = tree.split_zone(&root_id, SplitDirection::Horizontal, 0.95, false);
    assert!(result.is_err(), "ratio 0.95 should fail");

    // Valid ratios at boundaries should work
    let result = tree.split_zone(&root_id, SplitDirection::Horizontal, 0.1, false);
    assert!(result.is_ok(), "ratio 0.1 should succeed");
}

#[test]
fn merge_zones_restores_single_zone() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Split
    let new_zone_id = tree
        .split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .expect("split should succeed");
    assert_eq!(tree.get_all_zone_ids().len(), 2);

    // Merge
    let merged_id = tree
        .merge_zones(&root_id, &new_zone_id)
        .expect("merge should succeed");

    let zones = tree.get_all_zone_ids();
    assert_eq!(zones.len(), 1, "merge should leave one zone");
    assert!(tree.root().is_zone(), "root should be a zone after merge");
    assert!(tree.find_zone(&merged_id).is_some(), "merged zone should exist");
}

#[test]
fn swap_zones_exchanges_content() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Add panels to root zone
    tree.add_panel_to_zone(&root_id, "panel_a".to_string()).unwrap();
    tree.add_panel_to_zone(&root_id, "panel_b".to_string()).unwrap();

    // Split to create second zone
    let new_zone_id = tree
        .split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .expect("split should succeed");

    // Add panels to new zone
    tree.add_panel_to_zone(&new_zone_id, "panel_c".to_string()).unwrap();

    // Use the operation executor to swap
    use met_docking::flexible_layout::{LayoutOperation, LayoutOperationExecutor};
    let mut executor = LayoutOperationExecutor::new();
    executor
        .execute(
            LayoutOperation::SwapZones {
                zone1_id: root_id.clone(),
                zone2_id: new_zone_id.clone(),
            },
            &mut tree,
        )
        .expect("swap should succeed");

    // After swap, root zone should have panel_c, new zone should have panel_a and panel_b
    let root_panels = tree
        .find_zone(&root_id)
        .unwrap()
        .zone_content()
        .unwrap()
        .panels
        .clone();
    let new_panels = tree
        .find_zone(&new_zone_id)
        .unwrap()
        .zone_content()
        .unwrap()
        .panels
        .clone();

    assert_eq!(root_panels, vec!["panel_c"]);
    assert!(new_panels.contains(&"panel_a".to_string()));
    assert!(new_panels.contains(&"panel_b".to_string()));
}

#[test]
fn add_panel_to_zone() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    tree.add_panel_to_zone(&root_id, "test_panel".to_string())
        .expect("add panel should succeed");

    let content = tree.find_zone(&root_id).unwrap().zone_content().unwrap();
    assert!(content.panels.contains(&"test_panel".to_string()));

    // Panel cache should be updated
    assert_eq!(tree.find_panel_zone("test_panel"), Some(root_id));
}

#[test]
fn remove_panel_from_zone() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    tree.add_panel_to_zone(&root_id, "panel_x".to_string()).unwrap();
    assert!(tree.find_panel_zone("panel_x").is_some());

    tree.remove_panel("panel_x").expect("remove should succeed");

    let content = tree.find_zone(&root_id).unwrap().zone_content().unwrap();
    assert!(!content.panels.contains(&"panel_x".to_string()));
    assert!(tree.find_panel_zone("panel_x").is_none());
}

#[test]
fn move_panel_between_zones() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    tree.add_panel_to_zone(&root_id, "movable".to_string()).unwrap();

    let new_zone_id = tree
        .split_zone(&root_id, SplitDirection::Vertical, 0.5, false)
        .expect("split should succeed");

    tree.move_panel("movable", &new_zone_id, None)
        .expect("move should succeed");

    // Panel should be in new zone, not old zone
    let root_content = tree.find_zone(&root_id).unwrap().zone_content().unwrap();
    assert!(!root_content.panels.contains(&"movable".to_string()));

    let new_content = tree.find_zone(&new_zone_id).unwrap().zone_content().unwrap();
    assert!(new_content.panels.contains(&"movable".to_string()));

    assert_eq!(tree.find_panel_zone("movable"), Some(new_zone_id));
}

#[test]
fn find_zone_by_panel() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    tree.add_panel_to_zone(&root_id, "findme".to_string()).unwrap();

    let found = tree.find_panel_zone("findme");
    assert_eq!(found, Some(root_id));

    let not_found = tree.find_panel_zone("nonexistent");
    assert_eq!(not_found, None);
}

#[test]
fn deeply_nested_split_up_to_depth_limit() {
    let mut tree = LayoutTree::new();

    // Split repeatedly to build depth. Each split increases depth by 1.
    // The limit is 100. We should be able to get to depth ~90 safely.
    for i in 0..20 {
        let zones = tree.get_all_zone_ids();
        // Always split the last zone to increase depth linearly
        let target = zones.last().unwrap().clone();
        let result = tree.split_zone(&target, SplitDirection::Horizontal, 0.5, false);
        assert!(
            result.is_ok(),
            "split at depth {} should succeed, got {:?}",
            i,
            result.err()
        );
    }

    // Tree should have 21 zones (1 initial + 20 splits, each adding 1)
    assert_eq!(tree.get_all_zone_ids().len(), 21);
}

#[test]
fn remove_last_panel_does_not_crash() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    tree.add_panel_to_zone(&root_id, "only_panel".to_string()).unwrap();
    tree.remove_panel("only_panel").expect("remove should succeed");

    // Zone should still exist with no panels
    let content = tree.find_zone(&root_id).unwrap().zone_content().unwrap();
    assert!(content.panels.is_empty());
    assert!(content.active_panel.is_none());
}

#[test]
fn cache_consistency_after_operations() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Add panels
    tree.add_panel_to_zone(&root_id, "p1".to_string()).unwrap();
    tree.add_panel_to_zone(&root_id, "p2".to_string()).unwrap();

    // Split
    let zone2 = tree
        .split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .unwrap();

    // Move panel
    tree.move_panel("p2", &zone2, None).unwrap();

    // Add another panel
    tree.add_panel_to_zone(&zone2, "p3".to_string()).unwrap();

    // Verify caches match actual tree state
    assert_eq!(tree.find_panel_zone("p1"), Some(root_id.clone()));
    assert_eq!(tree.find_panel_zone("p2"), Some(zone2.clone()));
    assert_eq!(tree.find_panel_zone("p3"), Some(zone2.clone()));

    // Zone IDs in cache should match tree traversal
    let zone_ids = tree.get_all_zone_ids();
    assert_eq!(zone_ids.len(), 2);
    assert!(zone_ids.contains(&root_id));
    assert!(zone_ids.contains(&zone2));

    // Panels should be in the right zones
    let root_panels = tree.find_zone(&root_id).unwrap().zone_content().unwrap();
    assert_eq!(root_panels.panels, vec!["p1"]);

    let zone2_panels = tree.find_zone(&zone2).unwrap().zone_content().unwrap();
    assert!(zone2_panels.panels.contains(&"p2".to_string()));
    assert!(zone2_panels.panels.contains(&"p3".to_string()));
}
