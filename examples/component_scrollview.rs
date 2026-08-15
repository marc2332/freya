#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .child(ScrollView::new().height(Size::percent(50.)).child(
            rect().spacing(6.).padding(6.).children((0..30).map(|i| {
                Button::new()
                    .key(i)
                    .background((182, 119, 0))
                    .hover_background((222, 159, 40))
                    .child(format!("Item {i}"))
                    .into()
            })),
        ))
        .child(
            ScrollView::new().height(Size::percent(50.)).child(
                rect()
                    .direction(Direction::Horizontal)
                    .spacing(6.)
                    .padding(6.)
                    .children((0..30).map(|i| {
                        Button::new()
                            .key(i)
                            .background((0, 119, 182))
                            .hover_background((40, 159, 222))
                            .child(label().text(format!("Item {i}")).max_lines(1))
                            .into()
                    })),
            ),
        )
}
