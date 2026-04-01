//! Keyboard shortcuts overlay.
//!
//! Data-driven modal that displays keyboard shortcuts organized by section.
//! Supply custom sections or use the default set.

use dioxus::prelude::*;

/// A single keyboard shortcut.
#[derive(Clone, Debug, PartialEq)]
pub struct Shortcut {
    /// Key combination (e.g. "Ctrl+C", "Arrow Keys")
    pub key: String,
    /// What the shortcut does
    pub description: String,
}

impl Shortcut {
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

/// A group of related shortcuts.
#[derive(Clone, Debug, PartialEq)]
pub struct ShortcutSection {
    pub title: String,
    pub shortcuts: Vec<Shortcut>,
}

impl ShortcutSection {
    pub fn new(title: impl Into<String>, shortcuts: Vec<Shortcut>) -> Self {
        Self {
            title: title.into(),
            shortcuts,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ShortcutsOverlayProps {
    /// Whether the overlay is visible
    pub visible: Signal<bool>,
    /// Called when the overlay should close
    #[props(default)]
    pub on_close: EventHandler<()>,
    /// Custom sections (defaults to a generic set if omitted)
    #[props(default)]
    pub sections: Option<Vec<ShortcutSection>>,
    /// Overlay title
    #[props(default = "Keyboard Shortcuts".to_string())]
    pub title: String,
    #[props(default)]
    pub class: String,
}

/// Modal overlay displaying keyboard shortcuts.
///
/// ```rust,ignore
/// let show = use_signal(|| false);
/// rsx! {
///     button { onclick: move |_| show.set(true), "Show Shortcuts" }
///     ShortcutsOverlay {
///         visible: show,
///         on_close: move |_| show.set(false),
///     }
/// }
/// ```
#[component]
pub fn ShortcutsOverlay(props: ShortcutsOverlayProps) -> Element {
    if !(props.visible)() {
        return rsx! {};
    }

    let sections = props.sections.clone().unwrap_or_else(default_sections);

    rsx! {
        div {
            class: "met-shortcuts-backdrop {props.class}",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "met-shortcuts-panel",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "met-shortcuts-header",
                    h4 { "{props.title}" }
                    button {
                        class: "met-shortcuts-close",
                        onclick: move |_| props.on_close.call(()),
                        "×"
                    }
                }

                // Sections
                div { class: "met-shortcuts-body",
                    for section in sections {
                        div { class: "met-shortcuts-section",
                            h5 { {section.title} }
                            for shortcut in section.shortcuts {
                                div { class: "met-shortcuts-row",
                                    span { class: "met-shortcuts-key", {shortcut.key} }
                                    span { class: "met-shortcuts-desc", {shortcut.description} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Default shortcut sections (generic editing shortcuts).
pub fn default_sections() -> Vec<ShortcutSection> {
    vec![
        ShortcutSection::new(
            "Navigation",
            vec![
                Shortcut::new("Arrow Keys", "Move selection"),
                Shortcut::new("Home / End", "Start / end of row"),
                Shortcut::new("Page Up / Down", "Jump 10 rows"),
                Shortcut::new("Ctrl+Home / End", "First / last cell"),
            ],
        ),
        ShortcutSection::new(
            "Selection",
            vec![
                Shortcut::new("Enter", "Select"),
                Shortcut::new("Escape", "Cancel / clear"),
                Shortcut::new("Ctrl+A", "Select all"),
            ],
        ),
        ShortcutSection::new(
            "Clipboard",
            vec![
                Shortcut::new("Ctrl+C", "Copy"),
                Shortcut::new("Ctrl+V", "Paste"),
                Shortcut::new("Ctrl+X", "Cut"),
            ],
        ),
        ShortcutSection::new(
            "Search",
            vec![
                Shortcut::new("Ctrl+F", "Open search"),
                Shortcut::new("F3", "Next result"),
                Shortcut::new("Shift+F3", "Previous result"),
            ],
        ),
    ]
}
