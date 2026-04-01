//! Markdown parser — converts markdown text into a block/inline AST.

/// Block-level markdown element.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Heading { level: u8, inlines: Vec<Inline> },
    Paragraph { inlines: Vec<Inline> },
    CodeBlock { language: Option<String>, code: String },
    Blockquote { blocks: Vec<Block> },
    UnorderedList { items: Vec<Vec<Inline>> },
    OrderedList { items: Vec<Vec<Inline>> },
    HorizontalRule,
}

/// Inline markdown element.
#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    BoldItalic(Vec<Inline>),
    Code(String),
    Link { text: String, url: String },
    Image { alt: String, url: String },
}

/// Parse markdown text into a list of blocks.
pub fn parse_markdown(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Blank line — skip.
        if line.trim().is_empty() {
            i += 1;
            continue;
        }

        // Horizontal rule.
        let trimmed = line.trim();
        if (trimmed.starts_with("---") || trimmed.starts_with("***") || trimmed.starts_with("___"))
            && trimmed.chars().all(|c| c == '-' || c == '*' || c == '_' || c == ' ')
            && trimmed.len() >= 3
        {
            blocks.push(Block::HorizontalRule);
            i += 1;
            continue;
        }

        // Heading.
        if let Some(rest) = try_heading(line) {
            blocks.push(rest);
            i += 1;
            continue;
        }

        // Fenced code block.
        if trimmed.starts_with("```") {
            let lang = trimmed[3..].trim();
            let language = if lang.is_empty() { None } else { Some(lang.to_string()) };
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                if lines[i].trim().starts_with("```") {
                    i += 1;
                    break;
                }
                code_lines.push(lines[i]);
                i += 1;
            }
            blocks.push(Block::CodeBlock {
                language,
                code: code_lines.join("\n"),
            });
            continue;
        }

        // Blockquote.
        if trimmed.starts_with("> ") || trimmed == ">" {
            let mut quote_lines = Vec::new();
            while i < lines.len() {
                let l = lines[i].trim();
                if l.starts_with("> ") {
                    quote_lines.push(&l[2..]);
                } else if l == ">" {
                    quote_lines.push("");
                } else {
                    break;
                }
                i += 1;
            }
            let inner = parse_markdown(&quote_lines.join("\n"));
            blocks.push(Block::Blockquote { blocks: inner });
            continue;
        }

        // Unordered list.
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            let mut items = Vec::new();
            while i < lines.len() {
                let l = lines[i].trim();
                if l.starts_with("- ") || l.starts_with("* ") || l.starts_with("+ ") {
                    items.push(parse_inlines(&l[2..]));
                    i += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::UnorderedList { items });
            continue;
        }

        // Ordered list.
        if is_ordered_list_item(trimmed) {
            let mut items = Vec::new();
            while i < lines.len() {
                let l = lines[i].trim();
                if let Some(rest) = strip_ordered_prefix(l) {
                    items.push(parse_inlines(rest));
                    i += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::OrderedList { items });
            continue;
        }

        // Paragraph (collect contiguous non-blank, non-special lines).
        let mut para_text = String::new();
        while i < lines.len() {
            let l = lines[i];
            let lt = l.trim();
            if lt.is_empty()
                || try_heading(l).is_some()
                || lt.starts_with("```")
                || lt.starts_with("> ")
                || lt.starts_with("- ")
                || lt.starts_with("* ")
                || lt.starts_with("+ ")
                || is_ordered_list_item(lt)
                || is_horizontal_rule(lt)
            {
                break;
            }
            if !para_text.is_empty() {
                para_text.push(' ');
            }
            para_text.push_str(l);
            i += 1;
        }
        if !para_text.is_empty() {
            blocks.push(Block::Paragraph {
                inlines: parse_inlines(&para_text),
            });
        }
    }

    blocks
}

