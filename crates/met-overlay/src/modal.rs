//! Convenience alias: `Modal` is a dialog that is always modal.

use dioxus::prelude::*;

pub use dioxus_primitives::dialog::{DialogContent as ModalContent, DialogTitle as ModalTitle};

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub open: ReadSignal<Option<bool>>,
    #[props(default)]
    pub default_open: bool,
    #[props(default)]
    pub on_open_change: Callback<bool>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    rsx! {
        dioxus_primitives::dialog::DialogRoot {
            class: "met-modal {props.class}",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            is_modal: true,
            {props.children}
        }
    }
}
