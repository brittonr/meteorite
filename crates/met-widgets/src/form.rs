use dioxus::prelude::*;
use met_core::Size;

// ── Layout helpers ──────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct FormGroupProps {
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn FormGroup(props: FormGroupProps) -> Element {
    rsx! {
        div { class: "met-form-group {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FormLabelProps {
    pub text: String,
    #[props(default = false)]
    pub required: bool,
    #[props(default)]
    pub r#for: Option<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormLabel(props: FormLabelProps) -> Element {
    let req = if props.required { "met-form-label-required" } else { "" };

    rsx! {
        label {
            class: "met-form-label {req} {props.class}",
            r#for: props.r#for,
            {props.text}
        }
    }
}

// ── Inputs ──────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct FormInputProps {
    pub value: String,
    pub onchange: EventHandler<String>,
    #[props(default = "text".to_string())]
    pub r#type: String,
    #[props(default)]
    pub placeholder: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub required: bool,
    #[props(default)]
    pub size: Size,
    #[props(default = false)]
    pub error: bool,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormInput(props: FormInputProps) -> Element {
    let err = if props.error { "met-form-input-error" } else { "" };
    let class = format!("met-form-input {} {} {}", props.size.class(), err, props.class);

    rsx! {
        input {
            class: "{class}",
            r#type: "{props.r#type}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            required: props.required,
            id: props.id,
            oninput: move |e| props.onchange.call(e.value()),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FormTextareaProps {
    pub value: String,
    pub onchange: EventHandler<String>,
    #[props(default)]
    pub placeholder: String,
    #[props(default = 3)]
    pub rows: u32,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub error: bool,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormTextarea(props: FormTextareaProps) -> Element {
    let err = if props.error { "met-form-input-error" } else { "" };
    let class = format!("met-form-textarea {} {}", err, props.class);

    rsx! {
        textarea {
            class: "{class}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            rows: props.rows as i64,
            disabled: props.disabled,
            id: props.id,
            oninput: move |e| props.onchange.call(e.value()),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FormSelectProps {
    pub value: String,
    pub onchange: EventHandler<String>,
    pub children: Element,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub size: Size,
    #[props(default = false)]
    pub error: bool,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormSelect(props: FormSelectProps) -> Element {
    let err = if props.error { "met-form-input-error" } else { "" };
    let class = format!("met-form-select {} {} {}", props.size.class(), err, props.class);

    rsx! {
        select {
            class: "{class}",
            value: "{props.value}",
            disabled: props.disabled,
            id: props.id,
            onchange: move |e| props.onchange.call(e.value()),
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FormCheckboxProps {
    pub checked: bool,
    pub onchange: EventHandler<bool>,
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormCheckbox(props: FormCheckboxProps) -> Element {
    rsx! {
        label { class: "met-form-checkbox {props.class}",
            input {
                r#type: "checkbox",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |e| props.onchange.call(e.checked()),
            }
            {props.label}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FormErrorProps {
    pub message: String,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn FormError(props: FormErrorProps) -> Element {
    rsx! {
        div { class: "met-form-error {props.class}", {props.message} }
    }
}
