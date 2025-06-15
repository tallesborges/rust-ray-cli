use crate::events::types::{ProcessedEvent, QueryEvent};
use anyhow::Result;
use serde_json::Value;

pub fn process_query_event(content: &Value) -> Result<ProcessedEvent> {
    let sql = content
        .get("sql")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let duration_ms = content.get("time").and_then(Value::as_f64);

    let connection_name = content
        .get("connection_name")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    // Extract bindings if available
    let bindings = if let Some(bindings_value) = content.get("bindings") {
        if let Some(bindings_array) = bindings_value.as_array() {
            bindings_array.clone()
        } else {
            vec![bindings_value.clone()]
        }
    } else {
        Vec::new()
    };

    let affected_rows = content.get("affected_rows").and_then(Value::as_u64);

    Ok(ProcessedEvent::Query(QueryEvent {
        sql,
        bindings,
        duration_ms,
        connection_name,
        affected_rows,
    }))
}