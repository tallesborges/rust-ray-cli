use crate::events::{entry::EventEntry, Event};
use crate::ui::components::{border_color, text_primary_color, text_secondary_color};
use anyhow::Result;
use gpui::prelude::*;
use gpui::{div, rgb, Context, Div, FontWeight};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

pub struct HttpEvent;

#[derive(Clone, Debug)]
enum HttpEventType {
    Request,
    Response,
}

#[derive(Clone, Debug)]
struct HttpEventData {
    event_type: HttpEventType,
    url: String,
    method: Option<String>,
    status_code: Option<u64>,
    headers: HashMap<String, Value>,
    body: Option<Value>,
    duration_seconds: Option<f64>,
    connection_time_seconds: Option<f64>,
    size_bytes: Option<u64>,
    content_type: Option<String>,
}

impl Event for HttpEvent {
    fn process(payload: &Value) -> Result<EventEntry> {
        let content = payload
            .get("content")
            .ok_or_else(|| anyhow::anyhow!("Missing content in HTTP event"))?;

        let http_data = parse_http_event(content)?;

        let label = match http_data.event_type {
            HttpEventType::Request => "HTTP Request",
            HttpEventType::Response => "HTTP Response",
        };

        let method_or_status = match http_data.event_type {
            HttpEventType::Request => http_data.method.as_deref().unwrap_or("GET"),
            HttpEventType::Response => &http_data
                .status_code
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Response".to_string()),
        };

        let description = format!("{} {}", method_or_status, http_data.url);

        Ok(EventEntry::new("request", label, description, payload))
    }

    fn render(entry: &EventEntry, _cx: &mut Context<crate::ui::MyApp>) -> Div {
        if let Some(content) = entry.raw_payload.get("content") {
            if let Ok(http_data) = parse_http_event(content) {
                return div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .child(render_http_header(&http_data))
                    .child(render_http_details(&http_data))
                    .when(
                        http_data.duration_seconds.is_some()
                            || http_data.connection_time_seconds.is_some()
                            || http_data.size_bytes.is_some(),
                        |d| d.child(render_performance_metrics(&http_data)),
                    )
                    .child(render_origin_info(entry));
            }
        }
        div().child("Invalid HTTP event data")
    }
}

fn parse_http_event(content: &Value) -> Result<HttpEventData> {
    let values = content
        .get("values")
        .ok_or_else(|| anyhow::anyhow!("Missing values in HTTP event"))?;

    let method = values
        .get("Method")
        .and_then(Value::as_str)
        .map(|s| s.to_string());
    let status_code = values.get("Status").and_then(Value::as_u64);

    let event_type = if method.is_some() {
        HttpEventType::Request
    } else if status_code.is_some() {
        HttpEventType::Response
    } else {
        HttpEventType::Request
    };

    let url = values
        .get("URL")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let mut headers = HashMap::new();
    if let Some(headers_obj) = values.get("Headers").and_then(Value::as_object) {
        for (key, value) in headers_obj {
            headers.insert(key.clone(), value.clone());
        }
    }

    let body = values
        .get("Data")
        .cloned()
        .or_else(|| values.get("Body").cloned());

    let duration_seconds = values.get("Duration").and_then(Value::as_f64);
    let connection_time_seconds = values.get("Connection time").and_then(Value::as_f64);
    let size_bytes = values.get("Size").and_then(Value::as_u64);
    let content_type = values
        .get("Type")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    Ok(HttpEventData {
        event_type,
        url,
        method,
        status_code,
        headers,
        body,
        duration_seconds,
        connection_time_seconds,
        size_bytes,
        content_type,
    })
}

fn render_http_header(http_data: &HttpEventData) -> Div {
    div()
        .flex()
        .items_center()
        .gap_4()
        .child(
            div().px_3().py_1().rounded_md().bg(rgb(0x18181b)).child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(match http_data.event_type {
                        HttpEventType::Request => rgb(0x22c55e),
                        HttpEventType::Response => match http_data.status_code {
                            Some(status) if (200..300).contains(&status) => rgb(0x22c55e),
                            Some(status) if status >= 400 => rgb(0xef4444),
                            Some(status) if status >= 300 => rgb(0xf59e0b),
                            _ => text_secondary_color().into(),
                        },
                    })
                    .child(match http_data.event_type {
                        HttpEventType::Request => {
                            http_data.method.as_deref().unwrap_or("GET").to_string()
                        }
                        HttpEventType::Response => http_data
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
                .child(http_data.url.clone()),
        )
}

fn render_http_details(http_data: &HttpEventData) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .when(!http_data.headers.is_empty(), |d| {
            d.child(render_headers(http_data))
        })
        .when(http_data.body.is_some(), |d| {
            d.child(render_body(http_data))
        })
}

fn render_headers(http_data: &HttpEventData) -> Div {
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
                    let sorted_headers: BTreeMap<_, _> = http_data.headers.iter().collect();
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

fn render_body(http_data: &HttpEventData) -> Div {
    if let Some(body) = &http_data.body {
        let formatted_body = if http_data.content_type.as_deref() == Some("Json") {
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

fn render_performance_metrics(http_data: &HttpEventData) -> Div {
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
                .when(http_data.duration_seconds.is_some(), |d| {
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
                                        (http_data.duration_seconds.unwrap_or(0.0) * 1000.0)
                                            as u64
                                    )),
                            ),
                    )
                })
                .when(http_data.connection_time_seconds.is_some(), |d| {
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
                                        (http_data.connection_time_seconds.unwrap_or(0.0) * 1000.0)
                                            as u64
                                    )),
                            ),
                    )
                })
                .when(http_data.size_bytes.is_some(), |d| {
                    d.child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().text_color(text_secondary_color()).child("Size:"))
                            .child(
                                div()
                                    .font_family("monospace")
                                    .text_color(text_primary_color())
                                    .child(format_bytes(http_data.size_bytes.unwrap_or(0))),
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

