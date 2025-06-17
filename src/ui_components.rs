use gpui::prelude::*;
use gpui::{div, rgb, Div, FontWeight};

// Minimalist color palette inspired by shadcn
pub fn background_color() -> gpui::Hsla {
    rgb(0x09090b).into() // zinc-950
}
pub fn panel_background_color() -> gpui::Hsla {
    rgb(0x09090b).into() // Same as background for minimal look
}
pub fn card_background_color() -> gpui::Hsla {
    rgb(0x09090b).into() // No cards, same as background
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
pub fn accent_primary_color() -> gpui::Hsla {
    rgb(0xfafafa).into() // white text for minimal buttons
}
pub fn accent_hover_color() -> gpui::Hsla {
    rgb(0xe4e4e7).into() // zinc-200 on hover
}
pub fn selection_color() -> gpui::Hsla {
    rgb(0x18181b).into() // zinc-900
}
pub fn hover_color() -> gpui::Hsla {
    rgb(0x18181b).into() // zinc-900 - subtle hover
}

pub fn styled_button() -> Div {
    div()
        .px_2()
        .py_1()
        .text_sm()
        .text_color(text_secondary_color())
        .cursor_pointer()
        .hover(|style| style.text_color(text_primary_color()))
}

pub fn styled_card() -> Div {
    // No cards in minimal design, just spacing
    div()
        .py_4()
}

pub fn styled_label() -> Div {
    div().text_sm().text_color(text_secondary_color())
}

pub fn styled_value() -> Div {
    div()
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(text_primary_color())
}

pub fn copy_button(_text: String) -> Div {
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .cursor_pointer()
        .hover(|style| style.text_color(text_primary_color()))
        .child("copy")
}
