#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

/// Demonstrates absolute positioning with inner-sized elements.
/// All four colored boxes should be pinned to their respective corners
/// of the outer gray container, regardless of whether their size
/// is explicit or determined by their children.
fn app() -> impl IntoElement {
    rect().expanded().center().child(
        rect()
            .background((60, 60, 60))
            .color(Color::WHITE)
            .width(Size::percent(80.))
            .height(Size::percent(80.))
            .child(
                rect()
                    .background((15, 163, 242))
                    .width(Size::px(80.))
                    .height(Size::px(80.))
                    .position(Position::new_absolute().top(0.).left(0.))
                    .child("Fixed sized"),
            )
            .child(
                rect()
                    .background((15, 163, 242))
                    .position(Position::new_absolute().bottom(0.).right(0.))
                    .child("Auto size"),
            ),
    )
}
