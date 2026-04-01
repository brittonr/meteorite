use met_docking::flexible_layout::{
    ContainerLayout, LayoutNode, LayoutRenderer, LayoutTree, NodeId, Rect, SplitDirection,
    ZoneContent,
};
use met_docking::flexible_layout::operations::LayoutPreset;

fn make_bounds() -> Rect {
    Rect::new(0.0, 0.0, 1000.0, 800.0)
}

#[test]
fn render_single_zone_fills_bounds() {
    let tree = LayoutTree::new();
    let renderer = LayoutRenderer::new(&tree);
    let bounds = make_bounds();
    let layout = renderer.render(bounds);

    assert_eq!(layout.zone_rects.len(), 1, "should have exactly one zone rect");

    let rect = layout.zone_rects.values().next().unwrap();
    // Zone gets 2px padding on each side
    let padding = 2.0;
    assert!((rect.x - padding).abs() < 0.01);
    assert!((rect.y - padding).abs() < 0.01);
    assert!((rect.width - (bounds.width - 2.0 * padding)).abs() < 0.01);
    assert!((rect.height - (bounds.height - 2.0 * padding)).abs() < 0.01);
}

#[test]
fn render_horizontal_split_produces_two_rects() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    tree.split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .unwrap();

    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(make_bounds());

    assert_eq!(layout.zone_rects.len(), 2, "should have two zone rects after horizontal split");

    let rects: Vec<&Rect> = layout.zone_rects.values().collect();
    // One should be on the left half, one on the right half
    let left = rects.iter().min_by(|a, b| a.x.partial_cmp(&b.x).unwrap()).unwrap();
    let right = rects.iter().max_by(|a, b| a.x.partial_cmp(&b.x).unwrap()).unwrap();
    assert!(left.x < right.x, "left rect should have smaller x");
}

#[test]
fn render_vertical_split_produces_two_rects() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    tree.split_zone(&root_id, SplitDirection::Vertical, 0.5, false)
        .unwrap();

    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(make_bounds());

    assert_eq!(layout.zone_rects.len(), 2, "should have two zone rects after vertical split");

    let rects: Vec<&Rect> = layout.zone_rects.values().collect();
    let top = rects.iter().min_by(|a, b| a.y.partial_cmp(&b.y).unwrap()).unwrap();
    let bottom = rects.iter().max_by(|a, b| a.y.partial_cmp(&b.y).unwrap()).unwrap();
    assert!(top.y < bottom.y, "top rect should have smaller y");
}

#[test]
fn render_nested_splits() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // First split horizontally
    let zone2 = tree
        .split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .unwrap();

    // Then split the second zone vertically
    tree.split_zone(&zone2, SplitDirection::Vertical, 0.5, false)
        .unwrap();

    let renderer = LayoutRenderer::new(&tree);
    let bounds = make_bounds();
    let layout = renderer.render(bounds);

    assert_eq!(layout.zone_rects.len(), 3, "should have three zone rects");

    // All rects should be within bounds
    for rect in layout.zone_rects.values() {
        assert!(rect.x >= 0.0, "rect.x should be >= 0");
        assert!(rect.y >= 0.0, "rect.y should be >= 0");
        assert!(
            rect.x + rect.width <= bounds.width + 0.01,
            "rect right edge ({}) should be <= bounds width ({})",
            rect.x + rect.width,
            bounds.width
        );
        assert!(
            rect.y + rect.height <= bounds.height + 0.01,
            "rect bottom edge ({}) should be <= bounds height ({})",
            rect.y + rect.height,
            bounds.height
        );
    }
}

#[test]
fn split_handles_positioned_between_zones() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    tree.split_zone(&root_id, SplitDirection::Horizontal, 0.5, false)
        .unwrap();

    let renderer = LayoutRenderer::new(&tree);
    let bounds = make_bounds();
    let layout = renderer.render(bounds);

    assert_eq!(layout.split_handles.len(), 1, "should have one split handle");

    let handle = &layout.split_handles[0];
    assert_eq!(handle.direction, SplitDirection::Horizontal);

    // Handle should be positioned near the split point (50% of 1000 = 500)
    let expected_center = bounds.width * 0.5;
    let handle_center = handle.rect.x + handle.rect.width / 2.0;
    assert!(
        (handle_center - expected_center).abs() < 1.0,
        "handle center ({}) should be near split point ({})",
        handle_center,
        expected_center
    );
}

