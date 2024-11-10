use crate::events::{PayloadEntry, PayloadType, PayloadTypeFactory};
use eframe::egui;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub struct EventStorage {
    events: Mutex<Vec<(PayloadEntry, Arc<dyn PayloadType>)>>,
    factory: PayloadTypeFactory,
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            factory: PayloadTypeFactory::new(),
        }
    }

    pub fn add_event(&self, event: &Value) {
        let event_type = event.get("type").and_then(Value::as_str).unwrap_or("");
        println!("Processing event type: {}", event_type);
        println!("Event: {}", event);
        if let Some(processor) = self.factory.get_type(event_type) {
            let entry = processor.process(event);
            let mut events = self.events.lock().unwrap();
            events.push((entry, processor));
        } else {
            eprintln!("Unknown event type: {}", event_type);
        }
    }

    pub fn get_events(&self) -> Vec<PayloadEntry> {
        let events = self.events.lock().unwrap();
        events.iter().map(|(entry, _)| entry.clone()).collect()
    }

    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    pub fn display_details(&self, ui: &mut egui::Ui, index: usize) {
        let events = self.events.lock().unwrap();
        if let Some((entry, _)) = events.get(index) {
            crate::events::display_code(ui, &entry.content, &entry.content_type);
        }
    }
}

pub fn process_event(event: &Value, storage: &Arc<EventStorage>) {
    storage.add_event(event);
}
