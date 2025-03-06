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

                        // Clean HTML tags from event value
                        let clean_event = event.replace("<code>", "").replace("</code>", "");

                        // Build a more informative cache event display
                        let mut markdown =
                            alloc::format!("## Cache Operation: {}\n\n", clean_event);

                        // Show the cache key in a code block for better readability
                        markdown.push_str(&alloc::format!("### Key\n```\n{}\n```\n\n", key));

                        // Add expiration information if available
                        if let Some(expiration) =
                            values.get("Expiration in seconds").and_then(Value::as_u64)
                        {
                            let expiration_formatted = if expiration > 3600 {
                                alloc::format!(
                                    "{:.2} hours ({} seconds)",
                                    expiration as f64 / 3600.0,
                                    expiration
                                )
                            } else if expiration > 60 {
                                alloc::format!(
                                    "{:.1} minutes ({} seconds)",
                                    expiration as f64 / 60.0,
                                    expiration
                                )
                            } else {
                                alloc::format!("{} seconds", expiration)
                            };

                            markdown.push_str(&alloc::format!(
                                "### Expiration\n{}\n\n",
                                expiration_formatted
                            ));
                        }

                        // Extract and format the value if present
                        if let Some(val) = values.get("Value") {
                            markdown.push_str("### Value\n");

                            // Format based on value type
                            if val.is_array() && val.as_array().map_or(false, |arr| arr.is_empty())
                            {
                                markdown.push_str("```json\n[]\n```\n\n");
                                markdown.push_str("*Empty array*\n");
                            } else if val.is_object()
                                && val.as_object().map_or(false, |obj| obj.is_empty())
                            {
                                markdown.push_str("```json\n{}\n```\n\n");
                                markdown.push_str("*Empty object*\n");
                            } else if val.is_null() {
                                markdown.push_str("```\nnull\n```\n\n");
                                markdown.push_str("*Null value*\n");
                            } else {
                                match serde_json::to_string_pretty(val) {
                                    Ok(pretty_val) => {
                                        markdown.push_str("```json\n");
                                        markdown.push_str(&pretty_val);
                                        markdown.push_str("\n```\n");

                                        // Add value size info for large values
                                        let size = pretty_val.len();
                                        if size > 1000 {
                                            markdown.push_str(&alloc::format!(
                                                "\n*Size: {:.2} KB*\n",
                                                size as f64 / 1024.0
                                            ));
                                        }
                                    }
                                    Err(_) => {
                                        markdown.push_str("```\n[Could not format value]\n```\n")
                                    }
                                }
                            }
                        } else {
                            markdown.push_str("### Value\n*No value provided*\n");
                        }

                        // Add additional cache metadata if available
                        let mut has_metadata = false;
                        let mut metadata =
                            alloc::string::String::from("### Additional Information\n");

                        // Check for and add various cache metadata fields
                        for (field, label) in [
                            ("Tags", "Tags"),
                            ("Store", "Cache Store"),
                            ("TTL", "Original TTL"),
                        ] {
                            if let Some(field_value) = values.get(field).and_then(Value::as_str) {
                                if !has_metadata {
                                    has_metadata = true;
                                }
                                metadata.push_str(&alloc::format!(
                                    "- **{}**: {}\n",
                                    label,
                                    field_value
                                ));
                            }
                        }

                        if has_metadata {
                            markdown.push_str("\n");
                            markdown.push_str(&metadata);
                        }

                        entry.content = markdown;
                        entry.content_type = "markdown".to_string();
                        entry.label = alloc::format!("Cache: {}", clean_event);

                        // More detailed description for the event list
                        let description = match clean_event.as_str() {
                            "Hit" => alloc::format!("Cache hit for: {}", key),
                            "Missed" => alloc::format!("Cache miss for: {}", key),
                            "Key written" => alloc::format!("Cache write: {}", key),
                            "Forgotten" => alloc::format!("Cache key forgotten: {}", key),
                            _ => alloc::format!("{} ({})", clean_event, key),
                        };
                        entry.description = description;
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

                            entry.label = "HTTP: Request".to_string();
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

                            entry.label = "HTTP: Response".to_string();
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
