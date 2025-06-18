use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::{process_cache_event, process_http_event, process_table_event};
use crate::events::types::{ProcessedEvent, HttpEventType};
use crate::ui_components::{
    background_color, border_color, styled_card, text_monospace_color, text_secondary_color,
};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, Context, Div};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "Table".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
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

        // Set labels and descriptions based on event type
        match processed_event {
            ProcessedEvent::Cache(ref cache_event) => {
                entry.label = format!("Cache: {}", cache_event.operation);
                entry.description = match cache_event.operation.as_str() {
                    "Hit" => format!("Cache hit for: {}", cache_event.key),
                    "Missed" => format!("Cache miss for: {}", cache_event.key),
                    "Key written" => format!("Cache write: {}", cache_event.key),
                    "Forgotten" => format!("Cache key forgotten: {}", cache_event.key),
                    _ => format!("{} ({})", cache_event.operation, cache_event.key),
                };
            }
            ProcessedEvent::Http(ref http_event) => {
                match http_event.event_type {
                    HttpEventType::Request => {
                        entry.label = "HTTP: Request".to_string();
                        entry.description = http_event.url.clone();
                    }
                    HttpEventType::Response => {
                        entry.label = "HTTP: Response".to_string();
                        entry.description = if let Some(status_code) = http_event.status_code {
                            format!("{} - {}", http_event.url, status_code)
                        } else {
                            http_event.url.clone()
                        };
                    }
                }
            }
            ProcessedEvent::Table(ref table_event) => {
                entry.label = table_event.label.clone();
                entry.description = format!("{} data", table_event.label);
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unexpected event type from table processor"
                ));
            }
        }

    }

    Ok(entry)
}

pub fn render_table_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    let content = entry
        .raw_payload
        .get("content")
        .cloned()
        .unwrap_or_default();

    let label = content.get("label").and_then(Value::as_str).unwrap_or("");

    if label == "Cache" {
        render_cache_ui(&content, entry)
    } else if label == "Http" {
        crate::events::http::render_table_http_event(&content, entry)
    } else {
        render_generic_table_ui(&content, entry, label)
    }
}

fn render_cache_ui(content: &Value, entry: &EventEntry) -> Div {
    let values = content.get("values").unwrap_or(&Value::Null);
    
    let operation = values
        .get("Event")
        .and_then(Value::as_str)
        .unwrap_or("Unknown")
        .replace("<code>", "")
        .replace("</code>", "");
    
    let key = values
        .get("Key")
        .and_then(Value::as_str)
        .unwrap_or("Unknown");

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(render_cache_header(&operation))
        .child(render_cache_key(key.to_string()))
        .child(render_cache_value(values))
        .child(render_cache_metadata(values))
        .child(render_origin_info(entry))
}

fn render_cache_header(operation: &str) -> Div {
    styled_card().p_4().child(
        div()
            .flex()
            .flex_row()
            .gap_3()
            .items_center()
            .child(div().text_2xl().child("🗄️"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(gpui::rgb(0x3b82f6)) // Blue
                            .child(format!("Cache: {}", operation)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary_color())
                            .child("Cache Operation"),
                    ),
            ),
    )
}

fn render_cache_key(key: String) -> Div {
    styled_card().p_4().child(
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(text_secondary_color())
                    .child("🔑 Cache Key"),
            )
            .child(
                div()
                    .p_3()
                    .bg(background_color())
                    .border_1()
                    .border_color(border_color())
                    .rounded_lg()
                    .child(
                        div()
                            .font_family("monospace")
                            .text_sm()
                            .text_color(text_monospace_color())
                            .child(key),
                    ),
            ),
    )
}

fn render_cache_value(values: &Value) -> Div {
    if let Some(val) = values.get("Value") {
        styled_card().p_4().child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(text_secondary_color())
                        .child("💾 Cache Value"),
                )
                .child(
                    div()
                        .p_3()
                        .bg(gpui::rgb(0x1f2937))
                        .rounded_lg()
                        .max_h_64()
                        .overflow_hidden()
                        .child(
                            div()
                                .font_family("monospace")
                                .text_xs()
                                .text_color(gpui::rgb(0xd1d5db))
                                .child(serde_json::to_string_pretty(val).unwrap_or_default()),
                        ),
                ),
        )
    } else {
        div() // Empty div if no value
    }
}

