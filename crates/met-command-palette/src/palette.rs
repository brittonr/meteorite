//! Command palette Dioxus component.

use dioxus::prelude::*;

use crate::fuzzy::fuzzy_score;

/// A single command palette entry.
#[derive(Debug, Clone, PartialEq)]
pub struct PaletteItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub icon: Option<String>,
    pub group: Option<String>,
}

impl PaletteItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            icon: None,
            group: None,
        }
    }

    pub fn shortcut(mut self, s: impl Into<String>) -> Self {
        self.shortcut = Some(s.into());
        self
    }

    pub fn icon(mut self, i: impl Into<String>) -> Self {
        self.icon = Some(i.into());
        self
    }

    pub fn group(mut self, g: impl Into<String>) -> Self {
        self.group = Some(g.into());
        self
    }
}

/// Scored item with match positions for highlighting.
struct Scored {
    item_idx: usize,
    score: i32,
    positions: Vec<usize>,
}

fn score_items(items: &[PaletteItem], query: &str) -> Vec<Scored> {
    if query.is_empty() {
        return items
            .iter()
            .enumerate()
            .map(|(i, _)| Scored {
                item_idx: i,
                score: 0,
                positions: vec![],
            })
            .collect();
    }

    let mut scored: Vec<Scored> = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            fuzzy_score(&item.label, query).map(|m| Scored {
                item_idx: i,
                score: m.score,
                positions: m.positions,
            })
        })
        .collect();

    scored.sort_by(|a, b| b.score.cmp(&a.score));
    scored
}

#[derive(Props, Clone, PartialEq)]
pub struct CommandPaletteProps {
    /// Whether the palette is visible.
    pub open: bool,
    /// Available commands to search through.
    pub items: Vec<PaletteItem>,
    /// Called when an item is selected.
    pub on_select: EventHandler<String>,
    /// Called when the palette should close (Escape, backdrop click).
    pub on_close: EventHandler<()>,
    /// Placeholder text for the search input.
    #[props(default = "Type a command…".into())]
    pub placeholder: String,
    /// Extra CSS class.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CommandPalette(props: CommandPaletteProps) -> Element {
    let mut query = use_signal(String::new);
    let mut selected_idx = use_signal(|| 0usize);

    // Reset state when opened.
    use_effect({
        let open = props.open;
        move || {
            if open {
                query.set(String::new());
                selected_idx.set(0);
            }
        }
    });

    if !props.open {
        return rsx! {};
    }

    let results = score_items(&props.items, &query.read());
    let result_count = results.len();

    let class = format!("met-command-palette {}", props.class);

    rsx! {
        div {
            class: "met-command-palette-backdrop",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "{class}",
                onclick: move |evt: MouseEvent| evt.stop_propagation(),

                // Search input
                input {
                    class: "met-command-palette-input",
                    r#type: "text",
                    placeholder: "{props.placeholder}",
                    autofocus: true,
                    value: "{query}",
                    oninput: move |evt: FormEvent| {
                        query.set(evt.value().clone());
                        selected_idx.set(0);
                    },
                    onkeydown: {
                        let items = props.items.clone();
                        let on_select = props.on_select.clone();
                        let on_close = props.on_close.clone();
                        move |evt: KeyboardEvent| {
                            match evt.key() {
                                Key::Escape => {
                                    on_close.call(());
                                    evt.prevent_default();
                                }
                                Key::ArrowDown => {
                                    let cur = *selected_idx.read();
                                    if result_count > 0 {
                                        selected_idx.set((cur + 1) % result_count);
                                    }
                                    evt.prevent_default();
                                }
                                Key::ArrowUp => {
                                    let cur = *selected_idx.read();
                                    if result_count > 0 {
                                        selected_idx.set(if cur == 0 { result_count - 1 } else { cur - 1 });
                                    }
                                    evt.prevent_default();
                                }
                                Key::Enter => {
                                    let cur = *selected_idx.read();
                                    let scored = score_items(&items, &query.read());
                                    if let Some(s) = scored.get(cur) {
                                        on_select.call(items[s.item_idx].id.clone());
                                    }
                                    evt.prevent_default();
                                }
                                _ => {}
                            }
                        }
                    },
                }

                // Results list
                ul {
                    class: "met-command-palette-list",
                    role: "listbox",
                    for (ri, scored) in results.iter().enumerate() {
                        {
                            let item = &props.items[scored.item_idx];
                            let item_id = item.id.clone();
                            let is_selected = ri == *selected_idx.read();
                            let row_class = if is_selected { "met-command-palette-item met-command-palette-item-active" } else { "met-command-palette-item" };

                            // Build highlighted label spans.
                            let label_chars: Vec<char> = item.label.chars().collect();
                            let match_set: std::collections::BTreeSet<usize> =
                                scored.positions.iter().copied().collect();

                            let on_select = props.on_select.clone();
                            let icon = item.icon.clone();
                            let shortcut = item.shortcut.clone();

                            rsx! {
                                li {
                                    class: "{row_class}",
                                    role: "option",
                                    "aria-selected": if is_selected { "true" } else { "false" },
                                    onclick: move |_| on_select.call(item_id.clone()),
                                    onmouseenter: move |_| selected_idx.set(ri),

                                    if let Some(ref ic) = icon {
                                        span { class: "met-command-palette-icon", "{ic}" }
                                    }

                                    span { class: "met-command-palette-label",
                                        for (ci, ch) in label_chars.iter().enumerate() {
                                            if match_set.contains(&ci) {
                                                mark { "{ch}" }
                                            } else {
                                                span { "{ch}" }
                                            }
                                        }
                                    }

                                    if let Some(ref sc) = shortcut {
                                        span { class: "met-command-palette-shortcut", "{sc}" }
                                    }
                                }
                            }
                        }
                    }

                    if results.is_empty() {
                        li { class: "met-command-palette-empty", "No results" }
                    }
                }
            }
        }
    }
}
