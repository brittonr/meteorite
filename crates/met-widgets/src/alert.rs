use dioxus::prelude::*;
use met_core::Variant;

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// Alert variant (Success, Warning, Danger, Primary, etc.)
    #[props(default)]
    pub variant: Variant,
    /// Optional title line
    #[props(default)]
    pub title: Option<String>,
    /// Whether the alert can be dismissed
    #[props(default = false)]
    pub dismissible: bool,
    /// Called when dismiss button is clicked
    #[props(default)]
    pub ondismiss: Option<EventHandler<()>>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let icon = match props.variant {
        Variant::Success => "✓",
        Variant::Danger => "✕",
        Variant::Warning => "⚠",
        _ => "ℹ",
    };

    let class = format!("met-alert {} {}", props.variant.class(), props.class);

    rsx! {
        div { class: "{class}", role: "alert",
            span { class: "met-alert-icon", {icon} }
            div { class: "met-alert-content",
                if let Some(title) = &props.title {
                    div { class: "met-alert-title", {title.clone()} }
                }
                div { class: "met-alert-body", {props.children} }
            }
            if props.dismissible {
                button {
                    class: "met-alert-dismiss",
                    aria_label: "Dismiss",
                    onclick: move |_| {
                        if let Some(handler) = &props.ondismiss {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}
