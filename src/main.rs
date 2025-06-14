// main.rs
mod app;
mod event_details;
mod event_factory;
mod event_list;
mod event_storage;
mod server;
mod ui_components;
mod wasm_event_factory;

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
    use std::ffi::CStr;

    use anyhow::Result;
    use wasmtime::*;

    #[test]
    fn test_process_application_log() -> Result<()> {
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        let module = Module::from_file(
            &engine,
            "target/wasm32-unknown-unknown/release/event_application_log.wasm",
        )?;
        let instance = Instance::new(&mut store, &module, &[])?;

        let process_query =
            instance.get_typed_func::<(i32, i32), i32>(&mut store, "process_event")?;

        let test_input = r#"{"content": "Application log message"}"#;
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("failed to find memory export");

        let offset = 0;
        memory.write(&mut store, offset, test_input.as_bytes())?;

        let result_ptr =
            process_query.call(&mut store, (offset as i32, test_input.len() as i32))?;
        assert!(result_ptr > 0); // Check that we got a valid pointer

        // Read the result string from memory
        let memory_slice = memory.data(&store);
        let result_str = unsafe {
            CStr::from_ptr(&memory_slice[result_ptr as usize] as *const u8 as *const i8)
                .to_str()
                .expect("Invalid UTF-8")
        };
        let result: shared::EventEntry = serde_json::from_str(&result_str).expect("Invalid JSON");

        assert_eq!(result.label, "application_log");
        assert_eq!(result.content, test_input);
        assert_eq!(result.content_type, "json");
        Ok(())
    }
}
