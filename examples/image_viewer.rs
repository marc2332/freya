#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use bytes::Bytes;
use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut image = use_signal::<Option<(Bytes, PathBuf)>>(|| None);

    let open_image = move |_| {
        spawn(async move {
            let file = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(file) = file {
                let file_content = tokio::fs::read(file.path()).await;
                if let Ok(file_content) = file_content {
                    image.set(Some((Bytes::from(file_content), file.path().into())));
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
                if let Some((bytes, path)) = &*image.read() {
                    image {
                        aspect_ratio: "min",
                        image_data: dynamic_bytes(bytes.clone()),
                        cache_key: "{path:?}"
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
