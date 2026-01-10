#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use rfd::AsyncFileDialog;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    let mut pick = use_state::<Option<String>>(|| None);

    let on_pick = move |_| {
        spawn(async move {
            let file = AsyncFileDialog::new()
                .add_filter("Rust", &["rs"])
                .pick_file()
                .await;

            if let Some(file) = file {
                pick.set(Some(file.path().to_str().unwrap().to_string()));
            }
        });
    };

    rect()
        .expanded()
        .center()
        .child(Button::new().child("Pick file").on_press(on_pick))
        .child(pick.read().as_deref().unwrap_or("..."))
}
