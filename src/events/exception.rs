use crate::events::{entry::EventEntry, Event};
use crate::ui::components::{origin_info, section_header, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div};
use serde_json::Value;

pub struct ExceptionEvent;

impl Event for ExceptionEvent {
    fn process(payload: &Value) -> Result<EventEntry> {
        let content = payload
            .get("content")
            .ok_or_else(|| anyhow::anyhow!("Missing content in exception event"))?;

        let class = content
            .get("class")
            .and_then(Value::as_str)
            .unwrap_or("Exception");
        let message = content.get("message").and_then(Value::as_str).unwrap_or("");

        let description = if !message.is_empty() {
            format!("{}: {}", class, message)
        } else {
            class.to_string()
        };

        let truncated_description = if description.len() > 100 {
            format!("{}...", &description[..97])
        } else {
            description
        };

        Ok(EventEntry::new(
            "exception",
            "Exception",
            truncated_description,
            payload,
        ))
    }

    fn render(entry: &EventEntry, _cx: &mut Context<crate::ui::MyApp>) -> Div {
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
            .child(render_origin_info_section(entry))
    }
}

fn render_exception_details(content: &Value) -> Div {
    let class = content
        .get("class")
        .and_then(|c| c.as_str())
        .unwrap_or("Exception");
    let message = content.get("message").and_then(|m| m.as_str()).unwrap_or("");

    div().flex().flex_col().gap_2().child(
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
            .child(section_header(format!("{} frames", frames.len())))
            .child(render_frames(frames))
    } else {
        div()
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

fn render_origin_info_section(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(|l| l.as_u64())
            .unwrap_or(0);
        let hostname = origin.get("hostname").and_then(|h| h.as_str());

        if !file.is_empty() {
            return origin_info(file.to_string(), line.to_string(), hostname.map(String::from));
        }
    }
    div()
}
