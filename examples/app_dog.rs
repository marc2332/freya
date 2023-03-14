#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use reqwest::Url;
use serde::Deserialize;
use tokio::time::sleep;

fn main() {
    launch(app);
}

#[derive(Deserialize)]
struct DogApiResponse {
    message: String,
}

async fn fetch_random_dog() -> Option<Url> {
    let res = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .ok()?;
    let data = res.json::<DogApiResponse>().await.ok()?;
    data.message.parse().ok()
}

async fn fetch_image(url: Url) -> Option<Vec<u8>> {
    let res = reqwest::get(url).await.ok()?;
    let data = res.bytes().await.ok()?;
    Some(data.to_vec())
}

#[derive(PartialEq)]
pub enum ImageStatus {
    Loading,
    Stopped,
    Loaded,
}



fn app(cx: Scope) -> Element {
    let status = use_state(cx, || ImageStatus::Stopped);
    let image_bytes = use_state::<Option<Vec<u8>>>(cx, || None);

    let fetch = move || {
        to_owned![image_bytes, status];
        cx.spawn(async move {
            if let Some(url) = fetch_random_dog().await {
                status.set(ImageStatus::Loading);
                let img = fetch_image(url).await;
                sleep(Duration::from_millis(1000)).await;
                if let Some(img) = img {
                    // Image loaded
                    image_bytes.set(Some(img));
                    status.set(ImageStatus::Loaded)
                }
            }
        })
    };

    render!(
        container {
            background: "rgb(15, 15, 15)",
            width: "100%",
            height: "100%",
            container {
                width: "100%",
                height: "calc(100% - 58)",
                radius: "25",
                display: "center",
                direction: "both",
                if *status.get() == ImageStatus::Loading {
                    rsx!(
                        Loader {}
                    )
                }else if *status.get() == ImageStatus::Loaded {
                    rsx!{
                        image_bytes.as_ref().map(|bytes| {
                            let image_data = bytes_to_data(cx, bytes);
                            rsx!(
                                image {
                                    width: "100%",
                                    height: "100%",
                                    image_data: image_data
                                }
                            )
                        })
                    }
                }
            }
            container {
                padding: "10",
                height: "58",
                width: "100%",
                direction: "horizontal",
                Button {
                    onclick: move |_|  fetch(),
                    label {
                        "Fetch random Doggo!"
                    }
                }
            }
        }
    )
}
