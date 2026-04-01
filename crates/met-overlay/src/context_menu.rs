//! Themed context menu wrapping `dioxus_primitives::context_menu`.

pub use dioxus_primitives::context_menu::{
    ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Controlled open state
    pub open: ReadSignal<Option<bool>>,
    /// Default open state
    #[props(default)]
    pub default_open: bool,
    /// Called when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Disable the context menu
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub disabled: ReadSignal<bool>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    rsx! {
        dioxus_primitives::context_menu::ContextMenu {
            class: "met-context-menu {props.class}",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            {props.children}
        }
    }
}
