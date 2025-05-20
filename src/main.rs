// main.rs
mod app;
mod event_factory;
mod event_storage;
mod server;
mod tui_app;
mod tui_markdown;
mod wasm_event_factory;

use app::MyApp;
use clap::Parser;
use eframe::NativeOptions;
use event_storage::EventStorage;
use server::start_server;
use std::sync::Arc;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Flag to enable TUI mode
    #[arg(short, long)]
    tui: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = CliArgs::parse();
    let event_storage = Arc::new(EventStorage::new()); // Initialize EventStorage for both modes

    if cli_args.tui {
        // TUI Mode
        // The server is not typically started in TUI mode unless specifically required.
        // For now, we will only run the TUI application.
        tui_app::run_tui_app(event_storage)
    } else {
        // GUI Mode
        let server_storage_gui = Arc::clone(&event_storage);

        // Spawn the HTTP server for GUI mode
        tokio::spawn(async move {
            if let Err(e) = start_server(server_storage_gui).await {
                eprintln!("Server error (GUI mode): {}", e);
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
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
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
