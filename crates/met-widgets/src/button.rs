use dioxus::prelude::*;
use met_core::{Size, Variant};

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub size: Size,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
    #[props(default)]
    pub class: String,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = format!(
        "met-btn {} {} {}",
        props.variant.class(),
        props.size.class(),
        props.class
    );

    rsx! {
        button {
            class: "{class}",
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            if props.loading {
                span { class: "met-btn-spinner" }
            }
            {props.children}
        }
    }
}
