use crate::payload::PayloadEntry;
use crate::payload::PayloadStorage;
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

            StripBuilder::new(ui)
                .size(Size::remainder()) // for the table
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.render_table(ui);
                        });
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HTML Preview");
            self.render_html_preview(ui);
        });
    }
}

impl MyApp {
    fn render_html_preview(&self, ui: &mut egui::Ui) {
        let selected_entry = self.get_selected_entry();

        match selected_entry {
            Some(entry) => self.display_entry_details(ui, &entry),
            None => {
                ui.label("Select a row to view HTML preview");
            }
        }
    }

    fn get_selected_entry(&self) -> Option<PayloadEntry> {
        self.selected_row
            .and_then(|index| self.payload_storage.get_payloads().get(index).cloned())
    }

    fn display_entry_details(&self, ui: &mut egui::Ui, entry: &PayloadEntry) {
        ui.label("URL:");
        ui.label(&entry.url);
        ui.label("Method:");
        ui.label(&entry.method);
        ui.label("HTML Content:");
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&entry.html);
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
                    ui.strong("Timestamp");
                });
                header.col(|ui| {
                    ui.strong("Label");
                });
                header.col(|ui| {
                    ui.strong("Value");
                });
            });

        table.body(|body| {
            let payloads = self.payload_storage.get_payloads();
            body.rows(
                18.0,
                self.payload_storage.get_payloads().len(),
                |mut row| {
                    let entry = payloads.get(row.index()).unwrap();
                    row.set_selected(self.selected_row == Some(row.index()));

                    row.col(|ui| {
                        ui.label(entry.timestamp.as_str());
                    });
                    row.col(|ui| {
                        ui.label(entry.label.as_str());
                    });
                    row.col(|ui| {
                        ui.label(entry.url.as_str());
                    });

                    if row.response().clicked() {
                        self.selected_row = Some(row.index());
                    }
                },
            );
        });
    }
}
