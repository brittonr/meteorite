use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    pub checked: bool,
    #[props(default)]
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
    pub onchange: Option<EventHandler<bool>>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let class = format!(
        "met-switch {} {}",
        if props.checked { "met-switch-on" } else { "" },
        props.class
    );

    rsx! {
        label { class: "{class}",
            button {
                role: "switch",
                aria_checked: "{props.checked}",
                disabled: props.disabled,
                onclick: move |_| {
                    if let Some(handler) = &props.onchange {
                        handler.call(!props.checked);
                    }
                },
                span { class: "met-switch-thumb" }
            }
            if !props.label.is_empty() {
                span { "{props.label}" }
            }
        }
    }
}
