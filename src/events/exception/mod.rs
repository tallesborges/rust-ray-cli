use anyhow::Result;
use serde_json::Value;
use crate::events::base::{EventEntry, EventProcessor, extract_timestamp, extract_origin_info};

pub struct ExceptionProcessor;

impl EventProcessor for ExceptionProcessor {
    fn process(&self, payload: &Value) -> Result<EventEntry> {
        let mut entry = EventEntry {
            timestamp: extract_timestamp(payload),
            label: "Exception".to_string(),
            description: String::new(),
            content: String::new(),
            content_type: "markdown".to_string(),
        };

        if let Some(content) = payload.get("content") {
            // Get exception class and message
            let class = content
                .get("class")
                .and_then(|c| c.as_str())
                .unwrap_or("Unknown Exception");
            let message = content
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("");

            // Set description for list view
            entry.description = if !message.is_empty() {
                format!("{}: {}", class, message)
            } else {
                class.to_string()
            };

            // Truncate long descriptions
            if entry.description.len() > 100 {
                entry.description.truncate(97);
                entry.description.push_str("...");
            }

            // Start building markdown content
            let mut markdown = format!("## {}\n\n", class);

            if !message.is_empty() {
                markdown.push_str(&format!("**Error:** {}\n\n", message));
            }

            // Process stack trace if available
            if let Some(frames) = content.get("frames").and_then(|f| f.as_array()) {
                markdown.push_str("### Stack Trace\n\n");

                for (i, frame) in frames.iter().enumerate() {
                    let class = frame.get("class").and_then(|c| c.as_str()).unwrap_or("");
                    let method = frame.get("method").and_then(|m| m.as_str()).unwrap_or("");
                    let file = frame
                        .get("file_name")
                        .and_then(|f| f.as_str())
                        .unwrap_or("");
                    let line = frame
                        .get("line_number")
                        .and_then(|l| l.as_u64())
                        .unwrap_or(0);

                    markdown.push_str(&format!(
                        "{}. **{}**::**{}**() at {}:{}\n\n",
                        i + 1,
                        class,
                        method,
                        file,
                        line
                    ));

                    // Include code snippet if available
                    if let Some(snippet) = frame.get("snippet").and_then(|s| s.as_array()) {
                        markdown.push_str("```php\n");

                        for line_info in snippet {
                            let line_num = line_info
                                .get("line_number")
                                .and_then(|l| l.as_u64())
                                .unwrap_or(0);
                            let text = line_info.get("text").and_then(|t| t.as_str()).unwrap_or("");
                            let prefix = if line_num == line { "→ " } else { "  " };

                            markdown.push_str(&format!("{}{}: {}\n", prefix, line_num, text));
                        }

                        markdown.push_str("```\n\n");
                    }
                }
            }

            // Add origin information if available
            if let Some(origin) = extract_origin_info(payload) {
                markdown.push_str(&format!("**Source:** {}\n\n", origin));
            }

            entry.content = markdown;
        }

        Ok(entry)
    }

    fn display_name(&self) -> &'static str {
        "Exception"
    }
}