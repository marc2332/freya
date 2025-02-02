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

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let mut dog_url = use_signal(|| None);

    let onpress = move |_| {
        spawn(async move {
            if let Some(url) = fetch_random_dog().await {
                dog_url.set(Some(url))
            }
        });
    };

    rsx!(
        rect {
            background: "rgb(15, 15, 15)",
            width: "fill",
            height: "fill",
            content: "flex",
            rect {
                width: "100%",
                height: "flex(1)",
                main_align: "center",
                cross_align: "center",
                overflow: "clip",
                if let Some(url) = dog_url() {
                    NetworkImage {
                        url,
                        min_width: "300",
                        min_height: "300",
                    }
                }
            }
            rect {
                width: "fill",
                main_align: "center",
                cross_align: "center",
                padding: "10 0",
                Button {
                    onpress,
                    label {
                        "Fetch random Doggo!"
                    }
                }
            }
        }
    )
}
