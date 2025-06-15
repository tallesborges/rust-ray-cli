// main.rs
mod app;
mod event_details;
mod event_list;
mod event_storage;
mod events;
mod server;
mod ui_components;

use app::run_app;
use event_storage::EventStorage;
use server::start_server;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_storage = Arc::new(EventStorage::new());

    event_storage.info("Main", "Starting in GUI mode");

    // Log system information
    event_storage.info("Main", &format!("OS: {}", std::env::consts::OS));
    event_storage.info(
        "Main",
        &format!(
            "Current dir: {:?}",
            std::env::current_dir().unwrap_or_default()
        ),
    );

    let server_storage = Arc::clone(&event_storage);

    // Spawn the HTTP server
    event_storage.info("Main", "Starting HTTP server");
    tokio::spawn(async move {
        if let Err(e) = start_server(server_storage.clone()).await {
            server_storage.error("Main", &format!("Server error: {}", e));
        }
    });

    // Run the gpui application
    event_storage.info("Main", "Initializing GUI application");
    event_storage.info("Main", "Starting GUI event loop");

    run_app(event_storage)
}

#[cfg(test)]
mod tests {
    use crate::events::{process_event, EventEntry};
    use serde_json::json;

    #[test]
    fn test_process_application_log() {
        let test_event = json!({
            "type": "application_log",
            "content": {
                "value": "Test application log message"
            }
        });

        let result = process_event("application_log", &test_event).unwrap();
        assert_eq!(result.label, "Application Log");
        assert!(result.content.contains("Test application log message"));
        assert_eq!(result.content_type, "markdown");
    }

    #[test]
    fn test_process_log_event() {
        let test_event = json!({
            "type": "log",
            "content": {
                "values": ["Test log message", "Another value"]
            }
        });

        let result = process_event("log", &test_event).unwrap();
        assert_eq!(result.label, "log");
        assert!(result.content.contains("Test log message"));
        assert_eq!(result.content_type, "markdown");
    }
}
