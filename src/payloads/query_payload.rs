use crate::payloads::types::{PayloadEntry, PayloadType, process_common_payload, display_code};
use eframe::egui;
use serde_json::Value;

pub struct QueryPayload;

impl PayloadType for QueryPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "query");
        entry.content = payload
            .get("content")
            .and_then(|v| v.get("sql"))
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("SQL Query:");
        display_code(ui, &entry.content, "sql");
    }
}
