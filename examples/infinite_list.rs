#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use reqwest::{
    Client,
    Url,
};
use serde::Deserialize;

fn main() {
    launch_with_props(app, "Infinite List of Dogs", (500.0, 800.0));
}

async fn fetch_random_dog(client: &Client) -> Option<Url> {
    #[derive(Deserialize)]
    struct DogApiResponse {
        message: String,
    }

    let res = client
        .get("https://dog.ceo/api/breeds/image/random")
        .send()
        .await
        .ok()?;

    let data = res.json::<DogApiResponse>().await.ok()?;
    data.message.parse().ok()
}

fn app() -> Element {
    let client = use_signal(Client::new);
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
            spacing: "12",
            padding: "12",
            for i in 0..cards() {
                rect {
                    key: "{i}",
                    width: "fill",
                    spacing: "12",
                    content: "flex",
                    direction: "horizontal",
                    RandomImage { client }
                    RandomImage { client }
                }
            }
        }
    )
}

#[component]
fn RandomImage(client: ReadOnlySignal<Client>) -> Element {
    let url = use_resource(move || async move { fetch_random_dog(&client.read()).await });

    rsx!(
        rect {
            width: "flex",
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
