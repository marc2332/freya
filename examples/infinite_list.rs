#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use reqwest::Url;
use serde::Deserialize;

fn main() {
    launch_with_props(app, "Infinite List of Dogs", (500.0, 800.0));
}

async fn fetch_random_dog() -> Option<Url> {
    #[derive(Deserialize)]
    struct DogApiResponse {
        message: String,
    }

    let res = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .ok()?;
    let data = res.json::<DogApiResponse>().await.ok()?;
    data.message.parse().ok()
}

fn app() -> Element {
    let scroll_controller = use_scroll_controller(ScrollConfig::default);
    let mut cards = use_signal(|| 5);

    use_effect(move || {
        let y = scroll_controller.y();
        let layout = scroll_controller.layout();
        let y = y.read();
        let layout = layout.read();
        let end = layout.inner.height - layout.area.height();
        const MARGIN: i32 = 50;
        if -*y > end as i32 - MARGIN {
            *cards.write() += 1;
        }
    });

    rsx!(
        ScrollView {
            scroll_controller,
            spacing: "8",
            padding: "8",
            for i in 0..cards() {
                rect {
                    key: "{i}",
                    width: "100%",
                    spacing: "8",
                    direction: "horizontal",
                    RandomImage {}
                    RandomImage {}
                }
            }
        }
    )
}

#[component]
fn RandomImage() -> Element {
    let url = use_resource(|| async move { fetch_random_dog().await });

    rsx!(
        rect {
            width: "50%",
            height: "300",
            overflow: "clip",
            corner_radius: "8",
            if let Some(url) = url.read().clone().flatten() {
                NetworkImage {
                    width: "fill",
                    height: "fill",
                    aspect_ratio: "max",
                    url
                }
            }
        }
    )
}
