//! Markdown Dioxus component — renders parsed blocks as themed HTML.

use dioxus::prelude::*;

use crate::parse::{Block, Inline, parse_markdown};

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    /// Raw markdown string to render.
    pub content: String,
    /// Extra CSS class on the root element.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let blocks = use_memo({
        let content = props.content.clone();
        move || parse_markdown(&content)
    });

    let class = format!("met-markdown {}", props.class);

    rsx! {
        div { class: "{class}",
            for block in blocks.read().iter() {
                { render_block(block) }
            }
        }
    }
}

fn render_block(block: &Block) -> Element {
    match block {
        Block::Heading { level, inlines } => {
            let tag_class = format!("met-md-h{}", level);
            rsx! {
                div { class: "{tag_class}",
                    for inline in inlines.iter() {
                        { render_inline(inline) }
                    }
                }
            }
        }
        Block::Paragraph { inlines } => {
            rsx! {
                p { class: "met-md-p",
                    for inline in inlines.iter() {
                        { render_inline(inline) }
                    }
                }
            }
        }
        Block::CodeBlock { language, code } => {
            let lang_class = language
                .as_ref()
                .map(|l| format!("language-{l}"))
                .unwrap_or_default();
            let code = code.clone();
            rsx! {
                pre { class: "met-md-pre",
                    code { class: "{lang_class}", "{code}" }
                }
            }
        }
        Block::Blockquote { blocks } => {
            rsx! {
                blockquote { class: "met-md-blockquote",
                    for b in blocks.iter() {
                        { render_block(b) }
                    }
                }
            }
        }
        Block::UnorderedList { items } => {
            rsx! {
                ul { class: "met-md-ul",
                    for item in items.iter() {
                        li {
                            for inline in item.iter() {
                                { render_inline(inline) }
                            }
                        }
                    }
                }
            }
        }
        Block::OrderedList { items } => {
            rsx! {
                ol { class: "met-md-ol",
                    for item in items.iter() {
                        li {
                            for inline in item.iter() {
                                { render_inline(inline) }
                            }
                        }
                    }
                }
            }
        }
        Block::HorizontalRule => {
            rsx! {
                hr { class: "met-md-hr" }
            }
        }
    }
}

fn render_inline(inline: &Inline) -> Element {
    match inline {
        Inline::Text(t) => {
            let t = t.clone();
            rsx! { span { "{t}" } }
        }
        Inline::Bold(children) => {
            rsx! {
                strong {
                    for child in children.iter() {
                        { render_inline(child) }
                    }
                }
            }
        }
        Inline::Italic(children) => {
            rsx! {
                em {
                    for child in children.iter() {
                        { render_inline(child) }
                    }
                }
            }
        }
        Inline::BoldItalic(children) => {
            rsx! {
                strong {
                    em {
                        for child in children.iter() {
                            { render_inline(child) }
                        }
                    }
                }
            }
        }
        Inline::Code(code) => {
            let code = code.clone();
            rsx! { code { class: "met-md-code", "{code}" } }
        }
        Inline::Link { text, url } => {
            let text = text.clone();
            let url = url.clone();
            rsx! {
                a { class: "met-md-link", href: "{url}", target: "_blank", rel: "noopener", "{text}" }
            }
        }
        Inline::Image { alt, url } => {
            let alt = alt.clone();
            let url = url.clone();
            rsx! {
                img { class: "met-md-img", src: "{url}", alt: "{alt}" }
            }
        }
    }
}
