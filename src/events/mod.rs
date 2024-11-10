use chrono::Local;
use core::f32;
use eframe::egui;
use serde_json::Value;
mod application_log;
mod exception;
mod factory;
mod log;
mod query;
mod table;

pub use factory::EventTypeFactory;

pub fn process_common_event(payload: &Value, p_type: &str) -> EventEntry {
    EventEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        label: p_type.to_string(),
        description: String::new(),
        content: String::new(),
        content_type: "json".to_string(),
    }
}

pub fn display_code(ui: &mut egui::Ui, content: &str, language: &str) {
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

pub trait EventProcessor: Send + Sync {
    fn process(&self, payload: &Value) -> EventEntry;
}

#[derive(Clone, Debug)]
pub struct EventEntry {
    pub timestamp: String,
    pub label: String,
    pub description: String,
    pub content: String,
    pub content_type: String,
}
