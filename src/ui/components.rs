use gpui::prelude::*;
use gpui::{div, rgb, Div};

// Minimalist color palette inspired by shadcn
pub fn background_color() -> gpui::Hsla {
    rgb(0x09090b).into() // zinc-950
}
pub fn border_color() -> gpui::Hsla {
    rgb(0x27272a).into() // zinc-800 - subtle when needed
}
pub fn text_primary_color() -> gpui::Hsla {
    rgb(0xfafafa).into() // zinc-50
}
pub fn text_secondary_color() -> gpui::Hsla {
    rgb(0xa1a1aa).into() // zinc-400
}
pub fn text_monospace_color() -> gpui::Hsla {
    rgb(0xe4e4e7).into() // zinc-200
}
pub fn selection_color() -> gpui::Hsla {
    rgb(0x18181b).into() // zinc-900
}
pub fn hover_color() -> gpui::Hsla {
    rgb(0x18181b).into() // zinc-900 - subtle hover
}


pub fn copy_button() -> Div {
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .cursor_pointer()
        .hover(|style| style.text_color(text_primary_color()))
        .child("copy raw payload")
}

pub fn section_header(label: String) -> Div {
    div()
        .text_xs()
        .font_weight(gpui::FontWeight::MEDIUM)
        .text_color(text_secondary_color())
        .child(label)
}

pub fn code_box(content: impl IntoElement) -> Div {
    div()
        .p_4()
        .rounded_md()
        .bg(rgb(0x18181b))
        .border_1()
        .border_color(border_color())
        .child(
            div()
                .text_xs()
                .font_family("monospace")
                .text_color(text_primary_color())
                .child(content),
        )
}

pub fn origin_info(file: String, line: String, hostname: Option<String>) -> Div {
    let mut text = format!("{file}:{line}");
    if let Some(host) = hostname {
        text = format!("{text} • {host}");
    }
    
    div()
        .flex()
        .items_center()
        .gap_2()
        .text_xs()
        .text_color(text_secondary_color())
        .opacity(0.7)
        .child(text)
}

pub fn metric_row(label: String, value: impl IntoElement) -> Div {
    div()
        .flex()
        .gap_2()
        .child(div().text_color(text_secondary_color()).child(format!("{label}:")))
        .child(
            div()
                .font_family("monospace")
                .text_color(text_primary_color())
                .child(value),
        )
}
