// use chrono::Local;
use serde::{Deserialize, Serialize};

pub fn process_common_event(p_type: &str) -> EventEntry {
    EventEntry {
        timestamp: "00:00".to_string(),
        label: p_type.to_string(),
        description: String::new(),
        content: String::new(),
        content_type: "json".to_string(),
    }
}

pub trait EventProcessor: Send + Sync {
    fn process(&self, payload: &str) -> EventEntry;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content: String,
    pub content_type: String,
}

pub fn process_event<T: EventProcessor>(processor: &T, ptr: *const u8, len: usize) -> *mut u8 {
    let payload = unsafe {
        let slice = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(slice)
    };

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

pub fn free_string(ptr: *mut u8) {
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
