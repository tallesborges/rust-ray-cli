use std::{collections::HashMap, fs, sync::Arc};

use serde_json::Value;
use shared::{EventEntry, EventFactory, EventProcessor};

pub struct WasmEventFactory {
    processors: HashMap<String, Arc<dyn EventProcessor>>,
}

impl Default for WasmEventFactory {
    fn default() -> Self {
        WasmEventFactory {
            processors: HashMap::default(),
        }
    }
}

impl EventFactory for WasmEventFactory {
    fn make(&self, event: &Value) -> Option<EventEntry> {
        None
    }
}
