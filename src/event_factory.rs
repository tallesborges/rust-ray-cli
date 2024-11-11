use event_application_log::ApplicationLogEvent;
use event_exception::ExceptionEvent;
use event_log::LogEvent;
use event_query::QueryEvent;
use event_table::TableEvent;
use serde_json::Value;
use shared::{EventEntry, EventProcessor};
use std::collections::HashMap;
use std::sync::Arc;

pub trait EventFactory: Send + Sync {
    fn make(&self, event: &Value) -> Option<EventEntry>;
}

pub struct LocalEventFactory {
    processors: HashMap<String, Arc<dyn EventProcessor>>,
}

impl LocalEventFactory {
    pub fn new() -> Self {
        LocalEventFactory {
            processors: {
                let mut types = HashMap::new();
                types.insert(
                    "table".to_string(),
                    Arc::new(TableEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "log".to_string(),
                    Arc::new(LogEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "application_log".to_string(),
                    Arc::new(ApplicationLogEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "executed_query".to_string(),
                    Arc::new(QueryEvent) as Arc<dyn EventProcessor>,
                );
                types.insert(
                    "exception".to_string(),
                    Arc::new(ExceptionEvent) as Arc<dyn EventProcessor>,
                );
                types
            },
        }
    }
}

impl EventFactory for LocalEventFactory {
    fn make(&self, event: &Value) -> Option<EventEntry> {
        let event_type = event.get("type").and_then(Value::as_str).unwrap_or("");
        println!("Processing event type: {}", event_type);
        println!("Event: {}", event);

        let processor = self.processors.get(event_type)?;
        let event_str = serde_json::to_string(event).unwrap_or_default();
        Some(processor.process(&event_str))
    }
}
