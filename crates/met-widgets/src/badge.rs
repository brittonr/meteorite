use dioxus::prelude::*;
use met_core::Variant;

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class = format!("met-badge {} {}", props.variant.class(), props.class);

    rsx! {
        span { class: "{class}", {props.children} }
    }
}
