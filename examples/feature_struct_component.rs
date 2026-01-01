#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(WindowConfig::new(FpRender::from_render(App { value: 4 }))),
    )
}

struct App {
    value: u8,
}

impl Render for App {
    fn render(&self) -> impl IntoElement {
        format!("Value is {}", self.value)
    }
}
