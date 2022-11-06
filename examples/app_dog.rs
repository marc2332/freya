#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};
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
    Some(data.message.parse().ok()?)
}

async fn fetch_image(url: Url) -> Option<Vec<u8>> {
    let res = reqwest::get(url).await.ok()?;
    let data = res.bytes().await.ok()?;
    Some(data.to_vec())
}

fn app<'a>(cx: Scope<'a>) -> Element<'a> {
    let bytes = use_state(&cx, || None);
    let bytes_setter = bytes.setter();

    let fetch = move || {
        let bytes_setter = bytes_setter.clone();
        cx.spawn(async move {
            if let Some(url) = fetch_random_dog().await {
                if let Some(doggo) = fetch_image(url).await {
                    bytes_setter(Some(doggo))
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
                bytes.as_ref().map(|bytes| {
                    render!{
                        image {
                            width: "100%",
                            height: "100%",
                            image_data: &bytes
                        }
                    }
                })
            }
            container {
                padding: "20",
                height: "58",
                width: "100%",
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
