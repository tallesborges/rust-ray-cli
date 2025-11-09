use crate::events::{entry::EventEntry, Event};
use crate::ui::components::{metric_row, origin_info, text_monospace_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div, InteractiveText, StyledText};
use serde_json::Value;

pub struct QueryEvent;

impl Event for QueryEvent {
    fn process(payload: &Value) -> Result<EventEntry> {
        let content = payload
            .get("content")
            .ok_or_else(|| anyhow::anyhow!("Missing content in query event"))?;

        let sql = content
            .get("sql")
            .and_then(Value::as_str)
            .unwrap_or_default();

        let operation_type = sql
            .split_whitespace()
            .next()
            .unwrap_or("SQL")
            .to_uppercase();

        let label = format!("Query: {}", operation_type);

        let description_sql = if sql.len() > 50 {
            format!("{}...", sql[..50].trim())
        } else {
            sql.trim().to_string()
        };

        let description = if let Some(time) = content.get("time").and_then(Value::as_f64) {
            format!("{} ({}ms)", description_sql, time)
        } else {
            description_sql
        };

        Ok(EventEntry::new("query", label, description, payload))
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
            .child(render_query_metrics(&content))
            .child(render_sql_query(&content))
            .child(render_origin_info_section(entry))
    }
}

fn render_query_metrics(content: &Value) -> Div {
    let time = content.get("time").and_then(|t| t.as_f64()).unwrap_or(0.0);
    let connection = content
        .get("connection_name")
        .and_then(|c| c.as_str())
        .unwrap_or("default")
        .to_string();

    let time_display = if time < 1.0 {
        format!("{:.3}ms", time)
    } else if time < 1000.0 {
        format!("{:.1}ms", time)
    } else {
        format!("{:.2}s", time / 1000.0)
    };

    div()
        .flex()
        .flex_row()
        .gap_4()
        .text_xs()
        .child(metric_row("time".to_string(), time_display))
        .child(metric_row("connection".to_string(), connection))
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

fn render_origin_info_section(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(|l| l.as_u64())
            .unwrap_or(0);
        let hostname = origin.get("hostname").and_then(|h| h.as_str());

        if !file.is_empty() {
            return origin_info(
                file.to_string(),
                line.to_string(),
                hostname.map(String::from),
            );
        }
    }
    div()
}
