// app.rs
use eframe::egui;
use crate::payload::PayloadStorage;
use std::sync::Arc;
use egui_extras::{TableBuilder, Column};

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
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::remainder())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Timestamp");
                        });
                        header.col(|ui| {
                            ui.strong("Type");
                        });
                        header.col(|ui| {
                            ui.strong("URL");
                        });
                    });

                table.body(|mut body| {
                    for entry in self.payload_storage.get_payloads() {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&entry.timestamp);
                            });
                            row.col(|ui| {
                                ui.label(&entry.p_type);
                            });
                            row.col(|ui| {
                                ui.label(&entry.url);
                            });
                        });
                    }
                });
            });
        });

        ctx.request_repaint();
    }
}