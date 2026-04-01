use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "showcase",
            h1 { "meteorite showcase" }
            p { "Component library for Dioxus" }

            section {
                h2 { "Button" }
                met_widgets::Button {
                    onclick: move |_| count += 1,
                    "Clicked {count} times"
                }
                met_widgets::Button {
                    variant: met_core::Variant::Primary,
                    "Primary"
                }
                met_widgets::Button {
                    variant: met_core::Variant::Danger,
                    "Danger"
                }
            }

            section {
                h2 { "Badge" }
                met_widgets::Badge { "Default" }
                met_widgets::Badge { variant: met_core::Variant::Success, "Success" }
                met_widgets::Badge { variant: met_core::Variant::Warning, "Warning" }
            }

            section {
                h2 { "Progress" }
                met_widgets::Progress { value: 65.0 }
                met_widgets::Progress {
                    value: 30.0,
                    variant: met_core::Variant::Success,
                }
            }
        }
    }
}
