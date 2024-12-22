#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
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
                    .map(|s| alloc::format!("```json\n{}\n```", s))
                    .unwrap_or_default();

                entry.description = content
                    .get("URL")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

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

implement_ffi_interface!(TableEvent);
