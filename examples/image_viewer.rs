#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bytes::Bytes;
use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut image_bytes = use_signal::<Option<Bytes>>(|| None);
    let image_data = image_bytes
        .read()
        .as_ref()
        .map(|bytes| dynamic_bytes(bytes.clone()));

    let open_image = move |_| {
        spawn(async move {
            let file = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(file) = file {
                let file_content = tokio::fs::read(file.path()).await;
                if let Ok(file_content) = file_content {
                    image_bytes.set(Some(Bytes::from(file_content)));
                }
            }
        });
    };

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            padding: "16",
            main_align: "center",
            cross_align: "center",
            spacing: "20",
            rect {
                width: "90%",
                height: "90%",
                main_align: "center",
                cross_align: "center",
                if let Some(image_data) = image_data {
                    image {
                        width: "fill",
                        height: "fill",
                        image_data
                    }
                } else {
                    label {
                        "..."
                    }
                }
            }
            Button {
                onpress: open_image,
                label {
                    "Open Image"
                }
            }
        }
    )
}
