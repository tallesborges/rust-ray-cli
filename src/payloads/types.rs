use core::f32;
use std::{collections::HashMap, sync::Arc};

use chrono::Local;
use eframe::egui;
use serde_json::Value;

fn process_common_payload(payload: &Value, p_type: &str) -> PayloadEntry {
    PayloadEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        label: p_type.to_string(),
        description: String::new(), // Will be set by specific payload types
        content: String::new(),     // Will be set by specific payload types
    }
}

fn display_code(ui: &mut egui::Ui, content: &str, language: &str) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        let theme = egui_extras::syntax_highlighting::CodeTheme::from_style(&ui.ctx().style());
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                ui.ctx(),
                ui.style(),
                &theme,
                string,
                language,
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut content.clone())
                .code_editor()
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace.resolve(ui.style()))
                .desired_rows(10)
                .layouter(&mut layouter)
                .lock_focus(true),
        );
    });
}

pub trait PayloadType: Send + Sync {
    fn process(&self, payload: &Value) -> PayloadEntry;
    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry);
}

#[derive(Clone, Debug)]
pub struct PayloadEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content: String,
}

pub struct TablePayload;
impl PayloadType for TablePayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "table");

        if let Some(content) = payload.get("content").and_then(|c| c.get("values")) {
            let is_request = content
                .get("Method")
                .and_then(Value::as_str)
                .map_or(false, |m| m == "GET" || m == "POST");

            let field_name = if is_request { "Data" } else { "Body" };

            // Set content from the appropriate field
            entry.content = content
                .get(field_name)
                .and_then(|v| serde_json::to_string_pretty(v).ok())
                .unwrap_or_default();

            // Use URL as description
            entry.description = content
                .get("URL")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned();

            entry.label = if is_request {
                "Request".to_string()
            } else {
                "Response".to_string()
            };
        }

        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("URL:");
        ui.label(&entry.description);
        ui.strong("Content:");
        display_code(ui, &entry.content, "json");
    }
}

pub struct LogPayload;
impl PayloadType for LogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "log");
        entry.content = payload
            .get("content")
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("Log Content:");
        display_code(ui, &entry.content, "json");
    }
}

pub struct ExceptionPayload;
impl PayloadType for ExceptionPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "exception");
        entry.content = payload
            .get("content")
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("Exception Details:");
        display_code(ui, &entry.content, "json");
    }
}

pub struct QueryPayload;
impl PayloadType for QueryPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "query");
        entry.content = payload
            .get("content")
            .and_then(|v| v.get("sql"))
            .and_then(|v| serde_json::to_string_pretty(v).ok())
            .unwrap_or_default();
        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("SQL Query:");
        display_code(ui, &entry.content, "sql");
    }
}

pub struct ApplicationLogPayload;
impl PayloadType for ApplicationLogPayload {
    fn process(&self, payload: &Value) -> PayloadEntry {
        let mut entry = process_common_payload(payload, "application_log");
        entry.content = serde_json::to_string_pretty(payload).unwrap_or_default();
        entry
    }

    fn display_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.strong("Application Log:");
        display_code(ui, &entry.content, "json");
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
        types.insert(
            "executed_query".to_string(),
            Arc::new(QueryPayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "exception".to_string(),
            Arc::new(ExceptionPayload) as Arc<dyn PayloadType>,
        );
        Self { types }
    }

    pub fn get_type(&self, payload_type: &str) -> Option<Arc<dyn PayloadType>> {
        self.types.get(payload_type).cloned()
    }
}
