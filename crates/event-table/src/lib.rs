use core::{process_common_event, EventEntry, EventProcessor};
use serde_json::Value;

pub struct TableEvent;

impl EventProcessor for TableEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("table");

        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            if let Some(content) = v.get("content").and_then(|c| c.get("values")) {
                let is_request = content
                    .get("Method")
                    .and_then(Value::as_str)
                    .map_or(false, |m| m == "GET" || m == "POST");

                let field_name = if is_request { "Data" } else { "Body" };

                entry.content = content
                    .get(field_name)
                    .and_then(|v| serde_json::to_string_pretty(v).ok())
                    .unwrap_or_default();

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
        }

        entry
    }
}