#[test]
fn rects_within_bounds() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Build a complex tree
    let z2 = tree.split_zone(&root_id, SplitDirection::Horizontal, 0.3, false).unwrap();
    let z3 = tree.split_zone(&z2, SplitDirection::Vertical, 0.4, false).unwrap();
    tree.split_zone(&z3, SplitDirection::Horizontal, 0.6, false).unwrap();

    let renderer = LayoutRenderer::new(&tree);
    let bounds = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let layout = renderer.render(bounds);

    for (zone_id, rect) in &layout.zone_rects {
        assert!(
            rect.x >= 0.0,
            "zone {:?}: x ({}) should be >= 0",
            zone_id, rect.x
        );
        assert!(
            rect.y >= 0.0,
            "zone {:?}: y ({}) should be >= 0",
            zone_id, rect.y
        );
        assert!(
            rect.x + rect.width <= bounds.width + 0.01,
            "zone {:?}: right edge ({}) exceeds bounds width ({})",
            zone_id,
            rect.x + rect.width,
            bounds.width
        );
        assert!(
            rect.y + rect.height <= bounds.height + 0.01,
            "zone {:?}: bottom edge ({}) exceeds bounds height ({})",
            zone_id,
            rect.y + rect.height,
            bounds.height
        );
    }
}

#[test]
fn rects_do_not_overlap() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Build a moderately complex tree
    let z2 = tree.split_zone(&root_id, SplitDirection::Horizontal, 0.4, false).unwrap();
    let z3 = tree.split_zone(&z2, SplitDirection::Vertical, 0.5, false).unwrap();
    tree.split_zone(&z3, SplitDirection::Horizontal, 0.5, false).unwrap();

    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(make_bounds());

    let rects: Vec<(&NodeId, &Rect)> = layout.zone_rects.iter().collect();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            let (id_a, a) = rects[i];
            let (id_b, b) = rects[j];

            let x_overlap = a.x < b.x + b.width && b.x < a.x + a.width;
            let y_overlap = a.y < b.y + b.height && b.y < a.y + a.height;

            assert!(
                !(x_overlap && y_overlap),
                "Zones {:?} and {:?} overlap: {:?} vs {:?}",
                id_a, id_b, a, b
            );
        }
    }
}

#[test]
fn container_tabs_render_single_active() {
    // Build a tree with a container (tabs) holding two zone children
    let zone_a = LayoutNode::Zone {
        id: NodeId::from_string("zone_a"),
        content: ZoneContent::new(),
        constraints: None,
    };
    let zone_b = LayoutNode::Zone {
        id: NodeId::from_string("zone_b"),
        content: ZoneContent::new(),
        constraints: None,
    };

    let container = LayoutNode::Container {
        id: NodeId::from_string("container"),
        layout: ContainerLayout::Tabs,
        children: vec![zone_a, zone_b],
        active_child: Some(0), // Only zone_a should be rendered
    };

    let tree = LayoutTree::from_root(container);
    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(make_bounds());

    // Only the active child (zone_a) should have a rect
    assert!(
        layout.zone_rects.contains_key(&NodeId::from_string("zone_a")),
        "active child zone_a should be rendered"
    );
    assert!(
        !layout.zone_rects.contains_key(&NodeId::from_string("zone_b")),
        "inactive child zone_b should NOT be rendered"
    );
}

// =============================================================================
// Regression tests for split resize corruption fix
// =============================================================================

#[test]
fn split_handle_stores_parent_bounds() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();
    tree.split_zone(&root_id, SplitDirection::Horizontal, 0.3, false)
        .unwrap();

    let bounds = Rect::new(0.0, 0.0, 1280.0, 800.0);
    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(bounds);

    assert_eq!(layout.split_handles.len(), 1);
    let handle = &layout.split_handles[0];

    // Root-level split: parent bounds should be the full viewport
    assert!(
        (handle.parent_bounds.x - bounds.x).abs() < 0.01,
        "parent x should match bounds"
    );
    assert!(
        (handle.parent_bounds.y - bounds.y).abs() < 0.01,
        "parent y should match bounds"
    );
    assert!(
        (handle.parent_bounds.width - bounds.width).abs() < 0.01,
        "parent width should match bounds"
    );
    assert!(
        (handle.parent_bounds.height - bounds.height).abs() < 0.01,
        "parent height should match bounds"
    );
}

#[test]
fn nested_split_parent_bounds_differ_from_viewport() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Split horizontally: 30% left, 70% right
    let right_zone = tree
        .split_zone(&root_id, SplitDirection::Horizontal, 0.3, false)
        .unwrap();

    // Split the right zone vertically: 40% top, 60% bottom
    tree.split_zone(&right_zone, SplitDirection::Vertical, 0.4, false)
        .unwrap();

    let bounds = Rect::new(0.0, 0.0, 1280.0, 800.0);
    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(bounds);

    assert_eq!(layout.split_handles.len(), 2);

    // Find the nested vertical split (not the root horizontal one)
    let nested = layout
        .split_handles
        .iter()
        .find(|h| matches!(h.direction, SplitDirection::Vertical))
        .expect("should have a vertical split handle");

    // The nested split's parent bounds should be the right portion only,
    // NOT the full viewport
    assert!(
        nested.parent_bounds.x > 10.0,
        "nested parent x ({}) should be > 0 (it's the right side of a horizontal split)",
        nested.parent_bounds.x
    );
    assert!(
        nested.parent_bounds.width < bounds.width - 10.0,
        "nested parent width ({}) should be less than viewport width ({})",
        nested.parent_bounds.width,
        bounds.width
    );
}

