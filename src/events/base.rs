use anyhow::Result;
use gpui::Context;
use serde_json::Value;

/// Represents a processed event entry
#[derive(Clone, Debug)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content_type: String,
    pub event_type: String,
    pub raw_payload: Value,
}

/// Event processor enum for compile-time dispatch
#[derive(Debug, Clone)]
pub enum EventProcessor {
    Log,
    Exception,
    Query,
    ApplicationLog,
    Cache,
    Http,
    // Table removed - was an anti-pattern dispatcher
}

impl EventProcessor {
    /// Process a raw event payload into an EventEntry
    pub fn process(&self, payload: &Value) -> Result<EventEntry> {
        match self {
            Self::Log => crate::events::log::process(payload),
            Self::Exception => crate::events::exception::process(payload),
            Self::Query => crate::events::query::process(payload),
            Self::ApplicationLog => crate::events::application_log::process(payload),
            Self::Cache => crate::events::cache::process(payload),
            Self::Http => crate::events::http::process(payload),
            // Table removed - was an anti-pattern dispatcher
        }
    }
}

/// Function type for custom event UI renderers
pub type EventUIRenderer = fn(&EventEntry, &mut Context<crate::app::MyApp>) -> gpui::Div;

/// Helper function to extract timestamp from event payload
pub fn extract_timestamp(payload: &Value) -> String {
    payload
        .get("timestamp")
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string()
}
