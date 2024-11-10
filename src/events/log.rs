use crate::events::{process_common_event, EventEntry, EventProcessor};
use serde_json::Value;

pub struct LogEvent;

impl EventProcessor for LogEvent {
    fn process(&self, payload: &Value) -> EventEntry {
        let mut entry = process_common_event(payload, "log");
        entry.content = payload
            .get("content")
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }
}
