use met_core::{Palette, Theme, Tokens, Variant};

#[test]
fn dark_and_light_presets_differ() {
    let dark = Theme::dark();
    let light = Theme::light();
    assert_ne!(dark.palette.bg, light.palette.bg);
    assert_ne!(dark.palette.fg, light.palette.fg);
    assert_eq!(dark.name, "dark");
    assert_eq!(light.name, "light");
}

#[test]
fn to_css_vars_contains_all_builtin_vars() {
    let css = Theme::dark().to_css_vars();
    let expected = [
        "--met-bg", "--met-fg", "--met-primary", "--met-secondary",
        "--met-success", "--met-warning", "--met-danger", "--met-muted",
        "--met-border", "--met-radius-sm", "--met-radius-md", "--met-radius-lg",
        "--met-font-sm", "--met-font-md", "--met-font-lg",
        "--met-space-xs", "--met-space-sm", "--met-space-md",
        "--met-space-lg", "--met-space-xl",
    ];
    for var in expected {
        assert!(css.contains(var), "missing CSS var: {var}");
    }
}

#[test]
fn extra_palette_colors_emit_var_and_class() {
    let theme = Theme::builder("test")
        .extra_color("brand", "#ff6600")
        .extra_color("accent", "#9333ea")
        .build();
    let css = theme.to_css_vars();

    // Variables
    assert!(css.contains("--met-brand: #ff6600"), "missing --met-brand var");
    assert!(css.contains("--met-accent: #9333ea"), "missing --met-accent var");

    // Variant classes
    assert!(css.contains(".met-brand {"), "missing .met-brand class");
    assert!(css.contains(".met-accent {"), "missing .met-accent class");
    assert!(css.contains("var(--met-brand)"), "class should reference var");
}

#[test]
fn extra_tokens_emit_vars() {
    let theme = Theme::builder("test")
        .extra_token("shadow-lg", "0 8px 24px rgba(0,0,0,0.15)")
        .extra_token("transition", "0.2s ease")
        .build();
    let css = theme.to_css_vars();

    assert!(css.contains("--met-shadow-lg: 0 8px 24px rgba(0,0,0,0.15)"));
    assert!(css.contains("--met-transition: 0.2s ease"));
}

#[test]
fn builder_overrides_palette_and_tokens() {
    let theme = Theme::builder("custom")
        .palette(Palette {
            primary: "#000000".into(),
            ..Palette::light()
        })
        .tokens(Tokens {
            radius_md: "99px".into(),
            ..Tokens::default()
        })
        .build();

    assert_eq!(theme.name, "custom");
    assert_eq!(theme.palette.primary, "#000000");
    assert_eq!(theme.palette.bg, Palette::light().bg); // inherited
    assert_eq!(theme.tokens.radius_md, "99px");
    assert_eq!(theme.tokens.space_sm, Tokens::default().space_sm); // inherited
}

#[test]
fn variant_class_names() {
    assert_eq!(Variant::Default.class(), "met-default");
    assert_eq!(Variant::Primary.class(), "met-primary");
    assert_eq!(Variant::Danger.class(), "met-danger");
    assert_eq!(Variant::Ghost.class(), "met-ghost");
    assert_eq!(Variant::Custom("brand".into()).class(), "met-brand");
    assert_eq!(Variant::Custom("x-y-z".into()).class(), "met-x-y-z");
}

#[test]
fn variant_color_resolves_builtin() {
    let theme = Theme::dark();
    assert_eq!(theme.variant_color(&Variant::Primary), &theme.palette.primary);
    assert_eq!(theme.variant_color(&Variant::Success), &theme.palette.success);
    assert_eq!(theme.variant_color(&Variant::Ghost), "transparent");
}

#[test]
fn variant_color_resolves_custom() {
    let theme = Theme::builder("test")
        .extra_color("brand", "#ff6600")
        .build();

    assert_eq!(theme.variant_color(&Variant::Custom("brand".into())), "#ff6600");
    // Unknown custom falls back to fg
    assert_eq!(theme.variant_color(&Variant::Custom("nope".into())), &theme.palette.fg);
}

#[test]
fn css_output_is_deterministic() {
    let theme = Theme::builder("test")
        .extra_color("z-last", "#111")
        .extra_color("a-first", "#222")
        .extra_token("z-tok", "1")
        .extra_token("a-tok", "2")
        .build();

    let css1 = theme.to_css_vars();
    let css2 = theme.to_css_vars();
    assert_eq!(css1, css2);

    // BTreeMap ensures alphabetical order
    let a_pos = css1.find("--met-a-first").unwrap();
    let z_pos = css1.find("--met-z-last").unwrap();
    assert!(a_pos < z_pos, "extra colors should be in alphabetical order");
}

#[test]
fn default_theme_is_dark() {
    let t: Theme = Default::default();
    assert_eq!(t.name, "dark");
}
