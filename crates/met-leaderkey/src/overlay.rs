//! Dioxus rendering for the leader key popup.

use dioxus::prelude::*;
use ratcore::leaderkey::{LeaderAction, LeaderMenu, MenuInput};

/// Convert a Dioxus `Key` into a `MenuInput`.
fn key_to_input(key: &Key) -> MenuInput {
    match key {
        Key::Escape => MenuInput::Escape,
        Key::Character(s) => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(ch), None) => MenuInput::Char(ch),
                _ => MenuInput::Other,
            }
        }
        _ => MenuInput::Other,
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LeaderKeyOverlayProps<A: Clone + PartialEq + 'static> {
    /// The leader menu state (caller owns the Signal).
    pub menu: Signal<LeaderMenu<A>>,
    /// Called when a non-submenu action is triggered.
    #[props(default)]
    pub on_action: EventHandler<LeaderAction<A>>,
    /// Called when a Command action is triggered (convenience shorthand).
    #[props(default)]
    pub on_command: EventHandler<String>,
    #[props(default)]
    pub class: String,
}

/// Leader key popup overlay.
///
/// Renders a centered popup showing the current menu level's items.
/// Captures keyboard events when visible. Calls `on_action` or
/// `on_command` when an item is selected.
#[component]
pub fn LeaderKeyOverlay<A: Clone + PartialEq + 'static>(
    props: LeaderKeyOverlayProps<A>,
) -> Element {
    let mut menu = props.menu;

    if !menu.read().visible {
        return rsx! {};
    }

    // Snapshot current level for rendering (avoids borrow across rsx).
    let current_items: Vec<(char, String, bool)> = menu
        .read()
        .current()
        .map(|def| {
            def.items
                .iter()
                .map(|item| {
                    let is_submenu =
                        matches!(item.action, ratcore::leaderkey::LeaderAction::Submenu(_));
                    (item.key, item.label.clone(), is_submenu)
                })
                .collect()
        })
        .unwrap_or_default();

    let breadcrumb: Vec<String> = menu.read().breadcrumb().to_vec();
    let title = if breadcrumb.is_empty() {
        "Space".to_string()
    } else {
        format!("Space › {}", breadcrumb.join(" › "))
    };

    let depth = menu.read().depth();
    let hint = if depth > 0 { "esc back" } else { "esc close" };

    rsx! {
        div {
            class: "met-leader-backdrop {props.class}",
            tabindex: "0",
            autofocus: true,

            // Capture all keys while menu is open
            onkeydown: move |evt: KeyboardEvent| {
                evt.prevent_default();
                evt.stop_propagation();
                let input = key_to_input(&evt.key());
                let result = menu.write().handle_input(input);

                if let Some(action) = result {
                    match &action {
                        LeaderAction::Command(cmd) => {
                            props.on_command.call(cmd.clone());
                        }
                        _ => {}
                    }
                    props.on_action.call(action);
                }
            },

            // Click backdrop to close
            onclick: move |_| {
                menu.write().close();
            },

            // Popup panel
            div {
                class: "met-leader-panel",
                onclick: move |evt: MouseEvent| evt.stop_propagation(),

                // Title bar
                div {
                    class: "met-leader-title",
                    "{title}"
                }

                // Items
                div {
                    class: "met-leader-items",
                    for (key, label, is_submenu) in current_items {
                        div {
                            class: "met-leader-item",
                            onclick: {
                                move |_| {
                                    let result = menu.write().handle_char(key);
                                    if let Some(action) = result {
                                        match &action {
                                            LeaderAction::Command(cmd) => {
                                                props.on_command.call(cmd.clone());
                                            }
                                            _ => {}
                                        }
                                        props.on_action.call(action);
                                    }
                                }
                            },
                            span { class: "met-leader-key", "{key}" }
                            span { class: "met-leader-label",
                                "{label}"
                                if is_submenu { "…" }
                            }
                        }
                    }
                }

                // Hint
                div {
                    class: "met-leader-hint",
                    "{hint}"
                }
            }
        }
    }
}
