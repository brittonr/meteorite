use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    pub message: String,
    #[props(default = ToastLevel::Info)]
    pub level: ToastLevel,
    #[props(default)]
    pub class: String,
    pub on_dismiss: Option<EventHandler<()>>,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let level_class = match props.level {
        ToastLevel::Info => "met-toast-info",
        ToastLevel::Success => "met-toast-success",
        ToastLevel::Warning => "met-toast-warning",
        ToastLevel::Error => "met-toast-error",
    };
    let class = format!("met-toast {} {}", level_class, props.class);

    rsx! {
        div { class: "{class}", role: "alert",
            span { "{props.message}" }
            button {
                class: "met-toast-dismiss",
                onclick: move |_| {
                    if let Some(handler) = &props.on_dismiss {
                        handler.call(());
                    }
                },
                "×"
            }
        }
    }
}
