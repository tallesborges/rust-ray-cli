use anyhow::Result;
use serde_json::Value;

pub mod base;
pub mod log;
pub mod exception;
pub mod query;
pub mod table;
pub mod application_log;

pub use base::{EventEntry, EventProcessor, EventUIRenderer};

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

/// Get a custom UI renderer for the given event type
pub fn get_ui_renderer(event_type: &str) -> Option<EventUIRenderer> {
    match event_type {
        "log" => Some(log::render_log_event),
        "exception" => Some(exception::render_exception_event),
        "query" | "executed_query" => Some(query::render_query_event),
        // TODO: Add table and application_log renderers
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
            event_type: event_type.to_string(),
            raw_payload: payload.clone(),
        }),
    }
}