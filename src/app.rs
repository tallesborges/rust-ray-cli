use crate::event_storage::EventStorage;
use eframe::egui;
use eframe::egui::Sense;
use egui_commonmark::*;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::sync::Arc;

pub struct MyApp {
    payload_storage: Arc<EventStorage>,
    selected_row: Option<usize>,
    total_rows: usize,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, payload_storage: Arc<EventStorage>) -> Self {
        Self {
            payload_storage,
            selected_row: Some(0),
            total_rows: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_secs(1));

        if ctx.input(|i| i.pointer.any_pressed()) && self.selected_row.is_none() {
            self.selected_row = Some(0);
        }

        // Handle keyboard input
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            if let Some(current) = self.selected_row {
                self.selected_row = if current > 0 {
                    Some(current - 1)
                } else {
                    Some(0)
                };
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            if let Some(current) = self.selected_row {
                self.selected_row = if current + 1 < self.total_rows {
                    Some(current + 1)
                } else {
                    Some(current)
                };
            }
        }
        egui::SidePanel::left("table_panel").show(ctx, |ui| {
            ui.heading("Payload Processing Server");

            if ui.button("Clear").clicked() {
                self.payload_storage.clear_events();
                self.selected_row = Some(0);
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
            let payloads = self.payload_storage.get_events();
            self.total_rows = payloads.len();
            body.rows(18.0, self.total_rows, |mut row| {
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

pub fn display_code(ui: &mut egui::Ui, content: &str) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        let mut cache = CommonMarkCache::default();
        CommonMarkViewer::new().show(ui, &mut cache, content);
    });
}
