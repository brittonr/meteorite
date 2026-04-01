//! Themed slider wrapping `dioxus_primitives::slider`.

pub use dioxus_primitives::slider::SliderValue;

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Controlled value
    pub value: ReadSignal<Option<SliderValue>>,
    /// Default value when uncontrolled
    #[props(default = SliderValue::Single(0.0))]
    pub default_value: SliderValue,
    #[props(default = 0.0)]
    pub min: f64,
    #[props(default = 100.0)]
    pub max: f64,
    #[props(default = 1.0)]
    pub step: f64,
    /// Whether the slider is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Called when value changes
    #[props(default)]
    pub on_value_change: Callback<SliderValue>,
    /// Accessible label
    #[props(default)]
    pub label: ReadSignal<Option<String>>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        dioxus_primitives::slider::Slider {
            class: "met-slider {props.class}",
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            on_value_change: props.on_value_change,
            label: props.label,
            dioxus_primitives::slider::SliderTrack {
                dioxus_primitives::slider::SliderRange {}
            }
            dioxus_primitives::slider::SliderThumb {}
        }
    }
}
