#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    dioxus_core::AttributeValue,
    prelude::*,
};

fn main() {
    launch(app);
}

fn app() -> Element {
    // Each item in the vec consists of a range to highlight
    let highlights =
        AttributeValue::any_value(CustomAttributeValues::TextHighlights(vec![(0, 5), (7, 13)]));

    rsx!(
        paragraph {
            highlights,
            text {
                "Hello, World!!!!"
            }
        }
    )
}
