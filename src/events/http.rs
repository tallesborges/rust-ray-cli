use std::collections::BTreeMap;

use crate::events::base::{extract_timestamp, EventEntry};
use crate::events::processors::http::process_http_event;
use crate::events::types::{HttpEvent, HttpEventType, ProcessedEvent};
use crate::ui_components::{border_color, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, rgb, Context, Div, FontWeight};
use serde_json::Value;

pub fn process(payload: &Value) -> Result<EventEntry> {
    let mut entry = EventEntry {
        timestamp: extract_timestamp(payload),
        label: "request".to_string(),
        description: String::new(),
        content_type: "custom_ui".to_string(),
        event_type: "request".to_string(),
        raw_payload: payload.clone(),
    };

    if let Some(content) = payload.get("content") {
        // Process using the new architecture
        let processed_event = process_http_event(content)?;

        // Set labels and descriptions based on processed event
        if let ProcessedEvent::Http(ref http_event) = processed_event {
            entry.label = match http_event.event_type {
                HttpEventType::Request => "HTTP Request".to_string(),
                HttpEventType::Response => "HTTP Response".to_string(),
            };
            let method_or_status = match http_event.event_type {
                HttpEventType::Request => http_event.method.as_deref().unwrap_or("GET").to_string(),
                HttpEventType::Response => http_event
                    .status_code
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "Response".to_string()),
            };
            entry.description = format!("{} {}", method_or_status, http_event.url);
        } else {
            return Err(anyhow::anyhow!("Unexpected event type from http processor"));
        }
    }

    Ok(entry)
}

pub fn render_http_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    if let Some(content) = entry.raw_payload.get("content") {
        if let Ok(ProcessedEvent::Http(http_event)) = process_http_event(content) {
            div()
                .flex()
                .flex_col()
                .gap_6()
                .child(render_http_header(&http_event))
                .child(render_http_details(&http_event))
                .when(
                    http_event.duration_seconds.is_some()
                        || http_event.connection_time_seconds.is_some()
                        || http_event.size_bytes.is_some(),
                    |d| d.child(render_performance_metrics(&http_event)),
                )
                .child(render_origin_info(entry))
        } else {
            div().child("Invalid HTTP event data")
        }
    } else {
        div().child("Invalid HTTP event data")
    }
}

fn render_http_header(http_event: &HttpEvent) -> Div {
    div()
        .flex()
        .items_center()
        .gap_4()
        .child(
            div().px_3().py_1().rounded_md().bg(rgb(0x18181b)).child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(match http_event.event_type {
                        HttpEventType::Request => rgb(0x22c55e),
                        HttpEventType::Response => match http_event.status_code {
                            Some(status) if (200..300).contains(&status) => rgb(0x22c55e),
                            Some(status) if status >= 400 => rgb(0xef4444),
                            Some(status) if status >= 300 => rgb(0xf59e0b),
                            _ => text_secondary_color().into(),
                        },
                    })
                    .child(match http_event.event_type {
                        HttpEventType::Request => {
                            http_event.method.as_deref().unwrap_or("GET").to_string()
                        }
                        HttpEventType::Response => http_event
                            .status_code
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Response".to_string()),
                    }),
            ),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .font_family("monospace")
                .text_color(text_primary_color())
                .child(http_event.url.clone()),
        )
}

fn render_http_details(http_event: &HttpEvent) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .when(!http_event.headers.is_empty(), |d| {
            d.child(render_headers(http_event))
        })
        .when(http_event.body.is_some(), |d| {
            d.child(render_body(http_event))
        })
}

fn render_headers(http_event: &HttpEvent) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_secondary_color())
                .child("HEADERS"),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_3()
                .rounded_md()
                .bg(rgb(0x18181b))
                .border_1()
                .border_color(border_color())
                .children({
                    let sorted_headers: BTreeMap<_, _> = http_event.headers.iter().collect();
                    sorted_headers.into_iter().map(|(key, value)| {
                        div()
                            .flex()
                            .gap_2()
                            .text_xs()
                            .font_family("monospace")
                            .child(
                                div()
                                    .min_w_32()
                                    .text_color(text_secondary_color())
                                    .child(format!("{key}:")),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_color(text_primary_color())
                                    .child(value.to_string()),
                            )
                    })
                }),
        )
}

fn render_body(http_event: &HttpEvent) -> Div {
    if let Some(body) = &http_event.body {
        let formatted_body = if http_event.content_type.as_deref() == Some("Json") {
            serde_json::to_string_pretty(body).unwrap_or_else(|_| body.to_string())
        } else {
            body.to_string()
        };

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_secondary_color())
                    .child("BODY"),
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
                            .child(formatted_body),
                    ),
            )
    } else {
        div()
    }
}

fn render_performance_metrics(http_event: &HttpEvent) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_secondary_color())
                .child("PERFORMANCE"),
        )
        .child(
            div()
                .flex()
                .gap_6()
                .text_xs()
                .when(http_event.duration_seconds.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("Duration:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(format!(
                                        "{}ms",
                                        (http_event.duration_seconds.unwrap_or(0.0) * 1000.0)
                                            as u64
                                    )),
                            ),
                    )
                })
                .when(http_event.connection_time_seconds.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(text_secondary_color())
                                    .child("Connection:"),
                            )
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(format!(
                                        "{}ms",
                                        (http_event.connection_time_seconds.unwrap_or(0.0) * 1000.0)
                                            as u64
                                    )),
                            ),
                    )
                })
                .when(http_event.size_bytes.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("Size:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(format_bytes(http_event.size_bytes.unwrap_or(0))),
                            ),
                    )
                }),
        )
}

fn render_origin_info(entry: &EventEntry) -> Div {
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

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if size.fract() == 0.0 {
        format!("{:.0}{}", size, UNITS[unit_index])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}

