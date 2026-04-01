use dioxus::prelude::*;
use crate::modal::Modal;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    pub open: bool,
    #[props(default)]
    pub title: String,
    #[props(default)]
    pub class: String,
    pub on_close: Option<EventHandler<()>>,
    pub children: Element,
}

/// A modal dialog with a title bar and close button.
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let class = format!("met-dialog {}", props.class);

    rsx! {
        Modal { open: props.open, on_close: props.on_close.clone(),
            div { class: "{class}",
                if !props.title.is_empty() {
                    div { class: "met-dialog-header",
                        h3 { "{props.title}" }
                        button {
                            class: "met-dialog-close",
                            onclick: move |_| {
                                if let Some(handler) = &props.on_close {
                                    handler.call(());
                                }
                            },
                            "×"
                        }
                    }
                }
                div { class: "met-dialog-body", {props.children} }
            }
        }
    }
}
