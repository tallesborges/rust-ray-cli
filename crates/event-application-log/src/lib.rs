#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct ApplicationLogEvent;

impl EventProcessor for ApplicationLogEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("application_log");
        entry.label = "Application Log".to_string();

        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            if let Some(content) = v.get("content") {
                let value = content
                    .get("value")
                    .and_then(Value::as_str)
                    .unwrap_or_default();

                // Create a description from the log value (truncated if needed)
                if !value.is_empty() {
                    let description = if value.len() > 50 {
                        alloc::format!("{} ...", &value[..50].trim())
                    } else {
                        value.to_string()
                    };
                    entry.description = description;
                }

                // Create rich markdown content
                let mut markdown = alloc::string::String::from("## Application Log\n\n");

                // Add source information if available
                if let Some(origin) = v.get("origin") {
                    markdown.push_str("### Source\n\n");

                    if let Some(file) = origin.get("file").and_then(Value::as_str) {
                        // Extract just the filename from the path for cleaner display
                        let filename = file.split('/').last().unwrap_or(file);
                        markdown.push_str(&alloc::format!("- **File:** {}\n", filename));

                        // Add the full path in smaller text
                        markdown.push_str(&alloc::format!("  <small>{}</small>\n", file));
                    }

                    if let Some(line) = origin.get("line_number").and_then(Value::as_u64) {
                        markdown.push_str(&alloc::format!("- **Line:** {}\n", line));
                    }

                    if let Some(hostname) = origin.get("hostname").and_then(Value::as_str) {
                        markdown.push_str(&alloc::format!("- **Host:** {}\n", hostname));
                    }

                    markdown.push_str("\n");
                }

                // Add log content in a code block
                markdown.push_str("### Log Content\n\n```\n");
                markdown.push_str(value);
                markdown.push_str("\n```\n");

                entry.content = markdown;
                entry.content_type = "markdown".to_string();
            }
        } else {
            // Fallback if JSON parsing fails
            entry.content = alloc::format!("```\n{}\n```", payload);
            entry.description = if payload.len() > 50 {
                alloc::format!("{} ...", &payload[..50].trim())
            } else {
                payload.to_string()
            };
            entry.content_type = "markdown".to_string();
        }

        entry
    }
}

implement_ffi_interface!(ApplicationLogEvent);
