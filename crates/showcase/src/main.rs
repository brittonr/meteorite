use dioxus::prelude::*;
use met_core::{Size, Theme, ThemeProvider, Variant, use_theme_signal};
use met_layout::{HStack, VStack};
use met_overlay::{
    context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger},
    dialog::{Dialog, DialogContent, DialogDescription, DialogTitle},
    popover::{Popover, PopoverContent, PopoverTrigger},
    toast::{ToastProvider, ToastType, use_toast},
    tooltip::{Tooltip, TooltipContent, TooltipTrigger},
};
use met_widgets::{
    Alert, Badge, Button, Card, CardBody, CardFooter, CardHeader, Checkbox, CheckboxState,
    ContentLoader, Divider, FormField, Icon, IconName, InputType, Loader, LoaderType, Progress,
    Select, Slider, Spinner, Switch, Tabs, ValidationState,
    form::{FormCheckbox, FormGroup, FormInput, FormLabel},
    loader::SkeletonLoader,
    select::{SelectList, SelectOption, SelectTrigger, SelectValue},
    tabs::{TabContent, TabList, TabTrigger},
};

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::dark(),
            ToastProvider {
                Showcase {}
            }
        }
    }
}

// ── Main showcase ───────────────────────────────────────────────────

#[component]
fn Showcase() -> Element {
    let mut theme_sig = use_theme_signal();
    let is_dark = theme_sig.read().name == "dark";

    rsx! {
        div {
            style: "background: var(--met-bg); color: var(--met-fg); min-height: 100vh; padding: var(--met-space-lg); max-width: 960px; margin: 0 auto;",

            h1 { style: "margin-bottom: var(--met-space-lg);", "☄️ meteorite showcase" }

            // Theme toggle
            HStack {
                Button {
                    variant: Variant::Secondary,
                    onclick: move |_| {
                        let next = if is_dark { Theme::light() } else { Theme::dark() };
                        theme_sig.set(next);
                    },
                    if is_dark { "☀️ Light mode" } else { "🌙 Dark mode" }
                }
            }

            SectionDivider { title: "Buttons" }
            ButtonDemo {}

            SectionDivider { title: "Badges" }
            BadgeDemo {}

            SectionDivider { title: "Alerts" }
            AlertDemo {}

            SectionDivider { title: "Cards" }
            CardDemo {}

            SectionDivider { title: "Icons" }
            IconDemo {}

            SectionDivider { title: "Form Controls" }
            FormControlsDemo {}

            SectionDivider { title: "Form Field" }
            FormFieldDemo {}

            SectionDivider { title: "Checkbox & Switch" }
            CheckboxSwitchDemo {}

            SectionDivider { title: "Slider" }
            SliderDemo {}

            SectionDivider { title: "Select" }
            SelectDemo {}

            SectionDivider { title: "Progress" }
            ProgressDemo {}

            SectionDivider { title: "Tabs" }
            TabsDemo {}

            SectionDivider { title: "Spinner & Loader" }
            SpinnerLoaderDemo {}

            SectionDivider { title: "Tooltip" }
            TooltipDemo {}

            SectionDivider { title: "Dialog" }
            DialogDemo {}

            SectionDivider { title: "Popover" }
            PopoverDemo {}

            SectionDivider { title: "Context Menu" }
            ContextMenuDemo {}

            SectionDivider { title: "Toast" }
            ToastDemo {}

            SectionDivider { title: "Divider" }
            DividerDemo {}

            div { style: "height: var(--met-space-xl);" }
        }
    }
}

// ── Section divider helper ──────────────────────────────────────────

#[component]
fn SectionDivider(title: String) -> Element {
    rsx! {
        div { style: "margin-top: var(--met-space-xl);",
            Divider { text: title }
        }
    }
}

// ── Demos ───────────────────────────────────────────────────────────

#[component]
fn ButtonDemo() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        VStack {
            HStack {
                Button { onclick: move |_| count += 1, "Clicked {count} times" }
                Button { variant: Variant::Primary, "Primary" }
                Button { variant: Variant::Success, "Success" }
                Button { variant: Variant::Warning, "Warning" }
                Button { variant: Variant::Danger, "Danger" }
                Button { variant: Variant::Ghost, "Ghost" }
            }
            HStack {
                Button { size: Size::Sm, "Small" }
                Button { "Medium" }
                Button { size: Size::Lg, "Large" }
                Button { disabled: true, "Disabled" }
                Button { loading: true, "Loading" }
            }
        }
    }
}

#[component]
fn BadgeDemo() -> Element {
    rsx! {
        HStack {
            Badge { "Default" }
            Badge { variant: Variant::Primary, "Primary" }
            Badge { variant: Variant::Success, "Success" }
            Badge { variant: Variant::Warning, "Warning" }
            Badge { variant: Variant::Danger, "Error" }
        }
    }
}

