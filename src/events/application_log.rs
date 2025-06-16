use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::process_application_log_event;
use crate::events::types::ProcessedEvent;
use crate::ui_components::{
    background_color, border_color, styled_card, text_monospace_color, text_secondary_color,
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
        .gap_4()
        .child(render_app_log_header())
        .child(render_app_log_content(&content))
        .child(render_app_log_context(&content))
        .child(render_origin_info(entry))
}

fn render_app_log_header() -> Div {
    styled_card().p_4().child(
        div()
            .flex()
            .flex_row()
            .gap_3()
            .items_center()
            .child(div().text_2xl().child("📋"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(gpui::rgb(0x059669)) // Emerald
                            .child("Application Log"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary_color())
                            .child("Application Event"),
                    ),
            ),
    )
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

    styled_card().p_4().child(
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_secondary_color())
                            .child("📝 Log Content"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(match level {
                                "Error" => gpui::rgb(0xef4444),
                                "Warning" => gpui::rgb(0xf59e0b),
                                "Debug" => gpui::rgb(0x6b7280),
                                _ => gpui::rgb(0x3b82f6),
                            })
                            .text_color(gpui::white())
                            .text_xs()
                            .rounded_lg()
                            .child(level.to_string()),
                    )
                    .child(if let Some(ch) = channel {
                        div()
                            .px_2()
                            .py_1()
                            .bg(gpui::rgb(0x8b5cf6))
                            .text_color(gpui::white())
                            .text_xs()
                            .rounded_lg()
                            .child(ch.to_string())
                    } else {
                        div() // Empty div if no channel
                    }),
            )
            .child(
                div()
                    .p_3()
                    .bg(background_color())
                    .border_1()
                    .border_color(border_color())
                    .rounded_lg()
                    .child(
                        div()
                            .text_sm()
                            .line_height(gpui::relative(1.5))
                            .child(value.to_string()),
                    ),
            ),
    )
}

fn render_app_log_context(content: &Value) -> Div {
    if let Some(context) = content.get("context") {
        if !context.is_null() {
            return styled_card().p_4().child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_secondary_color())
                            .child("🔍 Context"),
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(gpui::rgb(0x1f2937))
                            .rounded_lg()
                            .max_h_64()
                            .overflow_hidden()
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_xs()
                                    .text_color(gpui::rgb(0xd1d5db))
                                    .child(serde_json::to_string_pretty(context).unwrap_or_default()),
                            ),
                    ),
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
            return styled_card().p_3().child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_secondary_color())
                            .child("🔍 Source"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_monospace_color())
                            .child(format!("{}:{} on {}", file, line, hostname)),
                    ),
            );
        }
    }
    div() // Empty div if no origin info
}
