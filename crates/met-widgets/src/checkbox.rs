//! Themed checkbox wrapping `dioxus_primitives::checkbox`.

pub use dioxus_primitives::checkbox::CheckboxState;

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Controlled checked state
    pub checked: ReadSignal<Option<CheckboxState>>,
    /// Default state when uncontrolled
    #[props(default = CheckboxState::Unchecked)]
    pub default_checked: CheckboxState,
    /// Text label rendered next to the checkbox
    #[props(default)]
    pub label: String,
    /// Whether the checkbox is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Called when checked state changes
    #[props(default)]
    pub on_checked_change: Callback<CheckboxState>,
    /// Form field name
    #[props(default)]
    pub name: ReadSignal<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        label { class: "met-checkbox {props.class}",
            dioxus_primitives::checkbox::Checkbox {
                checked: props.checked,
                default_checked: props.default_checked,
                disabled: props.disabled,
                on_checked_change: props.on_checked_change,
                name: props.name,
                dioxus_primitives::checkbox::CheckboxIndicator {
                    "✓"
                }
            }
            if !props.label.is_empty() {
                span { class: "met-checkbox-label", {props.label} }
            }
        }
    }
}
