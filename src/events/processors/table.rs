use crate::events::types::{ProcessedEvent, TableEvent};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub fn process_table_event(label: &str, values: &Value) -> Result<ProcessedEvent> {
    let mut data = HashMap::new();

    if let Some(obj) = values.as_object() {
        for (key, value) in obj {
            data.insert(key.clone(), value.clone());
        }
    } else {
        // If values is not an object, store it as a single entry
        data.insert("data".to_string(), values.clone());
    }

    Ok(ProcessedEvent::Table(TableEvent {
        label: label.to_string(),
        data,
    }))
}
