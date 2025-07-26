use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::cache::process_cache_event;
use crate::events::types::{CacheEvent, ProcessedEvent};
use crate::ui_components::{border_color, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, rgb, Context, Div, FontWeight};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "cache".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
        event_type: "cache".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Process using the new architecture
        let processed_event = process_cache_event(content)?;

        // Set labels and descriptions based on processed event
        if let ProcessedEvent::Cache(ref cache_event) = processed_event {
            entry.label = format!("Cache: {}", cache_event.operation);
            entry.description = match cache_event.operation.as_str() {
                "Hit" => format!("Cache hit for: {}", cache_event.key),
                "Missed" => format!("Cache miss for: {}", cache_event.key),
                "Key written" => format!("Cache write: {}", cache_event.key),
                "Forgotten" => format!("Cache key forgotten: {}", cache_event.key),
                _ => format!("{} ({})", cache_event.operation, cache_event.key),
            };
        } else {
            return Err(anyhow::anyhow!(
                "Unexpected event type from cache processor"
            ));
        }
    }

    Ok(entry)
}

pub fn render_cache_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    if let Some(content) = entry.raw_payload.get("content") {
        if let Ok(ProcessedEvent::Cache(cache_event)) = process_cache_event(content) {
            div()
                .flex()
                .flex_col()
                .gap_6()
                .child(render_cache_header(&cache_event))
                .child(render_cache_details(&cache_event))
                .when(has_cache_metadata(&cache_event), |d| {
                    d.child(render_cache_metadata(&cache_event))
                })
                .child(render_cache_origin_info(entry))
        } else {
            div().child("Invalid cache event data")
        }
    } else {
        div().child("Invalid cache event data")
    }
}

// Table-based cache rendering functions (for Ray API table events with Cache label)
pub fn render_table_cache_event(content: &Value, entry: &EventEntry) -> Div {
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
        .gap_6()
        .child(render_table_cache_header(&operation))
        .child(render_table_cache_key(key))
        .child(render_table_cache_value(values))
        .when(has_table_cache_metadata(values), |d| {
            d.child(render_table_cache_metadata(values))
        })
        .child(render_table_cache_origin_info(entry))
}

fn render_cache_header(cache_event: &CacheEvent) -> Div {
    let operation_color = match cache_event.operation.as_str() {
        "Hit" => rgb(0x22c55e),         // Green for hits
        "Missed" => rgb(0xf59e0b),      // Orange for misses
        "Key written" => rgb(0x3b82f6), // Blue for writes
        "Forgotten" => rgb(0xef4444),   // Red for deletions
        _ => text_secondary_color().into(),
    };

    div()
        .flex()
        .items_center()
        .gap_4()
        .child(
            div().px_3().py_1().rounded_md().bg(rgb(0x18181b)).child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(operation_color)
                    .child(cache_event.operation.clone()),
            ),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .font_family("monospace")
                .text_color(text_primary_color())
                .child(cache_event.key.clone()),
        )
}

fn render_table_cache_header(operation: &str) -> Div {
    let operation_color = match operation {
        "Hit" => rgb(0x22c55e),         // Green for hits
        "Missed" => rgb(0xf59e0b),      // Orange for misses
        "Key written" => rgb(0x3b82f6), // Blue for writes
        "Forgotten" => rgb(0xef4444),   // Red for deletions
        _ => text_secondary_color().into(),
    };

    div().flex().items_center().gap_4().child(
        div().px_3().py_1().rounded_md().bg(rgb(0x18181b)).child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(operation_color)
                .child(operation.to_string()),
        ),
    )
}

fn render_cache_details(cache_event: &CacheEvent) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .when(cache_event.value.is_some(), |d| {
            d.child(render_cache_value_minimal(cache_event))
        })
}

fn render_table_cache_key(key: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_secondary_color())
                .child("CACHE KEY"),
        )
        .child(
            div()
                .p_3()
                .rounded_md()
                .bg(rgb(0x18181b))
                .border_1()
                .border_color(border_color())
                .child(
                    div()
                        .font_family("monospace")
                        .text_sm()
                        .text_color(text_primary_color())
                        .child(key.to_string()),
                ),
        )
}

fn render_cache_value_minimal(cache_event: &CacheEvent) -> Div {
    if let Some(ref value) = cache_event.value {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_secondary_color())
                    .child("VALUE"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_md()
                    .bg(rgb(0x18181b))
                    .border_1()
                    .border_color(border_color())
                    .child(
                        div()
                            .text_xs()
                            .font_family("monospace")
                            .text_color(text_primary_color())
                            .max_w_full()
                            .child(
                                serde_json::to_string_pretty(value)
                                    .unwrap_or_else(|_| value.to_string()),
                            ),
                    ),
            )
    } else {
        div()
    }
}

fn render_table_cache_value(values: &Value) -> Div {
    if let Some(val) = values.get("Value") {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_secondary_color())
                    .child("VALUE"),
            )
            .child(
                div()
                    .p_4()
                    .rounded_md()
                    .bg(rgb(0x18181b))
                    .border_1()
                    .border_color(border_color())
                    .child(
                        div()
                            .text_xs()
                            .font_family("monospace")
                            .text_color(text_primary_color())
                            .max_w_full()
                            .child(serde_json::to_string_pretty(val).unwrap_or_default()),
                    ),
            )
    } else {
        div()
    }
}

