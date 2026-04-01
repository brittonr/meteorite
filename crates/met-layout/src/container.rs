use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    #[props(default = "1200px".to_string())]
    pub max_width: String,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// Centered container with max width.
#[component]
pub fn Container(props: ContainerProps) -> Element {
    let class = format!("met-container {}", props.class);

    rsx! {
        div {
            class: "{class}",
            style: "max-width: {props.max_width}; margin: 0 auto; padding: 0 16px;",
            {props.children}
        }
    }
}
