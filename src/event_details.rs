use crate::events::{get_ui_renderer, EventEntry};
use crate::ui_components::{
    copy_button, text_primary_color, text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, Context, Div};

pub struct EventDetailsProps<'a> {
    pub selected_entry: Option<&'a EventEntry>,
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
                .child(entry.label.clone())
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap_6()
                .child(render_metadata_item("time", &entry.timestamp, cx))
                .child(render_metadata_item("type", &entry.content_type, cx))
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
                .child(label.to_string())
        )
        .child(
            div()
                .text_xs()
                .text_color(text_primary_color())
                .child(value.to_string())
        )
        .child(copy_button(value.to_string()).on_mouse_down(
            gpui::MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.copy_to_clipboard(value_clone.clone(), cx);
            }),
        ))
}

fn render_event_content(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    // All event types now have custom UI renderers
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
                .child(
                    copy_button(
                        serde_json::to_string_pretty(&entry.raw_payload).unwrap_or_default(),
                    )
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener({
                            let payload_clone = entry.raw_payload.clone();
                            move |this, _event, _window, cx| {
                                let content = serde_json::to_string_pretty(&payload_clone)
                                    .unwrap_or_default();
                                this.copy_to_clipboard(content, cx);
                            }
                        }),
                    ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary_color())
                        .child("raw payload")
                ),
        )
        .child(
            div()
                .id("event-content")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .child(custom_renderer(entry, cx)),
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
