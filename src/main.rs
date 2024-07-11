// main.rs
mod app;
mod server;
mod payload;

use app::MyApp;
use server::start_server;
use payload::PayloadStorage;
use std::sync::Arc;
use eframe::NativeOptions;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let payload_storage = Arc::new(PayloadStorage::new());
    let server_storage = Arc::clone(&payload_storage);

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
        Box::new(|cc| Ok(Box::new(MyApp::new(cc, payload_storage)))),
    )
}