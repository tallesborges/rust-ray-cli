use crate::event_storage::EventStorage;
use gpui::prelude::*;
use gpui::{
    actions, div, px, rgb, size, uniform_list, App, Application, Bounds, Div, FontWeight,
    IntoElement, Render, TitlebarOptions, UniformListScrollHandle, Window, WindowBounds,
    WindowOptions,
};
use std::sync::Arc;

actions!(app, [Quit]);

pub struct MyApp {
    payload_storage: Arc<EventStorage>,
    selected_row: Option<usize>,
    total_rows: usize,
    scroll_handle: UniformListScrollHandle,
}

impl MyApp {
    pub fn new(payload_storage: Arc<EventStorage>) -> Self {
        Self {
            payload_storage,
            selected_row: Some(0),
            total_rows: 0,
            scroll_handle: UniformListScrollHandle::new(),
        }
    }

    pub fn clear_events(&mut self, _cx: &mut Context<Self>) {
        self.payload_storage.clear_events();
        self.selected_row = Some(0);
        _cx.notify();
    }

    pub fn select_row(&mut self, index: usize, _cx: &mut Context<Self>) {
        if index < self.total_rows {
            self.selected_row = Some(index);
            _cx.notify();
        }
    }

    pub fn move_selection_up(&mut self, _cx: &mut Context<Self>) {
        if let Some(current) = self.selected_row {
            if current > 0 {
                self.selected_row = Some(current - 1);
                _cx.notify();
            }
        }
    }

    pub fn move_selection_down(&mut self, _cx: &mut Context<Self>) {
        if let Some(current) = self.selected_row {
            if current + 1 < self.total_rows {
                self.selected_row = Some(current + 1);
                _cx.notify();
            }
        }
    }
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let events = self.payload_storage.get_events();
        self.total_rows = events.len();

        div()
            .flex()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .child(
                // Left panel - event list
                div()
                    .flex()
                    .flex_col()
                    .w_64()
                    .h_full()
                    .bg(rgb(0x252526))
                    .border_r_1()
                    .border_color(rgb(0x3e3e42))
                    .child(
                        // Header
                        div()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .items_center()
                            .p_4()
                            .border_b_1()
                            .border_color(rgb(0x3e3e42))
                            .child(
                                div()
                                    .text_lg()
                                    .text_center()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("Payload Processing Server"),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x007acc))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(rgb(0x005a9e)))
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.clear_events(cx);
                                        }),
                                    )
                                    .child("Clear"),
                            ),
                    )
                    .child(
                        // Event list with uniform list for better performance
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(if events.is_empty() {
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .h_full()
                                    .text_color(rgb(0x969696))
                                    .child("No events yet...")
                            } else {
                                div()
                                    .size_full()
                                    .child(
                                        uniform_list(
                                            cx.entity().clone(),
                                            "event_list",
                                            events.len(),
                                            {
                                                let events = events.clone();
                                                let selected_row = self.selected_row;
                                                move |_this, range, _window, cx| {
                                                    range.map(|index| {
                                                        let entry = &events[index];
                                                        let is_selected = selected_row == Some(index);
                                                        let bg_color = if is_selected {
                                                            rgb(0x094771)
                                                        } else {
                                                            rgb(0x252526)
                                                        };

                                                        div()
                                                            .id(("event", index))
                                                            .flex()
                                                            .flex_col()
                                                            .p_2()
                                                            .bg(bg_color)
                                                            .hover(|style| style.bg(rgb(0x2a2d2e)))
                                                            .cursor_pointer()
                                                            .on_mouse_down(
                                                                gpui::MouseButton::Left,
                                                                cx.listener(move |this, _event, _window, cx| {
                                                                    this.select_row(index, cx);
                                                                }),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x969696))
                                                                    .child(entry.timestamp.clone()),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .font_weight(FontWeight::MEDIUM)
                                                                    .text_color(rgb(0xcccccc))
                                                                    .child(entry.label.clone()),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x969696))
                                                                    .truncate()
                                                                    .child(entry.description.clone()),
                                                            )
                                                    }).collect()
                                                }
                                            }
                                        )
                                        .size_full()
                                        .track_scroll(self.scroll_handle.clone())
                                    )
                            }),
                    ),
            )
            .child(
                // Right panel - details view
                div().flex_1().h_full().p_4().overflow_hidden().child(
                    if let Some(index) = self.selected_row {
                        if let Some(entry) = events.get(index) {
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .h_full()
                                .child(
                                    // Entry details header
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .p_4()
                                        .bg(rgb(0x2d2d30))
                                        .rounded_lg()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .gap_2()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0x969696))
                                                        .child("Label:"),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(rgb(0xcccccc))
                                                        .child(entry.label.clone()),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .gap_2()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0x969696))
                                                        .child("Time:"),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0xcccccc))
                                                        .child(entry.timestamp.clone()),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .gap_2()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0x969696))
                                                        .child("Type:"),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0xcccccc))
                                                        .child(entry.content_type.clone()),
                                                ),
                                        ),
                                )
                                .child(
                                    // Content area
                                    div()
                                        .id("event-content")
                                        .overflow_y_scroll()
                                        .p_4()
                                        .h_full()
                                        .bg(rgb(0x1e1e1e))
                                        .rounded_lg()
                                        .border_1()
                                        .border_color(rgb(0x3e3e42))
                                        .child(match entry.content_type.as_str() {
                                            "json" => {
                                                // Pretty print JSON
                                                if let Ok(parsed) =
                                                    serde_json::from_str::<serde_json::Value>(
                                                        &entry.content,
                                                    )
                                                {
                                                    if let Ok(pretty) =
                                                        serde_json::to_string_pretty(&parsed)
                                                    {
                                                        div()
                                                            .font_family("monospace")
                                                            .text_sm()
                                                            .text_color(rgb(0xd4d4d4))
                                                            .child(pretty)
                                                    } else {
                                                        div()
                                                            .font_family("monospace")
                                                            .text_sm()
                                                            .text_color(rgb(0xd4d4d4))
                                                            .child(entry.content.clone())
                                                    }
                                                } else {
                                                    div()
                                                        .font_family("monospace")
                                                        .text_sm()
                                                        .text_color(rgb(0xd4d4d4))
                                                        .child(entry.content.clone())
                                                }
                                            }
                                            "markdown" => {
                                                // For now, just display as plain text
                                                // TODO: Add proper markdown rendering
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0xd4d4d4))
                                                    .child(entry.content.clone())
                                            }
                                            _ => {
                                                // Plain text
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0xd4d4d4))
                                                    .child(entry.content.clone())
                                            }
                                        }),
                                )
                        } else {
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .h_full()
                                .text_color(rgb(0x969696))
                                .child("No event selected")
                        }
                    } else {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h_full()
                            .text_color(rgb(0x969696))
                            .child("Select a row to view details")
                    },
                ),
            )
    }
}

pub fn run_app(payload_storage: Arc<EventStorage>) -> Result<(), Box<dyn std::error::Error>> {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Payload Processing Server".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| MyApp::new(payload_storage)),
        )
        .unwrap();

        cx.activate(true);
    });

    Ok(())
}
