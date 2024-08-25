// payload_storage.rs
use crate::payload_types::{PayloadEntry, PayloadType, PayloadTypeFactory};
use eframe::egui;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub struct PayloadStorage {
    payloads: Mutex<Vec<(PayloadEntry, Arc<dyn PayloadType>)>>,
    factory: PayloadTypeFactory,
}

impl PayloadStorage {
    pub fn new() -> Self {
        Self {
            payloads: Mutex::new(Vec::new()),
            factory: PayloadTypeFactory::new(),
        }
    }

    pub fn add_payload(&self, payload: &Value) {
        let payload_type = payload.get("type").and_then(Value::as_str).unwrap_or("");
        if let Some(processor) = self.factory.get_type(payload_type) {
            let entry = processor.process(payload);
            let mut payloads = self.payloads.lock().unwrap();
            payloads.push((entry, processor));
        } else {
            eprintln!("Unknown payload type: {}", payload_type);
        }
    }

    pub fn get_payloads(&self) -> Vec<PayloadEntry> {
        let payloads = self.payloads.lock().unwrap();
        payloads.iter().map(|(entry, _)| entry.clone()).collect()
    }

    pub fn clear_payloads(&self) {
        let mut payloads = self.payloads.lock().unwrap();
        payloads.clear();
    }

    pub fn display_details(&self, ui: &mut egui::Ui, index: usize) {
        let payloads = self.payloads.lock().unwrap();
        if let Some((entry, processor)) = payloads.get(index) {
            processor.display_details(ui, entry);
        }
    }
}

pub fn process_payload(payload: &Value, storage: &Arc<PayloadStorage>) {
    storage.add_payload(payload);
}
