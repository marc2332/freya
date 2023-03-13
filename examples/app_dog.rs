#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use reqwest::Url;
use serde::Deserialize;

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

fn app(cx: Scope) -> Element {
    let mut degrees = use_state(cx, || 0);
    let bytes = use_state(cx, || None);

    let fetch = move || {
        to_owned![bytes];
        cx.spawn(async move {
            if let Some(url) = fetch_random_dog().await {
                if let Some(doggo) = fetch_image(url).await {
                    bytes.set(Some(doggo))
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
                rotate: "{degrees}",
                bytes.as_ref().map(|bytes| {
                    let image_data = bytes_to_data(cx, bytes);
                    render!{
                        image {
                            width: "100%",
                            height: "100%",
                            image_data: image_data
                        }
                    }
                })
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
                Button {
                    onclick: move |_| degrees += 15,
                    label {
                        "Rotate right"
                    }
                }
                Button {
                    onclick: move |_| degrees -= 15,
                    label {
                        "Rotate left"
                    }
                }
            }
        }
    )
}
