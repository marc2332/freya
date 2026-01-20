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
        .center()
        .expanded()
        .horizontal()
        .spacing(16.)
        .child(rect().spacing(12.).children(filled_cards()))
        .child(rect().spacing(12.).children(outline_cards()))
}

fn filled_cards() -> [Element; 4] {
    [
        Card::new().child("Filled Normal").into(),
        Card::new().compact().child("Filled Compact").into(),
        Card::new().hoverable(true).child("Filled Hoverable").into(),
        Card::new()
            .hoverable(true)
            .on_press(|_| println!("Filled card pressed!"))
            .child("Filled Clickable")
            .into(),
    ]
}

fn outline_cards() -> [Element; 4] {
    [
        Card::new().outline().child("Outline Normal").into(),
        Card::new()
            .outline()
            .compact()
            .child("Outline Compact")
            .into(),
        Card::new()
            .outline()
            .hoverable(true)
            .child("Outline Hoverable")
            .into(),
        Card::new()
            .outline()
            .hoverable(true)
            .on_press(|_| println!("Outline card pressed!"))
            .width(Size::px(200.))
            .child("Outline Clickable")
            .into(),
    ]
}
