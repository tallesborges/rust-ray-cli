use crate::event_details::{render_event_details_panel, EventDetailsProps};
use crate::event_list::render_event_list_panel;
use crate::event_storage::EventStorage;
use crate::ui_components::background_color;
use gpui::prelude::*;
use gpui::{
    actions, div, px, size, App, Application, Bounds, ClipboardItem, IntoElement, Render,
    TitlebarOptions, UniformListScrollHandle, Window, WindowBounds, WindowOptions,
};
use std::sync::Arc;
use std::collections::HashSet;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

actions!(app, [Quit]);


pub struct MyApp {
    payload_storage: Arc<EventStorage>,
    selected_row: Option<usize>,
    total_rows: usize,
    scroll_handle: UniformListScrollHandle,
    event_type_filters: HashSet<String>,
    // Performance optimization: cached filtered events
    filter_cache: RefCell<HashMap<u64, Arc<Vec<crate::events::EventEntry>>>>,
    cache_generation: RefCell<u64>,
}

impl MyApp {
    pub fn new(payload_storage: Arc<EventStorage>) -> Self {
        let mut event_type_filters = HashSet::new();
        // Enable all event types by default - based on actual event types generated
        event_type_filters.insert("cache".to_string());
        event_type_filters.insert("http".to_string());
        event_type_filters.insert("log".to_string());
        event_type_filters.insert("query".to_string());
        event_type_filters.insert("exception".to_string());
        event_type_filters.insert("application_log".to_string());
        event_type_filters.insert("request".to_string());
        event_type_filters.insert("table".to_string());
        
        Self {
            payload_storage,
            selected_row: Some(0),
            total_rows: 0,
            scroll_handle: UniformListScrollHandle::new(),
            event_type_filters,
            filter_cache: RefCell::new(HashMap::new()),
            cache_generation: RefCell::new(0),
        }
    }

    pub fn clear_events(&mut self, _cx: &mut Context<Self>) {
        self.payload_storage.clear_events();
        self.selected_row = Some(0);
        self.invalidate_cache();
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

    pub fn toggle_event_type_filter(&mut self, event_type: String, cx: &mut Context<Self>) {
        if self.event_type_filters.contains(&event_type) {
            self.event_type_filters.remove(&event_type);
        } else {
            self.event_type_filters.insert(event_type);
        }
        self.selected_row = Some(0);
        self.invalidate_cache();
        cx.notify();
    }


    pub fn is_row_selected(&self, index: usize) -> bool {
        self.selected_row == Some(index)
    }

    // Highly optimized cached filtering with minimal allocations
    pub fn get_filtered_events(&self) -> Arc<Vec<crate::events::EventEntry>> {
        let filter_hash = self.calculate_filter_hash();
        
        if let Some(cached) = self.filter_cache.borrow().get(&filter_hash) {
            return cached.clone();
        }
        
        // Cache miss - compute filtered events  
        let all_events = self.payload_storage.get_events_optimized();
        
        // Use iterator adaptors for better performance
        let filtered: Vec<crate::events::EventEntry> = all_events
            .iter()
            .filter(|event| {
                // Filter by event type only
                self.event_type_filters.contains(&event.event_type)
            })
            .map(|arc_event| (**arc_event).clone())  // Dereference Arc then clone
            .collect();
        
        let filtered_arc = Arc::new(filtered);
        
        // Update cache
        self.filter_cache.borrow_mut().insert(filter_hash, filtered_arc.clone());
        
        filtered_arc
    }

    fn calculate_filter_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        
        // Hash the event type filters
        let mut filters: Vec<_> = self.event_type_filters.iter().collect();
        filters.sort();
        filters.hash(&mut hasher);
        
        // Hash the storage generation to invalidate cache when events change
        self.payload_storage.get_generation().hash(&mut hasher);
        
        hasher.finish()
    }
    
    // Invalidate cache when events change
    fn invalidate_cache(&self) {
        *self.cache_generation.borrow_mut() += 1;
        self.filter_cache.borrow_mut().clear();
    }
    
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Use cached filtered events to avoid expensive recomputation
        let events = self.get_filtered_events();
        self.total_rows = events.len();

        // Ensure stable selection - cache the selected entry to prevent changes during mouse events
        // This prevents header values from changing during mouse movement over the event list
        let selected_entry = if let Some(index) = self.selected_row {
            if index < events.len() {
                events.get(index)
            } else {
                // Handle case where events changed but selection index is stale
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
                events.as_ref(),  // Pass slice instead of owned vector
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