fn has_cache_metadata(cache_event: &CacheEvent) -> bool {
    cache_event.expiration_seconds.is_some()
        || cache_event.tags.is_some()
        || cache_event.store.is_some()
        || cache_event.ttl.is_some()
}

fn has_table_cache_metadata(values: &Value) -> bool {
    values.get("Expiration in seconds").is_some()
        || values.get("Tags").is_some()
        || values.get("Store").is_some()
        || values.get("TTL").is_some()
}

fn render_cache_metadata(cache_event: &CacheEvent) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_secondary_color())
                .child("METADATA"),
        )
        .child(
            div()
                .flex()
                .gap_6()
                .text_xs()
                .when(cache_event.expiration_seconds.is_some(), |d| {
                    d.child(render_expiration_metric(cache_event))
                })
                .when(cache_event.tags.is_some(), |d| {
                    d.child(render_tags_metric(cache_event))
                })
                .when(cache_event.store.is_some(), |d| {
                    d.child(render_store_metric(cache_event))
                })
                .when(cache_event.ttl.is_some(), |d| {
                    d.child(render_ttl_metric(cache_event))
                }),
        )
}

fn render_table_cache_metadata(values: &Value) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_secondary_color())
                .child("METADATA"),
        )
        .child(
            div()
                .flex()
                .gap_6()
                .text_xs()
                .when(values.get("Expiration in seconds").is_some(), |d| {
                    d.child(render_table_expiration_metric(values))
                })
                .when(values.get("Tags").is_some(), |d| {
                    d.child(render_table_tags_metric(values))
                })
                .when(values.get("Store").is_some(), |d| {
                    d.child(render_table_store_metric(values))
                })
                .when(values.get("TTL").is_some(), |d| {
                    d.child(render_table_ttl_metric(values))
                }),
        )
}

fn render_expiration_metric(cache_event: &CacheEvent) -> Div {
    if let Some(expiration) = cache_event.expiration_seconds {
        let expiration_text = format_duration(expiration);
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("Expires:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(expiration_text),
            )
    } else {
        div()
    }
}

fn render_table_expiration_metric(values: &Value) -> Div {
    if let Some(expiration) = values.get("Expiration in seconds").and_then(Value::as_u64) {
        let expiration_text = format_duration(expiration);
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("Expires:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(expiration_text),
            )
    } else {
        div()
    }
}

fn render_tags_metric(cache_event: &CacheEvent) -> Div {
    if let Some(ref tags) = cache_event.tags {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("Tags:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(tags.clone()),
            )
    } else {
        div()
    }
}

fn render_table_tags_metric(values: &Value) -> Div {
    if let Some(tags) = values.get("Tags").and_then(Value::as_str) {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("Tags:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(tags.to_string()),
            )
    } else {
        div()
    }
}

fn render_store_metric(cache_event: &CacheEvent) -> Div {
    if let Some(ref store) = cache_event.store {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("Store:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(store.clone()),
            )
    } else {
        div()
    }
}

fn render_table_store_metric(values: &Value) -> Div {
    if let Some(store) = values.get("Store").and_then(Value::as_str) {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("Store:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(store.to_string()),
            )
    } else {
        div()
    }
}

fn render_ttl_metric(cache_event: &CacheEvent) -> Div {
    if let Some(ref ttl) = cache_event.ttl {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("TTL:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(ttl.clone()),
            )
    } else {
        div()
    }
}

fn render_table_ttl_metric(values: &Value) -> Div {
    if let Some(ttl) = values.get("TTL").and_then(Value::as_str) {
        div()
            .flex()
            .gap_2()
            .child(div().text_color(text_secondary_color()).child("TTL:"))
            .child(
                div()
                    .font_family("monospace")
                    .text_color(text_primary_color())
                    .child(ttl.to_string()),
            )
    } else {
        div()
    }
}

fn format_duration(seconds: u64) -> String {
    if seconds > 3600 {
        format!("{:.1}h", seconds as f64 / 3600.0)
    } else if seconds > 60 {
        format!("{:.1}m", seconds as f64 / 60.0)
    } else {
        format!("{seconds}s")
    }
}

fn render_cache_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(Value::as_str).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(Value::as_u64)
            .unwrap_or(0);

        div()
            .flex()
            .items_center()
            .gap_2()
            .text_xs()
            .text_color(text_secondary_color())
            .opacity(0.7)
            .child(format!("{file}:{line}"))
    } else {
        div()
    }
}

fn render_table_cache_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(Value::as_str).unwrap_or("");
        let line = origin
            .get("line_number")
            .and_then(Value::as_u64)
            .unwrap_or(0);

        div()
            .flex()
            .items_center()
            .gap_2()
            .text_xs()
            .text_color(text_secondary_color())
            .opacity(0.7)
            .child(format!("{file}:{line}"))
    } else {
        div()
    }
}
