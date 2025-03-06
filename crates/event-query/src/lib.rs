#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct QueryEvent;

impl EventProcessor for QueryEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("query");

        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            if let Some(content) = v.get("content") {
                let sql = content
                    .get("sql")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let connection = content
                    .get("connection_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let time = content
                    .get("time")
                    .and_then(|v| v.as_f64())
                    .unwrap_or_default();

                // Extract the SQL operation type (SELECT, INSERT, UPDATE, etc.)
                let operation_type = if let Some(first_word) = sql.trim().split_whitespace().next()
                {
                    first_word.to_uppercase()
                } else {
                    "SQL".to_string()
                };

                // Generate a descriptive label
                entry.label = alloc::format!("Query: {}", operation_type);

                // Generate a more informative description from the SQL
                let description = if sql.len() > 50 {
                    // Truncate for description but keep full query in content
                    alloc::format!("{} ({}ms)", &sql[..50].trim(), time)
                } else {
                    alloc::format!("{} ({}ms)", sql.trim(), time)
                };
                entry.description = description;

                // Create a rich markdown presentation
                let mut markdown = alloc::string::String::from("## SQL Query\n\n");

                // Add basic query information
                markdown.push_str(&alloc::format!("**Operation:** {}\n\n", operation_type));
                markdown.push_str(&alloc::format!("**Connection:** {}\n\n", connection));

                // Format execution time with appropriate units
                let time_display = if time < 1.0 {
                    alloc::format!("{:.3} ms", time)
                } else if time < 1000.0 {
                    alloc::format!("{:.2} ms", time)
                } else {
                    alloc::format!("{:.2} s", time / 1000.0)
                };

                markdown.push_str(&alloc::format!("**Execution Time:** {}\n\n", time_display));

                // Add SQL in a code block with syntax highlighting
                markdown.push_str("### Query\n\n```sql\n");
                markdown.push_str(sql);
                markdown.push_str("\n```\n");

                // Add source information if available
                if let Some(origin) = v.get("origin") {
                    markdown.push_str("\n### Source\n\n");

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
                }

                entry.content = markdown;
                entry.content_type = "markdown".to_string();
            }
        }

        entry
    }
}

implement_ffi_interface!(QueryEvent);
