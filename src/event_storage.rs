use chrono::Local;
use eframe::egui;
use serde_json::Value;
use shared::{EventEntry, EventFactory};
use std::sync::{Arc, Mutex};

use crate::app;
// use crate::event_factory::LocalEventFactory;
use crate::wasm_event_factory::WasmEventFactory;

pub struct EventStorage {
    events: Mutex<Vec<EventEntry>>,
    factory: Box<dyn EventFactory>,
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            factory: Box::new(WasmEventFactory::default()),
        }
    }

    pub fn add_event(&self, event: &Value) {
        if let Some(mut entry) = self.factory.make(event) {
            if entry.timestamp.is_empty() {
                entry.timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            }

            let mut events = self.events.lock().unwrap();
            events.push(entry);
        }
    }

    pub fn get_events(&self) -> Vec<EventEntry> {
        let events = self.events.lock().unwrap();
        events.iter().rev().map(|entry| entry.clone()).collect()
    }

    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    pub fn display_details(&self, ui: &mut egui::Ui, index: usize) {
        let events = self.events.lock().unwrap();
        if let Some(entry) = events.get(events.len() - 1 - index) {
            app::display_code(ui, &entry.content);
        }
    }
}

pub fn process_event(event: &Value, storage: &Arc<EventStorage>) {
    storage.add_event(event);
}
