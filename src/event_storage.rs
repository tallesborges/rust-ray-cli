use chrono::Local;
use serde_json::Value;
use shared::{EventEntry, EventFactory};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

// use crate::event_factory::LocalEventFactory;
use crate::wasm_event_factory::WasmEventFactory;

#[derive(Clone, Debug, Copy)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

pub struct EventStorage {
    events: Mutex<Vec<EventEntry>>,
    factory: Box<dyn EventFactory>,
    in_tui_mode: Mutex<bool>,
    server_info: Mutex<String>,
    app_logs: Mutex<Vec<LogEntry>>, // Application logs with level and source
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            factory: Box::new(WasmEventFactory::default()),
            in_tui_mode: Mutex::new(false),
            server_info: Mutex::new(String::new()),
            app_logs: Mutex::new(Vec::new()),
        }
    }

    // Central logging methods
    pub fn log(&self, level: LogLevel, source: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = LogEntry {
            timestamp: timestamp.clone(),
            level,
            source: source.to_string(),
            message: message.to_string(),
        };
        
        // Add to internal logs, maintaining a maximum size of 1000 entries
        {
            let mut logs = self.app_logs.lock().unwrap();
            logs.push(log_entry.clone());
            
            // Keep only the last 1000 logs
            if logs.len() > 1000 {
                logs.remove(0); // Remove oldest log
            }
        }
        
        // In both TUI and non-TUI mode, we want to collect logs
        // But only in non-TUI mode do we want to print to console
        if !self.in_tui_mode() {
            let level_str = match level {
                LogLevel::Info => "INFO",
                LogLevel::Warning => "WARN",
                LogLevel::Error => "ERROR",
                LogLevel::Debug => "DEBUG",
            };
            
            let log_line = format!("[{}] [{} {}] {}", 
                timestamp, level_str, source, message);
                
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
    
    pub fn warn(&self, source: &str, message: &str) {
        self.log(LogLevel::Warning, source, message);
    }
    
    pub fn error(&self, source: &str, message: &str) {
        self.log(LogLevel::Error, source, message);
    }
    
    pub fn debug(&self, source: &str, message: &str) {
        self.log(LogLevel::Debug, source, message);
    }

    pub fn set_tui_mode(&self, enabled: bool) {
        let mut mode = self.in_tui_mode.lock().unwrap();
        *mode = enabled;
    }

    pub fn in_tui_mode(&self) -> bool {
        let mode = self.in_tui_mode.lock().unwrap();
        *mode
    }

    pub fn set_server_info(&self, info: String) {
        let mut server_info = self.server_info.lock().unwrap();
        *server_info = info;
    }

    pub fn get_server_info(&self) -> String {
        let server_info = self.server_info.lock().unwrap();
        server_info.clone()
    }

    pub fn log_server_error(&self, error: String) {
        self.error("Server", &error);
    }


    pub fn log_app_message(&self, message: String) {
        self.info("App", &message);
    }
    
    pub fn get_app_logs(&self) -> Vec<LogEntry> {
        let logs = self.app_logs.lock().unwrap();
        logs.clone()
    }
    
    pub fn clear_logs(&self) {
        let mut app_logs = self.app_logs.lock().unwrap();
        app_logs.clear();
        
        self.info("System", "All logs cleared");
    }
    
    // This method was not needed and could cause unsafe behavior

    pub fn add_event(&self, event: &Value) {
        let event_type = event.get("type").and_then(Value::as_str).unwrap_or("unknown");
        
        self.info("EventStorage", &format!("Processing event of type: {}", event_type));
        
        if let Some(mut entry) = self.factory.make(event) {
            if entry.timestamp.is_empty() {
                entry.timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            }

            self.info("EventStorage", &format!("Event processed successfully: {} ({})", entry.label, entry.content_type));
            
            let mut events = self.events.lock().unwrap();
            events.push(entry);
        } else {
            self.error("EventStorage", &format!("Failed to process event of type: {}", event_type));
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
    let event_type = event.get("type").and_then(Value::as_str).unwrap_or("unknown");
    storage.info("Processing", &format!("Received event of type: {}", event_type));
    storage.add_event(event);
}
