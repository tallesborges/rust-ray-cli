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
