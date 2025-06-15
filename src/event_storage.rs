use chrono::Local;
use serde_json::Value;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crate::events::{EventEntry, process_event as process_event_directly};

#[derive(Clone, Debug, Copy)]
pub enum LogLevel {
    Info,
    Error,
}

pub struct EventStorage {
    events: Mutex<Vec<EventEntry>>,
    server_info: Mutex<String>,
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            server_info: Mutex::new(String::new()),
        }
    }

    // Central logging methods
    pub fn log(&self, level: LogLevel, source: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Always print logs to console
        {
            let level_str = match level {
                LogLevel::Info => "INFO",
                LogLevel::Error => "ERROR",
            };

            let log_line = format!("[{}] [{} {}] {}", timestamp, level_str, source, message);

            match level {
                LogLevel::Error => {
                    let mut stderr = io::stderr();
                    let _ = writeln!(stderr, "{}", log_line);
                    let _ = stderr.flush();
                }
                _ => {
                    let mut stdout = io::stdout();
                    let _ = writeln!(stdout, "{}", log_line);
                    let _ = stdout.flush();
                }
            }
        }
    }

    // Convenience logging methods
    pub fn info(&self, source: &str, message: &str) {
        self.log(LogLevel::Info, source, message);
    }

    pub fn error(&self, source: &str, message: &str) {
        self.log(LogLevel::Error, source, message);
    }

    pub fn set_server_info(&self, info: String) {
        let mut server_info = self.server_info.lock().unwrap();
        *server_info = info;
    }

    // This method was not needed and could cause unsafe behavior

    pub fn add_event(&self, event: &Value) {
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown");

        self.info(
            "EventStorage",
            &format!("Processing event of type: {}", event_type),
        );

        match process_event_directly(event_type, event) {
            Ok(mut entry) => {
                if entry.timestamp.is_empty() {
                    entry.timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                }

                self.info(
                    "EventStorage",
                    &format!(
                        "Event processed successfully: {} ({})",
                        entry.label, entry.content_type
                    ),
                );

                let mut events = self.events.lock().unwrap();
                events.push(entry);
            }
            Err(e) => {
                self.error(
                    "EventStorage",
                    &format!("Failed to process event of type {}: {}", event_type, e),
                );
            }
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
}

pub fn process_event(event: &Value, storage: &Arc<EventStorage>) {
    let event_type = event
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    storage.info(
        "Processing",
        &format!("Received event of type: {}", event_type),
    );
    storage.add_event(event);
}
