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

use crate::event_factory::{EventFactory, LocalEventFactory};

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
