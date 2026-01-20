#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let text = "Hello cursor styles!";

    rect()
        .spacing(10.0)
        .padding(20.0)
        .child(
            paragraph()
                .span(Span::new(text).font_size(20.0))
                .cursor_index(1)
                .cursor_style(CursorStyle::Line)
                .cursor_color((60, 63, 67)),
        )
        .child(
            paragraph()
                .span(Span::new(text).font_size(20.0))
                .cursor_index(7)
                .cursor_style(CursorStyle::Block)
                .cursor_color((88, 101, 242)),
        )
        .child(
            paragraph()
                .span(Span::new(text).font_size(20.0))
                .cursor_index(11)
                .cursor_style(CursorStyle::Underline)
                .cursor_color((219, 68, 55)),
        )
}
