use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub open: bool,
    #[props(default)]
    pub class: String,
    pub on_close: Option<EventHandler<()>>,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.open {
        return rsx! {};
    }

    let class = format!("met-modal {}", props.class);

    rsx! {
        div { class: "met-modal-backdrop",
            onclick: move |_| {
                if let Some(handler) = &props.on_close {
                    handler.call(());
                }
            },
            div {
                class: "{class}",
                onclick: move |evt| evt.stop_propagation(),
                {props.children}
            }
        }
    }
}
