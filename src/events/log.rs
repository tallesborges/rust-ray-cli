use crate::events::{entry::EventEntry, Event};
use crate::ui::components::{origin_info, text_monospace_color, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div, InteractiveText, StyledText};
use serde_json::Value;

pub struct LogEvent;

impl Event for LogEvent {
    fn process(payload: &Value) -> Result<EventEntry> {
        let content = payload
            .get("content")
            .ok_or_else(|| anyhow::anyhow!("Missing content in log event"))?;

        let values = content
            .get("values")
            .ok_or_else(|| anyhow::anyhow!("Missing values in log event"))?;

        let message = if let Some(values_array) = values.as_array() {
            values_array
                .first()
                .and_then(Value::as_str)
                .unwrap_or("Empty log")
                .to_string()
        } else if let Some(msg) = values.as_str() {
            msg.to_string()
        } else {
            serde_json::to_string_pretty(values).unwrap_or_else(|_| "Log".to_string())
        };

        let description = if message.len() > 100 {
            format!("{}...", &message[..97])
        } else {
            message.clone()
        };

        let clean_description = description
            .replace('\n', " ")
            .replace("  ", " ")
            .trim()
            .to_string();

        Ok(EventEntry::new("log", "Log", clean_description, payload))
    }

    fn render(entry: &EventEntry, _cx: &mut Context<crate::ui::MyApp>) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(render_log_values(entry))
            .child(render_origin_info_section(entry))
    }
}

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
            div()
                .text_xs()
                .text_color(text_secondary_color())
                .opacity(0.5)
                .w_4()
                .child(format!("{}", index + 1)),
        )
        .child(div().flex_1().child(match value {
            Value::String(s) => div().text_sm().text_color(text_primary_color()).child(s.clone()),
            Value::Number(n) => div()
                .text_sm()
                .text_color(text_primary_color())
                .child(n.to_string()),
            Value::Bool(b) => div()
                .text_sm()
                .text_color(text_primary_color())
                .child(b.to_string()),
            Value::Null => div()
                .text_sm()
                .text_color(text_secondary_color())
                .opacity(0.5)
                .child("null"),
            Value::Object(_) | Value::Array(_) => {
                let formatted =
                    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string());
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
        }))
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
