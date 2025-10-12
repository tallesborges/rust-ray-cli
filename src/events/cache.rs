use crate::events::{entry::EventEntry, Event};
use crate::ui_components::{border_color, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, rgb, Context, Div, FontWeight};
use serde_json::Value;

pub struct CacheEvent;

#[derive(Clone, Debug)]
struct CacheEventData {
    operation: String,
    key: String,
    value: Option<Value>,
    expiration_seconds: Option<u64>,
    tags: Option<String>,
    store: Option<String>,
    ttl: Option<String>,
}

impl Event for CacheEvent {
    fn process(payload: &Value) -> Result<EventEntry> {
        let content = payload
            .get("content")
            .ok_or_else(|| anyhow::anyhow!("Missing content in cache event"))?;

        let cache_data = parse_cache_event(content)?;

        let label = format!("Cache: {}", cache_data.operation);
        let description = match cache_data.operation.as_str() {
            "Hit" => format!("Cache hit for: {}", cache_data.key),
            "Missed" => format!("Cache miss for: {}", cache_data.key),
            "Key written" => format!("Cache write: {}", cache_data.key),
            "Forgotten" => format!("Cache key forgotten: {}", cache_data.key),
            _ => format!("{} ({})", cache_data.operation, cache_data.key),
        };

        Ok(EventEntry::new("cache", label, description, payload))
    }

    fn render(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
        if let Some(content) = entry.raw_payload.get("content") {
            if let Ok(cache_data) = parse_cache_event(content) {
                return div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .child(render_cache_header(&cache_data))
                    .child(render_cache_details(&cache_data))
                    .when(has_cache_metadata(&cache_data), |d| {
                        d.child(render_cache_metadata(&cache_data))
                    })
                    .child(render_cache_origin_info(entry));
            }
        }
        div().child("Invalid cache event data")
    }
}

fn parse_cache_event(content: &Value) -> Result<CacheEventData> {
    let values = content
        .get("values")
        .ok_or_else(|| anyhow::anyhow!("Missing values in cache event"))?;

    let operation = values
        .get("Event")
        .and_then(Value::as_str)
        .unwrap_or("Unknown")
        .replace("<code>", "")
        .replace("</code>", "");

    let key = values
        .get("Key")
        .and_then(Value::as_str)
        .unwrap_or("Unknown")
        .to_string();

    let value = values.get("Value").cloned();
    let expiration_seconds = values.get("Expiration in seconds").and_then(Value::as_u64);
    let tags = values
        .get("Tags")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let store = values
        .get("Store")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let ttl = values
        .get("TTL")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    Ok(CacheEventData {
        operation,
        key,
        value,
        expiration_seconds,
        tags,
        store,
        ttl,
    })
}

fn render_cache_header(cache_data: &CacheEventData) -> Div {
    let operation_color = match cache_data.operation.as_str() {
        "Hit" => rgb(0x22c55e),
        "Missed" => rgb(0xf59e0b),
        "Key written" => rgb(0x3b82f6),
        "Forgotten" => rgb(0xef4444),
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
                    .child(cache_data.operation.clone()),
            ),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .font_family("monospace")
                .text_color(text_primary_color())
                .child(cache_data.key.clone()),
        )
}

fn render_cache_details(cache_data: &CacheEventData) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .when(cache_data.value.is_some(), |d| {
            d.child(render_cache_value(cache_data))
        })
}

fn render_cache_value(cache_data: &CacheEventData) -> Div {
    if let Some(ref value) = cache_data.value {
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

fn has_cache_metadata(cache_data: &CacheEventData) -> bool {
    cache_data.expiration_seconds.is_some()
        || cache_data.tags.is_some()
        || cache_data.store.is_some()
        || cache_data.ttl.is_some()
}

fn render_cache_metadata(cache_data: &CacheEventData) -> Div {
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
                .when(cache_data.expiration_seconds.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("Expires:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(format_duration(cache_data.expiration_seconds.unwrap())),
                            ),
                    )
                })
                .when(cache_data.tags.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("Tags:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(cache_data.tags.as_ref().unwrap().clone()),
                            ),
                    )
                })
                .when(cache_data.store.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("Store:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(cache_data.store.as_ref().unwrap().clone()),
                            ),
                    )
                })
                .when(cache_data.ttl.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("TTL:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(cache_data.ttl.as_ref().unwrap().clone()),
                            ),
                    )
                }),
        )
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
