use crate::payloads::{display_code, process_common_payload, PayloadEntry, PayloadType};
use eframe::egui;
use serde_json::Value;

pub struct ApplicationLogPayload;

impl PayloadType for ApplicationLogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "application_log");
        entry.content = serde_json::to_string_pretty(payload).unwrap_or_default();
        entry
    }

}
