//! Themed tabs wrapping `dioxus_primitives::tabs`.
//!
//! Re-exports the primitive sub-components directly and adds a
//! convenience `met-tabs` CSS class to the root.

pub use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger};

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Controlled active tab value
    pub value: ReadSignal<Option<String>>,
    /// Default active tab when uncontrolled
    #[props(default)]
    pub default_value: String,
    /// Called when the active tab changes
    #[props(default)]
    pub on_value_change: Callback<String>,
    /// Whether all tabs are disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    rsx! {
        dioxus_primitives::tabs::Tabs {
            class: "met-tabs {props.class}",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            {props.children}
        }
    }
}
