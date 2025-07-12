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

pub fn styled_card() -> Div {
    // No cards in minimal design, just spacing
    div().py_4()
}

pub fn copy_button() -> Div {
    div()
        .text_xs()
        .text_color(text_secondary_color())
        .cursor_pointer()
        .hover(|style| style.text_color(text_primary_color()))
        .child("copy raw payload")
}
