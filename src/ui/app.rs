use crate::storage::EventStorage;
use crate::events::EventType;
use crate::ui::components::background_color;
use crate::ui::event_details::{render_event_details_panel, EventDetailsProps};
use crate::ui::event_list::render_event_list_panel;
use gpui::prelude::*;
use gpui::{
    actions, div, px, size, App, Application, Bounds, ClipboardItem, IntoElement, Render,
    TitlebarOptions, UniformListScrollHandle, Window, WindowBounds, WindowOptions,
};
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

actions!(app, [Quit]);

pub struct MyApp {
    payload_storage: Arc<EventStorage>,
    selected_row: Option<usize>,
    total_rows: usize,
    scroll_handle: UniformListScrollHandle,
    event_type_filters: HashSet<EventType>,
}

impl MyApp {
    pub fn new(payload_storage: Arc<EventStorage>) -> Self {
        let event_type_filters = EventType::all().into_iter().collect::<HashSet<_>>();

        Self {
            payload_storage,
            selected_row: Some(0),
            total_rows: 0,
            scroll_handle: UniformListScrollHandle::new(),
            event_type_filters,
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

    pub fn copy_to_clipboard(&mut self, text: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    pub fn toggle_event_type_filter(&mut self, event_type: EventType, cx: &mut Context<Self>) {
        if self.event_type_filters.contains(&event_type) {
            self.event_type_filters.remove(&event_type);
        } else {
            self.event_type_filters.insert(event_type);
        }
        self.selected_row = Some(0);
        cx.notify();
    }

    pub fn is_row_selected(&self, index: usize) -> bool {
        self.selected_row == Some(index)
    }

    pub fn get_filtered_events(&self) -> Vec<crate::events::EventEntry> {
        let all_events = self.payload_storage.get_events_optimized();

        all_events
            .iter()
            .filter(|event| {
                if let Ok(event_type) = event.event_type.parse::<EventType>() {
                    self.event_type_filters.contains(&event_type)
                } else {
                    false
                }
            })
            .map(|arc_event| (**arc_event).clone())
            .collect()
    }
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let events = self.get_filtered_events();
        self.total_rows = events.len();

        let selected_entry = if let Some(index) = self.selected_row {
            if index < events.len() {
                events.get(index)
            } else {
                self.selected_row = if events.is_empty() { None } else { Some(0) };
                events.first()
            }
        } else {
            None
        };

        div()
            .flex()
            .bg(background_color())
            .size_full()
            .child(render_event_list_panel(
                &events,
                &self.event_type_filters,
                self.selected_row,
                &self.scroll_handle,
                cx,
            ))
            .child(render_event_details_panel(
                EventDetailsProps { selected_entry },
                cx,
            ))
    }
}

pub fn run_app(
    payload_storage: Arc<EventStorage>,
    shutdown_tx: tokio::sync::oneshot::Sender<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Wrap shutdown_tx in a Rc<RefCell> to allow it to be shared across closures
    let shutdown_tx = Rc::new(RefCell::new(Some(shutdown_tx)));

    Application::new().run(move |cx: &mut App| {
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
            |window, cx| {
                // Handle window close event
                let close_storage = payload_storage.clone();
                let close_tx = Rc::clone(&shutdown_tx);
                window.on_window_should_close(cx, move |_window, _cx| {
                    close_storage.info("App", "Window close requested");

                    // Send shutdown signal to server
                    if let Some(tx) = close_tx.borrow_mut().take() {
                        close_storage.info("App", "Sending shutdown signal to server");
                        let _ = tx.send(());
                    }

                    // Force exit immediately
                    close_storage.info("App", "Force exiting application");
                    std::process::exit(0);
                });

                cx.new(|_cx| MyApp::new(payload_storage))
            },
        )
        .unwrap();

        cx.activate(true);
    });

    Ok(())
}
