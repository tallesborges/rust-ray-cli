use crate::event_details::{render_event_details_panel, EventDetailsProps};
use crate::event_list::render_event_list_panel;
use crate::event_storage::EventStorage;
use crate::ui_components::background_color;
use gpui::prelude::*;
use gpui::{
    actions, div, px, size, App, Application, Bounds, IntoElement, Render, TitlebarOptions,
    UniformListScrollHandle, Window, WindowBounds, WindowOptions,
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

        let selected_entry = self.selected_row
            .and_then(|index| events.get(index));

        div()
            .flex()
            .bg(background_color())
            .size_full()
            .child(render_event_list_panel(
                &self.payload_storage,
                self.selected_row,
                &self.scroll_handle,
                cx,
            ))
            .child(render_event_details_panel(EventDetailsProps {
                selected_entry,
            }))
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
