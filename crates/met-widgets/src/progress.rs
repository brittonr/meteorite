//! Themed progress bar wrapping `dioxus_primitives::progress`.

pub use dioxus_primitives::progress::ProgressIndicator;

use dioxus::prelude::*;
use met_core::Variant;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Current value (0.0 to max)
    pub value: ReadSignal<Option<f64>>,
    /// Maximum value (default 100.0)
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        dioxus_primitives::progress::Progress {
            class: "met-progress {props.class}",
            value: props.value,
            max: props.max,
            ProgressIndicator {
                class: "met-progress-bar",
            }
        }
    }
}
