// payload.rs
use serde_json::{Value};
use std::sync::{Arc, Mutex};
use chrono::Local;

#[derive(Clone, Debug)]
pub struct PayloadEntry {
    pub timestamp: String,
    pub data: String,
    pub p_type : String,
    pub url : String,
    pub method : String,
    pub label: String,
}

pub struct PayloadStorage {
    payloads: Mutex<Vec<PayloadEntry>>,
}

impl PayloadStorage {
    pub fn new() -> Self {
        Self {
            payloads: Mutex::new(Vec::new()),
        }
    }

    pub fn add_payload(&self, entry: PayloadEntry) {
        let mut payloads = self.payloads.lock().unwrap();
        payloads.push(entry);
    }

    pub fn get_payloads(&self) -> Vec<PayloadEntry> {
        let payloads = self.payloads.lock().unwrap();
        payloads.clone()
    }
}

pub fn process_payload(payload: &Value, storage: &Arc<PayloadStorage>) {
    let entry = PayloadEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        data: payload.to_string(),
        p_type: payload.get("type")
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
        url: payload.get("content")
            .and_then(|c| c.get("values"))
            .and_then(|v| v.get("URL"))
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
        method: payload.get("content")
            .and_then(|c| c.get("values"))
            .and_then(|v| v.get("Method"))
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
        label: payload.get("content")
            .and_then(|c| c.get("values"))
            .and_then(|v| v.get("label"))
            .and_then(Value::as_str)
            .unwrap_or("").to_owned(),
    };

    storage.add_payload(entry);
}
