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
    use_init_theme(DARK_THEME);
    let mut dog_url = use_signal(|| None);

    let onclick = move |_| {
        spawn(async move {
            if let Some(url) = fetch_random_dog().await {
                dog_url.set(Some(url))
            }
        });
    };

    rsx!(
        rect {
            background: "rgb(15, 15, 15)",
            width: "100%",
            height: "100%",
            rect {
                overflow: "clip",
                width: "100%",
                height: "calc(100% - 60)",
                {dog_url.read().as_ref().map(|dog_url| rsx!(
                    NetworkImage {
                        url: dog_url.clone()
                    }
               ))}
            }
            rect {
                overflow: "clip",
                height: "60",
                width: "100%",
                main_align: "center",
                cross_align: "center",
                Button {
                    onclick,
                    label {
                        "Fetch random Doggo!"
                    }
                }
            }
        }
    )
}
