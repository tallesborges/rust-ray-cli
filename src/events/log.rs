use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::process_log_event;
use crate::events::types::ProcessedEvent;
use crate::ui_components::{
    border_color, text_monospace_color, text_primary_color, text_secondary_color,
};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div, InteractiveText, StyledText};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "log".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
        event_type: "log".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Process using the new architecture
        let processed_event = process_log_event(content)?;

        // Set labels and descriptions based on processed event
        if let ProcessedEvent::Log(ref log_event) = processed_event {
            entry.label = "Log".to_string();
            let description = if log_event.message.len() > 100 {
                format!("{}...", &log_event.message[..97])
            } else {
                log_event.message.clone()
            };
            // Clean up any JSON formatting for the description
            entry.description = description
                .replace('\n', " ")
                .replace("  ", " ")
                .trim()
                .to_string();
        } else {
            return Err(anyhow::anyhow!("Unexpected event type from log processor"));
        }
    }

    Ok(entry)
}

pub fn render_log_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(render_log_values(entry))
        .child(render_origin_info(entry))
}

// Header removed for minimal design

fn render_log_values(entry: &EventEntry) -> Div {
    let values = entry
        .raw_payload
        .get("content")
        .and_then(|c| c.get("values"))
        .cloned()
        .unwrap_or(Value::Array(vec![]));

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(render_values_list(&values))
}

fn render_values_list(values: &Value) -> Div {
    match values {
        Value::Array(arr) => {
            let mut container = div().flex().flex_col().gap_2();

            for (index, value) in arr.iter().enumerate() {
                container = container.child(render_single_value(index, value));
            }

            container
        }
        _ => render_single_value(0, values),
    }
}

fn render_single_value(index: usize, value: &Value) -> Div {
    div()
        .flex()
        .flex_row()
        .gap_4()
        .items_start()
        .child(
            // Minimal index
            div()
                .text_xs()
                .text_color(text_secondary_color())
                .opacity(0.5)
                .w_4()
                .child(format!("{}", index + 1)),
        )
        .child(
            // Value content
            div().flex_1().child(match value {
                Value::String(s) => render_string_value(s),
                Value::Number(n) => render_number_value(n),
                Value::Bool(b) => render_bool_value(*b),
                Value::Null => render_null_value(),
                Value::Object(_) | Value::Array(_) => render_complex_value(value),
            }),
        )
}

fn render_string_value(s: &str) -> Div {
    div()
        .text_sm()
        .text_color(text_primary_color())
        .child(s.to_string())
}

fn render_number_value(n: &serde_json::Number) -> Div {
    div()
        .text_sm()
        .text_color(text_primary_color())
        .child(n.to_string())
}

fn render_bool_value(b: bool) -> Div {
    div()
        .text_sm()
        .text_color(text_primary_color())
        .child(b.to_string())
}

fn render_null_value() -> Div {
    div()
        .text_sm()
        .text_color(text_secondary_color())
        .opacity(0.5)
        .child("null")
}

fn render_complex_value(value: &Value) -> Div {
    let formatted = serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string());

    div().py_2().child(
        div()
            .font_family("monospace")
            .text_xs()
            .text_color(text_monospace_color())
            .opacity(0.8)
            .child(InteractiveText::new(
                "complex-value",
                StyledText::new(formatted),
            )),
    )
}

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
