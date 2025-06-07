use crate::ui_components::{
    background_color, border_color, styled_card, styled_label, styled_monospace, styled_value,
    text_monospace_color, text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, Div, InteractiveText, IntoElement, StyledText};
use shared::EventEntry;

pub struct EventDetailsProps<'a> {
    pub selected_entry: Option<&'a EventEntry>,
}

pub fn render_event_details_panel(props: EventDetailsProps) -> Div {
    div()
        .flex_1()
        .h_full()
        .p_4()
        .overflow_hidden()
        .child(match props.selected_entry {
            Some(entry) => render_event_details(entry),
            None => render_no_selection_state(),
        })
}

fn render_event_details(entry: &EventEntry) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .h_full()
        .child(render_event_header(entry))
        .child(render_event_content(entry))
}

fn render_event_header(entry: &EventEntry) -> Div {
    styled_card()
        .flex()
        .flex_col()
        .gap_2()
        .child(render_header_row("Label:", &entry.label))
        .child(render_header_row("Time:", &entry.timestamp))
        .child(render_header_row("Type:", &entry.content_type))
}

fn render_header_row(label: &str, value: &str) -> Div {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .child(styled_label().child(label.to_string()))
        .child(styled_value().child(value.to_string()))
}

fn render_event_content(entry: &EventEntry) -> impl IntoElement {
    div()
        .id("event-content")
        .overflow_y_scroll()
        .p_4()
        .h_full()
        .bg(background_color())
        .rounded_lg()
        .border_1()
        .border_color(border_color())
        .child(match entry.content_type.as_str() {
            "json" => render_json_content(&entry.content),
            "markdown" => render_markdown_content(&entry.content),
            _ => render_plain_text_content(&entry.content),
        })
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
