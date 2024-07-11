// app.rs
use eframe::egui;
use crate::payload::PayloadStorage;
use std::sync::Arc;

pub struct MyApp {
    payload_storage: Arc<PayloadStorage>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, payload_storage: Arc<PayloadStorage>) -> Self {
        Self { payload_storage }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Payload Processing Server");

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("payloads_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Timestamp");
                        ui.label("URL");
                        ui.end_row();

                        for entry in self.payload_storage.get_payloads() {
                            ui.label(&entry.timestamp);
                            ui.label(&entry.url);
                            ui.end_row();
                        }
                    });
            });
        });

        ctx.request_repaint();
    }
}