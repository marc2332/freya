#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let values = use_hook(|| {
        vec![
            "Rust".to_string(),
            "Turbofish".to_string(),
            "Crabs".to_string(),
        ]
    });
    let mut selected_select = use_state(|| 0);

    rect().center().expanded().horizontal().spacing(6.).child(
        Select::new()
            .selected_item(values[selected_select()].to_string())
            .children(values.iter().enumerate().map(|(i, val)| {
                MenuItem::new()
                    .selected(selected_select() == i)
                    .on_press(move |_| selected_select.set(i))
                    .child(val.to_string())
                    .into()
            })),
    )
}
