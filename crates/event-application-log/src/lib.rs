#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]

use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");

        entry.content = serde_json::from_str::<Value>(payload)
            .ok()
            .and_then(|v| serde_json::to_string_pretty(&v).ok())
            .unwrap_or_default();

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
