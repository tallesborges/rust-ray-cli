use eframe::egui;
use event_application_log::ApplicationLogEvent;
use event_exception::ExceptionEvent;
use event_log::LogEvent;
use event_query::QueryEvent;
use event_table::TableEvent;
use serde_json::Value;
use shared::{EventEntry, EventProcessor};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::app;

pub struct EventStorage {
    events: Mutex<Vec<EventEntry>>,
    factory: HashMap<String, Arc<dyn EventProcessor>>,
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            factory: {
                let mut types = HashMap::new();
                types.insert(
                    "table".to_string(),
                    Arc::new(TableEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "log".to_string(),
                    Arc::new(LogEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "application_log".to_string(),
                    Arc::new(ApplicationLogEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "executed_query".to_string(),
                    Arc::new(QueryEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "exception".to_string(),
                    Arc::new(ExceptionEvent) as Arc<dyn EventProcessor>,
                );
                types
            },
        }
    }

    pub fn add_event(&self, event: &Value) {
        let event_type = event.get("type").and_then(Value::as_str).unwrap_or("");
        println!("Processing event type: {}", event_type);
        println!("Event: {}", event);
        if let Some(processor) = self.factory.get(event_type) {
            let event_str = serde_json::to_string(event).unwrap_or_default();
            let entry = processor.process(&event_str);
            let mut events = self.events.lock().unwrap();
            events.push(entry);
        } else {
            eprintln!("Unknown event type: {}", event_type);
        }
    }

    pub fn get_events(&self) -> Vec<EventEntry> {
        let events = self.events.lock().unwrap();
        events.iter().map(|entry| entry.clone()).collect()
    }

    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    pub fn display_details(&self, ui: &mut egui::Ui, index: usize) {
        let events = self.events.lock().unwrap();
        if let Some(entry) = events.get(index) {
            app::display_code(ui, &entry.content, &entry.content_type);
        }
    }
}

pub fn process_event(event: &Value, storage: &Arc<EventStorage>) {
    storage.add_event(event);
}
