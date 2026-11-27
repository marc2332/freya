#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let value = use_state(String::new);

    rect()
        .padding(20.)
        .spacing(12.)
        .child(
            ComboBox::new(
                value,
                vec![
                    "Rust".to_string(),
                    "Ruby".to_string(),
                    "Python".to_string(),
                    "TypeScript".to_string(),
                ],
            )
            .placeholder("Choose or type a language"),
        )
        .child(format!("Value: {}", value.read()))
}