fn try_heading(line: &str) -> Option<Block> {
    let trimmed = line.trim_start();
    let level = trimmed.bytes().take_while(|&b| b == b'#').count();
    if level >= 1 && level <= 6 && trimmed.as_bytes().get(level) == Some(&b' ') {
        let text = &trimmed[level + 1..];
        Some(Block::Heading {
            level: level as u8,
            inlines: parse_inlines(text),
        })
    } else {
        None
    }
}

fn is_horizontal_rule(s: &str) -> bool {
    (s.starts_with("---") || s.starts_with("***") || s.starts_with("___"))
        && s.chars().all(|c| c == '-' || c == '*' || c == '_' || c == ' ')
        && s.len() >= 3
}

fn is_ordered_list_item(s: &str) -> bool {
    strip_ordered_prefix(s).is_some()
}

fn strip_ordered_prefix(s: &str) -> Option<&str> {
    let dot_pos = s.find(". ")?;
    let prefix = &s[..dot_pos];
    if prefix.chars().all(|c| c.is_ascii_digit()) && !prefix.is_empty() {
        Some(&s[dot_pos + 2..])
    } else {
        None
    }
}

/// Parse inline markdown elements from a text fragment.
pub fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut result = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut buf = String::new();

    while i < chars.len() {
        // Image: ![alt](url)
        if chars[i] == '!' && i + 1 < chars.len() && chars[i + 1] == '[' {
            if let Some((alt, url, end)) = try_link_or_image(&chars, i + 1) {
                flush_text(&mut buf, &mut result);
                result.push(Inline::Image { alt, url });
                i = end;
                continue;
            }
        }

        // Link: [text](url)
        if chars[i] == '[' {
            if let Some((text, url, end)) = try_link_or_image(&chars, i) {
                flush_text(&mut buf, &mut result);
                result.push(Inline::Link { text, url });
                i = end;
                continue;
            }
        }

        // Inline code: `...`
        if chars[i] == '`' {
            if let Some((code, end)) = try_backtick(&chars, i) {
                flush_text(&mut buf, &mut result);
                result.push(Inline::Code(code));
                i = end;
                continue;
            }
        }

        // Bold-italic: ***...*** or ___...___
        if i + 2 < chars.len()
            && chars[i] == chars[i + 1]
            && chars[i] == chars[i + 2]
            && (chars[i] == '*' || chars[i] == '_')
        {
            let marker = chars[i];
            if let Some((inner, end)) = try_delimited(&chars, i + 3, marker, 3) {
                flush_text(&mut buf, &mut result);
                result.push(Inline::BoldItalic(parse_inlines(&inner)));
                i = end;
                continue;
            }
        }

        // Bold: **...** or __...__
        if i + 1 < chars.len()
            && chars[i] == chars[i + 1]
            && (chars[i] == '*' || chars[i] == '_')
        {
            let marker = chars[i];
            if let Some((inner, end)) = try_delimited(&chars, i + 2, marker, 2) {
                flush_text(&mut buf, &mut result);
                result.push(Inline::Bold(parse_inlines(&inner)));
                i = end;
                continue;
            }
        }

        // Italic: *...* or _..._
        if chars[i] == '*' || chars[i] == '_' {
            let marker = chars[i];
            if let Some((inner, end)) = try_delimited(&chars, i + 1, marker, 1) {
                flush_text(&mut buf, &mut result);
                result.push(Inline::Italic(parse_inlines(&inner)));
                i = end;
                continue;
            }
        }

        buf.push(chars[i]);
        i += 1;
    }

    flush_text(&mut buf, &mut result);
    result
}

fn flush_text(buf: &mut String, result: &mut Vec<Inline>) {
    if !buf.is_empty() {
        result.push(Inline::Text(buf.clone()));
        buf.clear();
    }
}

/// Try to parse `count` repetitions of `marker` as a closing delimiter,
/// starting at `start` in `chars`. Returns (inner text, index after closing delimiter).
fn try_delimited(chars: &[char], start: usize, marker: char, count: usize) -> Option<(String, usize)> {
    let mut j = start;
    while j + count <= chars.len() {
        let all_match = (0..count).all(|k| chars[j + k] == marker);
        if all_match {
            let inner: String = chars[start..j].iter().collect();
            if !inner.is_empty() {
                return Some((inner, j + count));
            }
        }
        j += 1;
    }
    None
}

