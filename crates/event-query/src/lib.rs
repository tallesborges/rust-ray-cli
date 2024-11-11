use core::{process_common_event, EventEntry, EventProcessor};
use serde_json::Value;

pub struct QueryEvent;

impl EventProcessor for QueryEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("query");
        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            entry.content = v
                .get("content")
                .and_then(|v| v.get("sql"))
                .and_then(|v| serde_json::to_string_pretty(v).ok())
                .unwrap_or_default();
        }
        entry.content_type = "sql".to_string();
        entry
    }
}

#[no_mangle]
pub extern "C" fn process_query(ptr: *const u8, len: usize) -> *mut EventEntry {
    let payload = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8_unchecked(slice)
    };
    
    let processor = QueryEvent;
    let entry = processor.process(payload);
    Box::into_raw(Box::new(entry))
}
