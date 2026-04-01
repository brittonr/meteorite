//! Themed popover wrapping `dioxus_primitives::popover`.

pub use dioxus_primitives::popover::{PopoverContent, PopoverTrigger};

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Controlled open state
    pub open: ReadSignal<Option<bool>>,
    /// Default open state
    #[props(default)]
    pub default_open: bool,
    /// Called when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    rsx! {
        dioxus_primitives::popover::PopoverRoot {
            class: "met-popover {props.class}",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            {props.children}
        }
    }
}
