#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut show_popup = use_state(|| true);

    rect()
        .maybe_child(show_popup().then(|| {
            Popup::new()
                .on_close_request(move |_| show_popup.set(false))
                .child(PopupTitle::new("Title".to_string()))
                .child(PopupContent::new().child("Hello, World!"))
                .child(
                    PopupButtons::new().child(
                        Button::new()
                            .on_press(move |_| show_popup.set(false))
                            .expanded()
                            .filled()
                            .child("Accept"),
                    ),
                )
        }))
        .child(
            Button::new()
                .child("Open")
                .on_press(move |_| show_popup.toggle()),
        )
}
