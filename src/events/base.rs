use anyhow::Result;
use serde_json::Value;
use gpui::Context;

/// Represents a processed event entry
#[derive(Clone, Debug)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content: String,
    pub content_type: String,
    pub event_type: String,      // Store the original event type
    pub raw_payload: Value,      // Store the original payload for custom rendering
}

/// Trait for processing events
pub trait EventProcessor: Send + Sync {
    /// Process a raw event payload into an EventEntry
    fn process(&self, payload: &Value) -> Result<EventEntry>;
}

/// Function type for custom event UI renderers
pub type EventUIRenderer = fn(&EventEntry, &mut Context<crate::app::MyApp>) -> gpui::Div;

/// Helper function to extract timestamp from event payload
pub fn extract_timestamp(payload: &Value) -> String {
    payload.get("timestamp")
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string()
}

/// Helper function to extract origin information from event payload
pub fn extract_origin_info(payload: &Value) -> Option<String> {
    let origin = payload.get("origin")?;
    
    let file = origin.get("file")?.as_str().unwrap_or("");
    let line = origin.get("line_number")?.as_u64().unwrap_or(0);
    let hostname = origin.get("hostname")?.as_str().unwrap_or("");
    
    if !file.is_empty() {
        Some(format!("{}:{} on {}", file, line, hostname))
    } else {
        None
    }
}