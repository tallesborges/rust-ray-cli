use crate::events::base::EventEntry;
use crate::ui_components::{
    background_color, border_color, styled_card, text_monospace_color, text_secondary_color,
};
use gpui::prelude::*;
use gpui::{div, Context, Div, InteractiveText, StyledText};
use serde_json::Value;

pub fn render_log_event(entry: &EventEntry, cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(render_log_header(entry))
        .child(render_log_values(entry))
        .child(render_origin_info(entry))
}

fn render_log_header(entry: &EventEntry) -> Div {
    styled_card()
        .p_3()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(text_secondary_color())
                .child("📝 Log Event")
        )
}

fn render_log_values(entry: &EventEntry) -> Div {
    let values = entry.raw_payload
        .get("content")
        .and_then(|c| c.get("values"))
        .cloned()
        .unwrap_or(Value::Array(vec![]));

    styled_card()
        .p_4()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(text_secondary_color())
                        .child("Values")
                )
                .child(render_values_list(&values))
        )
}

fn render_values_list(values: &Value) -> Div {
    match values {
        Value::Array(arr) => {
            let mut container = div()
                .flex()
                .flex_col()
                .gap_2();

            for (index, value) in arr.iter().enumerate() {
                container = container.child(render_single_value(index, value));
            }

            container
        }
        _ => render_single_value(0, values)
    }
}

fn render_single_value(index: usize, value: &Value) -> Div {
    div()
        .flex()
        .flex_row()
        .gap_3()
        .items_start()
        .p_3()
        .bg(background_color())
        .rounded_lg()
        .border_1()
        .border_color(border_color())
        .child(
            // Index badge
            div()
                .w_6()
                .h_6()
                .rounded_full()
                .bg(gpui::rgb(0x3b82f6))
                .text_color(gpui::white())
                .text_xs()
                .flex()
                .items_center()
                .justify_center()
                .child((index + 1).to_string())
        )
        .child(
            // Value content
            div()
                .flex_1()
                .child(match value {
                    Value::String(s) => render_string_value(s),
                    Value::Number(n) => render_number_value(n),
                    Value::Bool(b) => render_bool_value(*b),
                    Value::Null => render_null_value(),
                    Value::Object(_) | Value::Array(_) => render_complex_value(value),
                })
        )
}

fn render_string_value(s: &str) -> Div {
    div()
        .text_sm()
        .text_color(text_monospace_color())
        .child(format!("\"{}\"", s))
}

fn render_number_value(n: &serde_json::Number) -> Div {
    div()
        .text_sm()
        .text_color(gpui::rgb(0x10b981)) // Green for numbers
        .font_weight(gpui::FontWeight::MEDIUM)
        .child(n.to_string())
}

fn render_bool_value(b: bool) -> Div {
    div()
        .text_sm()
        .text_color(if b { gpui::rgb(0x10b981) } else { gpui::rgb(0xef4444) }) // Green/Red
        .font_weight(gpui::FontWeight::BOLD)
        .child(b.to_string())
}

fn render_null_value() -> Div {
    div()
        .text_sm()
        .text_color(gpui::rgb(0x6b7280)) // Gray
        .italic()
        .child("null")
}

fn render_complex_value(value: &Value) -> Div {
    let formatted = serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string());
    
    div()
        .max_h_32()
        .overflow_hidden()
        .p_2()
        .bg(gpui::rgb(0x1f2937))
        .rounded_lg()
        .child(
            div()
                .font_family("monospace")
                .text_xs()
                .text_color(text_monospace_color())
                .child(InteractiveText::new(
                    "complex-value",
                    StyledText::new(formatted),
                ))
        )
}

fn render_origin_info(entry: &EventEntry) -> Div {
    if let Some(origin) = entry.raw_payload.get("origin") {
        let file = origin.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line = origin.get("line_number").and_then(|l| l.as_u64()).unwrap_or(0);
        let hostname = origin.get("hostname").and_then(|h| h.as_str()).unwrap_or("");

        if !file.is_empty() {
            styled_card()
                .p_3()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(text_secondary_color())
                                .child("🔍 Source")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_monospace_color())
                                .child(format!("{}:{} on {}", file, line, hostname))
                        )
                )
        } else {
            div() // Empty div if no origin info
        }
    } else {
        div() // Empty div if no origin
    }
}