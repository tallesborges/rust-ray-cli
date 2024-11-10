use crate::payloads::{process_common_payload, PayloadEntry, PayloadType};
use serde_json::Value;

pub struct ExceptionPayload;

impl PayloadType for ExceptionPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "exception");
        entry.content = payload
            .get("content")
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }
}
