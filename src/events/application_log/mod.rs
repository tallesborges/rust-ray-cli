use anyhow::Result;
use serde_json::Value;
use crate::events::base::{EventEntry, EventProcessor, extract_timestamp, extract_origin_info};

pub struct ApplicationLogProcessor;

impl EventProcessor for ApplicationLogProcessor {
    fn process(&self, payload: &Value) -> Result<EventEntry> {
        let mut entry = EventEntry {
            timestamp: extract_timestamp(payload),
            label: "Application Log".to_string(),
            description: String::new(),
            content: String::new(),
            content_type: "markdown".to_string(),
            event_type: "application_log".to_string(),
            raw_payload: payload.clone(),
        };

        if let Some(content) = payload.get("content") {
            let value = content
                .get("value")
                .and_then(Value::as_str)
                .unwrap_or_default();

            // Create a description from the log value (truncated if needed)
            if !value.is_empty() {
                entry.description = if value.len() > 50 {
                    format!("{}...", &value[..50].trim())
                } else {
                    value.to_string()
                };
            }

            // Create rich markdown content
            let mut markdown = String::from("## Application Log\n\n");

            // Add source information if available
            if let Some(origin) = extract_origin_info(payload) {
                markdown.push_str(&format!("**Source:** {}\n\n", origin));
            }

            // Add log content in a code block
            markdown.push_str("### Log Content\n\n```\n");
            markdown.push_str(value);
            markdown.push_str("\n```\n");

            entry.content = markdown;
        }

        Ok(entry)
    }

    fn display_name(&self) -> &'static str {
        "Application Log"
    }
}