use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    #[props(default = "8px".to_string())]
    pub gap: String,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// Vertical stack layout.
#[component]
pub fn VStack(props: StackProps) -> Element {
    let class = format!("met-vstack {}", props.class);

    rsx! {
        div {
            class: "{class}",
            style: "display: flex; flex-direction: column; gap: {props.gap};",
            {props.children}
        }
    }
}

/// Horizontal stack layout.
#[component]
pub fn HStack(props: StackProps) -> Element {
    let class = format!("met-hstack {}", props.class);

    rsx! {
        div {
            class: "{class}",
            style: "display: flex; flex-direction: row; gap: {props.gap}; align-items: center;",
            {props.children}
        }
    }
}
