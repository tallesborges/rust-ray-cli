use crate::events::{process_common_event, EventEntry, EventProcessor};
use serde_json::Value;

pub struct ExceptionEvent;

impl EventProcessor for ExceptionEvent {
    fn process(&self, payload: &Value) -> EventEntry {
        let mut entry = process_common_event(payload, "exception");
        entry.content = payload
            .get("content")
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }
}
