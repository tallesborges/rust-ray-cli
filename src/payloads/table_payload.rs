use crate::payloads::{display_code, process_common_payload, PayloadEntry, PayloadType};
use eframe::egui;
use serde_json::Value;

pub struct TablePayload;

impl PayloadType for TablePayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "table");

        if let Some(content) = payload.get("content").and_then(|c| c.get("values")) {
            let is_request = content
                .get("Method")
                .and_then(Value::as_str)
                .map_or(false, |m| m == "GET" || m == "POST");

            let field_name = if is_request { "Data" } else { "Body" };

            // Set content from the appropriate field
            entry.content = content
                .get(field_name)
                .and_then(|v| serde_json::to_string_pretty(v).ok())
                .unwrap_or_default();

            // Use URL as description
            entry.description = content
                .get("URL")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned();

            entry.label = if is_request {
                "Request".to_string()
            } else {
                "Response".to_string()
            };
        }

        entry
    }

}
