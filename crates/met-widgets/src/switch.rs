//! Themed switch wrapping `dioxus_primitives::switch`.

pub use dioxus_primitives::switch::SwitchThumb;

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Controlled checked state
    pub checked: ReadSignal<Option<bool>>,
    /// Default state when uncontrolled
    #[props(default = false)]
    pub default_checked: bool,
    /// Text label rendered next to the switch
    #[props(default)]
    pub label: String,
    /// Whether the switch is disabled
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub disabled: ReadSignal<bool>,
    /// Called when checked state changes
    #[props(default)]
    pub on_checked_change: Callback<bool>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        label { class: "met-switch {props.class}",
            dioxus_primitives::switch::Switch {
                checked: props.checked,
                default_checked: props.default_checked,
                disabled: props.disabled,
                on_checked_change: props.on_checked_change,
                SwitchThumb {}
            }
            if !props.label.is_empty() {
                span { class: "met-switch-label", {props.label} }
            }
        }
    }
}
