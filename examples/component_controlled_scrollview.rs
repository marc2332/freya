#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let scroll_controller = use_scroll_controller(ScrollConfig::default);
    rect()
        .direction(Direction::Horizontal)
        .content(Content::Flex)
        .spacing(6.)
        .child(
            ScrollView::new_controlled(scroll_controller)
                .width(Size::flex(1.))
                .spacing(6.)
                .children(
                    (0..30).map(|_| rect().width(Size::fill()).background((182, 119, 0)).into()),
                ),
        )
        .child(
            ScrollView::new_controlled(scroll_controller)
                .width(Size::flex(1.))
                .spacing(6.)
                .children(
                    (0..30).map(|_| rect().width(Size::fill()).background((0, 119, 182)).into()),
                ),
        )
}
