#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

fn app() -> impl IntoElement {
    rect()
        .center()
        .expanded()
        .spacing(4.)
        .children_iter((0..3).map(|_| {
            Accordion::new()
                .header("Click to expand!")
                .child(LOREM_IPSUM)
                .into()
        }))
}
