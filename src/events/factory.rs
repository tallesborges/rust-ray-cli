use std::{collections::HashMap, sync::Arc};

use super::{
    application_log_payload::ApplicationLogPayload, exception_payload::ExceptionPayload,
    log_payload::LogPayload, query_payload::QueryPayload, table_payload::TablePayload, PayloadType,
};

pub struct PayloadTypeFactory {
    types: HashMap<String, Arc<dyn PayloadType>>,
}

impl PayloadTypeFactory {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        types.insert(
            "table".to_string(),
            Arc::new(TablePayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "log".to_string(),
            Arc::new(LogPayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "application_log".to_string(),
            Arc::new(ApplicationLogPayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "executed_query".to_string(),
            Arc::new(QueryPayload) as Arc<dyn PayloadType>,
        );
        types.insert(
            "exception".to_string(),
            Arc::new(ExceptionPayload) as Arc<dyn PayloadType>,
        );
        Self { types }
    }

    pub fn get_type(&self, payload_type: &str) -> Option<Arc<dyn PayloadType>> {
        self.types.get(payload_type).cloned()
    }
}
