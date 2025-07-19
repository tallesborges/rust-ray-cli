use crate::events::types::{LogEvent, ProcessedEvent};
use anyhow::Result;
use serde_json::Value;

pub fn process_log_event(content: &Value) -> Result<ProcessedEvent> {
    let values = content
        .get("values")
        .ok_or_else(|| anyhow::anyhow!("Missing values in log event"))?;

    // Extract log information
    let (level, message, context) = if let Some(values_array) = values.as_array() {
        // Handle array-based log values
        let first_value = values_array.first().cloned();
        let level = "Info".to_string(); // Default level for simple logs
        let message = match &first_value {
            Some(Value::String(s)) => s.clone(),
            Some(other) => serde_json::to_string_pretty(other).unwrap_or_default(),
            None => "Empty log".to_string(),
        };
        let context = if values_array.len() > 1 {
            Some(Value::Array(values_array[1..].to_vec()))
        } else {
            None
        };
        (level, message, context)
    } else if let Some(values_obj) = values.as_object() {
        // Handle object-based log values
        let level = values_obj
            .get("level")
            .and_then(Value::as_str)
            .unwrap_or("Info")
            .to_string();
        let message = values_obj
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let context = values_obj.get("context").cloned();
        (level, message, context)
    } else {
        // Handle single value logs
        let level = "Info".to_string();
        let message = serde_json::to_string_pretty(values).unwrap_or_default();
        (level, message, None)
    };

    Ok(ProcessedEvent::Log(LogEvent {
        level,
        message,
        context,
    }))
}
