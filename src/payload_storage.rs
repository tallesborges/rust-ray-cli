use crate::payloads::{PayloadEntry, PayloadType, PayloadTypeFactory};
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
        println!("Processing payload type: {}", payload_type);
        println!("Payload: {}", payload);
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
        if let Some((entry, _)) = payloads.get(index) {
            match entry.label.as_str() {
                "table" => crate::payloads::display_table_details(ui, entry),
                "log" => crate::payloads::display_log_details(ui, entry),
                "application_log" => crate::payloads::display_application_log_details(ui, entry),
                "executed_query" => crate::payloads::display_query_details(ui, entry),
                "exception" => crate::payloads::display_exception_details(ui, entry),
                _ => ui.label("Unknown payload type"),
            }
        }
    }
}

pub fn process_payload(payload: &Value, storage: &Arc<PayloadStorage>) {
    storage.add_payload(payload);
}
