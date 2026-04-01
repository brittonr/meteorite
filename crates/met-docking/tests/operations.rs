use met_docking::flexible_layout::{
    LayoutOperation, LayoutOperationExecutor, LayoutTree, SplitDirection,
};

#[test]
fn execute_split_operation() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    let mut executor = LayoutOperationExecutor::new();

    executor
        .execute(
            LayoutOperation::SplitZone {
                zone_id: root_id.clone(),
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                new_zone_first: false,
            },
            &mut tree,
        )
        .expect("split operation should succeed");

    assert_eq!(tree.get_all_zone_ids().len(), 2);
    assert!(tree.root().is_split());
}

#[test]
fn execute_merge_operation() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    let mut executor = LayoutOperationExecutor::new();

    // Split first
    executor
        .execute(
            LayoutOperation::SplitZone {
                zone_id: root_id.clone(),
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                new_zone_first: false,
            },
            &mut tree,
        )
        .unwrap();

    let zones = tree.get_all_zone_ids();
    assert_eq!(zones.len(), 2);
    let zone1 = zones[0].clone();
    let zone2 = zones[1].clone();

    // Merge
    executor
        .execute(
            LayoutOperation::MergeZones {
                zone1_id: zone1,
                zone2_id: zone2,
            },
            &mut tree,
        )
        .expect("merge operation should succeed");

    assert_eq!(tree.get_all_zone_ids().len(), 1);
    assert!(tree.root().is_zone());
}

#[test]
fn undo_restores_previous_state() {
    let mut tree = LayoutTree::new();
    let original_zones = tree.get_all_zone_ids().len();
    let root_id = tree.get_all_zone_ids()[0].clone();
    let mut executor = LayoutOperationExecutor::new();

    // Execute a split
    executor
        .execute(
            LayoutOperation::SplitZone {
                zone_id: root_id,
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                new_zone_first: false,
            },
            &mut tree,
        )
        .unwrap();
    assert_eq!(tree.get_all_zone_ids().len(), 2);

    // Undo
    executor.undo(&mut tree).expect("undo should succeed");
    assert_eq!(
        tree.get_all_zone_ids().len(),
        original_zones,
        "undo should restore original zone count"
    );
    assert!(tree.root().is_zone(), "root should be a zone after undo");
}

#[test]
fn redo_reapplies_operation() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    let mut executor = LayoutOperationExecutor::new();

    // Execute a split
    executor
        .execute(
            LayoutOperation::SplitZone {
                zone_id: root_id,
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                new_zone_first: false,
            },
            &mut tree,
        )
        .unwrap();

    // Undo
    executor.undo(&mut tree).unwrap();
    assert_eq!(tree.get_all_zone_ids().len(), 1);

    // Redo
    executor.redo(&mut tree).expect("redo should succeed");
    assert_eq!(tree.get_all_zone_ids().len(), 2);
    assert!(tree.root().is_split(), "root should be a split after redo");
}

#[test]
fn undo_empty_stack_errors() {
    let mut tree = LayoutTree::new();
    let mut executor = LayoutOperationExecutor::new();

    let result = executor.undo(&mut tree);
    assert!(result.is_err(), "undo on empty stack should fail");
}

#[test]
fn multiple_operations_undo_in_order() {
    let mut tree = LayoutTree::new();
    let mut executor = LayoutOperationExecutor::new();

    // Operation 1: split root
    let root_id = tree.get_all_zone_ids()[0].clone();
    executor
        .execute(
            LayoutOperation::SplitZone {
                zone_id: root_id.clone(),
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                new_zone_first: false,
            },
            &mut tree,
        )
        .unwrap();
    assert_eq!(tree.get_all_zone_ids().len(), 2);

    // Operation 2: add panel
    executor
        .execute(
            LayoutOperation::AddPanel {
                panel_id: "panel1".to_string(),
                zone_id: root_id.clone(),
            },
            &mut tree,
        )
        .unwrap();

    // Operation 3: split again
    let zones = tree.get_all_zone_ids();
    let target = zones.iter().find(|z| *z != &root_id).unwrap().clone();
    executor
        .execute(
            LayoutOperation::SplitZone {
                zone_id: target,
                direction: SplitDirection::Vertical,
                ratio: 0.5,
                new_zone_first: false,
            },
            &mut tree,
        )
        .unwrap();
    assert_eq!(tree.get_all_zone_ids().len(), 3);

    // Undo all 3
    executor.undo(&mut tree).unwrap(); // undo split #2
    assert_eq!(tree.get_all_zone_ids().len(), 2);

    executor.undo(&mut tree).unwrap(); // undo add panel
    let root_content = tree.find_zone(&root_id).unwrap().zone_content().unwrap();
    assert!(
        !root_content.panels.contains(&"panel1".to_string()),
        "panel should be removed after undo"
    );

    executor.undo(&mut tree).unwrap(); // undo split #1
    assert_eq!(tree.get_all_zone_ids().len(), 1);
    assert!(tree.root().is_zone());
}
