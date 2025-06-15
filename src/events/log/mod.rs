use anyhow::Result;
use serde_json::Value;
use crate::events::base::{EventEntry, EventProcessor, extract_timestamp, extract_origin_info};

pub struct LogProcessor;

impl EventProcessor for LogProcessor {
    fn process(&self, payload: &Value) -> Result<EventEntry> {
        let mut entry = EventEntry {
            timestamp: extract_timestamp(payload),
            label: "log".to_string(),
            description: String::new(),
            content: String::new(),
            content_type: "markdown".to_string(),
        };

        // Extract log values from content
        if let Some(content) = payload.get("content") {
            if let Some(values) = content.get("values") {
                if let Ok(pretty_json) = serde_json::to_string_pretty(values) {
                    entry.content = format!("```json\n{}\n```", pretty_json);
                    
                    // Create a short description for the list view
                    if let Some(first_value) = values.as_array().and_then(|arr| arr.first()) {
                        entry.description = match first_value {
                            Value::String(s) => s.clone(),
                            _ => first_value.to_string(),
                        };
                        // Truncate long descriptions
                        if entry.description.len() > 100 {
                            entry.description.truncate(97);
                            entry.description.push_str("...");
                        }
                    }
                }
            }
        }

        // Add origin information if available
        if let Some(origin) = extract_origin_info(payload) {
            entry.content.push_str(&format!("\n\n**Source:** {}", origin));
        }

        Ok(entry)
    }

    fn display_name(&self) -> &'static str {
        "Log"
    }
}