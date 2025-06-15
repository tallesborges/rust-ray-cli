use crate::events::types::{HttpEvent, HttpEventType, ProcessedEvent};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub fn process_http_event(content: &Value) -> Result<ProcessedEvent> {
    let values = content
        .get("values")
        .ok_or_else(|| anyhow::anyhow!("Missing values in HTTP event"))?;

    let method = values
        .get("Method")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let status_code = values.get("Status").and_then(Value::as_u64);

    // Determine if this is a request or response
    let event_type = if method.is_some() {
        HttpEventType::Request
    } else if status_code.is_some() {
        HttpEventType::Response
    } else {
        // Default to request if we can't determine
        HttpEventType::Request
    };

    let url = values
        .get("URL")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let success = values.get("Success").and_then(Value::as_bool);

    // Extract headers
    let mut headers = HashMap::new();
    if let Some(headers_obj) = values.get("Headers").and_then(Value::as_object) {
        for (key, value) in headers_obj {
            headers.insert(key.clone(), value.clone());
        }
    }

    let body = values
        .get("Body")
        .cloned()
        .or_else(|| values.get("Data").cloned());

    let duration_seconds = values.get("Duration").and_then(Value::as_f64);
    let connection_time_seconds = values.get("Connection time").and_then(Value::as_f64);
    let size_bytes = values.get("Size").and_then(Value::as_u64);
    let request_size_bytes = values.get("Request Size").and_then(Value::as_u64);
    let content_type = values
        .get("Type")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    Ok(ProcessedEvent::Http(HttpEvent {
        event_type,
        url,
        method,
        status_code,
        success,
        headers,
        body,
        duration_seconds,
        connection_time_seconds,
        size_bytes,
        request_size_bytes,
        content_type,
    }))
}
