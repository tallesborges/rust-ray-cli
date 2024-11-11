// main.rs
mod app;
mod event_storage;
mod server;

use app::MyApp;
use eframe::NativeOptions;
use event_storage::EventStorage;
use server::start_server;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let event_storage = Arc::new(EventStorage::new());
    let server_storage = Arc::clone(&event_storage);

    // Spawn the HTTP server
    tokio::spawn(async move {
        if let Err(e) = start_server(server_storage).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Run the eframe application
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Payload Processing Server",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc, event_storage)))),
    )
}

#[cfg(test)]
mod tests {
    use wasmtime::*;

    #[test]
    fn test_process_query() -> Result<()> {
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());

        let module = Module::from_file(
            &engine,
            "target/wasm32-unknown-unknown/release/event_application_log.wasm",
        )?;
        let instance = Instance::new(&mut store, &module, &[])?;

        let process_query =
            instance.get_typed_func::<(i32, i32), i32>(&mut store, "process_application_log")?;

        let test_input = r#"{"content": {"sql": "SELECT * FROM table"}}"#;
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("failed to find memory export");

        let offset = 0;
        memory.write(&mut store, offset, test_input.as_bytes())?;

        let result = process_query.call(&mut store, (offset as i32, test_input.len() as i32))?;

        assert!(result > 0); // Basic check that we got a valid pointer
        Ok(())
    }
}
