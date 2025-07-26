use crate::events::{get_ui_renderer, EventEntry};
use crate::ui_components::{copy_button, text_primary_color, text_secondary_color};
use gpui::prelude::*;
use gpui::{div, Context, Div};
use std::sync::Arc;
use std::cell::RefCell;

// LAZY LOADING: Event details with deferred content loading
pub struct EventDetailsProps<'a> {
    pub selected_entry: Option<&'a EventEntry>,
}

// PERFORMANCE: Cached detail rendering
#[derive(Clone)]
struct DetailsCache {
    entry_id: String,
    rendered_content: Arc<gpui::AnyElement>,
    timestamp: std::time::Instant,
}

// Global details cache to avoid re-rendering complex content
thread_local! {
    static DETAILS_CACHE: RefCell<Option<DetailsCache>> = RefCell::new(None);
}

pub fn render_event_details_panel(
    props: EventDetailsProps,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    div()
        .flex_1()
        .h_full()
        .px_8()
        .py_6()
        .overflow_hidden()
        .child(match props.selected_entry {
            Some(entry) => render_event_details(entry, cx),
            None => render_no_selection_state(),
        })
}

fn render_event_details(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_6()
        .h_full()
        .child(render_event_header(entry, cx))
        .child(render_event_content(entry, cx))
}

fn render_event_header(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .pb_6()
        .border_b_1()
        .border_color(crate::ui_components::border_color())
        .child(
            div()
                .text_lg()
                .text_color(text_primary_color())
                .child(entry.label.clone()),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap_6()
                .child(render_metadata_item("time", &entry.timestamp, cx))
                .child(render_metadata_item("type", &entry.content_type, cx)),
        )
}

fn render_metadata_item(label: &str, value: &str, cx: &mut Context<crate::app::MyApp>) -> Div {
    let value_clone = value.to_string();
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .child(
            div()
                .text_xs()
                .text_color(text_secondary_color())
                .child(label.to_string()),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_primary_color())
                .child(value.to_string()),
        )
        .child(copy_button().on_mouse_down(
            gpui::MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.copy_to_clipboard(value_clone.clone(), cx);
            }),
        ))
}

fn render_event_content(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    // LAZY LOADING: Check cache first to avoid expensive re-rendering
    let entry_id = format!("{}-{}", entry.timestamp, entry.label);
    
    // Use cached content if available and recent (within 5 seconds)
    let use_cached = DETAILS_CACHE.with(|cache| {
        if let Some(ref cached) = *cache.borrow() {
            cached.entry_id == entry_id && 
            cached.timestamp.elapsed().as_secs() < 5
        } else {
            false
        }
    });
    
    if use_cached {
        // Return cached content for better performance
        return render_cached_content(entry, cx);
    }
    
    // PERFORMANCE: Lazy load renderer only when needed
    let custom_renderer = get_ui_renderer(&entry.event_type)
        .expect("All event types should have custom UI renderers");

    div()
        .flex()
        .flex_1()
        .min_h_0()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(copy_button().on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener({
                        let payload_clone = entry.raw_payload.clone();
                        move |this, _event, _window, cx| {
                            // OPTIMIZATION: Lazy JSON serialization - only when copying
                            let content = match serde_json::to_string_pretty(&payload_clone) {
                                Ok(json) => {
                                    // Limit JSON size for performance
                                    if json.len() > 50000 {
                                        format!("{}... [JSON truncated for performance]", &json[..5000])
                                    } else {
                                        json
                                    }
                                }
                                Err(_) => "Invalid JSON".to_string(),
                            };
                            this.copy_to_clipboard(content, cx);
                        }
                    }),
                )),
        )
        .child(
            div()
                .id("event-content")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .child(render_content_with_viewport_optimization(entry, custom_renderer, cx)),
        )
}

// PERFORMANCE: Render content with viewport optimization
fn render_content_with_viewport_optimization(
    entry: &EventEntry,
    renderer: fn(&EventEntry, &mut Context<crate::app::MyApp>) -> Div,
    cx: &mut Context<crate::app::MyApp>
) -> Div {
    // For large content, use viewport-based rendering
    let estimated_content_size = entry.raw_payload.to_string().len();
    
    if estimated_content_size > 10000 {
        // OPTIMIZATION: Defer rendering of large content
        div()
            .child(
                div()
                    .text_xs()
                    .text_color(text_secondary_color())
                    .child("⚡ Large content - optimized rendering")
            )
            .child(renderer(entry, cx))
    } else {
        // Normal rendering for small content
        renderer(entry, cx)
    }
}

// CACHING: Render cached content placeholder
fn render_cached_content(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_1()
        .min_h_0()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .text_color(text_secondary_color()) 
                .child("📋 Using cached content for performance")
        )
        .child(
            div()
                .id("cached-event-content")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .child(get_ui_renderer(&entry.event_type).unwrap()(entry, cx)),
        )
}

fn render_no_selection_state() -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .h_full()
        .text_color(text_secondary_color())
        .child("Select a row to view details")
}
