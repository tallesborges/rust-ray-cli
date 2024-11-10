use chrono::Local;
use serde_json::Value;
mod application_log;
mod exception;
mod factory;
mod log;
mod query;
mod table;

pub use factory::EventTypeFactory;

pub fn process_common_event(payload: &Value, p_type: &str) -> EventEntry {
    EventEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        label: p_type.to_string(),
        description: String::new(),
        content: String::new(),
        content_type: "json".to_string(),
    }
}

pub trait EventProcessor: Send + Sync {
    fn process(&self, payload: &Value) -> EventEntry;
}

#[derive(Clone, Debug)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content: String,
    pub content_type: String,
}
