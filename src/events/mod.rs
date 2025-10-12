use anyhow::Result;
use serde_json::Value;
use gpui::{Context, Div};

pub mod application_log;
pub mod cache;
pub mod entry;
pub mod event_type;
pub mod exception;
pub mod http;
pub mod log;
pub mod query;

pub use entry::EventEntry;
pub use event_type::EventType;

/// Trait for event processing and rendering
pub trait Event {
    /// Process raw JSON payload into an EventEntry
    fn process(payload: &Value) -> Result<EventEntry>
    where
        Self: Sized;

    /// Render the event in the UI
    fn render(entry: &EventEntry, cx: &mut Context<crate::ui::MyApp>) -> Div;
}

/// Process an event with the appropriate processor
pub fn process_event(event_type: &str, payload: &Value) -> Result<EventEntry> {
    // Smart detection: if event_type is "table", check content.label to determine actual type
    let actual_event_type = if event_type == "table" {
        detect_table_event_type(payload)
    } else {
        event_type
    };

    match actual_event_type {
        "log" => log::LogEvent::process(payload),
        "exception" => exception::ExceptionEvent::process(payload),
        "query" | "executed_query" => query::QueryEvent::process(payload),
        "application_log" => application_log::ApplicationLogEvent::process(payload),
        "cache" => cache::CacheEvent::process(payload),
        "request" => http::HttpEvent::process(payload),
        _ => Ok(EventEntry::new(
            actual_event_type,
            format!("Unknown Event: {}", actual_event_type),
            "Unknown event type",
            payload,
        )),
    }
}

/// Render an event with the appropriate renderer
pub fn render_event(entry: &EventEntry, cx: &mut Context<crate::ui::MyApp>) -> Div {
    match entry.event_type.as_str() {
        "log" => log::LogEvent::render(entry, cx),
        "exception" => exception::ExceptionEvent::render(entry, cx),
        "query" | "executed_query" => query::QueryEvent::render(entry, cx),
        "application_log" => application_log::ApplicationLogEvent::render(entry, cx),
        "cache" => cache::CacheEvent::render(entry, cx),
        "request" => http::HttpEvent::render(entry, cx),
        _ => render_unknown_event(entry, cx),
    }
}

/// Fallback renderer for unknown event types
fn render_unknown_event(entry: &EventEntry, _cx: &mut Context<crate::ui::MyApp>) -> Div {
    use crate::ui::components::{border_color, text_primary_color, text_secondary_color};
    use gpui::div;
    use gpui::prelude::*;

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(text_primary_color())
                .child(format!("Unknown Event Type: {}", entry.event_type)),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_secondary_color())
                .child("This event type is not supported. Raw JSON payload:"),
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
                                .unwrap_or_else(|_| "Invalid JSON".to_string()),
                        ),
                ),
        )
}

/// Detect the actual event type from table events based on content.label
fn detect_table_event_type(payload: &Value) -> &str {
    if let Some(content) = payload.get("content") {
        if let Some(label) = content.get("label").and_then(|l| l.as_str()) {
            match label.to_lowercase().as_str() {
                "http" | "request" => "request",
                "cache" => "cache",
                "query" | "database" => "query",
                "log" => "log",
                "exception" | "error" => "exception",
                _ => "request", // Default to request for unknown labels
            }
        } else {
            "request"
        }
    } else {
        "request"
    }
}
