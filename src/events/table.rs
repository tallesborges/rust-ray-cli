use anyhow::Result;
use serde_json::Value;
use crate::events::base::{EventEntry, EventProcessor, extract_timestamp, extract_origin_info};

pub struct TableProcessor;

impl EventProcessor for TableProcessor {
    fn process(&self, payload: &Value) -> Result<EventEntry> {
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

            // Check if this is a cache event
            if label == "Cache" {
                self.process_cache_event(&mut entry, content)?;
            }
            // Handle HTTP requests and responses
            else if label == "Http" {
                self.process_http_event(&mut entry, content)?;
            }
            // Default handling for any other table type
            else if let Some(values) = content.get("values") {
                self.process_generic_table(&mut entry, label, values)?;
            }

            // Add origin information if available
            if let Some(origin) = extract_origin_info(payload) {
                entry.content.push_str(&format!("\n**Source:** {}\n", origin));
            }
        }

        Ok(entry)
    }

}

impl TableProcessor {
    fn process_cache_event(&self, entry: &mut EventEntry, content: &Value) -> Result<()> {
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
            let mut markdown = format!("## Cache Operation: {}\n\n", clean_event);

            // Show the cache key in a code block for better readability
            markdown.push_str(&format!("### Key\n```\n{}\n```\n\n", key));

            // Add expiration information if available
            if let Some(expiration) = values.get("Expiration in seconds").and_then(Value::as_u64) {
                let expiration_formatted = if expiration > 3600 {
                    format!("{:.2} hours ({} seconds)", expiration as f64 / 3600.0, expiration)
                } else if expiration > 60 {
                    format!("{:.1} minutes ({} seconds)", expiration as f64 / 60.0, expiration)
                } else {
                    format!("{} seconds", expiration)
                };

                markdown.push_str(&format!("### Expiration\n{}\n\n", expiration_formatted));
            }

            // Extract and format the value if present
            if let Some(val) = values.get("Value") {
                markdown.push_str("### Value\n");

                // Format based on value type
                if val.is_array() && val.as_array().map_or(false, |arr| arr.is_empty()) {
                    markdown.push_str("```json\n[]\n```\n\n");
                    markdown.push_str("*Empty array*\n");
                } else if val.is_object() && val.as_object().map_or(false, |obj| obj.is_empty()) {
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
                                markdown.push_str(&format!("\n*Size: {:.2} KB*\n", size as f64 / 1024.0));
                            }
                        }
                        Err(_) => markdown.push_str("```\n[Could not format value]\n```\n")
                    }
                }
            } else {
                markdown.push_str("### Value\n*No value provided*\n");
            }

            // Add additional cache metadata if available
            let mut has_metadata = false;
            let mut metadata = String::from("### Additional Information\n");

            // Check for and add various cache metadata fields
            for (field, label) in [("Tags", "Tags"), ("Store", "Cache Store"), ("TTL", "Original TTL")] {
                if let Some(field_value) = values.get(field).and_then(Value::as_str) {
                    if !has_metadata {
                        has_metadata = true;
                    }
                    metadata.push_str(&format!("- **{}**: {}\n", label, field_value));
                }
            }

            if has_metadata {
                markdown.push_str("\n");
                markdown.push_str(&metadata);
            }

            entry.content = markdown;
            entry.label = format!("Cache: {}", clean_event);

            // More detailed description for the event list
            entry.description = match clean_event.as_str() {
                "Hit" => format!("Cache hit for: {}", key),
                "Missed" => format!("Cache miss for: {}", key),
                "Key written" => format!("Cache write: {}", key),
                "Forgotten" => format!("Cache key forgotten: {}", key),
                _ => format!("{} ({})", clean_event, key),
            };
        }

        Ok(())
    }

    fn process_http_event(&self, entry: &mut EventEntry, content: &Value) -> Result<()> {
        if let Some(values) = content.get("values") {
            // Check if this is a request or response
            let method = values.get("Method").and_then(Value::as_str);
            let status = values.get("Status").and_then(Value::as_u64);

            let is_request = method.is_some();
            let is_response = status.is_some();

            // Get URL
            let url = values.get("URL").and_then(Value::as_str).unwrap_or("");

            let mut markdown = String::new();

            if is_request {
                self.process_http_request(&mut markdown, values, url, method.unwrap_or(""))?;
                entry.label = "HTTP: Request".to_string();
                entry.description = url.to_string();
            } else if is_response {
                self.process_http_response(&mut markdown, values, url, status.unwrap_or(0))?;
                entry.label = "HTTP: Response".to_string();
                entry.description = format!("{} - {}", url, status.unwrap_or(0));
            }

            entry.content = markdown;
        }

        Ok(())
    }

    fn process_http_request(&self, markdown: &mut String, values: &Value, url: &str, method: &str) -> Result<()> {
        // Start building markdown for request
        markdown.push_str("## HTTP Request\n\n");
        markdown.push_str(&format!("**URL:** {}\n\n", url));
        markdown.push_str(&format!("**Method:** {}\n\n", method));

        // Add content type if available
        if let Some(req_type) = values.get("Type").and_then(Value::as_str) {
            markdown.push_str(&format!("**Type:** {}\n\n", req_type));
        }

        // Add headers section if available
        if let Some(headers) = values.get("Headers").and_then(Value::as_object) {
            markdown.push_str("### Headers\n\n");
            for (key, value) in headers {
                if let Some(val_str) = value.as_str() {
                    markdown.push_str(&format!("- **{}:** {}\n", key, val_str));
                } else if let Some(val_array) = value.as_array() {
                    let joined = val_array
                        .iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    markdown.push_str(&format!("- **{}:** {}\n", key, joined));
                }
            }
            markdown.push_str("\n");
        }

        // For JSON data
        if let Some(data) = values.get("Data") {
            markdown.push_str("### Request Body\n\n```json\n");
            markdown.push_str(&serde_json::to_string_pretty(data).unwrap_or_default());
            markdown.push_str("\n```\n");
        }
        // For raw body
        else if let Some(body) = values.get("Body").and_then(Value::as_str) {
            markdown.push_str("### Request Body\n\n```\n");
            markdown.push_str(body);
            markdown.push_str("\n```\n");
        }

        Ok(())
    }

    fn process_http_response(&self, markdown: &mut String, values: &Value, url: &str, status_code: u64) -> Result<()> {
        // Start building markdown for response
        let success = values.get("Success").and_then(Value::as_bool).unwrap_or(false);

        markdown.push_str("## HTTP Response\n\n");
        markdown.push_str(&format!("**URL:** {}\n\n", url));
        markdown.push_str(&format!(
            "**Status:** {} ({})\n\n",
            status_code,
            if success { "Success" } else { "Failed" }
        ));

        // Add performance metrics in a detail section
        markdown.push_str("### Performance\n\n");

        // Add duration if available
        if let Some(duration) = values.get("Duration").and_then(Value::as_f64) {
            markdown.push_str(&format!("- **Duration:** {:.6}s\n", duration));
        }

        // Add connection time if available
        if let Some(conn_time) = values.get("Connection time").and_then(Value::as_f64) {
            markdown.push_str(&format!("- **Connection Time:** {:.6}s\n", conn_time));
        }

        // Add size information
        if let Some(size) = values.get("Size").and_then(Value::as_u64) {
            markdown.push_str(&format!("- **Size:** {} bytes\n", size));
        }

        // Add request size if available
        if let Some(req_size) = values.get("Request Size").and_then(Value::as_u64) {
            markdown.push_str(&format!("- **Request Size:** {} bytes\n", req_size));
        }

        markdown.push_str("\n");

        // Add headers section if available
        if let Some(headers) = values.get("Headers").and_then(Value::as_object) {
            markdown.push_str("### Headers\n\n");
            for (key, value) in headers {
                if let Some(val_str) = value.as_str() {
                    markdown.push_str(&format!("- **{}:** {}\n", key, val_str));
                } else if let Some(val_array) = value.as_array() {
                    let joined = val_array
                        .iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    markdown.push_str(&format!("- **{}:** {}\n", key, joined));
                }
            }
            markdown.push_str("\n");
        }

        // For response body
        if let Some(body) = values.get("Body") {
            if !body.is_null() {
                markdown.push_str("### Response Body\n\n```json\n");
                markdown.push_str(&serde_json::to_string_pretty(body).unwrap_or_default());
                markdown.push_str("\n```\n");
            } else {
                markdown.push_str("### Response Body\n\n*No response body*\n");
            }
        }

        Ok(())
    }

    fn process_generic_table(&self, entry: &mut EventEntry, label: &str, values: &Value) -> Result<()> {
        // Convert values to a more readable markdown table or representation
        let mut markdown = format!("## {}\n\n", label);

        // Convert fields to a bulleted list for better readability
        if let Some(obj) = values.as_object() {
            for (key, value) in obj {
                if value.is_object() || value.is_array() {
                    markdown.push_str(&format!(
                        "### {}\n\n```json\n{}\n```\n\n",
                        key,
                        serde_json::to_string_pretty(value).unwrap_or_default()
                    ));
                } else if let Some(val_str) = value.as_str() {
                    markdown.push_str(&format!("- **{}:** {}\n", key, val_str));
                } else if let Some(val_num) = value.as_u64() {
                    markdown.push_str(&format!("- **{}:** {}\n", key, val_num));
                } else if let Some(val_bool) = value.as_bool() {
                    markdown.push_str(&format!("- **{}:** {}\n", key, val_bool));
                } else if value.is_null() {
                    markdown.push_str(&format!("- **{}:** *null*\n", key));
                } else {
                    markdown.push_str(&format!("- **{}:** {}\n", key, value));
                }
            }
        } else {
            // Fallback to raw JSON if not an object
            markdown.push_str(&format!(
                "```json\n{}\n```",
                serde_json::to_string_pretty(values).unwrap_or_default()
            ));
        }

        entry.content = markdown;
        entry.label = label.to_string();
        entry.description = format!("{} data", label);

        Ok(())
    }
}