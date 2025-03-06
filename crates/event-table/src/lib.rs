#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct TableEvent;

impl EventProcessor for TableEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("table");

        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            if let Some(content) = v.get("content") {
                // Extract the label if available
                let label = content.get("label").and_then(Value::as_str).unwrap_or("");

                // Check if this is a cache event
                if label == "Cache" {
                    if let Some(values) = content.get("values") {
                        // Extract cache event details
                        let event = values
                            .get("Event")
                            .and_then(Value::as_str)
                            .unwrap_or("Unknown");

                        let key = values
                            .get("Key")
                            .and_then(Value::as_str)
                            .unwrap_or("Unknown");

                        // Remove HTML tags from event value if present
                        let clean_event = event.replace("<code>", "").replace("</code>", "");

                        // Format output with more readable presentation
                        entry.content = alloc::format!(
                            "## Cache Operation\n\n### Key\n```\n{}\n```\n\n### Event\n{}",
                            key,
                            clean_event
                        );

                        entry.content_type = "markdown".to_string();
                        entry.label = "Cache".to_string();
                        entry.description = alloc::format!("{} ({})", clean_event, key);
                    }
                }
                // Handle HTTP requests and responses
                else if label == "Http" {
                    if let Some(values) = content.get("values") {
                        // Check if this is a request or response
                        let method = values.get("Method").and_then(Value::as_str);
                        let status = values.get("Status").and_then(Value::as_u64);

                        let is_request = method.is_some();
                        let is_response = status.is_some();

                        // Get URL
                        let url = values.get("URL").and_then(Value::as_str).unwrap_or("");

                        let mut markdown = alloc::string::String::new();

                        if is_request {
                            // Start building markdown for request
                            markdown.push_str(&alloc::format!("## HTTP Request\n\n"));
                            markdown.push_str(&alloc::format!("**URL:** {}\n\n", url));
                            markdown.push_str(&alloc::format!(
                                "**Method:** {}\n\n",
                                method.unwrap_or("")
                            ));

                            // Add content type if available
                            if let Some(req_type) = values.get("Type").and_then(Value::as_str) {
                                markdown.push_str(&alloc::format!("**Type:** {}\n\n", req_type));
                            }

                            // Add headers section if available
                            if let Some(headers) = values.get("Headers").and_then(Value::as_object)
                            {
                                markdown.push_str("### Headers\n\n");
                                for (key, value) in headers {
                                    if let Some(val_str) = value.as_str() {
                                        markdown.push_str(&alloc::format!(
                                            "- **{}:** {}\n",
                                            key,
                                            val_str
                                        ));
                                    } else if let Some(val_array) = value.as_array() {
                                        let joined = val_array
                                            .iter()
                                            .filter_map(|v| v.as_str())
                                            .collect::<alloc::vec::Vec<_>>()
                                            .join(", ");
                                        markdown.push_str(&alloc::format!(
                                            "- **{}:** {}\n",
                                            key,
                                            joined
                                        ));
                                    }
                                }
                                markdown.push_str("\n");
                            }

                            // For JSON data
                            if let Some(data) = values.get("Data") {
                                markdown.push_str("### Request Body\n\n```json\n");
                                markdown.push_str(
                                    &serde_json::to_string_pretty(data).unwrap_or_default(),
                                );
                                markdown.push_str("\n```\n");
                            }
                            // For raw body
                            else if let Some(body) = values.get("Body").and_then(Value::as_str) {
                                markdown.push_str("### Request Body\n\n```\n");
                                markdown.push_str(body);
                                markdown.push_str("\n```\n");
                            }

                            entry.label = "HTTP Request".to_string();
                            entry.description = url.to_string();
                        } else if is_response {
                            // Start building markdown for response
                            let status_code = status.unwrap_or(0);
                            let success = values
                                .get("Success")
                                .and_then(Value::as_bool)
                                .unwrap_or(false);

                            markdown.push_str(&alloc::format!("## HTTP Response\n\n"));
                            markdown.push_str(&alloc::format!("**URL:** {}\n\n", url));
                            markdown.push_str(&alloc::format!(
                                "**Status:** {} ({})\n\n",
                                status_code,
                                if success { "Success" } else { "Failed" }
                            ));

                            // Add performance metrics in a detail section
                            markdown.push_str("### Performance\n\n");

                            // Add duration if available
                            if let Some(duration) = values.get("Duration").and_then(Value::as_f64) {
                                markdown.push_str(&alloc::format!(
                                    "- **Duration:** {:.6}s\n",
                                    duration
                                ));
                            }

                            // Add connection time if available
                            if let Some(conn_time) =
                                values.get("Connection time").and_then(Value::as_f64)
                            {
                                markdown.push_str(&alloc::format!(
                                    "- **Connection Time:** {:.6}s\n",
                                    conn_time
                                ));
                            }

                            // Add size information
                            if let Some(size) = values.get("Size").and_then(Value::as_u64) {
                                markdown.push_str(&alloc::format!("- **Size:** {} bytes\n", size));
                            }

                            // Add request size if available
                            if let Some(req_size) =
                                values.get("Request Size").and_then(Value::as_u64)
                            {
                                markdown.push_str(&alloc::format!(
                                    "- **Request Size:** {} bytes\n",
                                    req_size
                                ));
                            }

                            markdown.push_str("\n");

                            // Add headers section if available
                            if let Some(headers) = values.get("Headers").and_then(Value::as_object)
                            {
                                markdown.push_str("### Headers\n\n");
                                for (key, value) in headers {
                                    if let Some(val_str) = value.as_str() {
                                        markdown.push_str(&alloc::format!(
                                            "- **{}:** {}\n",
                                            key,
                                            val_str
                                        ));
                                    } else if let Some(val_array) = value.as_array() {
                                        let joined = val_array
                                            .iter()
                                            .filter_map(|v| v.as_str())
                                            .collect::<alloc::vec::Vec<_>>()
                                            .join(", ");
                                        markdown.push_str(&alloc::format!(
                                            "- **{}:** {}\n",
                                            key,
                                            joined
                                        ));
                                    }
                                }
                                markdown.push_str("\n");
                            }

                            // For response body
                            if let Some(body) = values.get("Body") {
                                if !body.is_null() {
                                    markdown.push_str("### Response Body\n\n```json\n");
                                    markdown.push_str(
                                        &serde_json::to_string_pretty(body).unwrap_or_default(),
                                    );
                                    markdown.push_str("\n```\n");
                                } else {
                                    markdown.push_str("### Response Body\n\n*No response body*\n");
                                }
                            }

                            entry.label = "HTTP Response".to_string();
                            entry.description = alloc::format!("{} - {}", url, status_code);
                        }

                        entry.content = markdown;
                        entry.content_type = "markdown".to_string();
                    }
                }
                // Default handling for any other table type
                else if let Some(values) = content.get("values") {
                    // Convert values to a more readable markdown table or representation
                    let mut markdown = alloc::format!("## {}\n\n", label);

                    // Convert fields to a bulleted list for better readability
                    if let Some(obj) = values.as_object() {
                        for (key, value) in obj {
                            if value.is_object() || value.is_array() {
                                markdown.push_str(&alloc::format!(
                                    "### {}\n\n```json\n{}\n```\n\n",
                                    key,
                                    serde_json::to_string_pretty(value).unwrap_or_default()
                                ));
                            } else if let Some(val_str) = value.as_str() {
                                markdown.push_str(&alloc::format!("- **{}:** {}\n", key, val_str));
                            } else if let Some(val_num) = value.as_u64() {
                                markdown.push_str(&alloc::format!("- **{}:** {}\n", key, val_num));
                            } else if let Some(val_bool) = value.as_bool() {
                                markdown.push_str(&alloc::format!("- **{}:** {}\n", key, val_bool));
                            } else if value.is_null() {
                                markdown.push_str(&alloc::format!("- **{}:** *null*\n", key));
                            } else {
                                markdown.push_str(&alloc::format!("- **{}:** {}\n", key, value));
                            }
                        }
                    } else {
                        // Fallback to raw JSON if not an object
                        markdown.push_str(&alloc::format!(
                            "```json\n{}\n```",
                            serde_json::to_string_pretty(values).unwrap_or_default()
                        ));
                    }

                    entry.content = markdown;
                    entry.content_type = "markdown".to_string();
                    entry.label = label.to_string();
                }
            }
        }

        entry
    }
}

implement_ffi_interface!(TableEvent);
