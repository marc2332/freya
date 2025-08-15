#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bytes::Bytes;
use freya::prelude::*;
use reqwest::Url;

#[cfg(not(feature = "remote-gif"))]
fn main() {
    panic!("Run with the 'remote-gif' feature");
}

#[cfg(feature = "remote-gif")]
fn main() {
    launch_with_props(app, "GIF", (960.0, 560.0));
}

static GIF: Bytes = Bytes::from_static(include_bytes!("./giphy.gif"));
static URL: &str = "https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExcnRiYTBtNDNydHpxbW80ZWJ3Zmg5cWtrNzZ5cm1vZTBjdTBscHg4aiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/CebmdRpDGykxqz8GbN/giphy.gif";

fn app() -> Element {
    rsx!(
        Gif {
            width: "fill",
            height: "50%",
            data: GIF.clone()
        }
        RemoteGif {
            width: "fill",
            height: "50%",
            url: URL.parse::<Url>().unwrap()
        }
    )
}
