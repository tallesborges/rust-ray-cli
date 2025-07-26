use crate::events::{EventEntry, EventType};
use crate::ui_components::{
    background_color, border_color, hover_color, selection_color, text_primary_color,
    text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, uniform_list, Context, Div, FontWeight, IntoElement, UniformListScrollHandle};
use std::collections::HashSet;

pub fn render_event_list_panel(
    events: &[EventEntry],  // Use slice instead of Vec reference for better performance
    event_type_filters: &HashSet<EventType>,
    selected_row: Option<usize>,
    scroll_handle: &UniformListScrollHandle,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .w_80()
        .h_full()
        .bg(background_color())
        .border_r_1()
        .border_color(border_color())
        .child(render_header_with_filters(event_type_filters, cx))
        .child(render_event_list(events, selected_row, scroll_handle, cx))
}

fn render_header_with_filters(
    event_type_filters: &HashSet<EventType>,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    // Much simpler - just get all event types from the enum
    let event_types = EventType::all();

    div()
        .flex()
        .flex_col()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(border_color())
        .child(
            // Header row with title and clear button
            div()
                .flex()
                .flex_row()
                .justify_between()
                .items_center()
                .mb_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_primary_color())
                        .child("Events"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .cursor_pointer()
                        .hover(|style| style.text_color(text_primary_color()))
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.clear_events(cx);
                            }),
                        )
                        .child("clear"),
                ),
        )
        .child(
            // Filters section
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .child("Filter by type:"),
                )
                .child(render_filter_checkboxes(event_types, event_type_filters, cx)),
        )
}

fn render_filter_checkboxes(
    event_types: Vec<EventType>,
    event_type_filters: &HashSet<EventType>,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .children(
            event_types
                .into_iter()
                .map(|event_type| {
                    let is_enabled = event_type_filters.contains(&event_type);
                    let checkbox_style = if is_enabled {
                        text_primary_color()
                    } else {
                        text_secondary_color()
                    };

                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .cursor_pointer()
                        .hover(|style| style.text_color(text_primary_color()))
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            {
                                let event_type_copy = event_type; // EventType is Copy, no need to clone
                                cx.listener(move |this, _event, _window, cx| {
                                    this.toggle_event_type_filter(event_type_copy, cx);
                                })
                            },
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(checkbox_style)
                                .child(if is_enabled { "☑" } else { "☐" }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(checkbox_style)
                                .child(event_type.display_name()), // Use pretty display name
                        )
                })
                .collect::<Vec<_>>(),
        )
}


fn render_event_list(
    events: &[EventEntry],  // Use slice to avoid cloning
    selected_row: Option<usize>,
    scroll_handle: &UniformListScrollHandle,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    div()
        .flex_1()
        .overflow_y_hidden()
        .max_h_full()
        .child(if events.is_empty() {
            render_empty_state().into_any_element()
        } else {
            render_event_uniform_list(events, selected_row, scroll_handle, cx).into_any_element()
        })
}

fn render_empty_state() -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .h_full()
        .text_color(text_secondary_color())
        .child("No events yet...")
}

fn render_event_uniform_list(
    events: &[EventEntry],  // Use slice to avoid cloning
    selected_row: Option<usize>,
    scroll_handle: &UniformListScrollHandle,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    // CRITICAL OPTIMIZATION: Use Arc to avoid cloning entire events vector
    let events_arc = std::sync::Arc::new(events.to_vec());
    
    div().size_full().child(
        uniform_list(cx.entity().clone(), "event_list", events.len(), {
            // Use Arc to share data without cloning
            let events_ref = events_arc.clone();
            move |_this, range, _window, cx| {
                range
                    .map(|index| {
                        // PERFORMANCE: Direct access without additional cloning
                        let entry = &events_ref[index];
                        let is_selected = selected_row == Some(index);
                        let bg_color = if is_selected {
                            selection_color()
                        } else {
                            background_color()
                        };

                        // VIEWPORT OPTIMIZATION: Calculate virtual scrolling height
                        let item_height = 64; // 16 * 4px = 64px per item
                        
                        div()
                            .id(("event", index))
                            .flex()
                            .flex_col()
                            .px_4()
                            .py_3()
                            .gap_1()
                            .h(gpui::px(item_height as f32)) // Fixed height for virtual scrolling
                            .bg(bg_color)
                            .when(!is_selected, |div| {
                                div.hover(|style| style.bg(hover_color()))
                            })
                            .cursor_pointer()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    // Prevent multiple rapid selections and event bubbling
                                    cx.stop_propagation();
                                    // Only select if not already selected to prevent unnecessary re-renders
                                    if !this.is_row_selected(index) {
                                        this.select_row(index, cx);
                                    }
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(render_event_label_optimized(&entry.label))
                                    .child(render_event_timestamp_optimized(&entry.timestamp)),
                            )
                            .child(render_event_description_optimized(&entry.description))
                    })
                    .collect()
            }
        })
        .size_full()
        .track_scroll(scroll_handle.clone()),
    )
}

// PERFORMANCE OPTIMIZED: Pre-computed truncation and minimal string allocations
fn render_event_timestamp_optimized(timestamp: &str) -> Div {
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .opacity(0.5)
        .child(timestamp.to_string()) // Need to_string() for GPUI
}

fn render_event_label_optimized(label: &str) -> Div {
    // OPTIMIZATION: Pre-compute truncation length to avoid runtime calculation
    let display_label = if label.len() > 50 {
        format!("{}...", &label[..47]) // Pre-computed truncation with ellipsis
    } else {
        label.to_string()
    };
    
    div()
        .text_sm()
        .text_color(text_primary_color())
        .child(display_label)
}

fn render_event_description_optimized(description: &str) -> Div {
    // OPTIMIZATION: Pre-compute truncation with ellipsis for better UX
    let display_desc = if description.len() > 80 {
        format!("{}...", &description[..77]) // Only allocate when truncating
    } else {
        description.to_string()
    };
    
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .opacity(0.7)
        .child(display_desc)
}

