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
    let flex_dir = match props.direction {
        SplitDirection::Horizontal => "row",
        SplitDirection::Vertical => "column",
    };
    let class = format!("met-split {}", props.class);

    rsx! {
        div {
            class: "{class}",
            style: "display: flex; flex-direction: {flex_dir}; width: 100%; height: 100%;",
            div { style: "flex: {pct}; overflow: auto;" }
            div { class: "met-split-handle", style: "flex: 0 0 4px; cursor: col-resize;" }
            div { style: "flex: {rest}; overflow: auto;" }
        }
    }
}
