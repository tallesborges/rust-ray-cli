use anyhow::Result;
use serde_json::Value;

pub mod base;
pub mod log;
pub mod exception;
pub mod query;
pub mod table;
pub mod application_log;

pub use base::{EventEntry, EventProcessor};

/// Create an event processor for the given event type
pub fn create_processor(event_type: &str) -> Option<Box<dyn EventProcessor>> {
    match event_type {
        "log" => Some(Box::new(log::LogProcessor)),
        "exception" => Some(Box::new(exception::ExceptionProcessor)),
        "query" | "executed_query" => Some(Box::new(query::QueryProcessor)),
        "table" => Some(Box::new(table::TableProcessor)),
        "application_log" => Some(Box::new(application_log::ApplicationLogProcessor)),
        _ => None,
    }
}

/// Process an event with the appropriate processor
pub fn process_event(event_type: &str, payload: &Value) -> Result<EventEntry> {
    match create_processor(event_type) {
        Some(processor) => processor.process(payload),
        None => Ok(EventEntry {
            timestamp: String::new(),
            label: format!("Unknown Event: {}", event_type),
            description: "Unknown event type".to_string(),
            content: payload.to_string(),
            content_type: "json".to_string(),
        }),
    }
}