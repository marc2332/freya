#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn cards() -> [Element; 3] {
    [1, 2, 3].map(|index| {
        Button::new()
            .on_press(move |_| println!("Pressed item {index}"))
            .child(format!("Item {index}"))
            .into_element()
    })
}

fn row(title: &str, content: Element) -> Element {
    rect()
        .width(Size::fill())
        .spacing(5.)
        .child(title.to_string())
        .child(content)
        .into()
}

fn app() -> impl IntoElement {
    ScrollView::new()
        .spacing(20.)
        .child(row(
            "horizontal",
            rect().horizontal().spacing(5.).children(cards()).into(),
        ))
        .child(row(
            "horizontal + reversed",
            rect()
                .horizontal()
                .backward_order()
                .spacing(5.)
                .children(cards())
                .into(),
        ))
        .child(row(
            "vertical",
            rect().vertical().spacing(5.).children(cards()).into(),
        ))
        .child(row(
            "vertical + reversed",
            rect()
                .vertical()
                .backward_order()
                .spacing(5.)
                .children(cards())
                .into(),
        ))
}
