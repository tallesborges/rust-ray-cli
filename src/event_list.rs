use crate::event_storage::EventStorage;
use crate::events::EventEntry;
use crate::ui_components::{
    background_color, border_color, hover_color, selection_color, text_primary_color,
    text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, uniform_list, Context, Div, FontWeight, IntoElement, UniformListScrollHandle};
use std::sync::Arc;

pub fn render_event_list_panel(
    payload_storage: &Arc<EventStorage>,
    selected_row: Option<usize>,
    scroll_handle: &UniformListScrollHandle,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    let events = payload_storage.get_events();

    div()
        .flex()
        .flex_col()
        .w_64()
        .h_full()
        .bg(background_color())
        .border_r_1()
        .border_color(border_color())
        .child(render_header(cx))
        .child(render_event_list(events, selected_row, scroll_handle, cx))
}

fn render_header(cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_row()
        .justify_between()
        .items_center()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(border_color())
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
        )
}

fn render_event_list(
    events: Vec<EventEntry>,
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
    events: Vec<EventEntry>,
    selected_row: Option<usize>,
    scroll_handle: &UniformListScrollHandle,
    cx: &mut Context<crate::app::MyApp>,
) -> Div {
    div().size_full().child(
        uniform_list(cx.entity().clone(), "event_list", events.len(), {
            let events = events.clone();
            move |_this, range, _window, cx| {
                range
                    .map(|index| {
                        let entry = &events[index];
                        let is_selected = selected_row == Some(index);
                        let bg_color = if is_selected {
                            selection_color()
                        } else {
                            background_color()
                        };

                        div()
                            .id(("event", index))
                            .flex()
                            .flex_col()
                            .px_4()
                            .py_3()
                            .gap_1()
                            .h_16()
                            .bg(bg_color)
                            .hover(|style| style.bg(hover_color()))
                            .cursor_pointer()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    this.select_row(index, cx);
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(render_event_label(&entry.label))
                                    .child(render_event_timestamp(&entry.timestamp)),
                            )
                            .child(render_event_description(&entry.description))
                    })
                    .collect()
            }
        })
        .size_full()
        .track_scroll(scroll_handle.clone()),
    )
}

fn render_event_timestamp(timestamp: &str) -> Div {
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .opacity(0.5)
        .child(timestamp.to_string())
}

fn render_event_label(label: &str) -> Div {
    div()
        .text_sm()
        .text_color(text_primary_color())
        .child(label.to_string())
}

fn render_event_description(description: &str) -> Div {
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .opacity(0.7)
        .truncate()
        .child(description.to_string())
}
