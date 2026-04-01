//! Markdown renderer component for Dioxus.
//!
//! Parses a subset of Markdown and emits themed Dioxus elements.
//! Supports headings, paragraphs, bold, italic, inline code, code blocks,
//! links, blockquotes, bullet/numbered lists, horizontal rules, and images.
//!
//! # Usage
//!
//! ```rust,ignore
//! use met_markdown::Markdown;
//!
//! rsx! {
//!     Markdown { content: "# Hello\n\nSome **bold** and *italic* text." }
//! }
//! ```

pub mod parse;
pub mod render;

pub use render::Markdown;
pub use parse::{Block, Inline, parse_markdown};
