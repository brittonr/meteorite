use dioxus::prelude::*;

/// Visual style of the loader animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoaderType {
    #[default]
    Dots,
    Bars,
    Pulse,
    Skeleton,
}

#[derive(Props, Clone, PartialEq)]
pub struct LoaderProps {
    #[props(default)]
    pub loader_type: LoaderType,
    /// Number of animated items (dots / bars). Max 10.
    #[props(default = 3)]
    pub count: usize,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Loader(props: LoaderProps) -> Element {
    let count = props.count.min(10);

    match props.loader_type {
        LoaderType::Dots => rsx! {
            div { class: "met-loader met-loader-dots {props.class}",
                for i in 0..count {
                    span {
                        key: "dot-{i}",
                        class: "met-loader-dot",
                        style: "animation-delay: {i * 150}ms",
                    }
                }
            }
        },
        LoaderType::Bars => rsx! {
            div { class: "met-loader met-loader-bars {props.class}",
                for i in 0..count {
                    span {
                        key: "bar-{i}",
                        class: "met-loader-bar",
                        style: "animation-delay: {i * 150}ms",
                    }
                }
            }
        },
        LoaderType::Pulse => rsx! {
            div { class: "met-loader met-loader-pulse {props.class}",
                div { class: "met-pulse-ring" }
                div { class: "met-pulse-ring" }
                div { class: "met-pulse-ring" }
            }
        },
        LoaderType::Skeleton => rsx! {
            SkeletonLoader { class: props.class }
        },
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SkeletonLoaderProps {
    /// Number of placeholder lines. Max 20.
    #[props(default = 3)]
    pub lines: usize,
    /// Show a circular avatar placeholder
    #[props(default = false)]
    pub show_avatar: bool,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn SkeletonLoader(props: SkeletonLoaderProps) -> Element {
    let lines = props.lines.min(20);

    rsx! {
        div { class: "met-skeleton {props.class}",
            if props.show_avatar {
                div { class: "met-skeleton-avatar" }
            }
            div { class: "met-skeleton-content",
                for i in 0..lines {
                    div {
                        key: "skel-{i}",
                        class: "met-skeleton-line",
                        style: if i == lines - 1 { "width: 80%" } else { "" },
                    }
                }
            }
        }
    }
}

/// Wraps children and shows a loader while `loading` is true.
#[derive(Props, Clone, PartialEq)]
pub struct ContentLoaderProps {
    pub loading: bool,
    pub children: Element,
    /// Custom loader element; defaults to `Loader { Dots }`.
    #[props(default)]
    pub loader: Option<Element>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ContentLoader(props: ContentLoaderProps) -> Element {
    rsx! {
        div { class: "met-content-loader {props.class}",
            if props.loading {
                if let Some(custom) = &props.loader {
                    {custom.clone()}
                } else {
                    Loader {}
                }
            } else {
                {props.children}
            }
        }
    }
}
