use crate::storage::EventStorage;
use gpui::Global;
use std::sync::Arc;

/// Global application state containing shared services and dependencies.
/// This struct holds immutable, shared services that are accessed globally.
pub struct AppState {
    pub payload_storage: Arc<EventStorage>,
}

impl Global for AppState {}

impl AppState {
    pub fn new(payload_storage: Arc<EventStorage>) -> Self {
        Self { payload_storage }
    }
}