#[test]
fn resize_all_handles_preserves_zone_count() {
    let mut tree = LayoutTree::from_preset(LayoutPreset::Seaglass).unwrap();
    let zone_count_before = tree.get_all_zone_ids().len();
    assert!(zone_count_before >= 4, "default preset should have multiple zones");

    let bounds = Rect::new(0.0, 0.0, 1280.0, 800.0);
    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(bounds);

    // Resize every split handle to 0.4
    for handle in &layout.split_handles {
        tree.update_split_ratio(&handle.id, 0.4).unwrap();
    }

    let zone_count_after = tree.get_all_zone_ids().len();
    assert_eq!(
        zone_count_before, zone_count_after,
        "resizing splits should never add or remove zones"
    );

    // Re-render and verify all zones have positive dimensions
    let layout_after = LayoutRenderer::new(&tree).render(bounds);
    for (zone_id, rect) in &layout_after.zone_rects {
        assert!(
            rect.width > 0.0,
            "zone {} has zero width after resize",
            zone_id.0
        );
        assert!(
            rect.height > 0.0,
            "zone {} has zero height after resize",
            zone_id.0
        );
    }
}

#[test]
fn render_is_idempotent_after_resize() {
    // Regression: the layout effect re-runs after mouseup and must produce
    // identical zone positions to the handler's inline call.
    let mut tree = LayoutTree::from_preset(LayoutPreset::Seaglass).unwrap();
    let bounds = Rect::new(0.0, 0.0, 1280.0, 800.0);

    // Render once to get handles
    let layout1 = LayoutRenderer::new(&tree).render(bounds);

    // Resize the first handle
    let handle = &layout1.split_handles[0];
    tree.update_split_ratio(&handle.id, 0.35).unwrap();

    // Render twice with the same tree and bounds
    let layout_a = LayoutRenderer::new(&tree).render(bounds);
    let layout_b = LayoutRenderer::new(&tree).render(bounds);

    // Zone positions must be bitwise identical
    for (zone_id, rect_a) in &layout_a.zone_rects {
        let rect_b = layout_b
            .zone_rects
            .get(zone_id)
            .unwrap_or_else(|| panic!("zone {:?} missing from second render", zone_id));
        assert_eq!(
            rect_a, rect_b,
            "zone {:?}: two renders of the same tree must produce identical rects",
            zone_id
        );
    }

    // Split handle positions must also be identical
    assert_eq!(
        layout_a.split_handles.len(),
        layout_b.split_handles.len(),
        "handle count must match"
    );
    for (ha, hb) in layout_a.split_handles.iter().zip(layout_b.split_handles.iter()) {
        assert_eq!(ha.rect, hb.rect, "handle {:?} rects must match", ha.id);
    }
}

#[test]
fn parent_relative_ratio_differs_from_viewport_ratio() {
    let mut tree = LayoutTree::new();
    let root_id = tree.get_all_zone_ids()[0].clone();

    // Vertical split: 10% top (like menu), 90% bottom
    let bottom = tree
        .split_zone(&root_id, SplitDirection::Vertical, 0.1, false)
        .unwrap();
    // Horizontal split on bottom: 20% left, 80% right
    let right = tree
        .split_zone(&bottom, SplitDirection::Horizontal, 0.2, false)
        .unwrap();
    // Vertical split on right: 50% top, 50% bottom
    tree.split_zone(&right, SplitDirection::Vertical, 0.5, false)
        .unwrap();

    let bounds = Rect::new(0.0, 0.0, 1280.0, 800.0);
    let renderer = LayoutRenderer::new(&tree);
    let layout = renderer.render(bounds);

    // Find the deepest vertical split (within the right panel)
    let deep_v_handle = layout
        .split_handles
        .iter()
        .filter(|h| matches!(h.direction, SplitDirection::Vertical))
        .max_by(|a, b| a.parent_bounds.y.partial_cmp(&b.parent_bounds.y).unwrap())
        .expect("should find the deepest vertical handle");

    // Simulate a mouse position at the handle
    let mouse_y = deep_v_handle.rect.y + 5.0;

    // Correct ratio: relative to parent bounds
    let correct_ratio = ((mouse_y - deep_v_handle.parent_bounds.y)
        / deep_v_handle.parent_bounds.height)
        .clamp(0.1, 0.9);

    // Wrong ratio: relative to viewport (the old bug)
    let wrong_ratio = (mouse_y / bounds.height).clamp(0.1, 0.9);

    // For nested splits, these must differ
    assert!(
        (correct_ratio - wrong_ratio).abs() > 0.001,
        "nested split: correct ratio ({correct_ratio:.4}) should differ from \
         viewport ratio ({wrong_ratio:.4})"
    );
}
