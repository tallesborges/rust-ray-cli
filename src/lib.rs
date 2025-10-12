// lib.rs - Expose modules for integration tests and external use
pub mod app;
pub mod event_details;
pub mod event_list;
pub mod event_storage;
pub mod events;
pub mod server;
pub mod ui_components;

// Re-export commonly used items
pub use event_storage::EventStorage;
pub use events::{process_event, EventEntry};
