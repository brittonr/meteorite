use dioxus::prelude::*;
use met_core::{Theme, ThemeProvider, Variant, use_theme_signal};

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::dark(),
            Showcase {}
        }
    }
}

#[component]
fn Showcase() -> Element {
    let mut count = use_signal(|| 0);
    let mut theme_sig = use_theme_signal();
    let is_dark = theme_sig.read().name == "dark";

    rsx! {
        div { class: "showcase",
            style: "background: var(--met-bg); color: var(--met-fg); min-height: 100vh; padding: var(--met-space-lg);",

            h1 { "meteorite showcase" }

            section {
                h2 { "Theme" }
                met_widgets::Button {
                    variant: Variant::Secondary,
                    onclick: move |_| {
                        let next = if is_dark { Theme::light() } else { Theme::dark() };
                        theme_sig.set(next);
                    },
                    if is_dark { "Switch to Light" } else { "Switch to Dark" }
                }
            }

            section {
                h2 { "Button" }
                met_layout::HStack {
                    met_widgets::Button {
                        onclick: move |_| count += 1,
                        "Clicked {count} times"
                    }
                    met_widgets::Button { variant: Variant::Primary, "Primary" }
                    met_widgets::Button { variant: Variant::Danger, "Danger" }
                    met_widgets::Button { variant: Variant::Ghost, "Ghost" }
                }
            }

            section {
                h2 { "Badge" }
                met_layout::HStack {
                    met_widgets::Badge { "Default" }
                    met_widgets::Badge { variant: Variant::Success, "Success" }
                    met_widgets::Badge { variant: Variant::Warning, "Warning" }
                    met_widgets::Badge { variant: Variant::Danger, "Error" }
                }
            }

            section {
                h2 { "Progress" }
                met_layout::VStack {
                    met_widgets::Progress { value: 65.0 }
                    met_widgets::Progress { value: 30.0, variant: Variant::Success }
                }
            }
        }
    }
}
