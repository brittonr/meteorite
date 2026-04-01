//! Multi-line text input component.

use dioxus::prelude::*;
use met_core::Size;

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    /// Current value
    #[props(default)]
    pub value: String,
    /// Placeholder text
    #[props(default)]
    pub placeholder: String,
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    /// Whether the textarea is disabled
    #[props(default)]
    pub disabled: bool,
    /// Whether the textarea is read-only
    #[props(default)]
    pub readonly: bool,
    /// Whether the textarea is required
    #[props(default)]
    pub required: bool,
    /// Number of visible rows
    #[props(default = 4)]
    pub rows: u32,
    /// Size variant
    #[props(default)]
    pub size: Size,
    /// Called on every input change
    #[props(default)]
    pub oninput: EventHandler<String>,
    /// Called on blur
    #[props(default)]
    pub onblur: EventHandler<()>,
    /// Called on focus
    #[props(default)]
    pub onfocus: EventHandler<()>,
}

/// Multi-line text input.
///
/// ```rust,ignore
/// Textarea {
///     value: "hello",
///     rows: 6,
///     oninput: move |v| text.set(v),
/// }
/// ```
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let disabled_class = if props.disabled { " met-textarea--disabled" } else { "" };
    let readonly_class = if props.readonly { " met-textarea--readonly" } else { "" };

    rsx! {
        textarea {
            class: "met-textarea {props.size.class()}{disabled_class}{readonly_class} {props.class}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            required: props.required,
            rows: props.rows,
            oninput: move |evt| props.oninput.call(evt.value()),
            onfocus: move |_| props.onfocus.call(()),
            onblur: move |_| props.onblur.call(()),
        }
    }
}
