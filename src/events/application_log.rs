use crate::events::base::{extract_origin_info, extract_timestamp, EventEntry};
use crate::events::processors::process_application_log_event;
use crate::events::renderers::{render_application_log_markdown, get_application_log_label, get_application_log_description};
use crate::events::types::ProcessedEvent;
use anyhow::Result;
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
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
        // Process using the new architecture
        let processed_event = process_application_log_event(content)?;

        // Render using the appropriate renderer
        if let ProcessedEvent::ApplicationLog(ref app_log_event) = processed_event {
            entry.content = render_application_log_markdown(app_log_event);
            entry.label = get_application_log_label(app_log_event);
            entry.description = get_application_log_description(app_log_event);
        } else {
            return Err(anyhow::anyhow!("Unexpected event type from application log processor"));
        }

        // Add origin information if available
        if let Some(origin) = extract_origin_info(payload) {
            entry
                .content
                .push_str(&format!("\n**Source:** {}\n", origin));
        }
    }

    Ok(entry)
}
