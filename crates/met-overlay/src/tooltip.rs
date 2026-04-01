//! Themed tooltip wrapping `dioxus_primitives::tooltip`.

pub use dioxus_primitives::tooltip::{TooltipContent, TooltipTrigger};

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Controlled open state
    pub open: ReadSignal<Option<bool>>,
    /// Default open state when uncontrolled
    #[props(default)]
    pub default_open: bool,
    /// Called when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Disable the tooltip
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        dioxus_primitives::tooltip::Tooltip {
            class: "met-tooltip {props.class}",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            {props.children}
        }
    }
}
