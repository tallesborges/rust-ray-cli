use crate::events::types::{ApplicationLogEvent, ProcessedEvent};
use anyhow::Result;
use serde_json::Value;

pub fn process_application_log_event(content: &Value) -> Result<ProcessedEvent> {
    let message = content
        .get("value")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let level = content
        .get("level")
        .and_then(Value::as_str)
        .unwrap_or("Info")
        .to_string();

    let channel = content
        .get("channel")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let context = content.get("context").cloned();

    Ok(ProcessedEvent::ApplicationLog(ApplicationLogEvent {
        level,
        message,
        context,
        channel,
    }))
}
