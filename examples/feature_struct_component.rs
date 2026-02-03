#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new_app(MyApp { value: 4 })))
}

struct MyApp {
    value: u8,
}

impl App for MyApp {
    fn render(&self) -> impl IntoElement {
        format!("Value is {}", self.value)
    }
}
