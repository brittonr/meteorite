use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContainerProps {
    /// Max width constraint. Defaults to 1200px.
    #[props(default = "1200px".to_string())]
    pub max_width: String,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// Centered container with max width.
#[component]
pub fn Container(props: ContainerProps) -> Element {
    rsx! {
        div {
            class: "met-container {props.class}",
            style: "max-width: {props.max_width};",
            {props.children}
        }
    }
}
