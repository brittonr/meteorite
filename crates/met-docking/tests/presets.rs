use met_docking::flexible_layout::{LayoutPreset, LayoutRenderer, Rect};

fn validate_preset_renders_cleanly(preset: LayoutPreset) {
    let tree = preset.build().expect("preset should build successfully");
    let renderer = LayoutRenderer::new(&tree);
    let bounds = Rect::new(0.0, 0.0, 1920.0, 1080.0);
    let layout = renderer.render(bounds);

    // Should produce at least one zone
    assert!(
        !layout.zone_rects.is_empty(),
        "preset should produce at least one zone rect"
    );

    // No constraint violations
    assert!(
        layout.violations.is_empty(),
        "preset should render without violations: {:?}",
        layout.violations
    );

    // All rects within bounds
    for (zone_id, rect) in &layout.zone_rects {
        assert!(
            rect.x >= 0.0 && rect.y >= 0.0,
            "zone {:?}: position ({}, {}) should be non-negative",
            zone_id, rect.x, rect.y
        );
        assert!(
            rect.x + rect.width <= bounds.width + 0.01,
            "zone {:?}: right edge exceeds bounds",
            zone_id
        );
        assert!(
            rect.y + rect.height <= bounds.height + 0.01,
            "zone {:?}: bottom edge exceeds bounds",
            zone_id
        );
    }
}

#[test]
fn preset_default_builds_valid_tree() {
    validate_preset_renders_cleanly(LayoutPreset::Default);
}

#[test]
fn preset_focus_builds_valid_tree() {
    validate_preset_renders_cleanly(LayoutPreset::Focus);
}

#[test]
fn preset_split_view_builds_valid_tree() {
    validate_preset_renders_cleanly(LayoutPreset::SplitView);
}

#[test]
fn preset_debug_builds_valid_tree() {
    validate_preset_renders_cleanly(LayoutPreset::Debug);
}

#[test]
fn preset_serialize_deserialize_roundtrip() {
    let tree = LayoutPreset::Default
        .build()
        .expect("default preset should build");

    let json = serde_json::to_string(&tree).expect("tree should serialize");
    let deserialized: met_docking::flexible_layout::LayoutTree =
        serde_json::from_str(&json).expect("tree should deserialize");

    // Re-render both and compare zone counts (PartialEq on LayoutTree
    // skips serde(skip) cache fields, so compare zone IDs instead)
    let original_zones = tree.get_all_zone_ids();
    let roundtrip_zones = deserialized.get_all_zone_ids();
    assert_eq!(
        original_zones.len(),
        roundtrip_zones.len(),
        "zone count should be preserved across serialization"
    );

    // Rendering should produce equivalent layouts
    let bounds = Rect::new(0.0, 0.0, 1920.0, 1080.0);

    let layout1 = LayoutRenderer::new(&tree).render(bounds);
    let layout2 = LayoutRenderer::new(&deserialized).render(bounds);

    assert_eq!(
        layout1.zone_rects.len(),
        layout2.zone_rects.len(),
        "rendered zone count should match after round-trip"
    );
}

#[test]
fn preset_by_name_returns_correct_variant() {
    assert!(matches!(
        LayoutPreset::by_name("default"),
        Some(LayoutPreset::Default)
    ));
    assert!(matches!(
        LayoutPreset::by_name("focus"),
        Some(LayoutPreset::Focus)
    ));
    assert!(matches!(
        LayoutPreset::by_name("split"),
        Some(LayoutPreset::SplitView)
    ));
    assert!(matches!(
        LayoutPreset::by_name("debug"),
        Some(LayoutPreset::Debug)
    ));
    // Unknown names return Custom variant
    assert!(matches!(
        LayoutPreset::by_name("something_else"),
        Some(LayoutPreset::Custom(_))
    ));
}
