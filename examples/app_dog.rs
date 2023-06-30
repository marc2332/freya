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

fn app(cx: Scope) -> Element {
    use_init_theme(cx, DARK_THEME);
    let dog_url = use_state(cx, || None);

    let fetch = move || {
        to_owned![dog_url];
        cx.spawn(async move {
            if let Some(url) = fetch_random_dog().await {
                dog_url.set(Some(url))
            }
        })
    };

    render!(
        rect {
            overflow: "clip",
            background: "rgb(15, 15, 15)",
            width: "100%",
            height: "100%",
            color: "white",
            rect {
                overflow: "clip",
                width: "100%",
                height: "calc(100% - 58)",
                corner_radius: "25",
                if let Some(dog_url) = dog_url.get() {
                   rsx!(
                        NetworkImage {
                            url: dog_url.clone()
                        }
                   )
                }
            }
            rect {
                overflow: "clip",
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
