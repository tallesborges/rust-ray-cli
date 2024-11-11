use chrono::Local;
use serde::{Serialize, Deserialize};

pub fn process_common_event(p_type: &str) -> EventEntry {
    EventEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
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
