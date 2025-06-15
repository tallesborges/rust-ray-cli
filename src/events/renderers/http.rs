use crate::events::types::{HttpEvent, HttpEventType};

pub fn render_http_markdown(http_event: &HttpEvent) -> String {
    match http_event.event_type {
        HttpEventType::Request => render_http_request_markdown(http_event),
        HttpEventType::Response => render_http_response_markdown(http_event),
    }
}

fn render_http_request_markdown(http_event: &HttpEvent) -> String {
    let mut markdown = String::from("## HTTP Request\n\n");

    markdown.push_str(&format!("**URL:** {}\n\n", http_event.url));

    if let Some(method) = &http_event.method {
        markdown.push_str(&format!("**Method:** {}\n\n", method));
    }

    if let Some(content_type) = &http_event.content_type {
        markdown.push_str(&format!("**Type:** {}\n\n", content_type));
    }

    // Add headers section if available
    if !http_event.headers.is_empty() {
        markdown.push_str("### Headers\n\n");
        for (key, value) in &http_event.headers {
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

    // Add request body if available
    if let Some(body) = &http_event.body {
        if body.is_object() || body.is_array() {
            markdown.push_str("### Request Body\n\n```json\n");
            markdown.push_str(&serde_json::to_string_pretty(body).unwrap_or_default());
            markdown.push_str("\n```\n");
        } else if let Some(body_str) = body.as_str() {
            markdown.push_str("### Request Body\n\n```\n");
            markdown.push_str(body_str);
            markdown.push_str("\n```\n");
        }
    }

    markdown
}

fn render_http_response_markdown(http_event: &HttpEvent) -> String {
    let mut markdown = String::from("## HTTP Response\n\n");

    markdown.push_str(&format!("**URL:** {}\n\n", http_event.url));

    if let Some(status_code) = http_event.status_code {
        let success_text = http_event
            .success
            .map(|s| if s { "Success" } else { "Failed" })
            .unwrap_or("Unknown");

        markdown.push_str(&format!(
            "**Status:** {} ({})\n\n",
            status_code, success_text
        ));
    }

    // Add performance metrics in a detail section
    markdown.push_str("### Performance\n\n");

    if let Some(duration) = http_event.duration_seconds {
        markdown.push_str(&format!("- **Duration:** {:.6}s\n", duration));
    }

    if let Some(conn_time) = http_event.connection_time_seconds {
        markdown.push_str(&format!("- **Connection Time:** {:.6}s\n", conn_time));
    }

    if let Some(size) = http_event.size_bytes {
        markdown.push_str(&format!("- **Size:** {} bytes\n", size));
    }

    if let Some(req_size) = http_event.request_size_bytes {
        markdown.push_str(&format!("- **Request Size:** {} bytes\n", req_size));
    }

    markdown.push_str("\n");

    // Add headers section if available
    if !http_event.headers.is_empty() {
        markdown.push_str("### Headers\n\n");
        for (key, value) in &http_event.headers {
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

    // Add response body if available
    if let Some(body) = &http_event.body {
        if !body.is_null() {
            markdown.push_str("### Response Body\n\n```json\n");
            markdown.push_str(&serde_json::to_string_pretty(body).unwrap_or_default());
            markdown.push_str("\n```\n");
        } else {
            markdown.push_str("### Response Body\n\n*No response body*\n");
        }
    }

    markdown
}

pub fn get_http_label(http_event: &HttpEvent) -> String {
    match http_event.event_type {
        HttpEventType::Request => "HTTP: Request".to_string(),
        HttpEventType::Response => "HTTP: Response".to_string(),
    }
}

pub fn get_http_description(http_event: &HttpEvent) -> String {
    match http_event.event_type {
        HttpEventType::Request => http_event.url.clone(),
        HttpEventType::Response => {
            if let Some(status_code) = http_event.status_code {
                format!("{} - {}", http_event.url, status_code)
            } else {
                http_event.url.clone()
            }
        }
    }
}
