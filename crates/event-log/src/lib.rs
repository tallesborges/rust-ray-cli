#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct LogEvent;

impl EventProcessor for LogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("log");
        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            entry.content = v
                .get("content")
                .and_then(|v| v.get("values"))
                .and_then(|v| serde_json::to_string_pretty(v).ok())
                .map(|s| alloc::format!("```json\n{}\n```", s))
                .unwrap_or_default();
        }
        entry
    }
}

implement_ffi_interface!(LogEvent);
