#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().expanded().center().child(
        SvgViewer::new(include_bytes!("./ferris.svg"))
            .width(Size::px(300.))
            .height(Size::px(300.)),
    )
}