#[component]
fn AlertDemo() -> Element {
    rsx! {
        VStack {
            Alert { variant: Variant::Success, title: "Done!".to_string(), "Operation completed." }
            Alert { variant: Variant::Warning, title: "Heads up".to_string(), "This might take a while." }
            Alert { variant: Variant::Danger, title: "Error".to_string(), dismissible: true, "Something went wrong." }
            Alert { "Informational alert with no title." }
        }
    }
}

#[component]
fn CardDemo() -> Element {
    rsx! {
        HStack { gap: "var(--met-space-md)".to_string(),
            Card {
                CardHeader { "Card Title" }
                CardBody { "Some content inside the card body." }
                CardFooter {
                    Button { size: Size::Sm, variant: Variant::Primary, "Action" }
                }
            }
            Card { variant: Variant::Primary, hoverable: true,
                CardBody { "Hoverable primary card." }
            }
            Card { padding: Size::Lg,
                CardBody { "Large padding card." }
            }
        }
    }
}

#[component]
fn IconDemo() -> Element {
    let icons = [
        IconName::Search, IconName::Settings, IconName::Home,
        IconName::Edit, IconName::Delete, IconName::Save,
        IconName::Check, IconName::Close, IconName::Download,
        IconName::Upload, IconName::Eye, IconName::Filter,
        IconName::Play, IconName::Pause, IconName::Refresh,
    ];

    rsx! {
        HStack { gap: "var(--met-space-md)".to_string(),
            for name in icons {
                Icon { icon: name }
            }
        }
        HStack { gap: "var(--met-space-md)".to_string(),
            Icon { icon: IconName::Info, size: Size::Xs }
            Icon { icon: IconName::Info, size: Size::Sm }
            Icon { icon: IconName::Info, size: Size::Md }
            Icon { icon: IconName::Info, size: Size::Lg }
            Icon { icon: IconName::Info, size: Size::Xl }
        }
    }
}

#[component]
fn FormControlsDemo() -> Element {
    let mut text = use_signal(|| "Hello".to_string());

    rsx! {
        VStack {
            FormGroup {
                FormLabel { text: "Text Input".to_string(), required: true }
                FormInput {
                    value: text(),
                    onchange: move |v: String| text.set(v),
                    placeholder: "Type something...".to_string(),
                }
            }
            FormGroup {
                FormLabel { text: "Disabled Input".to_string() }
                FormInput {
                    value: "Can't edit this".to_string(),
                    onchange: move |_: String| {},
                    disabled: true,
                }
            }
            FormGroup {
                FormLabel { text: "Checkbox".to_string() }
                FormCheckbox {
                    checked: false,
                    onchange: move |_: bool| {},
                    label: "Accept terms".to_string(),
                }
            }
        }
    }
}

#[component]
fn FormFieldDemo() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| "bad-email".to_string());

    rsx! {
        VStack {
            FormField {
                label: "Name".to_string(),
                value: name(),
                input_type: InputType::Text,
                on_change: move |v: String| name.set(v),
                required: true,
                placeholder: "Your name".to_string(),
                help_text: "Enter your full name".to_string(),
            }
            FormField {
                label: "Email".to_string(),
                value: email(),
                input_type: InputType::Email,
                on_change: move |v: String| email.set(v),
                validation: ValidationState::Error("Invalid email address".into()),
            }
            FormField {
                label: "Count".to_string(),
                value: "42".to_string(),
                input_type: InputType::Number { min: Some(0.0), max: Some(100.0), step: Some(1.0) },
                on_change: move |_: String| {},
                validation: ValidationState::Valid,
            }
        }
    }
}

#[component]
fn CheckboxSwitchDemo() -> Element {
    rsx! {
        HStack { gap: "var(--met-space-lg)".to_string(),
            VStack {
                h3 { "Checkbox" }
                Checkbox { default_checked: CheckboxState::Unchecked, label: "Unchecked".to_string() }
                Checkbox { default_checked: CheckboxState::Checked, label: "Checked".to_string() }
                Checkbox { default_checked: CheckboxState::Unchecked, label: "Disabled".to_string(), disabled: true }
            }
            VStack {
                h3 { "Switch" }
                Switch { default_checked: false, label: "Off".to_string() }
                Switch { default_checked: true, label: "On".to_string() }
                Switch { default_checked: false, label: "Disabled".to_string(), disabled: true }
            }
        }
    }
}

#[component]
fn SliderDemo() -> Element {
    rsx! {
        VStack {
            Slider { default_value: met_widgets::SliderValue::Single(40.0), min: 0.0, max: 100.0 }
            Slider { default_value: met_widgets::SliderValue::Single(75.0), min: 0.0, max: 100.0, step: 25.0 }
        }
    }
}

