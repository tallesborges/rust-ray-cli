use anyhow::Result;
use serde_json::Value;

pub mod application_log;
pub mod base;
pub mod cache;
pub mod exception;
pub mod http;
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
        "cache" => Some(EventProcessor::Cache),
        "request" => Some(EventProcessor::Http),
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
        "cache" => Some(cache::render_cache_event),
        "request" => Some(http::render_http_event),
        _ => Some(render_unknown_event), // Fallback for unknown event types
    }
}

/// Fallback renderer for unknown event types
fn render_unknown_event(entry: &EventEntry, _cx: &mut gpui::Context<crate::app::MyApp>) -> gpui::Div {
    use gpui::prelude::*;
    use gpui::div;
    use crate::ui_components::{text_primary_color, text_secondary_color, border_color};

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(text_primary_color())
                .child(format!("Unknown Event Type: {}", entry.event_type))
        )
        .child(
            div()
                .text_xs()
                .text_color(text_secondary_color())
                .child("This event type is not supported. Raw JSON payload:")
        )
        .child(
            div()
                .p_4()
                .rounded_md()
                .bg(gpui::rgb(0x18181b))
                .border_1()
                .border_color(border_color())
                .child(
                    div()
                        .text_xs()
                        .font_family("monospace")
                        .text_color(text_primary_color())
                        .child(
                            serde_json::to_string_pretty(&entry.raw_payload)
                                .unwrap_or_else(|_| "Invalid JSON".to_string())
                        ),
                ),
        )
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
