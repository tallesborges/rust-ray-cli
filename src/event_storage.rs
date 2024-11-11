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
    factory: Box<dyn EventFactory>,
}

pub trait EventFactory: Send + Sync {
    fn make(&self, event: &Value) -> Option<EventEntry>;
}

pub struct LocalEventFactory {
    processors: HashMap<String, Arc<dyn EventProcessor>>,
}

impl LocalEventFactory {
    fn new() -> Self {
        LocalEventFactory {
            processors: {
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
}
impl EventFactory for LocalEventFactory {
    fn make(&self, event: &Value) -> Option<EventEntry> {
        let event_type = event.get("type").and_then(Value::as_str).unwrap_or("");
        println!("Processing event type: {}", event_type);
        println!("Event: {}", event);

        let processor = self.processors.get(event_type)?;
        let event_str = serde_json::to_string(event).unwrap_or_default();
        Some(processor.process(&event_str))
    }
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            factory: Box::new(LocalEventFactory::new()),
        }
    }

    pub fn add_event(&self, event: &Value) {
        if let Some(entry) = self.factory.make(event) {
            let mut events = self.events.lock().unwrap();
            events.push(entry);
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