fn render_cache_metadata(values: &Value) -> Div {
    let mut has_metadata = false;
    let mut metadata_items: Vec<(String, String)> = Vec::new();

    // Check for expiration
    if let Some(expiration) = values.get("Expiration in seconds").and_then(Value::as_u64) {
        has_metadata = true;
        let expiration_text = if expiration > 3600 {
            format!("{:.2} hours ({} seconds)", expiration as f64 / 3600.0, expiration)
        } else if expiration > 60 {
            format!("{:.1} minutes ({} seconds)", expiration as f64 / 60.0, expiration)
        } else {
            format!("{} seconds", expiration)
        };
        metadata_items.push(("⏱️ Expiration".to_string(), expiration_text));
    }

    // Check for other metadata
    for (field, icon, label) in [
        ("Tags", "🏷️", "Tags"),
        ("Store", "🏪", "Cache Store"),
        ("TTL", "⏰", "Original TTL"),
    ] {
        if let Some(field_value) = values.get(field).and_then(Value::as_str) {
            has_metadata = true;
            let full_label = format!("{} {}", icon, label);
            metadata_items.push((full_label, field_value.to_string()));
        }
    }

    if has_metadata {
        styled_card().p_4().child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(text_secondary_color())
                        .child("ℹ️ Additional Information"),
                )
                .child({
                    let mut container = div().flex().flex_col().gap_2();
                    for (label, value) in metadata_items {
                        container = container.child(
                            div()
                                .flex()
                                .justify_between()
                                .items_center()
                                .p_2()
                                .bg(background_color())
                                .rounded_lg()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .child(label),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(text_secondary_color())
                                        .child(value),
                                ),
                        );
                    }
                    container
                }),
        )
    } else {
        div() // Empty div if no metadata
    }
}

fn render_generic_table_ui(content: &Value, entry: &EventEntry, label: &str) -> Div {
    let values = content.get("values").unwrap_or(&Value::Null);
    
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(render_table_header(label.to_string()))
        .child(render_table_data(values))
        .child(render_origin_info(entry))
}

fn render_table_header(label: String) -> Div {
    styled_card().p_4().child(
        div()
            .flex()
            .flex_row()
            .gap_3()
            .items_center()
            .child(div().text_2xl().child("📊"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(gpui::rgb(0x8b5cf6)) // Purple
                            .child(label),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary_color())
                            .child("Table Data"),
                    ),
            ),
    )
}

fn render_table_data(values: &Value) -> Div {
    if let Some(obj) = values.as_object() {
        styled_card().p_4().child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(text_secondary_color())
                        .child("📋 Data Fields"),
                )
                .child({
                    let mut container = div().flex().flex_col().gap_2();
                    for (key, value) in obj {
                        container = container.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .p_3()
                                .bg(background_color())
                                .border_1()
                                .border_color(border_color())
                                .rounded_lg()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .child(key.clone()),
                                )
                                .child(
                                    div()
                                        .font_family("monospace")
                                        .text_xs()
                                        .text_color(text_monospace_color())
                                        .child(serde_json::to_string_pretty(value).unwrap_or_default()),
                                ),
                        );
                    }
                    container
                }),
        )
    } else {
        styled_card().p_4().child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(text_secondary_color())
                        .child("📋 Raw Data"),
                )
                .child(
                    div()
                        .p_3()
                        .bg(gpui::rgb(0x1f2937))
                        .rounded_lg()
                        .child(
                            div()
                                .font_family("monospace")
                                .text_xs()
                                .text_color(gpui::rgb(0xd1d5db))
                                .child(serde_json::to_string_pretty(values).unwrap_or_default()),
                        ),
                ),
        )
    }
}

fn render_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(|l| l.as_u64())
            .unwrap_or(0);
        let hostname = origin
            .get("hostname")
            .and_then(|h| h.as_str())
            .unwrap_or("");

        if !file.is_empty() {
            return styled_card().p_3().child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_secondary_color())
                            .child("🔍 Source"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_monospace_color())
                            .child(format!("{}:{} on {}", file, line, hostname)),
                    ),
            );
        }
    }
    div() // Empty div if no origin info
}
