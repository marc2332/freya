#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let quantity = use_state(|| 2);
    let price = use_state(|| 4.5);

    let total = quantity() as f64 * price();

    rect()
        .center()
        .expanded()
        .spacing(8.)
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .cross_align(Alignment::center())
                .child("Quantity")
                .child(Input::new(quantity).placeholder("Quantity")),
        )
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .cross_align(Alignment::center())
                .child("Price")
                .child(Input::new(price).placeholder("Price")),
        )
        .child(format!("Total: {total:.2}"))
}
