use gpui::prelude::*;
use gpui::{div, rgb, Div, FontWeight};

pub fn background_color() -> gpui::Hsla {
    rgb(0x1e1e1e).into()
}
pub fn panel_background_color() -> gpui::Hsla {
    rgb(0x252526).into()
}
pub fn card_background_color() -> gpui::Hsla {
    rgb(0x2d2d30).into()
}
pub fn border_color() -> gpui::Hsla {
    rgb(0x3e3e42).into()
}
pub fn text_primary_color() -> gpui::Hsla {
    rgb(0xcccccc).into()
}
pub fn text_secondary_color() -> gpui::Hsla {
    rgb(0x969696).into()
}
pub fn text_monospace_color() -> gpui::Hsla {
    rgb(0xd4d4d4).into()
}
pub fn accent_primary_color() -> gpui::Hsla {
    rgb(0x007acc).into()
}
pub fn accent_hover_color() -> gpui::Hsla {
    rgb(0x005a9e).into()
}
pub fn selection_color() -> gpui::Hsla {
    rgb(0x094771).into()
}
pub fn hover_color() -> gpui::Hsla {
    rgb(0x2a2d2e).into()
}

pub fn styled_button() -> Div {
    div()
        .px_3()
        .py_1()
        .bg(accent_primary_color())
        .rounded_md()
        .cursor_pointer()
        .hover(|style| style.bg(accent_hover_color()))
        .text_color(text_primary_color())
}

pub fn styled_card() -> Div {
    div()
        .p_4()
        .bg(card_background_color())
        .rounded_lg()
        .border_1()
        .border_color(border_color())
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

pub fn styled_monospace() -> Div {
    div()
        .font_family("monospace")
        .text_sm()
        .text_color(text_monospace_color())
}

pub fn copy_button(text: String) -> Div {
    div()
        .px_2()
        .py_1()
        .bg(rgb(0x3c3c3c))
        .rounded_sm()
        .cursor_pointer()
        .hover(|style| style.bg(rgb(0x4c4c4c)))
        .text_color(text_secondary_color())
        .text_xs()
        .child("Copy")
}
