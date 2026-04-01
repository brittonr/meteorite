use dioxus::prelude::*;
use met_core::Size;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    pub value: String,
    pub options: Vec<SelectOption>,
    #[props(default)]
    pub size: Size,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
    pub onchange: Option<EventHandler<FormEvent>>,
}

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let class = format!("met-select {} {}", props.size.class(), props.class);

    rsx! {
        select {
            class: "{class}",
            value: "{props.value}",
            disabled: props.disabled,
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            for opt in props.options.iter() {
                option {
                    value: "{opt.value}",
                    selected: opt.value == props.value,
                    "{opt.label}"
                }
            }
        }
    }
}
