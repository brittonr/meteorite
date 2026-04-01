use dioxus::prelude::*;
use met_core::{Size, Variant};

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default)]
    pub variant: Variant,
    /// Padding size (Sm, Md, Lg, etc.)
    #[props(default)]
    pub padding: Size,
    /// Show hover effect
    #[props(default = false)]
    pub hoverable: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let hover = if props.hoverable || props.onclick.is_some() {
        "met-card-hoverable"
    } else {
        ""
    };
    let class = format!(
        "met-card {} {} {} {}",
        props.variant.class(),
        props.padding.class(),
        hover,
        props.class
    );

    rsx! {
        div {
            class: "{class}",
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div { class: "met-card-header {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardBodyProps {
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! {
        div { class: "met-card-body {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div { class: "met-card-footer {props.class}", {props.children} }
    }
}
