use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    /// Number of columns. Defaults to 12.
    #[props(default = 12)]
    pub columns: u32,
    /// Gap between cells. Defaults to `var(--met-space-md)`.
    #[props(default = "var(--met-space-md)".to_string())]
    pub gap: String,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    rsx! {
        div {
            class: "met-grid {props.class}",
            style: "grid-template-columns: repeat({props.columns}, 1fr); gap: {props.gap};",
            {props.children}
        }
    }
}
