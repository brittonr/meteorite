use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    #[props(default = 12)]
    pub columns: u32,
    #[props(default = "16px".to_string())]
    pub gap: String,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    let class = format!("met-grid {}", props.class);

    rsx! {
        div {
            class: "{class}",
            style: "display: grid; grid-template-columns: repeat({props.columns}, 1fr); gap: {props.gap};",
            {props.children}
        }
    }
}
