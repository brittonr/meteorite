use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    pub checked: bool,
    #[props(default)]
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
    pub onchange: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let class = format!("met-checkbox {}", props.class);

    rsx! {
        label { class: "{class}",
            input {
                r#type: "checkbox",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
            }
            if !props.label.is_empty() {
                span { "{props.label}" }
            }
        }
    }
}
