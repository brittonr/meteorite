use dioxus::prelude::*;
use dioxus_primitives::separator::Separator;

/// Line style for dividers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl DividerStyle {
    fn class(&self) -> &'static str {
        match self {
            Self::Solid => "met-divider-solid",
            Self::Dashed => "met-divider-dashed",
            Self::Dotted => "met-divider-dotted",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// Horizontal (default) or vertical
    #[props(default = true)]
    pub horizontal: bool,
    /// Line style
    #[props(default)]
    pub style: DividerStyle,
    /// Optional text label in the middle of the divider
    #[props(default)]
    pub text: Option<String>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Divider(props: DividerProps) -> Element {
    let class = format!("met-divider {} {}", props.style.class(), props.class);

    if let Some(text) = &props.text {
        rsx! {
            div { class: "met-divider-with-text {class}",
                Separator { horizontal: props.horizontal, decorative: true }
                span { class: "met-divider-text", {text.clone()} }
                Separator { horizontal: props.horizontal, decorative: true }
            }
        }
    } else {
        rsx! {
            Separator {
                class: "{class}",
                horizontal: props.horizontal,
                decorative: true,
            }
        }
    }
}
