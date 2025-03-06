#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct ExceptionEvent;

impl EventProcessor for ExceptionEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("exception");

        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            if let Some(content) = v.get("content") {
                // Get exception class and message
                let class = content
                    .get("class")
                    .and_then(|c| c.as_str())
                    .unwrap_or("Unknown Exception");
                let message = content
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("");

                // Start building markdown content
                let mut markdown = alloc::format!("## {}\n\n", class);

                if !message.is_empty() {
                    markdown.push_str(&alloc::format!("**Error:** {}\n\n", message));
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

                        markdown.push_str(&alloc::format!(
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
                                let text =
                                    line_info.get("text").and_then(|t| t.as_str()).unwrap_or("");
                                let prefix = if line_num == line { "→ " } else { "  " };

                                markdown.push_str(&alloc::format!(
                                    "{}{}: {}\n",
                                    prefix,
                                    line_num,
                                    text
                                ));
                            }

                            markdown.push_str("```\n\n");
                        }
                    }
                }

                entry.content = markdown;
                entry.content_type = "markdown".to_string();
            }
        }

        entry
    }
}

implement_ffi_interface!(ExceptionEvent);
