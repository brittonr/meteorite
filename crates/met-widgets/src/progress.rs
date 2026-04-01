use dioxus::prelude::*;
use met_core::Variant;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// 0.0 to 100.0
    pub value: f64,
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let clamped = props.value.clamp(0.0, 100.0);
    let class = format!("met-progress {} {}", props.variant.class(), props.class);

    rsx! {
        div { class: "{class}", role: "progressbar",
            aria_valuenow: "{clamped}",
            aria_valuemin: "0",
            aria_valuemax: "100",
            div {
                class: "met-progress-bar",
                style: "width: {clamped}%",
            }
        }
    }
}
