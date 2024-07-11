// payload.rs
use serde_json::{json, Value};
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
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let data = payload.to_string();

    let payload_json = json!(payload);

    let p_type = payload_json["type"].as_str().unwrap_or("").to_string();
    let method = payload_json["content"]["values"]["Method"].as_str().unwrap_or("").to_string();
    let url = payload_json["content"]["values"]["URL"].as_str().unwrap_or("").to_string();
    let label = payload_json["content"]["values"]["label"].as_str().unwrap_or("").to_string();

    let entry = PayloadEntry {
        timestamp,
        data,
        p_type,
        url,
        method,
        label,
    };

    storage.add_payload(entry);
    // println!("Processed payload: {:?}", payload);
}