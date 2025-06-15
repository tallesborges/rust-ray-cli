use crate::events::base::{extract_origin_info, extract_timestamp, EventEntry};
use crate::events::processors::{process_cache_event, process_http_event, process_table_event};
use crate::events::renderers::{
    get_cache_description, get_cache_label, get_http_description, get_http_label,
    get_table_description, get_table_label, render_cache_markdown, render_http_markdown,
    render_table_markdown,
};
use crate::events::types::ProcessedEvent;
use anyhow::Result;
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "Table".to_string(),
        description: String::new(),
        content: String::new(),
        content_type: "markdown".to_string(),
        event_type: "table".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Extract the label if available
        let label = content.get("label").and_then(Value::as_str).unwrap_or("");

        // Process using the new architecture
        let processed_event = if label == "Cache" {
            process_cache_event(content)?
        } else if label == "Http" {
            process_http_event(content)?
        } else if let Some(values) = content.get("values") {
            process_table_event(label, values)?
        } else {
            return Err(anyhow::anyhow!("Unknown table event format"));
        };

        // Render using the appropriate renderer
        match processed_event {
            ProcessedEvent::Cache(ref cache_event) => {
                entry.content = render_cache_markdown(cache_event);
                entry.label = get_cache_label(cache_event);
                entry.description = get_cache_description(cache_event);
            }
            ProcessedEvent::Http(ref http_event) => {
                entry.content = render_http_markdown(http_event);
                entry.label = get_http_label(http_event);
                entry.description = get_http_description(http_event);
            }
            ProcessedEvent::Table(ref table_event) => {
                entry.content = render_table_markdown(table_event);
                entry.label = get_table_label(table_event);
                entry.description = get_table_description(table_event);
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unexpected event type from table processor"
                ));
            }
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
