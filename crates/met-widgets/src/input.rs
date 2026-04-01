use dioxus::prelude::*;
use met_core::Size;

#[derive(Props, Clone, PartialEq)]
pub struct TextInputProps {
    pub value: String,
    #[props(default)]
    pub placeholder: String,
    #[props(default)]
    pub size: Size,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
    pub oninput: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let class = format!("met-input {} {}", props.size.class(), props.class);

    rsx! {
        input {
            r#type: "text",
            class: "{class}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
        }
    }
}
