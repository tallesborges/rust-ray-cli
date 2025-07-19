use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::process_application_log_event;
use crate::events::types::ProcessedEvent;
use crate::ui_components::{
    border_color, text_monospace_color, text_primary_color, text_secondary_color,
};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "Application Log".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
        event_type: "application_log".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Process using the new architecture
        let processed_event = process_application_log_event(content)?;

        // Set labels and descriptions based on processed event
        if let ProcessedEvent::ApplicationLog(ref app_log_event) = processed_event {
            entry.label = "Application Log".to_string();
            entry.description = if !app_log_event.message.is_empty() {
                if app_log_event.message.len() > 50 {
                    format!("{}...", &app_log_event.message[..50].trim())
                } else {
                    app_log_event.message.clone()
                }
            } else {
                "Empty log".to_string()
            };
        } else {
            return Err(anyhow::anyhow!("Unexpected event type from application log processor"));
        }
    }

    Ok(entry)
}

pub fn render_application_log_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    let content = entry
        .raw_payload
        .get("content")
        .cloned()
        .unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(render_app_log_content(&content))
        .child(render_app_log_context(&content))
        .child(render_origin_info(entry))
}


fn render_app_log_content(content: &Value) -> Div {
    let value = content
        .get("value")
        .and_then(Value::as_str)
        .unwrap_or_default();

    let level = content
        .get("level")
        .and_then(Value::as_str)
        .unwrap_or("Info");

    let channel = content
        .get("channel")
        .and_then(Value::as_str);

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            // Level and channel info with minimal styling
            div()
                .flex()
                .items_center()
                .gap_4()
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .opacity(0.7)
                        .child(level.to_string()),
                )
                .child(if let Some(ch) = channel {
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .opacity(0.5)
                        .child(format!("• {}", ch))
                } else {
                    div()
                }),
        )
        .child(
            // Main log message
            div()
                .text_sm()
                .text_color(text_primary_color())
                .line_height(gpui::relative(1.5))
                .child(value.to_string()),
        )
}

fn render_app_log_context(content: &Value) -> Div {
    if let Some(context) = content.get("context") {
        if !context.is_null() {
            return div()
                .pt_4()
                .border_t_1()
                .border_color(border_color())
                .child(
                    div()
                        .font_family("monospace")
                        .text_xs()
                        .text_color(text_monospace_color())
                        .opacity(0.8)
                        .max_h_64()
                        .overflow_hidden()
                        .child(serde_json::to_string_pretty(context).unwrap_or_default()),
                );
        }
    }
    div() // Empty div if no context
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
