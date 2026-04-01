use dioxus::prelude::*;

/// Color palette for a theme.
#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    pub bg: &'static str,
    pub fg: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub danger: &'static str,
    pub muted: &'static str,
    pub border: &'static str,
}

/// Theme configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            name: "dark",
            palette: Palette {
                bg: "#0a0a0a",
                fg: "#fafafa",
                primary: "#3b82f6",
                secondary: "#6366f1",
                success: "#22c55e",
                warning: "#eab308",
                danger: "#ef4444",
                muted: "#71717a",
                border: "#27272a",
            },
        }
    }

    pub fn light() -> Self {
        Theme {
            name: "light",
            palette: Palette {
                bg: "#ffffff",
                fg: "#09090b",
                primary: "#2563eb",
                secondary: "#4f46e5",
                success: "#16a34a",
                warning: "#ca8a04",
                danger: "#dc2626",
                muted: "#a1a1aa",
                border: "#e4e4e7",
            },
        }
    }
}

/// Provide a theme to the component tree via context.
pub fn use_theme() -> Theme {
    use_context::<Signal<Theme>>().read().clone()
}
