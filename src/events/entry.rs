use serde_json::Value;

/// Represents a processed event entry ready for display
#[derive(Clone, Debug)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub event_type: String,
    pub raw_payload: Value,
}

impl EventEntry {
    pub fn new(
        event_type: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
        payload: &Value,
    ) -> Self {
        Self {
            timestamp: extract_timestamp(payload),
            label: label.into(),
            description: description.into(),
            event_type: event_type.into(),
            raw_payload: payload.clone(),
        }
    }
}

/// Helper function to extract timestamp from event payload
pub fn extract_timestamp(payload: &Value) -> String {
    payload
        .get("timestamp")
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string()
}
