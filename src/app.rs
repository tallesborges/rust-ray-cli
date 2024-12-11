use crate::event_storage::EventStorage;
use eframe::egui;
use eframe::egui::Sense;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::sync::Arc;

pub struct MyApp {
    payload_storage: Arc<EventStorage>,
    selected_row: Option<usize>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, payload_storage: Arc<EventStorage>) -> Self {
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
                self.payload_storage.clear_events();
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
            let payloads = self.payload_storage.get_events();
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

pub fn display_code(ui: &mut egui::Ui, content: &str, language: &str) {
    ui.horizontal(|ui| {
        if ui.button("📋 Copy").clicked() {
            ui.output_mut(|o| o.copied_text = content.to_string());
        }
    });
    egui::ScrollArea::vertical().show(ui, |ui| {
        let theme = egui_extras::syntax_highlighting::CodeTheme::from_style(&ui.ctx().style());
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                ui.ctx(),
                ui.style(),
                &theme,
                string,
                language,
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut content.clone())
                .code_editor()
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace.resolve(ui.style()))
                .desired_rows(10)
                .layouter(&mut layouter)
                .lock_focus(true),
        );
    });
}
