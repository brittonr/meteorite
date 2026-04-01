//! Compound form field: label + input + validation + help text.

use dioxus::prelude::*;
use met_core::Size;

/// Option for select inputs.
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
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

/// Supported input types for [`FormField`].
#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Number {
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
    },
    Email,
    Password,
    TextArea {
        rows: Option<u32>,
    },
    Select {
        options: Vec<SelectOption>,
    },
    Checkbox,
}

/// Validation state displayed under the input.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ValidationState {
    #[default]
    None,
    Valid,
    Warning(String),
    Error(String),
}

#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    pub label: String,
    pub value: String,
    pub input_type: InputType,
    pub on_change: EventHandler<String>,

    #[props(default)]
    pub validation: ValidationState,
    #[props(default = false)]
    pub required: bool,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub help_text: Option<String>,
    #[props(default)]
    pub on_enter: Option<EventHandler<()>>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    let input_class = build_input_classes(&props);

    rsx! {
        div { class: "met-form-field {props.class}",

            // Label
            label { class: "met-form-label",
                "{props.label}"
                if props.required {
                    span { class: "met-required", " *" }
                }
                if let Some(help) = &props.help_text {
                    if help.len() <= 50 {
                        span { class: "met-help-icon", title: "{help}", "ⓘ" }
                    }
                }
            }

            // Input
            {render_input(&props, &input_class)}

            // Validation
            {render_validation(&props.validation)}

            // Long help text below the field
            if let Some(help) = &props.help_text {
                if help.len() > 50 {
                    div { class: "met-form-help", "{help}" }
                }
            }
        }
    }
}

// ── Helpers (pure functions) ────────────────────────────────────────

fn build_input_classes(props: &FormFieldProps) -> String {
    let size_class = props.size.class();
    let validation_class = match &props.validation {
        ValidationState::Error(_) => "met-has-error",
        ValidationState::Warning(_) => "met-has-warning",
        ValidationState::Valid => "met-has-success",
        ValidationState::None => "",
    };
    let disabled_class = if props.disabled { "met-disabled" } else { "" };

    format!("met-form-input {} {} {}", size_class, validation_class, disabled_class)
}

fn render_input(props: &FormFieldProps, class: &str) -> Element {
    let on_change = props.on_change.clone();
    let on_enter = props.on_enter.clone();
    let disabled = props.disabled;
    let placeholder = props.placeholder.clone().unwrap_or_default();
    let value = props.value.clone();
    let class = class.to_string();

    match &props.input_type {
        InputType::Text => rsx! {
            input {
                class: "{class}",
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: disabled,
                oninput: move |e| on_change.call(e.value()),
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        if let Some(h) = &on_enter { h.call(()); }
                    }
                },
            }
        },
        InputType::Number { min, max, step } => rsx! {
            input {
                class: "{class}",
                r#type: "number",
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: disabled,
                min: min.map(|v| v.to_string()),
                max: max.map(|v| v.to_string()),
                step: step.map(|v| v.to_string()),
                oninput: move |e| on_change.call(e.value()),
                onkeypress: move |e| {
                    if e.key() == Key::Enter {
                        if let Some(h) = &on_enter { h.call(()); }
                    }
                },
            }
        },
        InputType::Email => rsx! {
            input {
                class: "{class}",
                r#type: "email",
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: disabled,
                oninput: move |e| on_change.call(e.value()),
            }
        },
        InputType::Password => rsx! {
            input {
                class: "{class}",
                r#type: "password",
                value: "{value}",
                placeholder: "{placeholder}",
                disabled: disabled,
                oninput: move |e| on_change.call(e.value()),
            }
        },
        InputType::TextArea { rows } => {
            let rows = rows.unwrap_or(3);
            rsx! {
                textarea {
                    class: "{class}",
                    value: "{value}",
                    placeholder: "{placeholder}",
                    disabled: disabled,
                    rows: rows,
                    oninput: move |e| on_change.call(e.value()),
                }
            }
        }
        InputType::Select { options } => {
            let options = options.clone();
            rsx! {
                select {
                    class: "{class}",
                    disabled: disabled,
                    onchange: move |e| on_change.call(e.value()),
                    if !placeholder.is_empty() {
                        option { value: "", disabled: true, selected: value.is_empty(), "{placeholder}" }
                    }
                    for opt in options {
                        option {
                            value: "{opt.value}",
                            selected: value == opt.value,
                            disabled: opt.disabled,
                            "{opt.label}"
                        }
                    }
                }
            }
        }
        InputType::Checkbox => rsx! {
            input {
                r#type: "checkbox",
                checked: value == "true",
                disabled: disabled,
                onchange: move |e| {
                    on_change.call(if e.checked() { "true".into() } else { "false".into() });
                },
            }
        },
    }
}

fn render_validation(state: &ValidationState) -> Element {
    match state {
        ValidationState::Error(msg) => rsx! {
            div { class: "met-form-error", "⚠ {msg}" }
        },
        ValidationState::Warning(msg) => rsx! {
            div { class: "met-form-warning", "⚠ {msg}" }
        },
        ValidationState::Valid => rsx! {
            div { class: "met-form-success", "✓ Valid" }
        },
        ValidationState::None => rsx! {},
    }
}
