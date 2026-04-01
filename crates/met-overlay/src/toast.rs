//! Themed toast notifications wrapping `dioxus_primitives::toast`.

pub use dioxus_primitives::toast::{ToastOptions, ToastProvider, ToastType, Toasts, use_toast};

// Re-export the primitives directly — the styling is handled via CSS
// on the `data-type` attribute that the primitive already emits.
//
// Usage:
// ```
// use met_overlay::toast::{ToastProvider, use_toast, ToastType};
//
// #[component]
// fn App() -> Element {
//     rsx! {
//         ToastProvider {
//             MyPage {}
//         }
//     }
// }
//
// #[component]
// fn MyPage() -> Element {
//     let toast = use_toast();
//     rsx! {
//         button {
//             onclick: move |_| toast.add("Saved", None, ToastType::Success, None, false),
//             "Save"
//         }
//     }
// }
// ```
