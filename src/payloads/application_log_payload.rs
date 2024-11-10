use crate::payloads::types::{PayloadEntry, PayloadType, process_common_payload, display_code};
use eframe::egui;
use serde_json::Value;

pub struct ApplicationLogPayload;

impl PayloadType for ApplicationLogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "application_log");
        entry.content = serde_json::to_string_pretty(payload).unwrap_or_default();
        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("Application Log:");
        display_code(ui, &entry.content, "json");
    }
}
