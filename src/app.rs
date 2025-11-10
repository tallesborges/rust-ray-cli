use crate::app_state::AppState;
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
use std::sync::Arc;

actions!(app, [Quit]);

/// MyApp holds only UI-specific state.
/// Shared application services (like payload_storage) are accessed via the global AppState.
pub struct MyApp {
    selected_row: Option<usize>,
    total_rows: usize,
    scroll_handle: UniformListScrollHandle,
    event_type_filters: HashSet<EventType>,
    _watcher_task: gpui::Task<()>,
}

impl MyApp {
    pub fn new(_window: &Window, _cx: &mut Context<Self>) -> Self {
        let event_type_filters = EventType::all().into_iter().collect::<HashSet<_>>();

        Self {
            selected_row: Some(0),
            total_rows: 0,
            scroll_handle: UniformListScrollHandle::new(),
            event_type_filters,
            _watcher_task: gpui::Task::ready(()),
        }
    }

    pub fn start_watcher(&mut self, window: &Window, cx: &mut Context<Self>) {
        let storage = cx.global::<AppState>().payload_storage.clone();

        self._watcher_task = cx.spawn_in(window, async move |this, cx| {
            let mut last_count = storage.event_count();

            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(150))
                    .await;

                let cur = storage.event_count();
                if cur != last_count {
                    last_count = cur;

                    // Notify UI of changes
                    this.update(cx, |_, cx| cx.notify()).ok();
                }
            }
        });
    }

    pub fn clear_events(&mut self, cx: &mut Context<Self>) {
        let storage = cx.global::<AppState>().payload_storage.clone();
        storage.clear_events();
        self.selected_row = Some(0);
        cx.notify();
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

    pub fn get_filtered_events(&self, cx: &mut Context<Self>) -> Vec<crate::events::EventEntry> {
        let storage = cx.global::<AppState>().payload_storage.clone();
        let all_events = storage.get_events_optimized();

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
        let events = self.get_filtered_events(cx);
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
    payload_storage: Arc<crate::storage::EventStorage>,
) -> Result<(), Box<dyn std::error::Error>> {
    payload_storage.info("App", "Before Application::new().run()");

    Application::new().run(move |cx: &mut App| {
        payload_storage.info("App", "Inside Application::new().run() closure");

        // Initialize global AppState with shared services
        cx.set_global(AppState::new(payload_storage.clone()));

        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        match cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Payload Processing Server".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                // Create app entity and start watcher
                let app_entity = cx.new(|cx| MyApp::new(window, cx));
                app_entity.update(cx, |app, cx| app.start_watcher(window, cx));

                // Handle window close event
                let storage = payload_storage.clone();
                window.on_window_should_close(cx, move |_window, cx| {
                    storage.info("App", "Window close requested");
                    cx.quit();
                    true
                });

                app_entity
            },
        ) {
            Ok(_) => payload_storage.info("App", "Window opened successfully"),
            Err(e) => payload_storage.error("App", &format!("Failed to open window: {e}")),
        }

        cx.activate(true);
    });

    Ok(())
}
