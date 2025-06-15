use anyhow::Result;
use serde_json::Value;

/// Represents a processed event entry
#[derive(Clone, Debug)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content: String,
    pub content_type: String,
}

/// Trait for processing events
pub trait EventProcessor: Send + Sync {
    /// Process a raw event payload into an EventEntry
    fn process(&self, payload: &Value) -> Result<EventEntry>;
    
    /// Get the display name for this event type
    fn display_name(&self) -> &'static str;
}

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