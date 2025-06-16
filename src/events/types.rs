use serde_json::Value;
use std::collections::HashMap;

/// Structured data types for different event types
#[derive(Clone, Debug)]
pub enum ProcessedEvent {
    Cache(CacheEvent),
    Http(HttpEvent),
    Table(TableEvent),
    Log(LogEvent),
    Query(QueryEvent),
    Exception(ExceptionEvent),
    ApplicationLog(ApplicationLogEvent),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CacheEvent {
    pub operation: String, // "Hit", "Missed", "Key written", "Forgotten", etc.
    pub key: String,
    pub value: Option<Value>,
    pub expiration_seconds: Option<u64>,
    pub tags: Option<String>,
    pub store: Option<String>,
    pub ttl: Option<String>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct HttpEvent {
    pub event_type: HttpEventType,
    pub url: String,
    pub method: Option<String>,
    pub status_code: Option<u64>,
    pub success: Option<bool>,
    pub headers: HashMap<String, Value>,
    pub body: Option<Value>,
    pub duration_seconds: Option<f64>,
    pub connection_time_seconds: Option<f64>,
    pub size_bytes: Option<u64>,
    pub request_size_bytes: Option<u64>,
    pub content_type: Option<String>,
}

#[derive(Clone, Debug)]
pub enum HttpEventType {
    Request,
    Response,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TableEvent {
    pub label: String,
    pub data: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LogEvent {
    pub level: String,
    pub message: String,
    pub context: Option<Value>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct QueryEvent {
    pub sql: String,
    pub bindings: Vec<Value>,
    pub duration_ms: Option<f64>,
    pub connection_name: Option<String>,
    pub affected_rows: Option<u64>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ExceptionEvent {
    pub class: String,
    pub message: String,
    pub file: String,
    pub line: u64,
    pub stack_trace: Vec<StackFrame>,
    pub context: Option<Value>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct StackFrame {
    pub file: String,
    pub line: u64,
    pub function: String,
    pub class: Option<String>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ApplicationLogEvent {
    pub level: String,
    pub message: String,
    pub context: Option<Value>,
    pub channel: Option<String>,
}

