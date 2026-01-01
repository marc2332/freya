#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use freya_edit::Clipboard;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    let on_secondary_press = move |_| {
        ContextMenu::open(
            Menu::new().child(
                MenuItem::new()
                    .on_press(move |_| {
                        let _ = Clipboard::set("Right Click Me".to_string());
                    })
                    .child("Copy"),
            ),
        );
    };

    rect().expanded().center().child(
        label()
            .text("Right click to copy")
            .on_secondary_press(on_secondary_press),
    )
}
