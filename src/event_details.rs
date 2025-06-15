use crate::ui_components::{
    background_color, border_color, copy_button, styled_card, styled_label, styled_value,
    text_monospace_color, text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, Context, Div, InteractiveText, IntoElement, StyledText};
use crate::events::EventEntry;

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
        .p_4()
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
        .gap_4()
        .h_full()
        .child(render_event_header(entry, cx))
        .child(render_event_content(entry, cx))
}

fn render_event_header(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    styled_card()
        .flex()
        .flex_col()
        .gap_2()
        .child(render_header_row("Label:", &entry.label, cx))
        .child(render_header_row("Time:", &entry.timestamp, cx))
        .child(render_header_row("Type:", &entry.content_type, cx))
}

fn render_header_row(label: &str, value: &str, cx: &mut Context<crate::app::MyApp>) -> Div {
    let value_clone = value.to_string();
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .items_center()
                .child(styled_label().child(label.to_string()))
                .child(styled_value().child(value.to_string())),
        )
        .child(copy_button(value.to_string()).on_mouse_down(
            gpui::MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.copy_to_clipboard(value_clone.clone(), cx);
            }),
        ))
}

fn render_event_content(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    let content_clone = entry.content.clone();
    div()
        .flex()
        .flex_1()
        .min_h_0()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .pb_2()
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary_color())
                        .child("Content"),
                )
                .child(copy_button(entry.content.clone()).on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(move |this, _event, _window, cx| {
                        this.copy_to_clipboard(content_clone.clone(), cx);
                    }),
                )),
        )
        .child(
            div()
                .id("event-content")
                .flex_1()
                .min_h_0()
                .p_4()
                .bg(background_color())
                .rounded_lg()
                .border_1()
                .border_color(border_color())
                .overflow_y_scroll()
                .child(match entry.content_type.as_str() {
                    "json" => render_json_content(&entry.content),
                    "markdown" => render_markdown_content(&entry.content),
                    _ => render_plain_text_content(&entry.content),
                }),
        )
}

fn render_json_content(content: &str) -> Div {
    let formatted_content = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(content) {
        serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| content.to_string())
    } else {
        content.to_string()
    };

    div()
        .font_family("monospace")
        .text_sm()
        .text_color(text_monospace_color())
        .child(InteractiveText::new(
            "json-content",
            StyledText::new(formatted_content),
        ))
}

fn render_markdown_content(content: &str) -> Div {
    div()
        .text_sm()
        .text_color(text_monospace_color())
        .child(InteractiveText::new(
            "markdown-content",
            StyledText::new(content.to_string()),
        ))
}

fn render_plain_text_content(content: &str) -> Div {
    div()
        .text_sm()
        .text_color(text_monospace_color())
        .child(InteractiveText::new(
            "plain-content",
            StyledText::new(content.to_string()),
        ))
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
