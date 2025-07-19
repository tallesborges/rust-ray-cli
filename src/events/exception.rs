use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::process_exception_event;
use crate::events::types::ProcessedEvent;
use crate::ui_components::{border_color, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "Exception".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
        event_type: "exception".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Process using the new architecture
        let processed_event = process_exception_event(content)?;

        // Set labels and descriptions based on processed event
        if let ProcessedEvent::Exception(ref exception_event) = processed_event {
            entry.label = "Exception".to_string();
            let description = if !exception_event.message.is_empty() {
                format!("{}: {}", exception_event.class, exception_event.message)
            } else {
                exception_event.class.clone()
            };

            // Truncate long descriptions
            if description.len() > 100 {
                entry.description = format!("{}...", &description[..97]);
            } else {
                entry.description = description;
            }
        } else {
            return Err(anyhow::anyhow!(
                "Unexpected event type from exception processor"
            ));
        }
    }

    Ok(entry)
}

pub fn render_exception_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    let content = entry
        .raw_payload
        .get("content")
        .cloned()
        .unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(render_exception_details(&content))
        .child(render_stack_trace(&content))
        .child(render_origin_info(entry))
}

// Header removed for minimal design

fn render_exception_details(content: &Value) -> Div {
    let class = content
        .get("class")
        .and_then(|c| c.as_str())
        .unwrap_or("Exception");
    let message = content
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("");

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_sm()
                .text_color(text_primary_color())
                .child(if !message.is_empty() {
                    format!("{}: {}", class, message)
                } else {
                    class.to_string()
                }),
        )
}

fn render_stack_trace(content: &Value) -> Div {
    if let Some(frames) = content.get("frames").and_then(|f| f.as_array()) {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_xs()
                    .text_color(text_secondary_color())
                    .opacity(0.7)
                    .child(format!("{} frames", frames.len())),
            )
            .child(render_frames(frames))
    } else {
        div() // Empty div if no frames
    }
}

fn render_frames(frames: &[Value]) -> Div {
    let mut container = div().flex().flex_col().gap_2().max_h_96().overflow_hidden();

    for (index, frame) in frames.iter().enumerate() {
        container = container.child(render_single_frame(index, frame));
    }

    container
}

fn render_single_frame(index: usize, frame: &Value) -> Div {
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

    div()
        .flex()
        .flex_row()
        .gap_3()
        .items_start()
        .py_2()
        .child(
            div()
                .text_xs()
                .text_color(text_secondary_color())
                .opacity(0.5)
                .w_4()
                .child(format!("{}", index + 1)),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .text_color(text_primary_color())
                        .child(format!("{}::{}", class, method)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .opacity(0.7)
                        .child(format!("{}:{}", file, line)),
                ),
        )
}

// Code snippets removed for minimal design

fn render_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(|l| l.as_u64())
            .unwrap_or(0);
        let hostname = origin
            .get("hostname")
            .and_then(|h| h.as_str())
            .unwrap_or("");

        if !file.is_empty() {
            div()
                .pt_4()
                .border_t_1()
                .border_color(border_color())
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .opacity(0.7)
                        .child(format!("{}:{} • {}", file, line, hostname)),
                )
        } else {
            div() // Empty div if no origin info
        }
    } else {
        div() // Empty div if no origin
    }
}
