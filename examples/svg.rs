#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use dioxus_elements::AttributeValue;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

static FERRIS: &[u8] = include_bytes!("./ferris.svg");

fn app(cx: Scope) -> Element {
    cx.render(rsx!(svg {
        width: "100%",
        height: "100%",
        svg_data: AttributeValue::Bytes(FERRIS),
    }))
}
