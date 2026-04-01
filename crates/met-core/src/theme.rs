use std::collections::BTreeMap;

use dioxus::prelude::*;

/// Bundled component stylesheet.
const METEORITE_CSS: &str = include_str!("../assets/meteorite.css");

// ═══════════════════════════════════════════════════════════════════
// Palette
// ═══════════════════════════════════════════════════════════════════

/// Color palette for a theme.
///
/// The built-in fields cover the standard UI variants. Add arbitrary
/// colors via [`extra`](Palette::extra) — each entry becomes a
/// `--met-{key}` CSS variable and can be referenced with
/// [`Variant::Custom`](crate::Variant::Custom).
///
/// ```rust,ignore
/// Palette {
///     extra: BTreeMap::from([
///         ("brand".into(), "#ff6600".into()),
///         ("accent".into(), "#9333ea".into()),
///     ]),
///     ..Palette::dark()
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    pub bg: String,
    pub fg: String,
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub muted: String,
    pub border: String,
    /// Arbitrary extra colors. Each key `k` emits `--met-{k}` and
    /// generates a `.met-{k}` variant class in the CSS output.
    pub extra: BTreeMap<String, String>,
}

impl Palette {
    /// Dark palette defaults (same as `Theme::dark()`).
    pub fn dark() -> Self {
        Palette {
            bg: "#0a0a0a".into(),
            fg: "#fafafa".into(),
            primary: "#3b82f6".into(),
            secondary: "#6366f1".into(),
            success: "#22c55e".into(),
            warning: "#eab308".into(),
            danger: "#ef4444".into(),
            muted: "#71717a".into(),
            border: "#27272a".into(),
            extra: BTreeMap::new(),
        }
    }

