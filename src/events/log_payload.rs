use crate::events::{process_common_payload, PayloadEntry, PayloadType};
use serde_json::Value;

pub struct LogPayload;

impl PayloadType for LogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "log");
        entry.content = payload
            .get("content")
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }
}
