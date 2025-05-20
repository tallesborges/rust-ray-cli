use pulldown_cmark::{Event, Parser, Tag, CodeBlockKind};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActiveStyle {
    Bold,
    Italic,
    // Code, // Inline code is handled as a single event
}

pub fn parse_markdown<'a>(markdown_input: &'a str) -> Vec<Line<'a>> {
    let parser = Parser::new(markdown_input);
    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut current_spans: Vec<Span<'a>> = Vec::new();
    let mut style_stack: Vec<ActiveStyle> = Vec::new();
    let mut in_code_block = false;
    let code_block_style = Style::default().fg(Color::Cyan); // Style for entire code blocks
    let inline_code_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {
                        // Start a new line if current_spans is not empty from a previous non-paragraph element
                        if !current_spans.is_empty() {
                            lines.push(Line::from(std::mem::take(&mut current_spans)));
                        }
                    }
                    Tag::CodeBlock(kind) => {
                        if !current_spans.is_empty() {
                            lines.push(Line::from(std::mem::take(&mut current_spans)));
                        }
                        in_code_block = true;
                        // Optional: Add a hint for the language if available
                        if let CodeBlockKind::Fenced(lang) = kind {
                            if !lang.is_empty() {
                                current_spans.push(Span::styled(
                                    format!("[{}]", lang),
                                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                                ));
                                lines.push(Line::from(std::mem::take(&mut current_spans)));
                            }
                        }
                    }
                    Tag::Strong => {
                        style_stack.push(ActiveStyle::Bold);
                    }
                    Tag::Emphasis => {
                        style_stack.push(ActiveStyle::Italic);
                    }
                    // Tag::Code => style_stack.push(ActiveStyle::Code), // Inline Code is handled by Event::Code
                    _ => {} // Handle other tags as needed (e.g., Headers, Lists)
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::Paragraph => {
                        if !current_spans.is_empty() {
                            lines.push(Line::from(std::mem::take(&mut current_spans)));
                        }
                    }
                    Tag::CodeBlock(_) => {
                        if !current_spans.is_empty() { // Add any remaining content from the last line of code block
                            lines.push(Line::from(std::mem::take(&mut current_spans)));
                        }
                        in_code_block = false;
                    }
                    Tag::Strong => {
                        style_stack.retain(|&s| s != ActiveStyle::Bold);
                    }
                    Tag::Emphasis => {
                        style_stack.retain(|&s| s != ActiveStyle::Italic);
                    }
                    // Tag::Code => style_stack.retain(|&s| s != ActiveStyle::Code),
                    _ => {}
                }
            }
            Event::Text(text) => {
                let mut current_style = if in_code_block {
                    code_block_style
                } else {
                    Style::default()
                };

                for active_style in &style_stack {
                    match active_style {
                        ActiveStyle::Bold => current_style = current_style.add_modifier(Modifier::BOLD),
                        ActiveStyle::Italic => current_style = current_style.add_modifier(Modifier::ITALIC),
                        // ActiveStyle::Code => current_style = current_style.patch(inline_code_style), // Covered by Event::Code
                    }
                }
                current_spans.push(Span::styled(text.into_owned(), current_style));
            }
            Event::Code(text) => { // Inline code
                current_spans.push(Span::styled(text.into_owned(), inline_code_style));
            }
            Event::HardBreak | Event::SoftBreak => {
                if !current_spans.is_empty() || lines.last().map_or(false, |l| !l.spans.is_empty()) || event == Event::HardBreak {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
                 if event == Event::SoftBreak && in_code_block { // Preserve soft breaks as new lines in code blocks
                    // This is a simplification; pulldown-cmark might give text with newlines directly.
                    // If current_spans was empty due to previous SoftBreak, ensure we add an empty line.
                    // The check above handles non-empty current_spans. If it was empty, this does nothing.
                    // If we want to guarantee a new line for every soft break in code block:
                    // lines.push(Line::raw("")); // or Line::from(vec![]);
                }
            }
            Event::Rule => { // Horizontal Rule
                if !current_spans.is_empty() {
                     lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
                lines.push(Line::styled("---", Style::default().fg(Color::DarkGray)));
            }
            Event::TaskListMarker(checked) => {
                 current_spans.push(Span::raw(if checked { "[x] " } else { "[ ] " }));
            }
            // For now, other events like Start/End of Header, List, etc., will have their Text content processed.
            // More sophisticated rendering can be added later.
            _ => {}
        }
    }
    // Add any remaining spans as the last line
    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::{Color, Modifier, Style};

    #[test]
    fn test_parse_simple_paragraph() {
        let md = "Hello, world!";
        let lines = parse_markdown(md);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].spans.len(), 1);
        assert_eq!(lines[0].spans[0].content, "Hello, world!");
        assert_eq!(lines[0].spans[0].style, Style::default());
    }

    #[test]
    fn test_parse_bold_italic() {
        let md = "This is **bold** and *italic*.";
        let lines = parse_markdown(md);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].spans.len(), 4); // "This is ", "bold", " and ", "italic."
        assert_eq!(lines[0].spans[0].content, "This is ");
        assert_eq!(lines[0].spans[1].content, "bold");
        assert_eq!(lines[0].spans[1].style, Style::default().add_modifier(Modifier::BOLD));
        assert_eq!(lines[0].spans[2].content, " and ");
        assert_eq!(lines[0].spans[3].content, "italic.");
        assert_eq!(lines[0].spans[3].style, Style::default().add_modifier(Modifier::ITALIC));
    }
    
    #[test]
    fn test_parse_bold_within_italic() {
        let md = "*This is **bold and italic** text.*";
        let lines = parse_markdown(md);
        assert_eq!(lines.len(), 1);
        // Expected: "This is ", "bold and italic", " text." (all italic, middle one also bold)
        assert_eq!(lines[0].spans.len(), 3);
        assert_eq!(lines[0].spans[0].content, "This is ");
        assert_eq!(lines[0].spans[0].style, Style::default().add_modifier(Modifier::ITALIC));
        
        assert_eq!(lines[0].spans[1].content, "bold and italic");
        assert_eq!(lines[0].spans[1].style, Style::default().add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD));
        
        assert_eq!(lines[0].spans[2].content, " text.");
        assert_eq!(lines[0].spans[2].style, Style::default().add_modifier(Modifier::ITALIC));
    }


    #[test]
    fn test_inline_code() {
        let md = "Use `my_function()` for this.";
        let lines = parse_markdown(md);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].spans.len(), 3); // "Use ", "my_function()", " for this."
        assert_eq!(lines[0].spans[1].content, "my_function()");
        assert_eq!(lines[0].spans[1].style, Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC));
    }

    #[test]
    fn test_code_block() {
        let md = "```rust\nlet x = 10;\n```";
        let lines = parse_markdown(md);
        // Expected: "[rust]", "let x = 10;"
        assert_eq!(lines.len(), 2); 
        assert_eq!(lines[0].spans.len(), 1);
        assert_eq!(lines[0].spans[0].content, "[rust]");
        assert_eq!(lines[0].spans[0].style, Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC));

        assert_eq!(lines[1].spans.len(), 1);
        assert_eq!(lines[1].spans[0].content, "let x = 10;");
        assert_eq!(lines[1].spans[0].style, Style::default().fg(Color::Cyan));
    }

    #[test]
    fn test_code_block_no_lang() {
        let md = "```\nHello\nWorld\n```";
        let lines = parse_markdown(md);
        assert_eq!(lines.len(), 2); // "Hello", "World"
        assert_eq!(lines[0].spans[0].content, "Hello");
        assert_eq!(lines[0].spans[0].style, Style::default().fg(Color::Cyan));
        assert_eq!(lines[1].spans[0].content, "World");
        assert_eq!(lines[1].spans[0].style, Style::default().fg(Color::Cyan));
    }


    #[test]
    fn test_hard_break() {
        let md = "Line one\n\nLine two"; // This is paragraph break, not hard break
        let lines_p = parse_markdown(md);
        assert_eq!(lines_p.len(), 2); // Two paragraphs
        assert_eq!(lines_p[0].spans[0].content, "Line one");
        assert_eq!(lines_p[1].spans[0].content, "Line two");

        let md_hard = "Line one  \nLine two"; // This is a hard break
        let lines_h = parse_markdown(md_hard);
        assert_eq!(lines_h.len(), 2);
        assert_eq!(lines_h[0].spans[0].content, "Line one");
        assert_eq!(lines_h[1].spans[0].content, "Line two");
    }
    
    #[test]
    fn test_multiple_paragraphs() {
        let md = "First paragraph.\n\nSecond paragraph.";
        let lines = parse_markdown(md);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].spans[0].content, "First paragraph.");
        assert_eq!(lines[1].spans[0].content, "Second paragraph.");
    }
    
    #[test]
    fn test_empty_input() {
        let md = "";
        let lines = parse_markdown(md);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_only_spaces_and_newlines() {
        let md = "  \n\n   \n ";
        let lines = parse_markdown(md);
        // The parser might produce empty text events or nothing.
        // Current behavior: it produces lines with spaces, which is acceptable.
        // If specific trimming is needed, it would be an enhancement.
        // For now, let's check it doesn't panic and produces something.
        // Example: if it produces one line with "   " and another with " "
        // This test is more about ensuring stability than exact output for whitespace-only.
        if !lines.is_empty() {
            for line in lines {
                for span in line.spans {
                    assert!(span.content.trim().is_empty() || span.content.is_empty());
                }
            }
        } else {
            // Or it might produce no lines, which is also fine.
             assert!(lines.is_empty());
        }
    }
}
