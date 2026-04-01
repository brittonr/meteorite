use dioxus::prelude::*;

/// Bundled component stylesheet.
const METEORITE_CSS: &str = include_str!("../assets/meteorite.css");

/// Color palette for a theme.
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
}

/// Theme configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,
    pub palette: Palette,
    /// Per-component radius, spacing, font-size overrides.
    pub tokens: Tokens,
}

/// Design tokens for spacing, radius, and typography.
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
        }
    }
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
            palette: Palette {
                bg: "#0a0a0a".into(),
                fg: "#fafafa".into(),
                primary: "#3b82f6".into(),
                secondary: "#6366f1".into(),
                success: "#22c55e".into(),
                warning: "#eab308".into(),
                danger: "#ef4444".into(),
                muted: "#71717a".into(),
                border: "#27272a".into(),
            },
            tokens: Tokens::default(),
        }
    }

    pub fn light() -> Self {
        Theme {
            name: "light".into(),
            palette: Palette {
                bg: "#ffffff".into(),
                fg: "#09090b".into(),
                primary: "#2563eb".into(),
                secondary: "#4f46e5".into(),
                success: "#16a34a".into(),
                warning: "#ca8a04".into(),
                danger: "#dc2626".into(),
                muted: "#a1a1aa".into(),
                border: "#e4e4e7".into(),
            },
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

    /// Emit CSS custom properties for this theme.
    /// Inject the returned string into a `<style>` tag or the `:root` selector.
    pub fn to_css_vars(&self) -> String {
        let p = &self.palette;
        let t = &self.tokens;
        format!(
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
  --met-space-xl: {s_xl};
}}"#,
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
        )
    }

    /// Resolve a Variant to its palette color.
    pub fn variant_color(&self, variant: &crate::Variant) -> &str {
        match variant {
            crate::Variant::Default => &self.palette.fg,
            crate::Variant::Primary => &self.palette.primary,
            crate::Variant::Secondary => &self.palette.secondary,
            crate::Variant::Success => &self.palette.success,
            crate::Variant::Warning => &self.palette.warning,
            crate::Variant::Danger => &self.palette.danger,
            crate::Variant::Ghost => "transparent",
        }
    }
}

// ---------------------------------------------------------------------------
// Context helpers
// ---------------------------------------------------------------------------

/// Read the current theme from context.
/// Panics if no `ThemeProvider` is above this component in the tree.
pub fn use_theme() -> Theme {
    use_context::<Signal<Theme>>().read().clone()
}

/// Read the theme signal directly (avoids cloning on every render).
pub fn use_theme_signal() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}

// ---------------------------------------------------------------------------
// ThemeProvider component
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    #[props(default)]
    pub theme: Theme,
    pub children: Element,
}

/// Provides a `Theme` to the component subtree via context.
///
/// Wrap your app root (or any subtree) to set the active theme.
/// Nested providers override the parent -- useful for sections
/// that need a different palette.
///
/// ```rust,ignore
/// rsx! {
///     ThemeProvider { theme: Theme::light(),
///         // everything in here sees the light theme
///         MyApp {}
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme_signal = use_signal(|| props.theme.clone());
    use_context_provider(|| theme_signal);

    let vars_css = props.theme.to_css_vars();

    rsx! {
        style { "{vars_css}" }
        style { "{METEORITE_CSS}" }
        {props.children}
    }
}
