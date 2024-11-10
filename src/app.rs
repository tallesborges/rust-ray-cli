use crate::payload_storage::PayloadStorage;
use eframe::egui;
use eframe::egui::Sense;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::sync::Arc;

pub struct MyApp {
    payload_storage: Arc<PayloadStorage>,
    selected_row: Option<usize>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, payload_storage: Arc<PayloadStorage>) -> Self {
        Self {
            payload_storage,
            selected_row: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("table_panel").show(ctx, |ui| {
            ui.heading("Payload Processing Server");

            if ui.button("Clear").clicked() {
                self.payload_storage.clear_payloads();
                self.selected_row = None;
            }

            StripBuilder::new(ui)
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.render_table(ui);
                        });
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Preview");

            if let Some(index) = self.selected_row {
                self.payload_storage.display_details(ui, index);
            } else {
                ui.label("Select a row to view HTML preview");
            }
        });
    }
}

impl MyApp {
    fn render_table(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .sense(Sense::click())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .max_scroll_height(available_height)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("timestamp");
                });
                header.col(|ui| {
                    ui.strong("label");
                });
                header.col(|ui| {
                    ui.strong("description");
                });
            });

        table.body(|body| {
            let payloads = self.payload_storage.get_payloads();
            body.rows(18.0, payloads.len(), |mut row| {
                let index = row.index();
                row.set_selected(self.selected_row == Some(index));

                let entry = &payloads[index];

                row.col(|ui| {
                    ui.add(egui::Label::new(&entry.timestamp).selectable(false));
                });
                row.col(|ui| {
                    ui.add(egui::Label::new(&entry.label).selectable(false));
                });
                row.col(|ui| {
                    ui.add(egui::Label::new(&entry.description).selectable(false));
                });

                if row.response().clicked() {
                    self.selected_row = Some(index);
                }
            });
        });
    }
}
