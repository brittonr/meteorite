//! Status badge with variant-based styling.

use dioxus::prelude::*;

/// Status variants for the badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusVariant {
    #[default]
    Idle,
    Processing,
    Success,
    Error,
    Warning,
    Cached,
}

impl StatusVariant {
    fn icon(&self) -> &'static str {
        match self {
            Self::Idle => "⚪",
            Self::Processing => "🔄",
            Self::Success => "✅",
            Self::Error => "❌",
            Self::Warning => "⚠️",
            Self::Cached => "💾",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Processing => "Processing",
            Self::Success => "Success",
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Cached => "Cached",
        }
    }

    fn css_class(&self) -> &'static str {
        match self {
            Self::Idle => "met-status--idle",
            Self::Processing => "met-status--processing",
            Self::Success => "met-status--success",
            Self::Error => "met-status--error",
            Self::Warning => "met-status--warning",
            Self::Cached => "met-status--cached",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StatusBadgeProps {
    /// The status variant
    pub variant: StatusVariant,
    /// Override the default label text
    #[props(default)]
    pub text: String,
    /// Show a pulsing animation for processing status
    #[props(default)]
    pub animated: bool,
    #[props(default)]
    pub class: String,
}

/// Status badge with icon + label.
///
/// ```rust,ignore
/// StatusBadge { variant: StatusVariant::Success }
/// StatusBadge { variant: StatusVariant::Processing, animated: true }
/// StatusBadge { variant: StatusVariant::Error, text: "Failed to load" }
/// ```
#[component]
pub fn StatusBadge(props: StatusBadgeProps) -> Element {
    let label = if props.text.is_empty() {
        props.variant.label()
    } else {
        &props.text
    };

    let animated_class = if props.animated && props.variant == StatusVariant::Processing {
        " met-status--animated"
    } else {
        ""
    };

    let variant_class = props.variant.css_class();

    rsx! {
        div {
            class: "met-status-badge {variant_class}{animated_class} {props.class}",
            span { class: "met-status-icon", {props.variant.icon()} }
            span { class: "met-status-text", {label} }
        }
    }
}
