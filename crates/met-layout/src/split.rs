use dioxus::prelude::*;

/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Props, Clone, PartialEq)]
pub struct SplitProps {
    #[props(default)]
    pub direction: SplitDirection,
    /// Initial split ratio (0.0 to 1.0). Defaults to 0.5.
    #[props(default = 0.5)]
    pub ratio: f64,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Split(props: SplitProps) -> Element {
    let pct = (props.ratio.clamp(0.0, 1.0) * 100.0) as u32;
    let rest = 100 - pct;
    let dir = match props.direction {
        SplitDirection::Horizontal => "met-split-horizontal",
        SplitDirection::Vertical => "met-split-vertical",
    };

    rsx! {
        div { class: "met-split {dir} {props.class}",
            div { class: "met-split-pane", style: "flex: {pct};" }
            div { class: "met-split-handle" }
            div { class: "met-split-pane", style: "flex: {rest};" }
        }
    }
}
