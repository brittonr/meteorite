use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    /// Gap between items. Defaults to `var(--met-space-sm)`.
    #[props(default = "var(--met-space-sm)".to_string())]
    pub gap: String,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// Vertical stack layout.
#[component]
pub fn VStack(props: StackProps) -> Element {
    rsx! {
        div {
            class: "met-vstack {props.class}",
            style: "gap: {props.gap};",
            {props.children}
        }
    }
}

/// Horizontal stack layout.
#[component]
pub fn HStack(props: StackProps) -> Element {
    rsx! {
        div {
            class: "met-hstack {props.class}",
            style: "gap: {props.gap};",
            {props.children}
        }
    }
}
