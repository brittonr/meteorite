use dioxus::prelude::*;
use met_core::Size;

/// Icon names with SVG paths (Feather-style 24×24 viewBox).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    // Actions
    Add,
    Remove,
    Edit,
    Delete,
    Save,
    Cancel,
    Check,
    Close,
    Refresh,
    Search,
    Filter,
    Sort,

    // Navigation
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    Menu,
    Home,

    // Status
    Info,
    Warning,
    Error,
    Success,

    // UI
    Settings,
    Eye,
    EyeOff,
    Copy,
    Download,
    Upload,

    // Common
    Play,
    Pause,
    Stop,
}

impl IconName {
    /// SVG path data for the icon.
    pub fn svg_path(&self) -> &'static str {
        match self {
            Self::Add => "M12 5v14 M5 12h14",
            Self::Remove => "M5 12h14",
            Self::Edit => "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7 M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z",
            Self::Delete => "M3 6h18 M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2 M10 11v6 M14 11v6",
            Self::Save => "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z M17 21v-8H7v8 M7 3v5h8",
            Self::Cancel | Self::Close => "M18 6L6 18 M6 6l12 12",
            Self::Check => "M20 6L9 17l-5-5",
            Self::Refresh => "M23 4v6h-6 M1 20v-6h6 M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15",
            Self::Search => "M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16z M21 21l-4.35-4.35",
            Self::Filter => "M22 3H2l8 9.46V19l4 2v-8.54L22 3z",
            Self::Sort => "M11 5h10 M11 12h7 M11 19h4 M3 5l4 4 4-4",
            Self::ArrowUp => "M12 19V5 M5 12l7-7 7 7",
            Self::ArrowDown => "M12 5v14 M19 12l-7 7-7-7",
            Self::ArrowLeft => "M19 12H5 M12 19l-7-7 7-7",
            Self::ArrowRight => "M5 12h14 M12 5l7 7-7 7",
            Self::ChevronUp => "M18 15l-6-6-6 6",
            Self::ChevronDown => "M6 9l6 6 6-6",
            Self::ChevronLeft => "M15 18l-6-6 6-6",
            Self::ChevronRight => "M9 18l6-6-6-6",
            Self::Menu => "M3 12h18 M3 6h18 M3 18h18",
            Self::Home => "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z M9 22V12h6v10",
            Self::Info => "M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20z M12 16v-4 M12 8h.01",
            Self::Warning => "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z M12 9v4 M12 17h.01",
            Self::Error => "M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20z M15 9l-6 6 M9 9l6 6",
            Self::Success => "M22 11.08V12a10 10 0 1 1-5.93-9.14 M22 4L12 14.01l-3-3",
            Self::Settings => "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z",
            Self::Eye => "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z",
            Self::EyeOff => "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24 M1 1l22 22",
            Self::Copy => "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2 M9 14h6 M9 18h6 M9 10h6",
            Self::Download => "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3",
            Self::Upload => "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M17 8l-5-5-5 5 M12 3v12",
            Self::Play => "M5 3l14 9-14 9V3z",
            Self::Pause => "M6 4h4v16H6zM14 4h4v16h-4z",
            Self::Stop => "M4 4h16v16H4z",
        }
    }
}

fn icon_px(size: &Size) -> u32 {
    match size {
        Size::Xs => 12,
        Size::Sm => 16,
        Size::Md => 24,
        Size::Lg => 32,
        Size::Xl => 48,
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub icon: IconName,
    #[props(default)]
    pub size: Size,
    /// CSS color value; defaults to `currentColor`.
    #[props(default = "currentColor".to_string())]
    pub color: String,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let px = icon_px(&props.size);

    rsx! {
        svg {
            class: "met-icon {props.class}",
            width: "{px}",
            height: "{px}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{props.color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "{props.icon.svg_path()}" }
        }
    }
}
