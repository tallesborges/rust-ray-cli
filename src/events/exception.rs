use anyhow::Result;
use serde_json::Value;
use crate::events::base::{EventEntry, EventProcessor, extract_timestamp, extract_origin_info};
use crate::ui_components::{
    background_color, border_color, styled_card, text_monospace_color, text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, Context, Div};

pub struct ExceptionProcessor;

impl EventProcessor for ExceptionProcessor {
    fn process(&self, payload: &Value) -> Result<EventEntry> {
        let mut entry = EventEntry {
            timestamp: extract_timestamp(payload),
            label: "Exception".to_string(),
            description: String::new(),
            content: String::new(),
            content_type: "markdown".to_string(),
            event_type: "exception".to_string(),
            raw_payload: payload.clone(),
        };

        if let Some(content) = payload.get("content") {
            // Get exception class and message
            let class = content
                .get("class")
                .and_then(|c| c.as_str())
                .unwrap_or("Unknown Exception");
            let message = content
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("");

            // Set description for list view
            entry.description = if !message.is_empty() {
                format!("{}: {}", class, message)
            } else {
                class.to_string()
            };

            // Truncate long descriptions
            if entry.description.len() > 100 {
                entry.description.truncate(97);
                entry.description.push_str("...");
            }

            // Start building markdown content
            let mut markdown = format!("## {}\n\n", class);

            if !message.is_empty() {
                markdown.push_str(&format!("**Error:** {}\n\n", message));
            }

            // Process stack trace if available
            if let Some(frames) = content.get("frames").and_then(|f| f.as_array()) {
                markdown.push_str("### Stack Trace\n\n");

                for (i, frame) in frames.iter().enumerate() {
                    let class = frame.get("class").and_then(|c| c.as_str()).unwrap_or("");
                    let method = frame.get("method").and_then(|m| m.as_str()).unwrap_or("");
                    let file = frame
                        .get("file_name")
                        .and_then(|f| f.as_str())
                        .unwrap_or("");
                    let line = frame
                        .get("line_number")
                        .and_then(|l| l.as_u64())
                        .unwrap_or(0);

                    markdown.push_str(&format!(
                        "{}. **{}**::**{}**() at {}:{}\n\n",
                        i + 1,
                        class,
                        method,
                        file,
                        line
                    ));

                    // Include code snippet if available
                    if let Some(snippet) = frame.get("snippet").and_then(|s| s.as_array()) {
                        markdown.push_str("```php\n");

                        for line_info in snippet {
                            let line_num = line_info
                                .get("line_number")
                                .and_then(|l| l.as_u64())
                                .unwrap_or(0);
                            let text = line_info.get("text").and_then(|t| t.as_str()).unwrap_or("");
                            let prefix = if line_num == line { "→ " } else { "  " };

                            markdown.push_str(&format!("{}{}: {}\n", prefix, line_num, text));
                        }

                        markdown.push_str("```\n\n");
                    }
                }
            }

            // Add origin information if available
            if let Some(origin) = extract_origin_info(payload) {
                markdown.push_str(&format!("**Source:** {}\n\n", origin));
            }

            entry.content = markdown;
        }

        Ok(entry)
    }

}

pub fn render_exception_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    let content = entry.raw_payload.get("content").cloned().unwrap_or_default();
    
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(render_exception_header(&content))
        .child(render_exception_details(&content))
        .child(render_stack_trace(&content))
        .child(render_origin_info(entry))
}

fn render_exception_header(content: &Value) -> Div {
    let class = content
        .get("class")
        .and_then(|c| c.as_str())
        .unwrap_or("Unknown Exception")
        .to_string();
    
    styled_card()
        .p_4()
        .child(
            div()
                .flex()
                .flex_row()
                .gap_3()
                .items_center()
                .child(
                    div()
                        .text_2xl()
                        .child("💥")
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_lg()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(gpui::rgb(0xef4444)) // Red
                                .child(class)
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(text_secondary_color())
                                .child("Exception")
                        )
                )
        )
}

fn render_exception_details(content: &Value) -> Div {
    let message = content
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("")
        .to_string();

    if !message.is_empty() {
        styled_card()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_secondary_color())
                            .child("📋 Error Message")
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(gpui::rgb(0xfef2f2)) // Light red background
                            .border_1()
                            .border_color(gpui::rgb(0xfecaca))
                            .rounded_lg()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(gpui::rgb(0x991b1b)) // Dark red text
                                    .child(message)
                            )
                    )
            )
    } else {
        div() // Empty div if no message
    }
}