fn try_backtick(chars: &[char], start: usize) -> Option<(String, usize)> {
    let mut j = start + 1;
    while j < chars.len() {
        if chars[j] == '`' {
            let code: String = chars[start + 1..j].iter().collect();
            return Some((code, j + 1));
        }
        j += 1;
    }
    None
}

fn try_link_or_image(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    // start should point at '['
    if chars[start] != '[' {
        return None;
    }
    let mut j = start + 1;
    // Find closing ]
    while j < chars.len() && chars[j] != ']' {
        j += 1;
    }
    if j >= chars.len() {
        return None;
    }
    let text: String = chars[start + 1..j].iter().collect();
    j += 1; // skip ]
    if j >= chars.len() || chars[j] != '(' {
        return None;
    }
    j += 1; // skip (
    let url_start = j;
    while j < chars.len() && chars[j] != ')' {
        j += 1;
    }
    if j >= chars.len() {
        return None;
    }
    let url: String = chars[url_start..j].iter().collect();
    Some((text, url, j + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_levels() {
        let blocks = parse_markdown("# H1\n## H2\n### H3");
        assert_eq!(blocks.len(), 3);
        match &blocks[0] {
            Block::Heading { level, .. } => assert_eq!(*level, 1),
            _ => panic!("expected heading"),
        }
    }

    #[test]
    fn code_block() {
        let blocks = parse_markdown("```rust\nfn main() {}\n```");
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            Block::CodeBlock { language, code } => {
                assert_eq!(language.as_deref(), Some("rust"));
                assert_eq!(code, "fn main() {}");
            }
            _ => panic!("expected code block"),
        }
    }

    #[test]
    fn bold_italic_inline() {
        let inlines = parse_inlines("hello **bold** and *italic*");
        assert_eq!(inlines.len(), 4);
        assert!(matches!(&inlines[1], Inline::Bold(..)));
        assert!(matches!(&inlines[3], Inline::Italic(..)));
    }

    #[test]
    fn inline_code() {
        let inlines = parse_inlines("use `foo::bar` here");
        assert_eq!(inlines.len(), 3);
        assert!(matches!(&inlines[1], Inline::Code(s) if s == "foo::bar"));
    }

    #[test]
    fn link() {
        let inlines = parse_inlines("see [docs](https://example.com) ok");
        assert!(matches!(&inlines[1], Inline::Link { text, url } if text == "docs" && url == "https://example.com"));
    }

    #[test]
    fn image() {
        let inlines = parse_inlines("![alt](img.png)");
        assert!(matches!(&inlines[0], Inline::Image { alt, url } if alt == "alt" && url == "img.png"));
    }

    #[test]
    fn unordered_list() {
        let blocks = parse_markdown("- one\n- two\n- three");
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            Block::UnorderedList { items } => assert_eq!(items.len(), 3),
            _ => panic!("expected list"),
        }
    }

    #[test]
    fn ordered_list() {
        let blocks = parse_markdown("1. first\n2. second");
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            Block::OrderedList { items } => assert_eq!(items.len(), 2),
            _ => panic!("expected ordered list"),
        }
    }

    #[test]
    fn blockquote() {
        let blocks = parse_markdown("> quoted text");
        assert_eq!(blocks.len(), 1);
        assert!(matches!(&blocks[0], Block::Blockquote { .. }));
    }

    #[test]
    fn horizontal_rule() {
        let blocks = parse_markdown("---");
        assert_eq!(blocks.len(), 1);
        assert!(matches!(&blocks[0], Block::HorizontalRule));
    }

    #[test]
    fn paragraph() {
        let blocks = parse_markdown("hello world");
        assert_eq!(blocks.len(), 1);
        assert!(matches!(&blocks[0], Block::Paragraph { .. }));
    }
}