#[component]
fn SelectDemo() -> Element {
    rsx! {
        div { style: "max-width: 300px;",
            Select {
                default_value: Some("apple".to_string()),
                SelectTrigger { SelectValue {} }
                SelectList {
                    SelectOption::<String> { value: "apple".to_string(), index: 0usize, "Apple" }
                    SelectOption::<String> { value: "banana".to_string(), index: 1usize, "Banana" }
                    SelectOption::<String> { value: "cherry".to_string(), index: 2usize, "Cherry" }
                    SelectOption::<String> { value: "grape".to_string(), index: 3usize, "Grape" }
                }
            }
        }
    }
}

#[component]
fn ProgressDemo() -> Element {
    rsx! {
        VStack {
            Progress { value: 25.0 }
            Progress { value: 65.0 }
            Progress { value: 100.0 }
        }
    }
}

#[component]
fn TabsDemo() -> Element {
    rsx! {
        Tabs { default_value: "tab1".to_string(),
            TabList {
                TabTrigger { value: "tab1", index: 0usize, "Overview" }
                TabTrigger { value: "tab2", index: 1usize, "Details" }
                TabTrigger { value: "tab3", index: 2usize, "Settings" }
            }
            TabContent { value: "tab1", index: 0usize,
                p { "This is the overview panel." }
            }
            TabContent { value: "tab2", index: 1usize,
                p { "Here are the details." }
            }
            TabContent { value: "tab3", index: 2usize,
                p { "Settings go here." }
            }
        }
    }
}

#[component]
fn SpinnerLoaderDemo() -> Element {
    rsx! {
        VStack {
            HStack { gap: "var(--met-space-lg)".to_string(),
                Spinner { size: Size::Sm, label: "Small".to_string() }
                Spinner { label: "Medium".to_string() }
                Spinner { size: Size::Lg, label: "Large".to_string() }
            }
            HStack { gap: "var(--met-space-lg)".to_string(),
                Loader {}
                Loader { loader_type: LoaderType::Bars }
                Loader { loader_type: LoaderType::Pulse }
            }
            SkeletonLoader { lines: 3, show_avatar: true }
            ContentLoader { loading: false,
                p { "Content is loaded!" }
            }
        }
    }
}

#[component]
fn TooltipDemo() -> Element {
    rsx! {
        HStack { gap: "var(--met-space-lg)".to_string(),
            Tooltip {
                TooltipTrigger {
                    Button { "Hover me" }
                }
                TooltipContent {
                    "This is a tooltip"
                }
            }
            Tooltip {
                TooltipTrigger {
                    Badge { variant: Variant::Primary, "Info" }
                }
                TooltipContent {
                    side: dioxus_primitives::ContentSide::Bottom,
                    "Tooltip on the bottom"
                }
            }
        }
    }
}

#[component]
fn DialogDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button { variant: Variant::Primary, onclick: move |_| open.set(true), "Open Dialog" }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                DialogTitle { "Confirm Action" }
                DialogDescription { "Are you sure you want to proceed?" }
                HStack {
                    Button { variant: Variant::Ghost, onclick: move |_| open.set(false), "Cancel" }
                    Button { variant: Variant::Primary, onclick: move |_| open.set(false), "Confirm" }
                }
            }
        }
    }
}

#[component]
fn PopoverDemo() -> Element {
    rsx! {
        Popover {
            PopoverTrigger {
                Button { "Open Popover" }
            }
            PopoverContent {
                VStack {
                    h3 { style: "margin: 0;", "Popover Title" }
                    p { style: "margin: 0; color: var(--met-muted);", "Some content inside the popover." }
                }
            }
        }
    }
}

#[component]
fn ContextMenuDemo() -> Element {
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                div {
                    style: "padding: var(--met-space-lg); border: 1px dashed var(--met-border); border-radius: var(--met-radius-md); text-align: center; color: var(--met-muted);",
                    "Right-click here"
                }
            }
            ContextMenuContent {
                ContextMenuItem { value: "cut", index: 0usize, "Cut" }
                ContextMenuItem { value: "copy", index: 1usize, "Copy" }
                ContextMenuItem { value: "paste", index: 2usize, "Paste" }
            }
        }
    }
}

#[component]
fn ToastDemo() -> Element {
    let toast = use_toast();

    rsx! {
        HStack {
            Button { variant: Variant::Success,
                onclick: move |_| { toast.show("Saved".into(), ToastType::Success, Default::default()); },
                "Success Toast"
            }
            Button { variant: Variant::Danger,
                onclick: move |_| { toast.show("Failed to save".into(), ToastType::Error, Default::default()); },
                "Error Toast"
            }
            Button { variant: Variant::Warning,
                onclick: move |_| { toast.show("Heads up".into(), ToastType::Warning, Default::default()); },
                "Warning Toast"
            }
        }
    }
}

#[component]
fn DividerDemo() -> Element {
    rsx! {
        VStack {
            Divider {}
            Divider { text: "OR".to_string() }
            HStack { gap: "var(--met-space-md)".to_string(),
                span { "Left" }
                Divider { horizontal: false }
                span { "Right" }
            }
        }
    }
}
