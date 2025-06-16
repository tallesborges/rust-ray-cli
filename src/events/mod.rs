use anyhow::Result;
use serde_json::Value;

pub mod application_log;
pub mod base;
pub mod exception;
pub mod log;
pub mod processors;
pub mod query;
pub mod table;
pub mod types;

pub use base::{EventEntry, EventProcessor, EventUIRenderer};

/// Create an event processor for the given event type
pub fn create_processor(event_type: &str) -> Option<EventProcessor> {
    match event_type {
        "log" => Some(EventProcessor::Log),
        "exception" => Some(EventProcessor::Exception),
        "query" | "executed_query" => Some(EventProcessor::Query),
        "table" => Some(EventProcessor::Table),
        "application_log" => Some(EventProcessor::ApplicationLog),
        _ => None,
    }
}

/// Get a custom UI renderer for the given event type
pub fn get_ui_renderer(event_type: &str) -> Option<EventUIRenderer> {
    match event_type {
        "log" => Some(log::render_log_event),
        "exception" => Some(exception::render_exception_event),
        "query" | "executed_query" => Some(query::render_query_event),
        "table" => Some(table::render_table_event),
        "application_log" => Some(application_log::render_application_log_event),
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
            content_type: "json".to_string(),
            event_type: event_type.to_string(),
            raw_payload: payload.clone(),
        }),
    }
}