fn render_stack_trace(content: &Value) -> Div {
    if let Some(frames) = content.get("frames").and_then(|f| f.as_array()) {
        styled_card()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(text_secondary_color())
                                    .child("🔍 Stack Trace")
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(gpui::rgb(0x3b82f6))
                                    .text_color(gpui::white())
                                    .text_xs()
                                    .rounded_lg()
                                    .child(format!("{} frames", frames.len()))
                            )
                    )
                    .child(render_frames(frames))
            )
    } else {
        div() // Empty div if no frames
    }
}

fn render_frames(frames: &[Value]) -> Div {
    let mut container = div()
        .flex()
        .flex_col()
        .gap_2()
        .max_h_96()
        .overflow_hidden();

    for (index, frame) in frames.iter().enumerate() {
        container = container.child(render_single_frame(index, frame));
    }

    container
}

fn render_single_frame(index: usize, frame: &Value) -> Div {
    let class = frame.get("class").and_then(|c| c.as_str()).unwrap_or("");
    let method = frame.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let file = frame.get("file_name").and_then(|f| f.as_str()).unwrap_or("");
    let line = frame.get("line_number").and_then(|l| l.as_u64()).unwrap_or(0);

    div()
        .flex()
        .flex_col()
        .gap_2()
        .p_3()
        .bg(background_color())
        .border_1()
        .border_color(border_color())
        .rounded_lg()
        .child(
            // Frame header
            div()
                .flex()
                .flex_row()
                .gap_3()
                .items_center()
                .child(
                    // Frame number
                    div()
                        .w_8()
                        .h_8()
                        .rounded_full()
                        .bg(if index == 0 { gpui::rgb(0xef4444) } else { gpui::rgb(0x6b7280) })
                        .text_color(gpui::white())
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child((index + 1).to_string())
                )
                .child(
                    // Method info
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(text_monospace_color())
                                .child(format!("{}::{}", class, method))
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_secondary_color())
                                .child(format!("{}:{}", file, line))
                        )
                )
        )
        .child(render_code_snippet(frame, line))
}

fn render_code_snippet(frame: &Value, current_line: u64) -> Div {
    if let Some(snippet) = frame.get("snippet").and_then(|s| s.as_array()) {
        div()
            .mt_2()
            .p_3()
            .bg(gpui::rgb(0x1f2937)) // Dark background for code
            .rounded_lg()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_secondary_color())
                            .mb_2()
                            .child("📝 Code Context")
                    )
                    .child(render_snippet_lines(snippet, current_line))
            )
    } else {
        div() // Empty div if no snippet
    }
}

fn render_snippet_lines(snippet: &[Value], current_line: u64) -> Div {
    let mut container = div().font_family("monospace").text_xs();

    for line_info in snippet {
        let line_num = line_info
            .get("line_number")
            .and_then(|l| l.as_u64())
            .unwrap_or(0);
        let text = line_info.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string();
        
        let is_current = line_num == current_line;
        
        container = container.child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .py_1()
                .px_2()
                .rounded_lg()
                .bg(if is_current { gpui::rgb(0x7c2d12) } else { gpui::rgb(0x000000) })
                .child(
                    // Line number
                    div()
                        .w_12()
                        .text_right()
                        .text_color(if is_current { gpui::rgb(0xfca5a5) } else { gpui::rgb(0x6b7280) })
                        .font_weight(if is_current { gpui::FontWeight::BOLD } else { gpui::FontWeight::NORMAL })
                        .child(format!("{}", line_num))
                )
                .child(
                    // Arrow for current line
                    div()
                        .w_4()
                        .text_color(gpui::rgb(0xef4444))
                        .font_weight(gpui::FontWeight::BOLD)
                        .child(if is_current { "→" } else { " " })
                )
                .child(
                    // Code text
                    div()
                        .flex_1()
                        .text_color(if is_current { gpui::rgb(0xfef2f2) } else { gpui::rgb(0xd1d5db) })
                        .child(text)
                )
        );
    }

    container
}

fn render_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin.get("line_number").and_then(|l| l.as_u64()).unwrap_or(0);
        let hostname = origin.get("hostname").and_then(|h| h.as_str()).unwrap_or("");

        if !file.is_empty() {
            styled_card()
                .p_3()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(text_secondary_color())
                                .child("🔍 Source")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_monospace_color())
                                .child(format!("{}:{} on {}", file, line, hostname))
                        )
                )
        } else {
            div() // Empty div if no origin info
        }
    } else {
        div() // Empty div if no origin
    }
}