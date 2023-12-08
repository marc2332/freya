#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        ExternalLink { url: "https://duckduckgo.com/", label { font_size: "25", "https://duckduckgo.com/" } }
        ExternalLink { url: "https://www.google.com/", label { font_size: "25", "https://www.google.com/" } }
        ExternalLink { url: "https://github.com/marc2332/freya", show_tooltip: false, label { font_size: "25", "Freya Source Code (no tooltip)" } }
    )
}
