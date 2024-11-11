use shared::{process_common_event, EventEntry, EventProcessor};

pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");
        entry.content = payload.to_string();
        entry
    }
}

#[no_mangle]
pub extern "C" fn process_application_log(ptr: *const u8, len: usize) -> *mut EventEntry {
    let payload = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8_unchecked(slice)
    };

    let processor = ApplicationLogEvent;
    let entry = processor.process(payload);
    Box::into_raw(Box::new(entry))
}
