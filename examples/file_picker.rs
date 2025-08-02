#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_params(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let mut selection = use_signal(|| None);

    let onpress = move |_| async move {
        let file = rfd::AsyncFileDialog::new().pick_file().await;
        *selection.write() = file.map(|h| h.path().to_path_buf());
    };

    let text = selection
        .read()
        .as_ref()
        .and_then(|p| p.to_str())
        .unwrap_or("Pick")
        .to_owned();

    rsx!(
        Button {
            onpress,
            label { {text} }
        }

    )
}
