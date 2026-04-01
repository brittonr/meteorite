use dioxus::prelude::*;
use met_core::{Size, Variant};

#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub variant: Variant,
    /// Optional text label next to the spinner
    #[props(default)]
    pub label: Option<String>,
    /// Center the spinner in its container
    #[props(default = false)]
    pub centered: bool,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let center = if props.centered { "met-spinner-centered" } else { "" };
    let class = format!(
        "met-spinner {} {} {} {}",
        props.size.class(),
        props.variant.class(),
        center,
        props.class
    );

    rsx! {
        div { class: "{class}", role: "status",
            svg {
                class: "met-spinner-svg",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                circle {
                    class: "met-spinner-track",
                    cx: "12",
                    cy: "12",
                    r: "10",
                    stroke: "currentColor",
                    stroke_width: "4",
                }
                path {
                    class: "met-spinner-arc",
                    fill: "currentColor",
                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                }
            }
            if let Some(label) = &props.label {
                span { class: "met-spinner-label", {label.clone()} }
            }
        }
    }
}

/// Full-screen loading overlay with spinner.
#[derive(Props, Clone, PartialEq)]
pub struct LoadingOverlayProps {
    pub visible: bool,
    #[props(default = "Loading...".to_string())]
    pub message: String,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn LoadingOverlay(props: LoadingOverlayProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    rsx! {
        div { class: "met-loading-overlay {props.class}",
            Spinner {
                size: Size::Lg,
                centered: true,
                label: props.message,
            }
        }
    }
}
