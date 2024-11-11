#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
use shared::{process_common_event, EventEntry, EventProcessor};

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");
        entry.content = String::from(payload);
        entry
    }
}

#[no_mangle]
pub extern "C" fn process_application_log(ptr: *const u8, len: usize) -> *mut u8 {
    let payload = unsafe {
        let slice = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(slice)
    };

    let processor = ApplicationLogEvent;
    let entry = processor.process(payload);

    // Serialize the EventEntry to JSON string
    let json = serde_json::to_string(&entry).unwrap_or_default();

    // Convert the string to a byte vector
    let mut bytes = json.into_bytes();
    // Add null terminator for C-style strings
    bytes.push(0);

    // Convert to raw pointer and forget the allocation to prevent dropping
    let ptr = bytes.as_mut_ptr();
    core::mem::forget(bytes);

    ptr
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut u8) {
    unsafe {
        if !ptr.is_null() {
            // Find length by searching for null terminator
            let mut len = 0;
            while *ptr.add(len) != 0 {
                len += 1;
            }
            // Reconstruct the vector to properly deallocate
            Vec::from_raw_parts(ptr, len, len + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_ffi_process_application_log() {
        let payload = r#"{"some": "test log message"}"#;
        let payload_bytes = payload.as_bytes();

        let result_ptr = process_application_log(payload_bytes.as_ptr(), payload_bytes.len());

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
