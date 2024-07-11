// payload.rs
use serde_json::{Value};
use std::sync::{Arc, Mutex};
use chrono::Local;
use eframe::egui::TextBuffer;

#[derive(Clone, Debug)]
pub struct PayloadEntry {
    pub timestamp: String,
    pub data: String,
    pub p_type : String,
    pub html : String,
    pub url : String,
    pub method : String,
    pub label: String,
}

pub struct PayloadStorage {
    payloads: Mutex<Vec<PayloadEntry>>,
}

impl PayloadStorage {
    pub fn new() -> Self {
        Self {
            payloads: Mutex::new(Vec::new()),
        }
    }

    pub fn add_payload(&self, entry: PayloadEntry) {
        let mut payloads = self.payloads.lock().unwrap();
        payloads.push(entry);
    }

    pub fn get_payloads(&self) -> Vec<PayloadEntry> {
        let payloads = self.payloads.lock().unwrap();
        payloads.clone()
    }
}

pub fn parse_html(html: &str) -> String {
    let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    dom.query_selector("pre")
        .and_then(|mut e| e.next())
        .and_then(|n| n.get(parser))
        .map(|e| e.inner_text(parser))
        .map(|s| s.to_string())
        .unwrap_or(html.to_string())
}


pub fn get_html(payload: &Value) -> String {
    let content = payload.get("content").and_then(|c| c.get("values"));

    match payload.get("type").and_then(Value::as_str).unwrap_or("") {
        "table" => {
            let is_request = content
                .and_then(|v| v.get("Method"))
                .and_then(Value::as_str)
                .map_or(false, |m| m == "GET" || m == "POST");

            let field_name = if is_request { "Data" } else { "Body" };

            content
                .and_then(|v| v.get(field_name))
                .and_then(Value::as_str)
                .map(parse_html)
                .unwrap_or_default()
        },
        "log" => content
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(Value::as_str)
            .map(parse_html)
            .unwrap_or_default(),
        _ => String::new(),
    }
}

pub fn process_payload(payload: &Value, storage: &Arc<PayloadStorage>) {

    let entry = PayloadEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        data: payload.to_string(),
        p_type: payload.get("type")
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
        html: get_html(payload),
        url: payload.get("content")
            .and_then(|c| c.get("values"))
            .and_then(|v| v.get("URL"))
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
        method: payload.get("content")
            .and_then(|c| c.get("values"))
            .and_then(|v| v.get("Method"))
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
        label: payload.get("content")
            .and_then(|c| c.get("values"))
            .and_then(|v| v.get("label"))
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
    };

    eprintln!("Parsed Type: {}", entry.p_type);
    eprintln!("Parsed URL: {}", entry.url);
    eprintln!("Parsed HTML: {}", entry.html);

    storage.add_payload(entry);
}
