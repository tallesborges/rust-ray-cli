use core::{process_common_event, EventEntry, EventProcessor};
use serde_json::Value;

pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");
        entry.content = payload.to_string();
        entry
    }
}
