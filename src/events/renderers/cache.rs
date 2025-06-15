use crate::events::types::CacheEvent;

pub fn render_cache_markdown(cache_event: &CacheEvent) -> String {
    let mut markdown = format!("## Cache Operation: {}\n\n", cache_event.operation);

    // Show the cache key in a code block for better readability
    markdown.push_str(&format!("### Key\n```\n{}\n```\n\n", cache_event.key));

    // Add expiration information if available
    if let Some(expiration) = cache_event.expiration_seconds {
        let expiration_formatted = if expiration > 3600 {
            format!(
                "{:.2} hours ({} seconds)",
                expiration as f64 / 3600.0,
                expiration
            )
        } else if expiration > 60 {
            format!(
                "{:.1} minutes ({} seconds)",
                expiration as f64 / 60.0,
                expiration
            )
        } else {
            format!("{} seconds", expiration)
        };

        markdown.push_str(&format!("### Expiration\n{}\n\n", expiration_formatted));
    }

    // Extract and format the value if present
    if let Some(val) = &cache_event.value {
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
                Err(_) => markdown.push_str("```\n[Could not format value]\n```\n"),
            }
        }
    } else {
        markdown.push_str("### Value\n*No value provided*\n");
    }

    // Add additional cache metadata if available
    let mut has_metadata = false;
    let mut metadata = String::from("### Additional Information\n");

    if let Some(tags) = &cache_event.tags {
        if !has_metadata {
            has_metadata = true;
        }
        metadata.push_str(&format!("- **Tags**: {}\n", tags));
    }

    if let Some(store) = &cache_event.store {
        if !has_metadata {
            has_metadata = true;
        }
        metadata.push_str(&format!("- **Cache Store**: {}\n", store));
    }

    if let Some(ttl) = &cache_event.ttl {
        if !has_metadata {
            has_metadata = true;
        }
        metadata.push_str(&format!("- **Original TTL**: {}\n", ttl));
    }

    if has_metadata {
        markdown.push_str("\n");
        markdown.push_str(&metadata);
    }

    markdown
}

pub fn get_cache_label(cache_event: &CacheEvent) -> String {
    format!("Cache: {}", cache_event.operation)
}

pub fn get_cache_description(cache_event: &CacheEvent) -> String {
    match cache_event.operation.as_str() {
        "Hit" => format!("Cache hit for: {}", cache_event.key),
        "Missed" => format!("Cache miss for: {}", cache_event.key),
        "Key written" => format!("Cache write: {}", cache_event.key),
        "Forgotten" => format!("Cache key forgotten: {}", cache_event.key),
        _ => format!("{} ({})", cache_event.operation, cache_event.key),
    }
}
