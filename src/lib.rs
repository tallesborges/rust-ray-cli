// lib.rs - Expose modules for integration tests and external use
pub mod app_state;
pub mod events;
pub mod server;
pub mod storage;
pub mod ui;

// Re-export commonly used items
pub use app_state::AppState;
pub use events::{process_event, EventEntry};
pub use storage::EventStorage;
pub use ui::MyApp;
