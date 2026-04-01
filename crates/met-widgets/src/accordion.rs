//! Accordion (collapsible sections) wrapping `dioxus_primitives::accordion`.

use dioxus::prelude::*;

pub use dioxus_primitives::accordion::{
    Accordion as PrimitiveAccordion, AccordionContent, AccordionItem, AccordionTrigger,
};

/// Data for a single accordion section.
#[derive(Clone, PartialEq)]
pub struct AccordionSection {
    pub id: String,
    pub title: String,
    pub content: Element,
    pub icon: Option<String>,
    pub disabled: bool,
}

impl AccordionSection {
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: Element) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content,
            icon: None,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Sections to render
    pub items: Vec<AccordionSection>,
    /// Allow multiple sections open at once
    #[props(default)]
    pub allow_multiple: bool,
    /// IDs of sections that start open
    #[props(default)]
    pub default_open: Vec<String>,
    /// Allow collapsing all sections (default true)
    #[props(default = true)]
    pub collapsible: bool,
    #[props(default)]
    pub class: String,
}

/// Themed accordion with collapsible sections.
///
/// ```rust,ignore
/// Accordion {
///     items: vec![
///         AccordionSection::new("s1", "Settings", rsx! { p { "Content here" } }),
///         AccordionSection::new("s2", "Advanced", rsx! { p { "More stuff" } }),
///     ],
///     default_open: vec!["s1".into()],
/// }
/// ```
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        PrimitiveAccordion {
            allow_multiple_open: props.allow_multiple,
            collapsible: props.collapsible,
            class: "met-accordion {props.class}",

            for (index, item) in props.items.iter().enumerate() {
                {
                    let is_default_open = props.default_open.contains(&item.id);
                    let has_icon = item.icon.clone();
                    let title = item.title.clone();
                    let content = item.content.clone();
                    rsx! {
                        AccordionItem {
                            index: index,
                            disabled: item.disabled,
                            default_open: is_default_open,
                            class: "met-accordion-item",

                            AccordionTrigger {
                                class: "met-accordion-trigger",
                                div { class: "met-accordion-trigger-content",
                                    span { class: "met-accordion-chevron", "▶" }
                                    if let Some(icon) = has_icon {
                                        span { class: "met-accordion-icon", {icon} }
                                    }
                                    span { class: "met-accordion-title", {title} }
                                }
                            }

                            AccordionContent {
                                class: "met-accordion-content",
                                {content}
                            }
                        }
                    }
                }
            }
        }
    }
}
