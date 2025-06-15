use crate::events::types::{CacheEvent, ProcessedEvent};
use anyhow::Result;
use serde_json::Value;

pub fn process_cache_event(content: &Value) -> Result<ProcessedEvent> {
    let values = content
        .get("values")
        .ok_or_else(|| anyhow::anyhow!("Missing values in cache event"))?;

    let operation = values
        .get("Event")
        .and_then(Value::as_str)
        .unwrap_or("Unknown")
        .replace("<code>", "")
        .replace("</code>", "");

    let key = values
        .get("Key")
        .and_then(Value::as_str)
        .unwrap_or("Unknown")
        .to_string();

    let value = values.get("Value").cloned();

    let expiration_seconds = values.get("Expiration in seconds").and_then(Value::as_u64);

    let tags = values
        .get("Tags")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let store = values
        .get("Store")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let ttl = values
        .get("TTL")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    Ok(ProcessedEvent::Cache(CacheEvent {
        operation,
        key,
        value,
        expiration_seconds,
        tags,
        store,
        ttl,
    }))
}