    /// Light palette defaults (same as `Theme::light()`).
    pub fn light() -> Self {
        Palette {
            bg: "#ffffff".into(),
            fg: "#09090b".into(),
            primary: "#2563eb".into(),
            secondary: "#4f46e5".into(),
            success: "#16a34a".into(),
            warning: "#ca8a04".into(),
            danger: "#dc2626".into(),
            muted: "#a1a1aa".into(),
            border: "#e4e4e7".into(),
            extra: BTreeMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Tokens
// ═══════════════════════════════════════════════════════════════════

/// Design tokens for spacing, radius, and typography.
///
/// Add arbitrary tokens via [`extra`](Tokens::extra) — each entry
/// becomes a `--met-{key}` CSS variable.
///
/// ```rust,ignore
/// Tokens {
///     extra: BTreeMap::from([
///         ("shadow-sm".into(), "0 1px 2px rgba(0,0,0,0.05)".into()),
///         ("transition".into(), "0.15s ease".into()),
///     ]),
///     ..Tokens::default()
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Tokens {
    pub radius_sm: String,
    pub radius_md: String,
    pub radius_lg: String,
    pub font_sm: String,
    pub font_md: String,
    pub font_lg: String,
    pub space_xs: String,
    pub space_sm: String,
    pub space_md: String,
    pub space_lg: String,
    pub space_xl: String,
    /// Arbitrary extra tokens. Each key `k` emits `--met-{k}`.
    pub extra: BTreeMap<String, String>,
}

impl Default for Tokens {
    fn default() -> Self {
        Tokens {
            radius_sm: "4px".into(),
            radius_md: "6px".into(),
            radius_lg: "12px".into(),
            font_sm: "0.875rem".into(),
            font_md: "1rem".into(),
            font_lg: "1.25rem".into(),
            space_xs: "4px".into(),
            space_sm: "8px".into(),
            space_md: "16px".into(),
            space_lg: "24px".into(),
            space_xl: "32px".into(),
            extra: BTreeMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Theme
// ═══════════════════════════════════════════════════════════════════

/// Theme configuration.
///
/// # Quick start
/// ```rust,ignore
/// // Use a preset
/// ThemeProvider { theme: Theme::dark(), App {} }
///
/// // Swap at runtime
/// let mut theme = use_signal(|| Theme::dark());
/// rsx! {
///     ThemeProvider { theme: theme(),
///         button { onclick: move |_| theme.set(Theme::light()), "Toggle" }
///     }
/// }
///
/// // Custom palette + extra tokens
/// ThemeProvider {
///     theme: Theme::builder("corporate")
///         .palette(Palette { primary: "#003366".into(), ..Palette::light() })
///         .extra_color("brand", "#ff6600")
///         .extra_token("shadow-lg", "0 8px 24px rgba(0,0,0,0.15)")
///         .build(),
///     App {}
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,
    pub palette: Palette,
    pub tokens: Tokens,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            name: "dark".into(),
            palette: Palette::dark(),
            tokens: Tokens::default(),
        }
    }

    pub fn light() -> Self {
        Theme {
            name: "light".into(),
            palette: Palette::light(),
            tokens: Tokens::default(),
        }
    }

    /// Build a custom theme from a palette, using default tokens.
    pub fn custom(name: impl Into<String>, palette: Palette) -> Self {
        Theme {
            name: name.into(),
            palette,
            tokens: Tokens::default(),
        }
    }

    /// Start a builder for a custom theme.
    pub fn builder(name: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder {
            name: name.into(),
            palette: Palette::dark(),
            tokens: Tokens::default(),
        }
    }

    /// Emit CSS custom properties for this theme.
    ///
    /// Includes built-in variables, extra palette colors (with
    /// matching variant classes), and extra tokens.
    pub fn to_css_vars(&self) -> String {
        let p = &self.palette;
        let t = &self.tokens;

        let mut css = format!(
            r#":root {{
  --met-bg: {bg};
  --met-fg: {fg};
  --met-primary: {primary};
  --met-secondary: {secondary};
  --met-success: {success};
  --met-warning: {warning};
  --met-danger: {danger};
  --met-muted: {muted};
  --met-border: {border};
  --met-radius-sm: {r_sm};
  --met-radius-md: {r_md};
  --met-radius-lg: {r_lg};
  --met-font-sm: {f_sm};
  --met-font-md: {f_md};
  --met-font-lg: {f_lg};
  --met-space-xs: {s_xs};
  --met-space-sm: {s_sm};
  --met-space-md: {s_md};
  --met-space-lg: {s_lg};
  --met-space-xl: {s_xl};"#,
            bg = p.bg,
            fg = p.fg,
            primary = p.primary,
            secondary = p.secondary,
            success = p.success,
            warning = p.warning,
            danger = p.danger,
            muted = p.muted,
            border = p.border,
            r_sm = t.radius_sm,
            r_md = t.radius_md,
            r_lg = t.radius_lg,
            f_sm = t.font_sm,
            f_md = t.font_md,
            f_lg = t.font_lg,
            s_xs = t.space_xs,
            s_sm = t.space_sm,
            s_md = t.space_md,
            s_lg = t.space_lg,
            s_xl = t.space_xl,
        );

        // Extra palette colors → CSS variables
        for (key, val) in &p.extra {
            css.push_str(&format!("\n  --met-{key}: {val};"));
        }

        // Extra tokens → CSS variables
        for (key, val) in &t.extra {
            css.push_str(&format!("\n  --met-{key}: {val};"));
        }

        css.push_str("\n}\n");

        // Extra palette colors → variant classes
        for (key, _) in &p.extra {
            css.push_str(&format!(
                ".met-{key} {{ --_c: var(--met-{key}); --_bg: color-mix(in srgb, var(--met-{key}) 15%, transparent); }}\n"
            ));
        }

        css
    }

    /// Resolve a Variant to its palette color string.
    pub fn variant_color(&self, variant: &crate::Variant) -> &str {
        match variant {
            crate::Variant::Default => &self.palette.fg,
            crate::Variant::Primary => &self.palette.primary,
            crate::Variant::Secondary => &self.palette.secondary,
            crate::Variant::Success => &self.palette.success,
            crate::Variant::Warning => &self.palette.warning,
            crate::Variant::Danger => &self.palette.danger,
            crate::Variant::Ghost => "transparent",
            crate::Variant::Custom(name) => self
                .palette
                .extra
                .get(name.as_str())
                .map(|s| s.as_str())
                .unwrap_or(&self.palette.fg),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ThemeBuilder
// ═══════════════════════════════════════════════════════════════════

/// Fluent builder for constructing a [`Theme`].
///
/// ```rust,ignore
/// let theme = Theme::builder("ocean")
///     .palette(Palette { primary: "#0ea5e9".into(), ..Palette::dark() })
///     .extra_color("brand", "#ff6600")
///     .extra_color("accent", "#9333ea")
///     .extra_token("shadow-lg", "0 8px 24px rgba(0,0,0,0.15)")
///     .tokens(Tokens { radius_md: "8px".into(), ..Tokens::default() })
///     .build();
/// ```
pub struct ThemeBuilder {
    name: String,
    palette: Palette,
    tokens: Tokens,
}

impl ThemeBuilder {
    /// Set the full palette (you can spread defaults with `..Palette::dark()`).
    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
    }

    /// Set the full token set (spread defaults with `..Tokens::default()`).
    pub fn tokens(mut self, tokens: Tokens) -> Self {
        self.tokens = tokens;
        self
    }

    /// Add a custom color. Generates `--met-{name}` and `.met-{name}` class.
    pub fn extra_color(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.palette.extra.insert(name.into(), value.into());
        self
    }

    /// Add a custom token. Generates `--met-{name}`.
    pub fn extra_token(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.tokens.extra.insert(name.into(), value.into());
        self
    }

    pub fn build(self) -> Theme {
        Theme {
            name: self.name,
            palette: self.palette,
            tokens: self.tokens,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Context helpers
// ═══════════════════════════════════════════════════════════════════

/// Read the current theme from context (clones on each call).
/// Panics if no `ThemeProvider` is above this component in the tree.
pub fn use_theme() -> Theme {
    use_context::<Signal<Theme>>().read().clone()
}

/// Read the theme signal directly (avoids cloning on every render).
pub fn use_theme_signal() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}

// ═══════════════════════════════════════════════════════════════════
// ThemeProvider
// ═══════════════════════════════════════════════════════════════════

#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    #[props(default)]
    pub theme: Theme,
    pub children: Element,
}

/// Provides a [`Theme`] to the component subtree via context.
///
/// Wrap your app root (or any subtree) to set the active theme.
/// Changing the `theme` prop at runtime re-renders all subscribers.
/// Nested providers override the parent.
///
/// ```rust,ignore
/// // Static
/// rsx! {
///     ThemeProvider { theme: Theme::light(), MyApp {} }
/// }
///
/// // Runtime switching
/// let mut theme = use_signal(|| Theme::dark());
/// rsx! {
///     ThemeProvider { theme: theme(),
///         button { onclick: move |_| theme.set(Theme::light()), "Go light" }
///         MyApp {}
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    // Provide the signal once; sync it with the prop on every render.
    let mut theme_signal = use_context_provider(|| Signal::new(props.theme.clone()));

    // Keep the signal in sync when the prop changes (runtime switching).
    use_effect(move || {
        theme_signal.set(props.theme.clone());
    });

    let vars_css = use_memo(move || theme_signal.read().to_css_vars());

    rsx! {
        style { "{vars_css}" }
        style { "{METEORITE_CSS}" }
        {props.children}
    }
}
