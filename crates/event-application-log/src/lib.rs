#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");

        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            let value = v
                .get("content")
                .and_then(|v| v.get("value"))
                .and_then(Value::as_str)
                .unwrap_or_default();

            let file = v
                .get("origin")
                .and_then(|v| v.get("file"))
                .and_then(Value::as_str);
            let line = v
                .get("origin")
                .and_then(|v| v.get("line_number"))
                .and_then(Value::as_u64);

            entry.content = alloc::format!(
                "### Application Log\n\n**File:** {}\n\n**Line:** {}\n\n```\n{}\n```",
                file.unwrap_or_default(),
                line.unwrap_or_default(),
                value
            );
        } else {
            entry.content = payload.to_string();
        }

        entry
    }
}

implement_ffi_interface!(ApplicationLogEvent);

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_ffi_process_application_log() {
        let payload = r#"{"some": "test log message"}"#;
        let payload_bytes = payload.as_bytes();

        let result_ptr = process_event(payload_bytes.as_ptr(), payload_bytes.len());

        unsafe {
            let c_str = CStr::from_ptr(result_ptr as *const i8);
            let result_str = c_str.to_str().unwrap();
            let result: EventEntry = serde_json::from_str(result_str).unwrap();

            assert_eq!(result.label, "application_log");
            assert_eq!(result.content, payload);

            free_string(result_ptr);
        }
    }
}
