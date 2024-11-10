use std::{collections::HashMap, sync::Arc};

use super::{
    application_log::ApplicationLogEvent, exception::ExceptionEvent, log::LogEvent,
    query::QueryEvent, table::TableEvent, EventProcessor,
};

pub struct EventTypeFactory {
    types: HashMap<String, Arc<dyn EventProcessor>>,
}

impl EventTypeFactory {
    pub fn new() -> Self {
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
        Self { types }
    }

    pub fn get_type(&self, payload_type: &str) -> Option<Arc<dyn EventProcessor>> {
        self.types.get(payload_type).cloned()
    }
}
