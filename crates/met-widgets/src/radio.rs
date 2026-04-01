//! Radio button group wrapping `dioxus_primitives::radio_group`.

use dioxus::prelude::*;
use met_core::Size;

pub use dioxus_primitives::radio_group::{RadioGroup as PrimitiveRadioGroup, RadioItem};

/// Orientation of the radio group layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioOrientation {
    Horizontal,
    #[default]
    Vertical,
}

/// A single option in a radio group.
#[derive(Clone, PartialEq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Controlled selected value
    pub value: Signal<String>,
    /// Available options
    pub options: Vec<RadioOption>,
    /// Layout orientation
    #[props(default)]
    pub orientation: RadioOrientation,
    /// Size variant
    #[props(default)]
    pub size: Size,
    /// Disable the entire group
    #[props(default)]
    pub disabled: bool,
    /// Called when the selection changes
    #[props(default)]
    pub on_change: EventHandler<String>,
    #[props(default)]
    pub class: String,
}

/// Themed radio button group.
///
/// ```rust,ignore
/// let color = use_signal(|| "red".to_string());
/// RadioGroup {
///     value: color,
///     options: vec![
///         RadioOption::new("red", "Red"),
///         RadioOption::new("blue", "Blue"),
///     ],
///     on_change: move |v| color.set(v),
/// }
/// ```
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let orientation_class = match props.orientation {
        RadioOrientation::Horizontal => "met-radio--horizontal",
        RadioOrientation::Vertical => "met-radio--vertical",
    };
    let mut value_signal = props.value;

    rsx! {
        PrimitiveRadioGroup {
            value: Some(value_signal.read().clone()),
            on_value_change: move |val: String| {
                value_signal.set(val.clone());
                props.on_change.call(val);
            },
            disabled: props.disabled,
            horizontal: props.orientation == RadioOrientation::Horizontal,
            class: "met-radio {orientation_class} {props.size.class()} {props.class}",

            for (index, option) in props.options.iter().enumerate() {
                {
                    let opt_disabled = option.disabled || props.disabled;
                    let label_class = if opt_disabled { "met-radio-label met-radio-label--disabled" } else { "met-radio-label" };
                    let label_text = option.label.clone();
                    let opt_value = option.value.clone();
                    rsx! {
                        label {
                            class: label_class,
                            RadioItem {
                                value: opt_value,
                                index: index,
                                disabled: opt_disabled,
                                class: "met-radio-item",
                                span { class: "met-radio-circle",
                                    span { class: "met-radio-dot" }
                                }
                            }
                            span { class: "met-radio-text", {label_text} }
                        }
                    }
                }
            }
        }
    }
}
