use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    pub text: String,
    #[props(default)]
    pub position: TooltipPosition,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let pos_class = match props.position {
        TooltipPosition::Top => "met-tooltip-top",
        TooltipPosition::Bottom => "met-tooltip-bottom",
        TooltipPosition::Left => "met-tooltip-left",
        TooltipPosition::Right => "met-tooltip-right",
    };
    let class = format!("met-tooltip {} {}", pos_class, props.class);

    rsx! {
        div { class: "{class}",
            {props.children}
            span { class: "met-tooltip-text", "{props.text}" }
        }
    }
}
