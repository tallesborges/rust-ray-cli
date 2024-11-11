#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
use shared::{EventEntry, EventProcessor};

extern crate alloc;

use alloc::string::String;

pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = shared::process_common_event("application_log");
        entry.content = String::from(payload);
        entry
    }
}

#[no_mangle]
pub extern "C" fn process_event(ptr: *const u8, len: usize) -> *mut u8 {
    let processor = ApplicationLogEvent;
    shared::process_event(&processor, ptr, len)
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut u8) {
    shared::free_string(ptr);
}

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

            // Don't forget to free the memory
            free_string(result_ptr);
        }
    }
}
