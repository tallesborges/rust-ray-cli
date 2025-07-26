use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::process_query_event;
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
        label: "Query".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
        event_type: "query".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Process using the new architecture
        let processed_event = process_query_event(content)?;

        // Set labels and descriptions based on processed event
        if let ProcessedEvent::Query(ref query_event) = processed_event {
            // Extract the SQL operation type (SELECT, INSERT, UPDATE, etc.)
            let operation_type =
                if let Some(first_word) = query_event.sql.split_whitespace().next() {
                    first_word.to_uppercase()
                } else {
                    "SQL".to_string()
                };
            entry.label = format!("Query: {operation_type}");

            let description_sql = if query_event.sql.len() > 50 {
                format!("{}...", &query_event.sql[..50].trim())
            } else {
                query_event.sql.trim().to_string()
            };

            if let Some(time) = query_event.duration_ms {
                entry.description = format!("{description_sql} ({time}ms)");
            } else {
                entry.description = description_sql;
            }
        } else {
            return Err(anyhow::anyhow!(
                "Unexpected event type from query processor"
            ));
        }
    }

    Ok(entry)
}

pub fn render_query_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    let content = entry
        .raw_payload
        .get("content")
        .cloned()
        .unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(render_query_metrics(&content))
        .child(render_sql_query(&content))
        .child(render_origin_info(entry))
}

// Header removed for minimal design

fn render_query_metrics(content: &Value) -> Div {
    let time = content.get("time").and_then(|t| t.as_f64()).unwrap_or(0.0);
    let connection = content
        .get("connection_name")
        .and_then(|c| c.as_str())
        .unwrap_or("default")
        .to_string();

    let time_display = if time < 1.0 {
        format!("{time:.3}ms")
    } else if time < 1000.0 {
        format!("{time:.1}ms")
    } else {
        format!("{:.2}s", time / 1000.0)
    };

    div()
        .flex()
        .flex_row()
        .gap_4()
        .text_xs()
        .text_color(text_secondary_color())
        .child(
            div()
                .flex()
                .flex_row()
                .gap_1()
                .child(div().opacity(0.5).child("time:"))
                .child(div().text_color(text_primary_color()).child(time_display)),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap_1()
                .child(div().opacity(0.5).child("connection:"))
                .child(div().child(connection)),
        )
}

fn render_sql_query(content: &Value) -> Div {
    let sql = content.get("sql").and_then(|s| s.as_str()).unwrap_or("");

    div().py_2().child(
        div()
            .font_family("monospace")
            .text_sm()
            .text_color(text_monospace_color())
            .opacity(0.9)
            .child(InteractiveText::new(
                "sql-query",
                StyledText::new(sql.to_string()),
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
                        .child(format!("{file}:{line} • {hostname}")),
                )
        } else {
            div() // Empty div if no origin info
        }
    } else {
        div() // Empty div if no origin
    }
}
