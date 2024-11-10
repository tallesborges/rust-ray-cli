use crate::events::{process_common_event, EventEntry, EventProcessor};
use serde_json::Value;

pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &Value) -> EventEntry {
        let mut entry = process_common_event(payload, "application_log");
        entry.content = serde_json::to_string_pretty(payload).unwrap_or_default();
        entry
    }
}
