//! Tree Dioxus component.

use std::collections::BTreeSet;

use dioxus::prelude::*;

use crate::model::{TreeItem, compute_visible_rows};

#[derive(Props, Clone, PartialEq)]
pub struct TreeProps {
    /// Flat list of tree items with parent pointers.
    pub items: Vec<TreeItem>,
    /// Called when a node is selected (clicked or Enter pressed).
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,
    /// Called when expand/collapse state changes. Receives the full set.
    #[props(default)]
    pub on_toggle: Option<EventHandler<BTreeSet<String>>>,
    /// Externally controlled expanded set. If omitted, the tree manages its own.
    #[props(default)]
    pub expanded: Option<BTreeSet<String>>,
    /// Show guide lines (│ ├ └) connecting nodes. Default true.
    #[props(default = true)]
    pub show_guides: bool,
    /// Extra CSS class on the root element.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Tree(props: TreeProps) -> Element {
    let mut internal_expanded = use_signal(BTreeSet::<String>::new);

    let expanded = match &props.expanded {
        Some(ext) => ext.clone(),
        None => internal_expanded.read().clone(),
    };

    let rows = use_memo({
        let items = props.items.clone();
        let exp = expanded.clone();
        move || compute_visible_rows(&items, &exp)
    });

    let class = format!("met-tree {}", props.class);

    rsx! {
        ul {
            class: "{class}",
            role: "tree",
            for row in rows.read().iter() {
                {
                    let row_id = row.id.clone();
                    let row_label = row.label.clone();
                    let row_icon = row.icon.clone();
                    let depth = row.depth;
                    let has_children = row.has_children;
                    let is_expanded = row.is_expanded;
                    let is_last = row.is_last_sibling;
                    let ancestors_last = row.ancestors_last.clone();
                    let show_guides = props.show_guides;

                    let toggle_id = row_id.clone();
                    let select_id = row_id.clone();
                    let on_select = props.on_select.clone();
                    let on_toggle = props.on_toggle.clone();
                    let ext_expanded = props.expanded.clone();
                    let current_expanded = expanded.clone();

                    rsx! {
                        li {
                            class: "met-tree-row",
                            role: "treeitem",
                            "aria-expanded": if has_children { if is_expanded { "true" } else { "false" } } else { "" },
                            style: "padding-left: {depth as f32 * 1.25}rem;",
                            tabindex: "0",
                            onkeydown: {
                                let tid = toggle_id.clone();
                                let sid = select_id.clone();
                                let on_sel = on_select.clone();
                                let on_tog = on_toggle.clone();
                                let ext_exp = ext_expanded.clone();
                                let cur_exp = current_expanded.clone();
                                move |evt: KeyboardEvent| {
                                    match evt.key() {
                                        Key::Enter => {
                                            if has_children {
                                                let mut next = cur_exp.clone();
                                                if next.contains(&tid) { next.remove(&tid); } else { next.insert(tid.clone()); }
                                                if ext_exp.is_none() { internal_expanded.set(next.clone()); }
                                                if let Some(ref h) = on_tog { h.call(next); }
                                            }
                                            if let Some(ref h) = on_sel { h.call(sid.clone()); }
                                            evt.prevent_default();
                                        }
                                        Key::Character(ref c) if c == " " => {
                                            if has_children {
                                                let mut next = cur_exp.clone();
                                                if next.contains(&tid) { next.remove(&tid); } else { next.insert(tid.clone()); }
                                                if ext_exp.is_none() { internal_expanded.set(next.clone()); }
                                                if let Some(ref h) = on_tog { h.call(next); }
                                            }
                                            evt.prevent_default();
                                        }
                                        Key::ArrowRight if has_children && !is_expanded => {
                                            let mut next = cur_exp.clone();
                                            next.insert(tid.clone());
                                            if ext_exp.is_none() { internal_expanded.set(next.clone()); }
                                            if let Some(ref h) = on_tog { h.call(next); }
                                            evt.prevent_default();
                                        }
                                        Key::ArrowLeft if has_children && is_expanded => {
                                            let mut next = cur_exp.clone();
                                            next.remove(&tid);
                                            if ext_exp.is_none() { internal_expanded.set(next.clone()); }
                                            if let Some(ref h) = on_tog { h.call(next); }
                                            evt.prevent_default();
                                        }
                                        _ => {}
                                    }
                                }
                            },

                            // Guide lines
                            if show_guides {
                                span { class: "met-tree-guides",
                                    for anc_is_last in ancestors_last.iter() {
                                        if *anc_is_last {
                                            span { class: "met-tree-guide met-tree-guide-blank", "  " }
                                        } else {
                                            span { class: "met-tree-guide met-tree-guide-pipe", "│ " }
                                        }
                                    }
                                    if depth > 0 {
                                        if is_last {
                                            span { class: "met-tree-guide met-tree-guide-corner", "└ " }
                                        } else {
                                            span { class: "met-tree-guide met-tree-guide-tee", "├ " }
                                        }
                                    }
                                }
                            }

                            // Toggle chevron
                            if has_children {
                                span {
                                    class: "met-tree-toggle",
                                    onclick: {
                                        let tid2 = toggle_id.clone();
                                        let on_tog2 = on_toggle.clone();
                                        let ext_exp2 = ext_expanded.clone();
                                        let cur_exp2 = current_expanded.clone();
                                        move |evt: MouseEvent| {
                                            evt.stop_propagation();
                                            let mut next = cur_exp2.clone();
                                            if next.contains(&tid2) { next.remove(&tid2); } else { next.insert(tid2.clone()); }
                                            if ext_exp2.is_none() { internal_expanded.set(next.clone()); }
                                            if let Some(ref h) = on_tog2 { h.call(next); }
                                        }
                                    },
                                    if is_expanded { "▾" } else { "▸" }
                                }
                            } else {
                                span { class: "met-tree-toggle met-tree-toggle-leaf", " " }
                            }

                            // Icon
                            if let Some(ref ic) = row_icon {
                                span { class: "met-tree-icon", "{ic}" }
                            }

                            // Label
                            span {
                                class: "met-tree-label",
                                onclick: {
                                    let sid2 = select_id.clone();
                                    let on_sel2 = on_select.clone();
                                    move |_| {
                                        if let Some(ref h) = on_sel2 { h.call(sid2.clone()); }
                                    }
                                },
                                "{row_label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
