//! Themed select wrapping `dioxus_primitives::select`.

pub use dioxus_primitives::select::{
    SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption, SelectTrigger,
    SelectValue,
};

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Controlled value
    #[props(default)]
    pub value: ReadSignal<Option<Option<String>>>,
    /// Default value when uncontrolled
    #[props(default)]
    pub default_value: Option<String>,
    /// Called when selection changes
    #[props(default)]
    pub on_value_change: Callback<Option<String>>,
    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    /// Placeholder text shown when nothing is selected
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    rsx! {
        dioxus_primitives::select::Select::<String> {
            class: "met-select {props.class}",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            placeholder: props.placeholder,
            {props.children}
        }
    }
}
