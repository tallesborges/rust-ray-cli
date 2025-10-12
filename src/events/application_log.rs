use crate::events::{entry::EventEntry, Event};
use crate::ui_components::{
    border_color, text_monospace_color, text_primary_color, text_secondary_color,
};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div};
use serde_json::Value;

pub struct ApplicationLogEvent;

impl Event for ApplicationLogEvent {
    fn process(payload: &Value) -> Result<EventEntry> {
        let content = payload
            .get("content")
            .ok_or_else(|| anyhow::anyhow!("Missing content in application log event"))?;

        let value = content.get("value").and_then(Value::as_str).unwrap_or("");

        let description = if !value.is_empty() {
            if value.len() > 50 {
                format!("{}...", value[..50].trim())
            } else {
                value.to_string()
            }
        } else {
            "Empty log".to_string()
        };

        Ok(EventEntry::new(
            "application_log",
            "Application Log",
            description,
            payload,
        ))
    }

    fn render(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
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
}

fn render_app_log_content(content: &Value) -> Div {
    let value = content.get("value").and_then(Value::as_str).unwrap_or_default();
    let level = content.get("level").and_then(Value::as_str).unwrap_or("Info");
    let channel = content.get("channel").and_then(Value::as_str);

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
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
    div()
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
            return div()
                .pt_4()
                .border_t_1()
                .border_color(border_color())
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .opacity(0.7)
                        .child(format!("{file}:{line} • {hostname}")),
                );
        }
    }
    div()
}
