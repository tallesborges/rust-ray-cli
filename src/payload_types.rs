use std::{collections::HashMap, sync::Arc};

use chrono::Local;
use eframe::egui;
use serde_json::Value;

pub trait PayloadType: Send + Sync {
    fn process(&self, payload: &Value) -> PayloadEntry;
    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry);
}

#[derive(Clone, Debug)]
pub struct PayloadEntry {
    pub timestamp: String,
    pub data: String,
    pub p_type: String,
    pub html: String,
    pub url: String,
    pub method: String,
    pub label: String,
}

pub struct TablePayload;
impl PayloadType for TablePayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        PayloadEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            data: payload.to_string(),
            p_type: "table".to_string(),
            html: {
                let content = payload.get("content").and_then(|c| c.get("values"));

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
            url: payload
                .get("content")
                .and_then(|c| c.get("values"))
                .and_then(|v| v.get("URL"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned(),
            method: payload
                .get("content")
                .and_then(|c| c.get("values"))
                .and_then(|v| v.get("Method"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned(),
            label: payload
                .get("content")
                .and_then(|c| c.get("values"))
                .and_then(|v| v.get("label"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned(),
        }
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.label("URL:");
        ui.label(&entry.url);
        ui.label("Method:");
        ui.label(&entry.method);
        ui.label("HTML Content:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&entry.html);
        });
    }
}

pub struct LogPayload;
impl PayloadType for LogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        PayloadEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            data: payload.to_string(),
            p_type: "log".to_string(),
            html: payload
                .get("content")
                .and_then(|v| v.get("value"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            url: "".to_string(),
            method: "".to_string(),
            label: "".to_string(),
        }
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.label("Log Content:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&entry.html);
        });
    }
}

pub struct ApplicationLogPayload;
impl PayloadType for ApplicationLogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        PayloadEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            data: payload.to_string(),
            p_type: "application_log".to_string(),
            html: payload
                .get("content")
                .and_then(|v| v.get("value"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            url: "".to_string(),
            method: "".to_string(),
            label: "".to_string(),
        }
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.label("Application Log Content:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&entry.html);
        });
    }
}
pub struct PayloadTypeFactory {
    types: HashMap<String, Arc<dyn PayloadType>>,
}

impl PayloadTypeFactory {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        types.insert(
            "table".to_string(),
            Arc::new(TablePayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "log".to_string(),
            Arc::new(LogPayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "application_log".to_string(),
            Arc::new(ApplicationLogPayload) as Arc<dyn PayloadType>,
        );
        Self { types }
    }

    pub fn get_type(&self, payload_type: &str) -> Option<Arc<dyn PayloadType>> {
        self.types.get(payload_type).cloned()
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
