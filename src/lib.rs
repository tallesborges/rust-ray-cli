// lib.rs - Expose modules for integration tests and external use
pub mod events;
pub mod event_storage;
pub mod ui_components;
pub mod performance;
pub mod server;
pub mod app;
pub mod event_details;
pub mod event_list;

// Re-export commonly used items
pub use events::{process_event, EventEntry};
pub use event_storage::EventStorage;