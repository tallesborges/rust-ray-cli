#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
use shared::{process_common_event, EventEntry, EventProcessor};

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;

pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");
        entry.content = String::from(payload);
        entry
    }
}

#[no_mangle]
pub extern "C" fn process_application_log(ptr: *const u8, len: usize) -> *mut EventEntry {
    let payload = unsafe {
        let slice = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(slice)
    };

    let processor = ApplicationLogEvent;
    let entry = processor.process(payload);
    Box::into_raw(Box::new(entry))
}
